use chrono::Utc;
use platform_models::DeploymentModel;
use serde::Deserialize;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiBotsPostResponse {
  pub id: Uuid,
}

#[derive(Debug, Default, Deserialize)]
struct ApiDeploymentsPostRequest {
  pub bot_id: Uuid,
  pub name: String,
  pub account_id: i64,
  #[serde(with = "kit_b64::serde")]
  pub environment: Vec<u8>,
}

pub async fn api_deployments_post(
  mut req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile,
    bot_repository,
    deployments_repository,
    ctrader_account_repository,
    bot_service,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let body = uhttp::body::json::<ApiDeploymentsPostRequest>(&mut req.body()).await?;

  let Some(bot) = bot_repository
    .get_by_profile_id(&profile.id)
    .await?
    .into_iter()
    .find(|b| b.id == body.bot_id)
  else {
    res.write_head(StatusCode::NOT_FOUND).await?;
    return Ok(());
  };

  let Some(ctarder_account) = ctrader_account_repository
    .find_by_profile_id(&profile.id)
    .await?
    .into_iter()
    .find(|a| a.account_id == body.account_id)
  else {
    res.write_head(StatusCode::NOT_FOUND).await?;
    return Ok(());
  };

  let deployment = DeploymentModel {
    id: Uuid::now_v7(),
    bot_id: bot.id,
    account_id: ctarder_account.account_id,
    name: body.name,
    environment: body.environment,
    active: true,
    created_at: Utc::now(),
  };

  deployments_repository.upsert(deployment.clone()).await?;

  bot_service.start_deployment(&deployment, &bot).await?;

  let response = serde_json::to_vec(&ApiBotsPostResponse { id: deployment.id })?;

  res.header().add("Content-Type", "application/json").await?;
  res.write_all(&response).await?;
  res.write_head(StatusCode::OK).await?;
  Ok(())
}
