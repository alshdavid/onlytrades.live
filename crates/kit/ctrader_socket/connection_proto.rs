use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use kit_ctrader_proto::ProtoHeartbeatEvent;
use kit_ctrader_proto::ProtoMessage;
use kit_ctrader_proto::ProtoOaPayloadType;
use kit_ctrader_proto::ProtoOaVersionReq;
use kit_ctrader_proto::ProtoPayloadType;
use prost::Message;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::ReadHalf;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio_native_tls::TlsConnector;
use tokio_native_tls::TlsStream;

use super::constants::HOST_DEMO;
use super::constants::HOST_LIVE;
use super::constants::PORT_DEMO;
use super::constants::PORT_LIVE;

pub struct CTraderConnectionRawOptions {
  pub live: bool,
}

#[derive(Clone, Debug)]
pub struct CTraderConnectionRaw {
  listeners: Arc<Mutex<Vec<UnboundedSender<ProtoMessage>>>>,
  writer: Arc<Mutex<WriteHalf<TlsStream<TcpStream>>>>,
}

impl CTraderConnectionRaw {
  pub async fn connect(options: CTraderConnectionRawOptions) -> anyhow::Result<Self> {
    let host = if options.live { HOST_LIVE } else { HOST_DEMO };
    let port = if options.live { PORT_LIVE } else { PORT_DEMO };

    let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;

    let connector =
      TlsConnector::from(tokio_native_tls::native_tls::TlsConnector::builder().build()?);

    let tls_stream = connector.connect(host, stream).await?;

    let (reader, writer) = tokio::io::split(tls_stream);

    let writer = Arc::new(Mutex::new(writer));
    let listeners = Arc::new(Mutex::new(Vec::new()));

    tokio::task::spawn(CTraderConnectionRaw::task_heart_beat(Arc::clone(&writer)));
    tokio::task::spawn(CTraderConnectionRaw::task_read(
      reader,
      Arc::clone(&listeners),
    ));

    Ok(Self { writer, listeners })
  }

  async fn task_heart_beat(
    writer: Arc<Mutex<WriteHalf<TlsStream<TcpStream>>>>
  ) -> anyhow::Result<()> {
    loop {
      tokio::time::sleep(Duration::from_secs(10)).await;

      let message = ProtoMessage {
        payload_type: ProtoPayloadType::HeartbeatEvent.as_u32(),
        payload: Some(ProtoHeartbeatEvent::default().encode_to_vec()),
        client_msg_id: None,
      };

      // println!("-> {} [HB]", message.payload_type);
      //
      let message = add_be_header(message.encode_to_vec());

      let mut writer = writer.lock().await;
      if writer.write_all(&message).await.is_err() {
        eprintln!("Failed to write heartbeat");
        break;
      }
      if writer.flush().await.is_err() {
        eprintln!("Failed to flush heartbeat");
        break;
      };
    }

    Ok(())
  }

  async fn task_read(
    mut reader: ReadHalf<TlsStream<TcpStream>>,
    listeners: Arc<Mutex<Vec<UnboundedSender<ProtoMessage>>>>,
  ) -> anyhow::Result<()> {
    let heartbeat_event = ProtoPayloadType::HeartbeatEvent.as_u32();

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

      let result = ProtoMessage::decode(body_buf.as_slice()).unwrap();
      if result.payload_type == heartbeat_event {
        // println!("<- {} [HB]", result.payload_type);
        continue;
      }
      // println!("<- {}", result.payload_type);

      listeners
        .lock()
        .await
        .retain(|sender| sender.send(result.clone()).is_ok());
    }
    Ok(())
  }

  pub async fn send(
    &self,
    msg: ProtoMessage,
  ) -> std::result::Result<(), ProtoError> {
    // println!("-> {} [{:?}]", msg.payload_type, msg.client_msg_id);

    let encoded = msg.encode_to_vec();
    let frame = add_be_header(encoded);

    let mut writer = self.writer.lock().await;
    if writer.write_all(&frame).await.is_err() {
      return Err(ProtoError::SocketWriterError);
    };
    if writer.flush().await.is_err() {
      return Err(ProtoError::SocketFlushError);
    };
    Ok(())
  }

  pub async fn subscribe(&self) -> UnboundedReceiver<ProtoMessage> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<ProtoMessage>();
    self.listeners.lock().await.push(tx);
    rx
  }

  /// Return the round-trip latency for the socket connection
  pub async fn latency(&self) -> std::result::Result<u128, ProtoError> {
    let now_ms = timestamp()?;

    let id = format!("::internal::latency::{}::", now_ms);

    let message = ProtoMessage {
      payload_type: ProtoOaPayloadType::ProtoOaVersionReq.as_u32(),
      payload: Some(ProtoOaVersionReq::default().encode_to_vec()),
      client_msg_id: Some(id.clone()),
    };

    let mut rx = self.subscribe().await;

    let _ = self.send(message).await;
    let version_event = ProtoOaPayloadType::ProtoOaVersionRes.as_u32();

    while let Some(msg) = rx.recv().await {
      if msg.payload_type == version_event && msg.client_msg_id.as_ref().is_some_and(|m| m == &id) {
        let recv_ms = timestamp()?;
        return Ok(recv_ms - now_ms);
      }
    }

    // Todo implement timeout
    Err(ProtoError::SocketTimeout)
  }
}

fn timestamp() -> std::result::Result<u128, ProtoError> {
  let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) else {
    return Err(ProtoError::TimeError);
  };
  Ok(timestamp.as_millis())
}

fn add_be_header(payload: Vec<u8>) -> Vec<u8> {
  let len = payload.len() as u32;
  let mut result = Vec::with_capacity(4 + payload.len());
  result.extend_from_slice(&len.to_be_bytes());
  result.extend_from_slice(&payload);
  result
}

#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum ProtoError {
  #[error("The protocol message contains no body: [{0:?}]")]
  NoBody(Option<String>),
  #[error("Failed to parse the protocol message body: {0}")]
  CannotParseBody(String),
  #[error("Failed to write to the socket")]
  SocketWriterError,
  #[error("Failed to flush the socket")]
  SocketFlushError,
  #[error("Socket request timed out")]
  SocketTimeout,
  #[error("Unknown payload type: [{0:?}] {1}")]
  UnknownPayloadType(Option<String>, i32),
  #[error("Payload parse error: [{0:?}] {1}")]
  PayloadParseError(Option<String>, u32),
  #[error("Error getting current time")]
  TimeError,
}

pub trait ProtoMessageParse {
  fn try_decode_payload<R: Message + Default + Debug>(&self) -> std::result::Result<R, ProtoError>;
  fn try_payload_type(&self) -> std::result::Result<ProtoOaPayloadType, ProtoError>;
}

impl ProtoMessageParse for ProtoMessage {
  fn try_decode_payload<R: Message + Default + Debug>(&self) -> std::result::Result<R, ProtoError> {
    let Some(body_bytes) = self.payload.as_ref() else {
      return Err(ProtoError::NoBody(self.client_msg_id.clone()));
    };

    match R::decode(body_bytes.as_slice()) {
      Ok(result) => Ok(result),
      Err(error) => Err(ProtoError::CannotParseBody(error.to_string())),
    }
  }

  fn try_payload_type(&self) -> std::result::Result<ProtoOaPayloadType, ProtoError> {
    let payload_i32 = match i32::try_from(self.payload_type) {
      Ok(val) => val,
      Err(_) => {
        return Err(ProtoError::PayloadParseError(
          self.client_msg_id.clone(),
          self.payload_type,
        ));
      }
    };

    match ProtoOaPayloadType::try_from(payload_i32) {
      Ok(payload_type) => Ok(payload_type),
      Err(_) => Err(ProtoError::UnknownPayloadType(
        self.client_msg_id.clone(),
        payload_i32,
      )),
    }
  }
}

pub trait PayloadTypeExt {
  fn as_u32(&self) -> u32;
}

impl PayloadTypeExt for ProtoPayloadType {
  fn as_u32(&self) -> u32 {
    i32::from(*self) as u32
  }
}

impl PayloadTypeExt for ProtoOaPayloadType {
  fn as_u32(&self) -> u32 {
    i32::from(*self) as u32
  }
}
