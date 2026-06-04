use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentModel {
  pub id: Uuid,
  pub bot_id: Uuid,
  pub account_id: i64,
  pub name: String,
  pub environment: Vec<u8>,
  pub active: bool,
  pub created_at: DateTime<Utc>,
}
