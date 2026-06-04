use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;

use crate::ctx::Ctx;

#[derive(Debug, Serialize)]
struct ApiBotsGetResponse {
  pub bots: Vec<ApiBotsGetResponseBot>,
}

#[derive(Debug, Serialize)]
struct ApiBotsGetResponseBot {
  pub id: String,
  pub name: String,
  pub kind: String,
  pub language: String,
  pub created_at: DateTime<Utc>,
}

pub async fn api_bots_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile,
    bot_repository,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let bots = bot_repository.get_by_profile_id(&profile.id).await?;

  let response = serde_json::to_vec(&ApiBotsGetResponse {
    bots: {
      let mut b = Vec::new();
      for bot in bots {
        b.push(ApiBotsGetResponseBot {
          id: bot.id.to_string(),
          name: bot.name,
          kind: bot.kind,
          language: bot.language,
          created_at: bot.created_at,
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
