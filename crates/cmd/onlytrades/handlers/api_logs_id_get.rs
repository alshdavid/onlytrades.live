use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiLogsGetIdResponse {
  pub logs: Vec<ApiLogsGetIdResponseLogs>,
}

#[derive(Debug, Default, Serialize)]
struct ApiLogsGetIdResponseLogs {
  pub log_level: i32,
  pub message: String,
  pub created_at: DateTime<Utc>,
}

pub async fn api_logs_id_get(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    log_service,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(audience) = req.url_param("id") else {
    res.write_head(StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let key = format!("{}:{}", profile.id, audience);

  let mut logs = Vec::<ApiLogsGetIdResponseLogs>::new();
  for log in log_service.get_logs(&key, 100).await {
    logs.push(ApiLogsGetIdResponseLogs {
      log_level: log.log_level.into(),
      message: log.message,
      created_at: log.created_at,
    });
  }

  let response = serde_json::to_string(&ApiLogsGetIdResponse { logs })?;

  res.header().set("Content-Type", "application/json").await?;
  res.write_all(response.as_bytes()).await?;
  Ok(())
}
