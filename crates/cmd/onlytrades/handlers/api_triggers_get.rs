use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use uhttp::*;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiTriggerGetResponse {
  pub triggers: Vec<ApiTriggerGetResponseTrigger>,
}

#[derive(Debug, Default, Serialize)]
struct ApiTriggerGetResponseTrigger {
  pub id: Uuid,
  pub name: String,
  pub platform: String,
  pub status: String,
  pub created_at: DateTime<Utc>,
}

pub async fn api_triggers_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    trigger_repository,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let triggers = trigger_repository.get_by_profile_id(&profile.id).await?;

  let response = serde_json::to_string(&ApiTriggerGetResponse {
    triggers: {
      let mut result = Vec::new();
      for trigger in triggers {
        result.push(ApiTriggerGetResponseTrigger {
          id: trigger.id,
          name: trigger.name,
          platform: trigger.platform,
          status: trigger.status,
          created_at: trigger.created_at,
        });
      }
      result
    },
  })?;

  res.header().set("Content-Type", "application/json").await?;
  res.write_all(response.as_bytes()).await?;
  Ok(())
}
