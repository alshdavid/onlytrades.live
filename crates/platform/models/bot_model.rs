use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotModel {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub name: String,
  pub language: String,
  pub kind: String,
  pub handler: Vec<u8>,
  pub created_at: DateTime<Utc>,
}
