use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

use super::DenoProcess;
use super::DenoProcessOptions;
use super::SandboxType;

pub static CODE: &str = include_str!("./code/engine.ts");

pub struct DenoInstance {
  pub id: Uuid,
  child: DenoProcess,
  tx: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
  rx: Option<tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>>,
  // rx_done: Option<tokio::sync::mpsc::UnboundedReceiver<()>>,
}

impl DenoInstance {
  pub async fn new(
    mut options: DenoProcessOptions,
    code: &str,
  ) -> anyhow::Result<Self> {
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let local_addr = listener.local_addr()?;
    let (tx_writer, mut rx_writer) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let (tx_reader, rx_reader) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    // let (tx_done, rx_done) = tokio::sync::mpsc::unbounded_channel::<()>();

    tokio::spawn(async move {
      let Ok((stream, _)) = listener.accept().await else {
        eprintln!("Failed to accept connection");
        return;
      };

      let (mut reader, mut writer) = stream.into_split();

      tokio::spawn(async move {
        loop {
          let mut header_buf = [0u8; 4];
          match reader.read_exact(&mut header_buf).await {
            Ok(_) => {}
            Err(_e) => {
              // eprintln!("Connection closed or read error: {}", e);
              return;
            }
          };

          let len = u32::from_be_bytes(header_buf) as usize;
          let mut body_buf: Vec<u8> = vec![0u8; len];
          match reader.read_exact(&mut body_buf).await {
            Ok(_n) => {}
            Err(_e) => {
              // eprintln!("Read error: {}", e);
              return;
            }
          };

          let _ = tx_reader.send(body_buf);
        }
      });

      while let Some(bytes) = rx_writer.recv().await {
        let payload = add_be_header(bytes);
        if let Err(_e) = writer.write_all(&payload).await {
          // eprintln!("Write error: {}", e);
          break;
        }
      }
    });

    options
      .env
      .insert("PORT".to_string(), local_addr.port().to_string());

    let hostname = match options.sandbox {
      SandboxType::None => "127.0.0.1".to_string(),
      SandboxType::Podman => "host.containers.internal".to_string(),
    };
    options
      .env
      .insert("HOSTNAME".to_string(), hostname.to_string());

    let code = format!("{}\n\n{}", CODE, code);
    let child = DenoProcess::launch(options, &code).await?;

    Ok(Self {
      id: Uuid::now_v7(),
      child,
      tx: tx_writer,
      rx: Some(rx_reader),
      // rx_done: Some(rx_done),
    })
  }

  // pub fn send(
  //   &self,
  //   payload: impl Into<Vec<u8>>,
  // ) {
  //   let _ = self.tx.send(payload.into());
  // }

  pub async fn stdout(&self) -> UnboundedReceiver<String> {
    self.child.stdout().await
  }

  pub async fn stderr(&self) -> UnboundedReceiver<String> {
    self.child.stderr().await
  }

  pub async fn exited(&self) -> UnboundedReceiver<()> {
    self.child.exited().await
  }

  pub fn send_kill(&self) {
    self.child.send_kill();
  }

  pub fn reader(&mut self) -> anyhow::Result<tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>> {
    self.rx.take().context("Receiver already taken")
  }

  pub fn writer(&self) -> tokio::sync::mpsc::UnboundedSender<Vec<u8>> {
    self.tx.clone()
  }

  // pub fn stats(&self) -> anyhow::Result<CommandChildSystemInfo> {
  //   self.child.stats()
  // }
}

fn add_be_header(payload: Vec<u8>) -> Vec<u8> {
  let len = payload.len() as u32;
  let mut result = Vec::with_capacity(4 + payload.len());
  result.extend_from_slice(&len.to_be_bytes());
  result.extend_from_slice(&payload);
  result
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DenoIPCMessage(
  pub String,
  pub Option<serde_json::Value>,
  pub Option<String>,
);
