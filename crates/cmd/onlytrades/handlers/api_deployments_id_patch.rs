use platform_models::DeploymentModel;
use serde::Deserialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Deserialize)]
struct ApiDeploymentsIdPatchRequest {
  pub active: Option<bool>,
}

pub async fn api_deployments_id_patch(
  mut req: uhttp::Request,
  res: uhttp::Response,
  Ctx {
    profile,
    deployments_repository,
    bot_repository,
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

  let body = uhttp::body::json::<ApiDeploymentsIdPatchRequest>(&mut req.body()).await?;

  'block: {
    if let Some(active) = body.active {
      let running = bot_service
        .get_all()
        .into_iter()
        .find(|m| m.profile_id == profile.id && m.deployment_id == deployment.id);

      if active && running.is_none() {
        let Some(bot) = bot_repository.get_by_id(&deployment.bot_id).await? else {
          break 'block;
        };

        bot_service.start_deployment(&deployment, &bot).await?;

        deployments_repository
          .upsert(DeploymentModel {
            id: deployment.id,
            bot_id: deployment.bot_id,
            account_id: deployment.account_id,
            name: deployment.name,
            environment: deployment.environment,
            active,
            created_at: deployment.created_at,
          })
          .await?;
      } else if !active && running.is_some() {
        bot_service.stop_deployment(&deployment.id);

        deployments_repository
          .upsert(DeploymentModel {
            id: deployment.id,
            bot_id: deployment.bot_id,
            account_id: deployment.account_id,
            name: deployment.name,
            environment: deployment.environment,
            active,
            created_at: deployment.created_at,
          })
          .await?;
      }
    }
  }

  res.header().add("Content-Type", "application/json").await?;
  res.write_head(StatusCode::NO_CONTENT).await?;
  Ok(())
}
