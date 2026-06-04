use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CtidProfile {
  pub user_id: i64,
}

impl From<super::super::messages::ProtoOaCtidProfile> for CtidProfile {
  fn from(profile: super::super::messages::ProtoOaCtidProfile) -> Self {
    CtidProfile {
      user_id: profile.user_id,
    }
  }
}
