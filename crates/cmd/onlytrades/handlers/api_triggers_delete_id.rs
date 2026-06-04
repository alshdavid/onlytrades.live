use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

pub async fn api_triggers_delete_id(
  req: uhttp::Request,
  res: uhttp::Response,
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

  trigger_repository.delete(&trigger.id).await?;

  res.write_head(StatusCode::NO_CONTENT).await?;
  Ok(())
}
