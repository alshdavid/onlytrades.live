use kit_ctrader_proto::ProtoOaRefreshTokenRes;

/// * Response to the ProtoOARefreshTokenReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct RefreshTokenRes {
  pub client_msg_id: Option<String>,
  /// The Access Token issued for providing access to the Trader's Account.
  pub access_token: String,
  /// bearer
  pub token_type: String,
  /// Access Token expiration in seconds.
  pub expires_in: i64,
  /// Your new Refresh Token.
  pub refresh_token: String,
}

impl From<ProtoOaRefreshTokenRes> for RefreshTokenRes {
  fn from(value: ProtoOaRefreshTokenRes) -> Self {
    RefreshTokenRes {
      client_msg_id: None,
      access_token: value.access_token,
      token_type: value.token_type,
      expires_in: value.expires_in,
      refresh_token: value.refresh_token,
    }
  }
}
