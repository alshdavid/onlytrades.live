use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::unbounded_channel;

use crate::CTraderResponseType;
use crate::connection_proto::CTraderConnectionRaw;
use crate::connection_proto::CTraderConnectionRawOptions;
use crate::connection_proto::ProtoError;

pub struct CTraderConnectionOptions {
  pub live: bool,
}

pub struct CTraderConnection {
  conn: CTraderConnectionRaw,
}

impl CTraderConnection {
  pub async fn connect(options: CTraderConnectionOptions) -> anyhow::Result<Self> {
    let conn =
      CTraderConnectionRaw::connect(CTraderConnectionRawOptions { live: options.live }).await?;

    Ok(Self { conn })
  }

  pub async fn subscribe(&self) -> UnboundedReceiver<Result<CTraderResponseType, ProtoError>> {
    let (tx, rx) = unbounded_channel::<Result<CTraderResponseType, ProtoError>>();

    let mut rx_original = self.conn.subscribe().await;

    tokio::task::spawn(async move {
      while let Some(msg) = rx_original.recv().await {
        let _ = tx.send(CTraderResponseType::try_from(msg));
      }
    });

    rx
  }
}
