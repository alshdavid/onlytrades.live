use kit_ctrader_proto::ProtoOaExpectedMarginReq;

/// * Request for getting the margin estimate according to leverage profiles. Can be used before sending a new order request. This doesn't consider ACCORDING_TO_GSL margin calculation type, as this calculation is trivial: usedMargin = (VWAP price of the position - GSL price) * volume * Quote2Deposit.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ExpectedMarginReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: i64,
  /// Volume represented in 0.01 of a unit (e.g. 1000 in protocol means 10.00 units).
  pub volume: Vec<i64>,
}

impl From<ProtoOaExpectedMarginReq> for ExpectedMarginReq {
  fn from(value: ProtoOaExpectedMarginReq) -> Self {
    ExpectedMarginReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
      volume: value.volume,
    }
  }
}
