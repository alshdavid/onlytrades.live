use kit_ctrader_proto::ProtoOaUnsubscribeLiveTrendbarRes;

/// * Response to the ProtoOASubscribeLiveTrendbarReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct UnsubscribeLiveTrendbarRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaUnsubscribeLiveTrendbarRes> for UnsubscribeLiveTrendbarRes {
  fn from(value: ProtoOaUnsubscribeLiveTrendbarRes) -> Self {
    UnsubscribeLiveTrendbarRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
