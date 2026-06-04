use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;

use dashmap::DashMap;
use prost::Message;
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

use super::messages::ProtoHeartbeatEvent;
use super::messages::ProtoMessage;
use super::messages::ProtoOaErrorRes;
use super::messages::ProtoOaPayloadType;
use super::messages::ProtoPayloadType;

pub const HOST_DEMO: &str = "demo.ctraderapi.com";
pub const PORT_DEMO: u32 = 5035;
pub const HOST_LIVE: &str = "live.ctraderapi.com";
pub const PORT_LIVE: u32 = 5035;

pub struct CTraderSocketClientOptions {
  pub live: bool,
}

pub struct CTraderSocketClient {
  // Wrapped directly in Arc; DashMap handles internal thread safety concurrently
  messages: Arc<DashMap<String, UnboundedSender<ProtoMessage>>>,
  listeners: Arc<DashMap<u32, Vec<UnboundedSender<ProtoMessage>>>>,
  listeners_fallback: Arc<Mutex<Vec<UnboundedSender<ProtoMessage>>>>,
  writer: Arc<Mutex<WriteHalf<TlsStream<TcpStream>>>>,
  id_counter: AtomicU64,
}

impl CTraderSocketClient {
  pub async fn connect(options: CTraderSocketClientOptions) -> anyhow::Result<Self> {
    let host = if options.live { HOST_LIVE } else { HOST_DEMO };
    let port = if options.live { PORT_LIVE } else { PORT_DEMO };

    let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;

    let connector =
      TlsConnector::from(tokio_native_tls::native_tls::TlsConnector::builder().build()?);

    let tls_stream = connector.connect(host, stream).await?;

    let (reader, writer) = tokio::io::split(tls_stream);

    let writer = Arc::new(Mutex::new(writer));
    // Instantiate plain DashMap
    let messages = Arc::new(DashMap::new());
    let listeners = Arc::new(DashMap::new());
    let listeners_fallback = Arc::new(Mutex::new(Vec::new()));

    tokio::task::spawn(CTraderSocketClient::task_heart_beat(Arc::clone(&writer)));
    tokio::task::spawn(CTraderSocketClient::task_read(
      reader,
      Arc::clone(&messages),
      Arc::clone(&listeners),
      Arc::clone(&listeners_fallback),
    ));

    Ok(Self {
      writer,
      messages,
      listeners,
      id_counter: Default::default(),
      listeners_fallback,
    })
  }

  async fn task_heart_beat(
    writer: Arc<Mutex<WriteHalf<TlsStream<TcpStream>>>>
  ) -> anyhow::Result<()> {
    loop {
      tokio::time::sleep(Duration::from_secs(10)).await;

      let message = ProtoMessage {
        payload_type: payload_type(ProtoPayloadType::HeartbeatEvent),
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
    messages: Arc<DashMap<String, UnboundedSender<ProtoMessage>>>,
    listeners: Arc<DashMap<u32, Vec<UnboundedSender<ProtoMessage>>>>,
    listeners_fallback: Arc<Mutex<Vec<UnboundedSender<ProtoMessage>>>>,
  ) -> anyhow::Result<()> {
    let heartbeat_event = payload_type(ProtoPayloadType::HeartbeatEvent);

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

      if let Some(message_id) = result.client_msg_id.clone() {
        messages.remove_if(&message_id, |_k, sender| sender.send(result).is_err());
        continue;
      };

      // println!("<- {} [{}]", result.payload_type, message_id);

      if let Some(mut senders) = listeners.get_mut(&result.payload_type) {
        senders.retain(|sender| sender.send(result.clone()).is_ok());
      } else {
        listeners_fallback
          .lock()
          .await
          .retain(|sender| sender.send(result.clone()).is_ok());
      };
    }
    Ok(())
  }

  fn next_id(&self) -> String {
    format!("{}", self.id_counter.fetch_add(1, Ordering::Relaxed))
  }

  pub async fn send_raw(
    &self,
    msg: ProtoMessage,
  ) -> std::result::Result<(), ProtoError> {
    // println!("-> {} [{:?}]", msg.payload_type, msg.client_msg_id);

    let encoded = msg.encode_to_vec();
    let frame = add_be_header(encoded);

    let mut writer = self.writer.lock().await;
    if writer.write_all(&frame).await.is_err() {
      return Err(ProtoError::error_code("ERROR_WRITER_WRITE"));
    };
    if writer.flush().await.is_err() {
      return Err(ProtoError::error_code("ERROR_WRITER_FLUSH"));
    };
    Ok(())
  }

  pub async fn send_and_receive_oneshot<R: Message + Default + Debug>(
    &self,
    payload_type: impl Into<i32>,
    payload: impl Message,
  ) -> std::result::Result<R, ProtoError> {
    let mut rx = self.send_and_subscribe(payload_type, payload).await?;
    let Some(resp) = rx.recv().await else {
      return Err(ProtoError::error_code("ERROR_CHAN_RECEIVE"));
    };
    resp?.try_decode_body::<R>()
  }

  pub async fn send_and_receive_oneshot_proto(
    &self,
    message: ProtoMessage,
  ) -> std::result::Result<ProtoMessage, ProtoError> {
    let mut rx = self.send_and_subscribe_proto(message).await?;
    let Some(resp) = rx.recv().await else {
      return Err(ProtoError::error_code("ERROR_CHAN_RECEIVE"));
    };
    resp
  }

  pub async fn send_and_subscribe_proto(
    &self,
    mut message: ProtoMessage,
  ) -> std::result::Result<ProtoReceiver, ProtoError> {
    let message_id = match &message.client_msg_id {
      Some(message_id) => message_id.clone(),
      None => {
        let message_id = self.next_id();
        message.client_msg_id.replace(message_id.clone());
        message_id
      }
    };

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<ProtoMessage>();
    self.messages.insert(message_id.clone(), tx);

    self.send_raw(message).await?;

    Ok(ProtoReceiver {
      inner: rx,
      // message_id: ProtoReceiverType::Message(message_id),
      // messages: Arc::clone(&self.messages),
    })
  }

  pub async fn subscribe_proto(
    &self,
    event_type: impl Into<i32>,
  ) -> std::result::Result<ProtoReceiver, ProtoError> {
    let event_type = payload_type(event_type);
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<ProtoMessage>();
    self.listeners.entry(event_type).or_default().push(tx);
    Ok(ProtoReceiver { inner: rx })
  }

  pub async fn subscribe_fallback_proto(&self) -> std::result::Result<ProtoReceiver, ProtoError> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<ProtoMessage>();
    self.listeners_fallback.lock().await.push(tx);
    Ok(ProtoReceiver { inner: rx })
  }

  pub async fn send_and_subscribe(
    &self,
    payload_type: impl Into<i32>,
    payload: impl Message,
  ) -> std::result::Result<ProtoReceiver, ProtoError> {
    let msg = ProtoMessage {
      payload_type: payload_type.into() as u32,
      payload: Some(payload.encode_to_vec()),
      client_msg_id: None,
    };
    self.send_and_subscribe_proto(msg).await
  }
}

fn add_be_header(payload: Vec<u8>) -> Vec<u8> {
  let len = payload.len() as u32;
  let mut result = Vec::with_capacity(4 + payload.len());
  result.extend_from_slice(&len.to_be_bytes());
  result.extend_from_slice(&payload);
  result
}

pub fn payload_type(input: impl Into<i32>) -> u32 {
  let a: i32 = input.into();
  a as u32
}

#[derive(Debug, Default, Error)]
pub struct ProtoError {
  /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
  pub error_code: String,
  /// The error description.
  pub description: String,
  /// The Unix time in seconds when the current maintenance session will be ended.
  pub maintenance_end_timestamp: Option<i64>,
  /// When you hit rate limit with errorCode=BLOCKED_PAYLOAD_TYPE, this field will contain amount of seconds until related payload type will be unlocked.
  pub retry_after: Option<u64>,
}

impl std::fmt::Display for ProtoError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl ProtoError {
  fn error_code(code: &str) -> Self {
    Self {
      error_code: code.to_string(),
      description: Default::default(),
      maintenance_end_timestamp: Default::default(),
      retry_after: Default::default(),
    }
  }
}

pub struct ProtoReceiver {
  inner: UnboundedReceiver<ProtoMessage>,
}

impl ProtoReceiver {
  pub async fn recv(&mut self) -> Option<std::result::Result<ProtoMessage, ProtoError>> {
    let msg = self.inner.recv().await?;

    if msg.payload_type == ProtoOaPayloadType::ProtoOaErrorRes as u32 {
      let Some(body_bytes) = msg.payload else {
        return Some(Err(ProtoError {
          error_code: "ERROR_NO_BODY".to_string(),
          description: Default::default(),
          maintenance_end_timestamp: None,
          retry_after: None,
        }));
      };

      let Ok(body) = ProtoOaErrorRes::decode(body_bytes.as_slice()) else {
        return Some(Err(ProtoError {
          error_code: "ERROR_CANNOT_PARSE_BODY".to_string(),
          description: Default::default(),
          maintenance_end_timestamp: None,
          retry_after: None,
        }));
      };

      return Some(Err(ProtoError {
        error_code: body.error_code,
        description: body.description.unwrap_or_default(),
        maintenance_end_timestamp: body.maintenance_end_timestamp,
        retry_after: body.retry_after,
      }));
    }

    Some(Ok(msg))
  }
}

pub trait ProtoMessageParse {
  fn try_decode_body<R: Message + Default + Debug>(&self) -> std::result::Result<R, ProtoError>;
}

impl ProtoMessageParse for ProtoMessage {
  fn try_decode_body<R: Message + Default + Debug>(&self) -> std::result::Result<R, ProtoError> {
    let Some(body_bytes) = self.payload.as_ref() else {
      return Err(ProtoError::error_code("ERROR_NO_BODY"));
    };

    let Ok(result) = R::decode(body_bytes.as_slice()) else {
      return Err(ProtoError::error_code("ERROR_CANT_PARSE_BODY"));
    };

    Ok(result)
  }
}
