use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtraderTokenModel {
  pub token_id: Uuid,
  pub refresh_token: String,
  pub access_token: String,
  pub access_token_expires_at: DateTime<Utc>,
}
