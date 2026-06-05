use kit_ctrader_proto::ProtoOaSubscribeDepthQuotesRes;

/// * Response to the ProtoOASubscribeDepthQuotesReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SubscribeDepthQuotesRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaSubscribeDepthQuotesRes> for SubscribeDepthQuotesRes {
  fn from(value: ProtoOaSubscribeDepthQuotesRes) -> Self {
    SubscribeDepthQuotesRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
