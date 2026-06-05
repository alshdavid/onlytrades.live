use kit_ctrader_proto::ProtoOaSubscribeDepthQuotesReq;

/// * Request for subscribing on depth of market of the specified symbol.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SubscribeDepthQuotesReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: Vec<i64>,
}

impl From<ProtoOaSubscribeDepthQuotesReq> for SubscribeDepthQuotesReq {
  fn from(value: ProtoOaSubscribeDepthQuotesReq) -> Self {
    SubscribeDepthQuotesReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
    }
  }
}
