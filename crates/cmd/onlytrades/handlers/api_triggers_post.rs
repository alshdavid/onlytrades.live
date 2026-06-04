use chrono::Utc;
use platform_models::TriggerModel;
use serde::Deserialize;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::middleware;

#[derive(Debug, Default, Serialize)]
struct ApiTriggerPostResponse {
  pub id: Uuid,
}

#[derive(Debug, Default, Deserialize)]
struct ApiTriggerPostRequest {
  pub account_id: i64,
  pub name: String,
  pub platform: String,
}

pub async fn api_triggers_post(
  mut req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    ctrader_account_repository,
    trigger_repository,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let body = uhttp::body::json::<ApiTriggerPostRequest>(&mut req.body()).await?;

  let Some(ctrader_account) = middleware::with_ctrader_account(
    &req,
    &res,
    &profile.id,
    &body.account_id,
    &ctrader_account_repository,
  )
  .await?
  else {
    return Ok(());
  };

  let trigger_id = Uuid::new_v4();

  trigger_repository
    .upsert(TriggerModel {
      id: trigger_id,
      profile_id: profile.id,
      ctrader_account_id: ctrader_account.account_id,
      name: body.name,
      platform: body.platform,
      status: "active".to_string(), // TODO
      created_at: Utc::now(),
    })
    .await?;

  let response = serde_json::to_string(&ApiTriggerPostResponse { id: trigger_id })?;

  res.header().set("Content-Type", "application/json").await?;
  res.write_all(response.as_bytes()).await?;
  Ok(())
}
