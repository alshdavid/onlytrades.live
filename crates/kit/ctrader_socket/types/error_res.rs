use kit_ctrader_proto::ProtoOaErrorRes;

/// * Generic response when an ERROR occurred.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ErrorRes {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: Option<i64>,
  /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
  pub error_code: String,
  /// The error description.
  pub description: Option<String>,
  /// The Unix time in seconds when the current maintenance session will be ended.
  pub maintenance_end_timestamp: Option<i64>,
  /// When you hit rate limit with errorCode=BLOCKED_PAYLOAD_TYPE, this field will contain amount of seconds until related payload type will be unlocked.
  pub retry_after: Option<u64>,
}

impl From<ProtoOaErrorRes> for ErrorRes {
  fn from(value: ProtoOaErrorRes) -> Self {
    ErrorRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      error_code: value.error_code,
      description: value.description,
      maintenance_end_timestamp: value.maintenance_end_timestamp,
      retry_after: value.retry_after,
    }
  }
}
