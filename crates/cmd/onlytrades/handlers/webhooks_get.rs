use chrono::Utc;
use serde::Serialize;
use uhttp::*;

use crate::ctx::Ctx;

#[derive(Debug, Serialize)]
pub struct WebhookGetResponse {
  pub timestamp: String,
}

pub async fn webhooks_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  _ctx: Ctx,
) -> uhttp::Result<()> {
  let body = serde_json::to_vec(&WebhookGetResponse {
    timestamp: Utc::now().to_rfc3339(),
  })?;
  res.write_all(&body).await?;
  res.write_head(StatusCode::OK).await?;
  Ok(())
}
