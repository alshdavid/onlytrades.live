use std::collections::HashSet;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileDataViewModel {
  pub id: Uuid,
  /// The primary contact email verified via Auth0
  pub email: String,
  /// Stored as a UTC timestamp
  pub created_at: DateTime<Utc>,
  /// List of auth identities
  pub identities: Vec<IdentityModel>,
  /// List of ctrader accounts
  pub ctrader_accounts: Vec<CtraderAccountModel>,
  /// List of ctrader accounts
  pub ctrader_tokens: Option<CtraderTokenModel>,
  /// List of permissions
  pub permissions: HashSet<ProfilePermission>,
}
