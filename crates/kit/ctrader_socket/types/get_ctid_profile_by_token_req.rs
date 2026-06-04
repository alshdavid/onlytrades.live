use kit_ctrader_proto::ProtoOaGetCtidProfileByTokenReq;

/// * Request for getting details of Trader's profile. Limited due to GDRP requirements.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetCtidProfileByTokenReq {
  pub client_msg_id: Option<String>,
  /// The Access Token issued for providing access to the Trader's Account.
  pub access_token: String,
}

impl From<ProtoOaGetCtidProfileByTokenReq> for GetCtidProfileByTokenReq {
  fn from(value: ProtoOaGetCtidProfileByTokenReq) -> Self {
    GetCtidProfileByTokenReq {
      client_msg_id: None,
      access_token: value.access_token,
    }
  }
}
