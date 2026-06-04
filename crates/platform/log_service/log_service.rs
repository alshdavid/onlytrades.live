#![allow(unused)]
use std::collections::HashMap;
use std::time::Duration;

use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::params;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;
use tokio::time::interval;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum LogLevel {
  Error = 0,
  Warn = 1,
  Info = 2,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
  pub log_level: LogLevel,
  pub audience: String,
  pub message: String,
  pub created_at: DateTime<Utc>,
}

enum LogCommand {
  Emit {
    level: LogLevel,
    audience: String,
    message: String,
    time: DateTime<Utc>,
  },
  GetLogs {
    audience: String,
    responder: oneshot::Sender<Vec<LogEntry>>,
    count: u64,
  },
  SubscribeLogs {
    audience: String,
    responder: UnboundedSender<LogEntry>,
  },
  ClearAudience {
    audience: String,
  },
}

#[derive(Clone)]
pub struct LogService {
  sender: mpsc::Sender<LogCommand>,
}

impl LogService {
  pub async fn new(mut db: Connection) -> anyhow::Result<Self> {
    db.execute(
      "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            log_level INTEGER NOT NULL,
            audience TEXT NOT NULL,
            message TEXT NOT NULL,
            created_at TEXT NOT NULL
        );",
      (),
    )
    .await?;

    db.execute(
      "CREATE INDEX IF NOT EXISTS idx_logs_audience ON logs(audience);",
      (),
    )
    .await?;
    db.execute(
      "CREATE INDEX IF NOT EXISTS idx_logs_created_at ON logs(created_at);",
      (),
    )
    .await?;

    let (tx, mut rx) = mpsc::channel::<LogCommand>(1024);
    let mut cleanup_ticker = interval(Duration::from_secs(3600));

    tokio::spawn(async move {
      let mut subscribers = HashMap::<String, Vec<UnboundedSender<LogEntry>>>::new();

      loop {
        tokio::select! {
            // Handle incoming requests from the application
            Some(cmd) = rx.recv() => {
                match cmd {
                    LogCommand::Emit { level, audience, message, time } => {
                        let level_i32: i32 = level.into();

                        if let Some(subscribers) = subscribers.get_mut(&audience) {
                          subscribers.retain(|tx| tx.send(LogEntry {
                            log_level: level,
                            audience: audience.clone(),
                            message: message.clone(),
                            created_at: time
                          }).is_ok());
                        }

                        let _ = db.execute(
                            "INSERT INTO logs (log_level, audience, message, created_at) VALUES (?1, ?2, ?3, ?4)",
                            params![level_i32, audience, message, time.to_rfc3339()],
                        ).await;
                    }
                    LogCommand::GetLogs { audience, responder, count } => {
                        let mut entries = Vec::new();
                        if let Ok(mut rows) = db.query(
                            "SELECT log_level, audience, message, created_at FROM logs WHERE audience = ?1 ORDER BY created_at DESC LIMIT ?2",
                            params![audience, count],
                        ).await {
                            while let Ok(Some(row)) = rows.next().await {
                                if let (Ok(lvl), Ok(aud), Ok(msg), Ok(ts)) = (
                                    row.get::<i32>(0), row.get::<String>(1), row.get::<String>(2), row.get::<String>(3)
                                )
                                    && let (Ok(log_level), Ok(created_at)) = (
                                        LogLevel::try_from(lvl),
                                        DateTime::parse_from_rfc3339(&ts).map(|dt| dt.with_timezone(&Utc))
                                    ) {
                                        entries.push(LogEntry { log_level, audience: aud, message: msg, created_at });
                                    }
                            }
                        }
                        let _ = responder.send(entries); // Send back to caller
                    }
                    LogCommand::ClearAudience { audience } => {
                        let _ = db.execute("DELETE FROM logs WHERE audience = ?1", params![audience]).await;
                    }
                    LogCommand::SubscribeLogs { audience, responder } => {
                      subscribers.entry(audience).or_default().push(responder);
                    },
                }
            }
            // Handle the hourly cleanup cycle completely in-stride
            _ = cleanup_ticker.tick() => {
                let cutoff = Utc::now() - chrono::Duration::days(3);
                let _ = db.execute("DELETE FROM logs WHERE created_at < ?1", params![cutoff.to_rfc3339()]).await;
            }
            // If all handles drop, the channel closes and the loop naturally terminates
            else => break,
        }
      }
    });

    Ok(Self { sender: tx })
  }

  // Fire-and-forget: extremely fast sync API
  pub fn info(
    &self,
    audience: &str,
    msg: &str,
  ) {
    drop(self.sender.try_send(LogCommand::Emit {
      level: LogLevel::Info,
      audience: audience.to_string(),
      message: msg.to_string(),
      time: Utc::now(),
    }));
  }

  pub fn warn(
    &self,
    audience: &str,
    msg: &str,
  ) {
    drop(self.sender.try_send(LogCommand::Emit {
      level: LogLevel::Warn,
      audience: audience.to_string(),
      message: msg.to_string(),
      time: Utc::now(),
    }));
  }

  pub fn error(
    &self,
    audience: &str,
    msg: &str,
  ) {
    drop(self.sender.try_send(LogCommand::Emit {
      level: LogLevel::Error,
      audience: audience.to_string(),
      message: msg.to_string(),
      time: Utc::now(),
    }));
  }

  // Reading needs a oneshot channel to get data out of the actor loop
  pub async fn get_logs(
    &self,
    audience: &str,
    count: u64,
  ) -> Vec<LogEntry> {
    let (tx, rx) = oneshot::channel();
    if self
      .sender
      .send(LogCommand::GetLogs {
        audience: audience.to_string(),
        responder: tx,
        count,
      })
      .await
      .is_err()
    {
      return vec![];
    }
    rx.await.unwrap_or_default()
  }

  pub async fn clear_audience(
    &self,
    audience: &str,
  ) {
    drop(self.sender.try_send(LogCommand::ClearAudience {
      audience: audience.to_string(),
    }));
  }

  pub fn subscribe(
    &self,
    audience: &str,
  ) -> UnboundedReceiver<LogEntry> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    drop(self.sender.try_send(LogCommand::SubscribeLogs {
      audience: audience.to_string(),
      responder: tx,
    }));
    rx
  }
}
