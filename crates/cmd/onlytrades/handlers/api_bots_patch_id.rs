use platform_models::BotModel;
use serde::Deserialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Deserialize)]
struct ApiBotsIdPatchRequest {
  #[serde(with = "kit_b64::serde")]
  pub handler: Vec<u8>,
}

pub async fn api_bots_patch_id(
  mut req: uhttp::Request,
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

  let body = uhttp::body::json::<ApiBotsIdPatchRequest>(&mut req.body()).await?;

  bot_repository
    .upsert(BotModel {
      id: bot.id,
      profile_id: bot.profile_id,
      name: bot.name,
      language: bot.language,
      kind: bot.kind,
      handler: body.handler,
      created_at: bot.created_at,
    })
    .await?;

  bot_service.restart_deployments(&bot.id).await?;

  res.write_head(StatusCode::NO_CONTENT).await?;
  Ok(())
}
