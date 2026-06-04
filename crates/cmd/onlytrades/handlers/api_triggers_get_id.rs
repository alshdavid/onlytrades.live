use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiTriggerGetIdResponse {
  pub id: Uuid,
  pub name: String,
  pub platform: String,
  pub status: String,
  pub account_id: String,
  pub created_at: DateTime<Utc>,
}

pub async fn api_triggers_get_id(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    trigger_repository,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(bot_id) = req.url_param("id") else {
    res.write_head(StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let bot_id = Uuid::parse_str(bot_id)?;

  let Some(trigger) = trigger_repository
    .get_by_profile_id(&profile.id)
    .await?
    .into_iter()
    .find(|t| t.id == bot_id)
  else {
    res.write_head(StatusCode::NOT_FOUND).await?;
    return Ok(());
  };

  let response = serde_json::to_string(&ApiTriggerGetIdResponse {
    id: trigger.id,
    name: trigger.name,
    platform: trigger.platform,
    status: trigger.status,
    account_id: trigger.ctrader_account_id.to_string(),
    created_at: trigger.created_at,
  })?;

  res.header().set("Content-Type", "application/json").await?;
  res.write_all(response.as_bytes()).await?;
  Ok(())
}
