use chrono::Utc;
use platform_models::BotModel;
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
struct ApiBotsPostRequest {
  pub name: String,
  #[serde(with = "kit_b64::serde")]
  pub handler: Vec<u8>,
  pub kind: String,
  pub language: String,
}

pub async fn api_bots_post(
  mut req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile,
    bot_repository,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let body = uhttp::body::json::<ApiBotsPostRequest>(&mut req.body()).await?;

  let id = Uuid::now_v7();

  bot_repository
    .upsert(BotModel {
      id,
      profile_id: profile.id,
      name: body.name,
      language: body.language,
      kind: body.kind,
      handler: body.handler,
      created_at: Utc::now(),
    })
    .await?;

  let response = serde_json::to_vec(&ApiBotsPostResponse { id })?;

  res.header().add("Content-Type", "application/json").await?;
  res.write_all(&response).await?;
  res.write_head(StatusCode::OK).await?;
  Ok(())
}
