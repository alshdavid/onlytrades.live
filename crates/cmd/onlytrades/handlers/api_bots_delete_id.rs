use platform_bot_service::DeploymentMeta;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

pub async fn api_bots_delete_id(
  req: uhttp::Request,
  res: uhttp::Response,
  Ctx {
    profile,
    bot_repository,
    bot_service,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(bot_id) = req.url_param("id") else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let bot_id = Uuid::parse_str(bot_id)?;

  let bots = bot_repository.get_by_profile_id(&profile.id).await?;
  let Some(bot) = bots.into_iter().find(|bot| bot.id == bot_id) else {
    res.write_head(StatusCode::NOT_FOUND).await?;
    return Ok(());
  };

  bot_repository.delete(&bot.id).await?;

  let running_all = bot_service
    .get_all()
    .into_iter()
    .filter(|m| m.profile_id == profile.id && m.bot_id == bot.id)
    .collect::<Vec<DeploymentMeta>>();

  for running in running_all {
    bot_service.stop_deployment(&running.deployment_id);
  }

  res.write_head(StatusCode::NO_CONTENT).await?;
  Ok(())
}
