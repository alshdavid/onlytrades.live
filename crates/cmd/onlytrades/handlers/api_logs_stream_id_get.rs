use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiLogsGetIdResponse {
  pub log_level: i32,
  pub message: String,
  pub created_at: DateTime<Utc>,
}

pub async fn api_logs_stream_id_get(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx { log_service, .. }: Ctx,
) -> uhttp::Result<()> {
  let Some(audience) = req.url_param("id") else {
    res.write_head(StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  res
    .header()
    .add("Content-Type", "text/event-stream")
    .await?;

  res.header().add("Transfer-Encoding", "chunked").await?;
  res.write_head(uhttp::StatusCode::OK).await?;

  for log in log_service.get_logs(audience, 100).await {
    let frame = serde_json::to_string(&ApiLogsGetIdResponse {
      log_level: log.log_level.into(),
      message: log.message,
      created_at: log.created_at,
    })?;

    res
      .write_all(format!("data: {}\n\n", frame).as_bytes())
      .await?;
  }

  let mut rx = log_service.subscribe(audience);
  while let Some(log) = rx.recv().await {
    let frame = serde_json::to_string(&ApiLogsGetIdResponse {
      log_level: log.log_level.into(),
      message: log.message,
      created_at: log.created_at,
    })?;

    res
      .write_all(format!("data: {}\n\n", frame).as_bytes())
      .await?;
  }

  Ok(())
}
