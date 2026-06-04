use kit_ctrader_proto::ProtoOaGetCtidProfileByTokenRes;

use super::CtidProfile;

/// * Response to the ProtoOAGetCtidProfileByTokenReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetCtidProfileByTokenRes {
  pub client_msg_id: Option<String>,
  /// Trader's profile.
  pub profile: CtidProfile,
}

impl From<ProtoOaGetCtidProfileByTokenRes> for GetCtidProfileByTokenRes {
  fn from(value: ProtoOaGetCtidProfileByTokenRes) -> Self {
    GetCtidProfileByTokenRes {
      client_msg_id: None,
      profile: CtidProfile::from(value.profile),
    }
  }
}
