use kit_ctrader_proto::ProtoOaApplicationAuthReq;

/// * Request for the authorizing an application to work with the cTrader platform Proxies.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ApplicationAuthReq {
  pub client_msg_id: Option<String>,
  /// The unique Client ID provided during the registration.
  pub client_id: String,
  /// The unique Client Secret provided during the registration.
  pub client_secret: String,
}

impl From<ProtoOaApplicationAuthReq> for ApplicationAuthReq {
  fn from(value: ProtoOaApplicationAuthReq) -> Self {
    ApplicationAuthReq {
      client_msg_id: None,
      client_id: value.client_id,
      client_secret: value.client_secret,
    }
  }
}
