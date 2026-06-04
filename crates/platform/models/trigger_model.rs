use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerModel {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub ctrader_account_id: i64,
  pub name: String,
  pub platform: String,
  pub status: String,
  pub created_at: DateTime<Utc>,
}
