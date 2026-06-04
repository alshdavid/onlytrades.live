use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ProfileModel {
  pub id: Uuid,
  /// The primary contact email verified via Auth0
  pub email: String,
  /// Stored as a UTC timestamp
  pub created_at: DateTime<Utc>,
}
