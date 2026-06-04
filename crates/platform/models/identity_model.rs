use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentityModel {
  /// The full Auth0 subject ID (e.g., "google-oauth2|12345")
  pub sub: String,
  /// Foreign key linking back to ProfileModel.id
  pub profile_id: Uuid,
  /// The provider extracted from the 'sub' prefix (e.g., "google-oauth2")
  pub provider: String,
  /// The last time this specific identity was used to log in
  pub last_login: DateTime<Utc>,
}
