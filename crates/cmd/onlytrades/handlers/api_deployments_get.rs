use chrono::DateTime;
use chrono::Utc;
use platform_bot_service::DeploymentMeta;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiDeploymentsGetResponse {
  pub deployments: Vec<ApiDeploymentsGetResponseDeployment>,
}

#[derive(Debug, Default, Serialize)]
struct ApiDeploymentsGetResponseDeployment {
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

pub async fn api_deployments_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile,
    deployments_repository,
    bot_service,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let deployments = deployments_repository
    .get_by_profile_id(&profile.id)
    .await?;

  let running_all = bot_service
    .get_all()
    .into_iter()
    .filter(|m| m.profile_id == profile.id)
    .collect::<Vec<DeploymentMeta>>();

  let response = serde_json::to_vec(&ApiDeploymentsGetResponse {
    deployments: {
      let mut b = Vec::new();

      for deployment in deployments {
        let running = running_all
          .iter()
          .find(|m| m.deployment_id == deployment.id);

        b.push(ApiDeploymentsGetResponseDeployment {
          id: deployment.id,
          bot_id: deployment.bot_id,
          name: deployment.name,
          account_id: deployment.account_id,
          environment: kit_b64::encode(&deployment.environment),
          active: deployment.active,
          created_at: deployment.created_at,
          running: running.is_some(),
          started_at: running.map(|m| m.created_at),
        });
      }
      b
    },
  })?;

  res.header().add("Content-Type", "application/json").await?;
  res.write_all(&response).await?;
  res.write_head(StatusCode::OK).await?;
  Ok(())
}
