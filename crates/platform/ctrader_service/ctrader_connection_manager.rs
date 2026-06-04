use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use kit_ctrader::CTraderSocketClient;
use kit_ctrader::CTraderSocketClientOptions;
use kit_ctrader::{self};
use tokio::sync::Mutex;

struct CTraderManagedConnection {
  last_used: chrono::DateTime<Utc>,
  conn: Arc<CTraderSocketClient>,
}

pub struct CTraderConnectionManager {
  ctrader_client_id: String,
  ctrader_client_secret: String,
  connections: Arc<Mutex<HashMap<String, CTraderManagedConnection>>>,
  gc_handle: tokio::task::JoinHandle<()>,
}

impl CTraderConnectionManager {
  pub fn new(
    ctrader_client_id: &str,
    ctrader_client_secret: &str,
  ) -> Self {
    let connections = Arc::new(Mutex::new(
      HashMap::<String, CTraderManagedConnection>::new(),
    ));

    // tokio::task::spawn({
    //   let connections = Arc::clone(&connections);

    //   async move {
    //     loop {
    //       tokio::time::sleep(Duration::from_secs(5)).await;
    //       let conns = connections.lock().await;
    //       dbg!(&conns.keys());
    //     }
    //   }
    // });

    // Shutdown unused sockets after 5 minute of inactivity
    let gc_handle = tokio::task::spawn({
      let connections = Arc::clone(&connections);

      async move {
        loop {
          tokio::time::sleep(Duration::from_secs(5)).await;
          let now = Utc::now();
          let expiration_threshold = now - chrono::Duration::seconds(15);
          let mut conns = connections.lock().await;

          conns.retain(|_account_id, managed_conn| managed_conn.last_used > expiration_threshold);
        }
      }
    });

    Self {
      connections,
      ctrader_client_id: ctrader_client_id.to_string(),
      ctrader_client_secret: ctrader_client_secret.to_string(),
      gc_handle,
    }
  }

  pub(super) async fn get_or_connect(
    &self,
    connection_name: &str,
    account_id: &i64,
    account_live: bool,
    ctrader_access_token: &str,
  ) -> anyhow::Result<Arc<CTraderSocketClient>> {
    let mut ctrader_socket_connections = self.connections.lock().await;
    if let Some(conn) = ctrader_socket_connections.get_mut(connection_name) {
      conn.last_used = Utc::now();
      return Ok(Arc::clone(&conn.conn));
    }

    let con_options = CTraderSocketClientOptions { live: account_live };
    let conn = Arc::new(CTraderSocketClient::connect(con_options).await?);

    conn
      .send_and_receive_oneshot::<kit_ctrader_proto::ProtoOaApplicationAuthRes>(
        kit_ctrader_proto::ProtoOaPayloadType::ProtoOaApplicationAuthReq,
        kit_ctrader_proto::ProtoOaApplicationAuthReq {
          payload_type: Some(
            kit_ctrader_proto::ProtoOaPayloadType::ProtoOaApplicationAuthReq.into(),
          ),
          client_id: self.ctrader_client_id.clone(),
          client_secret: self.ctrader_client_secret.clone(),
        },
      )
      .await?;

    conn
      .send_and_receive_oneshot::<kit_ctrader_proto::ProtoOaAccountAuthRes>(
        kit_ctrader_proto::ProtoOaPayloadType::ProtoOaAccountAuthReq,
        kit_ctrader_proto::ProtoOaAccountAuthReq {
          payload_type: Some(
            kit_ctrader_proto::ProtoOaPayloadType::ProtoOaAccountAuthReq.into(),
          ),
          ctid_trader_account_id: *account_id,
          access_token: ctrader_access_token.to_string(),
        },
      )
      .await?;

    let managed_con = CTraderManagedConnection {
      last_used: chrono::Utc::now(),
      conn: Arc::clone(&conn),
    };
    ctrader_socket_connections.insert(connection_name.to_string(), managed_con);

    Ok(conn)
  }
}

impl Drop for CTraderConnectionManager {
  fn drop(&mut self) {
    self.gc_handle.abort();
  }
}
