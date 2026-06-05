use kit_ctrader_proto::ProtoOaCashFlowHistoryListReq;

/// * Request for getting Trader's historical data of deposits and withdrawals.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct CashFlowHistoryListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The Unix time from which the search starts >=0 (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
  pub from_timestamp: i64,
  /// The Unix time where to stop searching <= 2147483646000 (19th Jan 2038).
  pub to_timestamp: i64,
}

impl From<ProtoOaCashFlowHistoryListReq> for CashFlowHistoryListReq {
  fn from(value: ProtoOaCashFlowHistoryListReq) -> Self {
    CashFlowHistoryListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      from_timestamp: value.from_timestamp,
      to_timestamp: value.to_timestamp,
    }
  }
}
