use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

pub async fn api_deployments_id_delete(
  req: uhttp::Request,
  res: uhttp::Response,
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

  if let Some(running) = bot_service
    .get_all()
    .into_iter()
    .find(|m| m.profile_id == profile.id && m.deployment_id == deployment.id)
  {
    bot_service.stop_deployment(&running.deployment_id);
  };

  deployments_repository.delete(&deployment.id).await?;

  res.write_head(StatusCode::NO_CONTENT).await?;
  Ok(())
}
