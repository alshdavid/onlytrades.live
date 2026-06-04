use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Serialize)]
struct ApiBotsGetIdResponse {
  pub id: Uuid,
  pub name: String,
  pub kind: String,
  pub language: String,
  pub handler: Option<String>,
  pub created_at: DateTime<Utc>,
}

pub async fn api_bots_get_id(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile,
    bot_repository,
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

  let handler = match bot.language.as_str() {
    "typescript-v1" => Some(kit_b64::encode(&bot.handler)),
    _ => None,
  };

  let response = serde_json::to_vec(&ApiBotsGetIdResponse {
    id: bot.id,
    name: bot.name,
    kind: bot.kind,
    language: bot.language,
    handler,
    created_at: bot.created_at,
  })?;

  res.header().add("Content-Type", "application/json").await?;
  res.write_all(&response).await?;
  res.write_head(StatusCode::OK).await?;
  Ok(())
}
