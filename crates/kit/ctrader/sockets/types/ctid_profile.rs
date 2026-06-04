use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CtidProfile {
  pub user_id: i64,
}

impl From<kit_ctrader_proto::ProtoOaCtidProfile> for CtidProfile {
  fn from(profile: kit_ctrader_proto::ProtoOaCtidProfile) -> Self {
    CtidProfile {
      user_id: profile.user_id,
    }
  }
}
