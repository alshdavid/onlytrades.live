use kit_ctrader_proto::ProtoOaRefreshTokenReq;

/// * Request to refresh the access token using refresh token of granted trader's account.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct RefreshTokenReq {
  pub client_msg_id: Option<String>,
  /// The Refresh Token issued for updating Access Token.
  pub refresh_token: String,
}

impl From<ProtoOaRefreshTokenReq> for RefreshTokenReq {
  fn from(value: ProtoOaRefreshTokenReq) -> Self {
    RefreshTokenReq {
      client_msg_id: None,
      refresh_token: value.refresh_token,
    }
  }
}
