use kit_ctrader_proto::ProtoOaUnsubscribeDepthQuotesReq;

/// * Request for unsubscribing from the depth of market of the specified symbol.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct UnsubscribeDepthQuotesReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: Vec<i64>,
}

impl From<ProtoOaUnsubscribeDepthQuotesReq> for UnsubscribeDepthQuotesReq {
  fn from(value: ProtoOaUnsubscribeDepthQuotesReq) -> Self {
    UnsubscribeDepthQuotesReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
    }
  }
}
