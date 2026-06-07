// #![deny(unused_crate_dependencies)]
mod ctrader_connection_manager;
mod ctrader_service;
mod new_order;
mod symbols_list;

// pub use self::ctrader_connection_manager::*;
// pub use self::ctrader_service::*;
pub use self::ctrader_service_2::*;

mod ctrader_service_2 {
  use std::sync::Arc;

  use anyhow::Context;
  use chrono::TimeDelta;
  use chrono::Utc;
  use kit_ctrader_rest::client::CTraderRestClient;
  use kit_ctrader_socket::AccountAuthReq;
  use kit_ctrader_socket::ApplicationAuthReq;
  use kit_ctrader_socket::CTraderRequestType;
  use kit_ctrader_socket::CTraderResponseType;
  use kit_ctrader_socket::connection::CTraderConnection;
  use kit_ctrader_socket::connection::CTraderConnectionExt;
  use kit_ctrader_socket::connection::CTraderConnectionOptions;
  use platform_models::CtraderTokenModel;
  use platform_repository_turso::CtraderAccountRepository;
  use platform_repository_turso::CtraderTokenRepository;
  use uuid::Uuid;

  pub struct CTraderService {
    ctrader_client_id: String,
    ctrader_client_secret: String,
    ctrader_rest_client: Arc<CTraderRestClient>,
    ctrader_token_repository: Arc<CtraderTokenRepository>,
    ctrader_account_repository: Arc<CtraderAccountRepository>,
  }

  impl CTraderService {
    pub fn new(
      ctrader_client_id: &str,
      ctrader_client_secret: &str,
      ctrader_rest_client: &Arc<CTraderRestClient>,
      ctrader_token_repository: &Arc<CtraderTokenRepository>,
      ctrader_account_repository: &Arc<CtraderAccountRepository>,
    ) -> Self {
      Self {
        ctrader_client_id: ctrader_client_id.to_string(),
        ctrader_client_secret: ctrader_client_secret.to_string(),
        ctrader_rest_client: Arc::clone(&ctrader_rest_client),
        ctrader_token_repository: Arc::clone(&ctrader_token_repository),
        ctrader_account_repository: Arc::clone(&ctrader_account_repository),
      }
    }

    pub async fn new_connection(
      &self,
      profile_id: &Uuid,
      account_id: &i64,
    ) -> anyhow::Result<CTraderConnection> {
      let ctrader_tokens = self
        .ctrader_token_repository
        .get_tokens_for_profile(profile_id)
        .await?
        .context("Profile has no ctrader tokens")?;

      let mut ctrader_access_token = ctrader_tokens.access_token;

      if self
        .ctrader_rest_client
        .has_expired(&ctrader_tokens.access_token_expires_at)
      {
        let ctrader_tokens = self
          .ctrader_rest_client
          .oauth_refresh(&ctrader_tokens.refresh_token)
          .await?;

        self
          .ctrader_token_repository
          .upsert(
            &CtraderTokenModel {
              token_id: Uuid::now_v7(),
              refresh_token: ctrader_tokens.refresh_token.clone(),
              access_token: ctrader_tokens.access_token.clone(),
              access_token_expires_at: Utc::now()
                + TimeDelta::seconds(ctrader_tokens.expires_in as i64),
            },
            profile_id,
          )
          .await?;

        ctrader_access_token = ctrader_tokens.access_token;
      }

      let account = self
        .ctrader_account_repository
        .find_by_profile_id(&profile_id)
        .await
        .context("No accounts for that profile")?
        .into_iter()
        .find(|a| &a.account_id == account_id)
        .context("Account not in profile")?;

      let conn = CTraderConnection::connect(CTraderConnectionOptions { live: account.live })
        .await
        .context("Unable to connect to ctrader socket")?;

      let mut rx = conn.subscribe().await;

      conn
        .send(CTraderRequestType::ApplicationAuthReq(ApplicationAuthReq {
          client_msg_id: None,
          client_id: self.ctrader_client_id.clone(),
          client_secret: self.ctrader_client_secret.clone(),
        }))
        .await?;

      match rx.recv().await.context("connection ended unexpectedly")? {
        Ok(CTraderResponseType::ApplicationAuthRes(_)) => {}
        Ok(_) => return Err(anyhow::anyhow!("Invalid response")),
        Err(err) => return Err(anyhow::anyhow!(err)),
      };

      conn
        .send(CTraderRequestType::AccountAuthReq(AccountAuthReq {
          client_msg_id: None,
          ctid_trader_account_id: account.account_id.clone(),
          access_token: ctrader_access_token,
        }))
        .await?;

      match rx.recv().await.context("connection ended unexpectedly")? {
        Ok(CTraderResponseType::AccountAuthRes(_)) => {}
        Ok(_) => return Err(anyhow::anyhow!("Invalid response")),
        Err(err) => return Err(anyhow::anyhow!(err)),
      };

      drop(rx);

      Ok(conn)
    }
  }
}
