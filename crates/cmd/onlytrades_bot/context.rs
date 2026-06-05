use std::sync::Arc;

use kit_ctrader_socket::CTraderRequestType;
use kit_ctrader_socket::CTraderResponseType;
use kit_ctrader_socket::connection_proto::ProtoError;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::unbounded_channel;

use super::Result;

pub struct Context {
  response_listeners: Arc<Mutex<Vec<UnboundedSender<std::result::Result<CTraderResponseType, ProtoError>>>>>,
  tx_requests: UnboundedSender<CTraderRequestType>,
  account_id: i64,
}

impl Context {
  pub(crate) async fn new() -> Result<Self> {
    // Another alternative is to use a unix domain socket.
    // Strictly speaking that's probably better.
    // I will add an option to use a unix socket or tcp socket

    let port = std::env::var("PORT")?;
    let hostname = std::env::var("HOSTNAME")?;
    let account_id = std::env::var("ACCOUNT_ID")?.parse::<i64>()?;

    let stream = TcpStream::connect(format!("{}:{}", hostname, port)).await?;
    let (reader, writer) = tokio::io::split(stream);

    let (tx_requests, rx_requests) = unbounded_channel::<CTraderRequestType>();
    let response_listeners = Arc::new(Mutex::new(Vec::new()));

    tokio::task::spawn(Context::task_read(reader, Arc::clone(&response_listeners)));
    tokio::task::spawn(Context::task_write(writer, rx_requests));

    Ok(Self {
      response_listeners,
      tx_requests,
      account_id,
    })
  }

  pub fn account_id(&self) -> i64 {
    self.account_id
  }

  pub async fn subscribe(&self) -> UnboundedReceiver<std::result::Result<CTraderResponseType, ProtoError>> {
    let (tx, rx) = unbounded_channel();
    self.response_listeners.lock().await.push(tx);
    rx
  }

  pub fn send(
    &self,
    message: CTraderRequestType,
  ) -> Result<()> {
    dbg!(&message);
    self.tx_requests.send(message)?;
    Ok(())
  }

  async fn task_read(
    mut reader: ReadHalf<TcpStream>,
    response_listeners: Arc<Mutex<Vec<UnboundedSender<std::result::Result<CTraderResponseType, ProtoError>>>>>,
  ) -> Result<()> {
    loop {
      let mut header_buf = [0u8; 4];
      match reader.read_exact(&mut header_buf).await {
        Ok(_) => {}
        Err(e) => {
          eprintln!("Connection closed or read error: {}", e);
          break;
        }
      };

      let len = u32::from_be_bytes(header_buf) as usize;

      let mut body_buf: Vec<u8> = vec![0u8; len];
      match reader.read_exact(&mut body_buf).await {
        Ok(_n) => {}
        Err(e) => {
          eprintln!("Read error: {}", e);
          break;
        }
      };

      // TODO: rkyv
      let result = serde_json::from_slice::<std::result::Result<CTraderResponseType, ProtoError>>(body_buf.as_slice())?;

      response_listeners
        .lock()
        .await
        .retain(|sender| sender.send(result.clone()).is_ok());
    }
    Ok(())
  }

  async fn task_write(
    mut writer: WriteHalf<TcpStream>,
    mut rx_requests: UnboundedReceiver<CTraderRequestType>,
  ) -> Result<()> {
    while let Some(request) = rx_requests.recv().await {
      let bytes = add_be_header(serde_json::to_vec(&request)?);
      writer.write_all(&bytes).await?;
      writer.flush().await?;
    }
    Ok(())
  }
}

fn add_be_header(payload: Vec<u8>) -> Vec<u8> {
  let len = payload.len() as u32;
  let mut result = Vec::with_capacity(4 + payload.len());
  result.extend_from_slice(&len.to_be_bytes());
  result.extend_from_slice(&payload);
  result
}
