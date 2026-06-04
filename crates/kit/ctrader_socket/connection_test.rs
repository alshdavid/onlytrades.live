/**
 * This is an integration test which also serves as documentation on
 * how to connect to the ctrader socket server and how to properly
 * authenticate an account
 */
use std::env;

use tokio::sync::mpsc::unbounded_channel;

use crate::AccountAuthReq;
use crate::ApplicationAuthReq;
use crate::CTraderResponseType;
use crate::ReconcileReq;
use crate::ReconcileRes;
use crate::connection::*;

#[derive(Debug, Clone)]
pub struct Env {
  pub ctrader_client_id: String,
  pub ctrader_client_secret: String,
  pub ctrader_access_token: String,
  pub ctrader_account_id: i64,
  pub ctrader_account_live: bool,
}

impl Env {
  pub fn from_env() -> anyhow::Result<Self> {
    Ok(Self {
      ctrader_client_id: env::var("CTRADER_CLIENT_ID")?,
      ctrader_client_secret: env::var("CTRADER_CLIENT_SECRET")?,
      // Obtain these from the CTrader Rest API
      ctrader_access_token: env::var("CTRADER_ACCESS_TOKEN")?,
      ctrader_account_id: env::var("CTRADER_ACCOUNT_ID")?.parse()?,
      ctrader_account_live: env::var("CTRADER_ACCOUNT_LIVE")? == "true",
    })
  }
}

#[tokio::test]
async fn should_authenticate() -> anyhow::Result<()> {
  let Ok(env) = Env::from_env() else {
    println!(">>> SKIPPING TEST: RUN_INTEGRATION_TESTS env var not set.");
    return Ok(());
  };

  // Create a connection
  let connection = CTraderConnection::connect(CTraderConnectionOptions {
    live: env.ctrader_account_live,
  })
  .await?;

  let (tx_application_auth, mut rx_application_auth) = unbounded_channel::<()>();
  let (tx_account_auth, mut rx_account_auth) = unbounded_channel::<()>();
  let (tx_reconcile, mut rx_reconcile) = unbounded_channel::<ReconcileRes>();

  tokio::task::spawn({
    let mut rx = connection.subscribe().await;

    async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Ok(CTraderResponseType::ApplicationAuthRes(res)) => {
            assert!(
              res.client_msg_id.unwrap_or_default() == "1",
              "Did not get the correct client message"
            );
            let _ = tx_application_auth.send(());
          }
          Ok(CTraderResponseType::AccountAuthRes(res)) => {
            assert!(
              res.client_msg_id.unwrap_or_default() == "2",
              "Did not get the correct client message"
            );
            let _ = tx_account_auth.send(());
          }
          Ok(CTraderResponseType::ReconcileRes(res)) => {
            assert!(
              res.client_msg_id.clone().unwrap_or_default() == "3",
              "Did not get the correct client message"
            );
            let _ = tx_reconcile.send(res);
          }
          Ok(_) => panic!("Should not get anything else"),
          Err(err) => panic!("Failed to parse payload type: {:?}", err),
        }
      }
    }
  });

  connection
    .send(crate::CTraderRequestType::ApplicationAuthReq(
      ApplicationAuthReq {
        client_msg_id: Some("1".to_string()),
        client_id: env.ctrader_client_id.clone(),
        client_secret: env.ctrader_client_secret.clone(),
      },
    ))
    .await?;

  rx_application_auth.recv().await;

  connection
    .send(crate::CTraderRequestType::AccountAuthReq(AccountAuthReq {
      client_msg_id: Some("2".to_string()),
      ctid_trader_account_id: env.ctrader_account_id.clone(),
      access_token: env.ctrader_access_token.clone(),
    }))
    .await?;

  rx_account_auth.recv().await;

  connection
    .send(crate::CTraderRequestType::ReconcileReq(ReconcileReq {
      client_msg_id: Some("3".to_string()),
      ctid_trader_account_id: env.ctrader_account_id.clone(),
      return_protection_orders: None,
    }))
    .await?;

  let reconcile_res = rx_reconcile.recv().await;

  let Some(reconcile_res) = reconcile_res else {
    panic!("Failed to get body");
  };

  assert!(
    reconcile_res.ctid_trader_account_id == env.ctrader_account_id,
    "Incorrect response"
  );

  Ok(())
}
