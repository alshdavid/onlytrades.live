use thiserror::Error;

#[derive(Debug, Error)]
pub enum OnlyTradesError {
  #[error("Failed to get variable {0}")]
  VarError(#[from] std::env::VarError),
  #[error("{0}")]
  ParseIntError(#[from] std::num::ParseIntError),
  #[error("{0}")]
  IoError(#[from] std::io::Error),
  #[error("Send failed: channel closed")]
  SendError,
  #[error("{0}")]
  SerdeError(#[from] serde_json::Error),
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for OnlyTradesError {
  fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
    OnlyTradesError::SendError
  }
}

pub type Result<T> = std::result::Result<T, OnlyTradesError>;
