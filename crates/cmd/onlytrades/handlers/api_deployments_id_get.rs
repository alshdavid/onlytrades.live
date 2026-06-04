use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiDeploymentsIdGetResponse {
  pub id: Uuid,
  pub bot_id: Uuid,
  pub name: String,
  pub account_id: i64,
  pub environment: String,
  pub active: bool,
  pub created_at: DateTime<Utc>,
  pub running: bool,
  pub started_at: Option<DateTime<Utc>>,
}

pub async fn api_deployments_id_get(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile,
    deployments_repository,
    bot_service,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(deployment_id) = req.url_param("id") else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let deployment_id = Uuid::parse_str(deployment_id)?;

  let Some(deployment) = deployments_repository
    .get_by_profile_id(&profile.id)
    .await?
    .into_iter()
    .find(|d| d.id == deployment_id)
  else {
    res.write_head(StatusCode::NOT_FOUND).await?;
    return Ok(());
  };

  let running = bot_service
    .get_all()
    .into_iter()
    .find(|m| m.profile_id == profile.id && m.deployment_id == deployment.id);

  let response = serde_json::to_vec(&ApiDeploymentsIdGetResponse {
    id: deployment.id,
    bot_id: deployment.bot_id,
    name: deployment.name,
    account_id: deployment.account_id,
    environment: kit_b64::encode(&deployment.environment),
    active: deployment.active,
    created_at: deployment.created_at,
    running: match &running {
      Some(m) => m.alive,
      None => false,
    },
    started_at: running.as_ref().map(|m| m.created_at),
  })?;

  res.header().add("Content-Type", "application/json").await?;
  res.write_all(&response).await?;
  res.write_head(StatusCode::OK).await?;
  Ok(())
}
