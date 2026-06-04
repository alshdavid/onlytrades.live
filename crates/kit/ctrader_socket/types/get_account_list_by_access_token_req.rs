use kit_ctrader_proto::ProtoOaGetAccountListByAccessTokenReq;

/// * Request for getting the list of granted trader's account for the access token.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetAccountListByAccessTokenReq {
  pub client_msg_id: Option<String>,
  /// The Access Token issued for providing access to the Trader's Account.
  pub access_token: String,
}

impl From<ProtoOaGetAccountListByAccessTokenReq> for GetAccountListByAccessTokenReq {
  fn from(value: ProtoOaGetAccountListByAccessTokenReq) -> Self {
    GetAccountListByAccessTokenReq {
      client_msg_id: None,
      access_token: value.access_token,
    }
  }
}
