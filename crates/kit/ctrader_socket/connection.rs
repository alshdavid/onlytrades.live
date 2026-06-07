use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use kit_ctrader_proto::ProtoMessage;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::unbounded_channel;

use crate::CTraderRequestType;
use crate::CTraderResponseType;
use crate::VersionReq;
use crate::connection_proto::CTraderConnectionRaw;
use crate::connection_proto::CTraderConnectionRawOptions;
use crate::connection_proto::ProtoError;

pub struct CTraderConnectionOptions {
  pub live: bool,
}

#[derive(Clone, Debug)]
pub struct CTraderConnection {
  conn: Arc<CTraderConnectionRaw>,
}

impl CTraderConnection {
  pub async fn connect(options: CTraderConnectionOptions) -> anyhow::Result<Self> {
    let conn =
      CTraderConnectionRaw::connect(CTraderConnectionRawOptions { live: options.live }).await?;

    Ok(Self {
      conn: Arc::new(conn),
    })
  }
}

impl CTraderConnectionExt for CTraderConnection {
  fn subscribe(
    &self
  ) -> impl Future<Output = UnboundedReceiver<Result<CTraderResponseType, ProtoError>>> + Send + Sync
  {
    return async {
      let (tx, rx) = unbounded_channel::<Result<CTraderResponseType, ProtoError>>();

      let mut rx_original = self.conn.subscribe().await;

      tokio::task::spawn(async move {
        while let Some(msg) = rx_original.recv().await {
          let _ = tx.send(CTraderResponseType::try_from(msg));
        }
      });

      rx
    };
  }

  fn send(
    &self,
    message: CTraderRequestType,
  ) -> impl Future<Output = Result<(), ProtoError>> + Send + Sync {
    async { self.conn.send(Into::<ProtoMessage>::into(message)).await }
  }
}

fn timestamp() -> std::result::Result<u128, ProtoError> {
  let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) else {
    return Err(ProtoError::TimeError);
  };
  Ok(timestamp.as_millis())
}

pub trait CTraderConnectionExt: Send + Sync {
  fn subscribe(
    &self
  ) -> impl Future<Output = UnboundedReceiver<Result<CTraderResponseType, ProtoError>>> + Send + Sync;

  fn send(
    &self,
    message: CTraderRequestType,
  ) -> impl Future<Output = Result<(), ProtoError>> + Send + Sync;

  fn latency(&self) -> impl Future<Output = Result<u128, ProtoError>> + Send {
    async {
      let now_ms = timestamp()?;

      let id = format!("::internal::latency::{}::", now_ms);

      let message = CTraderRequestType::VersionReq(VersionReq {
        client_msg_id: Some(id.clone()),
      });

      let mut rx = self.subscribe().await;

      let _ = self.send(message).await;

      while let Some(msg) = rx.recv().await {
        match msg {
          Ok(CTraderResponseType::VersionRes(res)) => {
            if res.client_msg_id.as_ref().is_some_and(|m| m == &id) {
              let recv_ms = timestamp()?;
              return Ok(recv_ms - now_ms);
            }
          }
          Ok(_) => {}
          Err(_) => {}
        }
      }

      // Todo implement timeout
      Err(ProtoError::SocketTimeout)
    }
  }
}
