use kit_ctrader_proto::ProtoOaDealListReq;

/// * Request for getting Trader's deals historical data (execution details).
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct DealListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The Unix time from which the search starts >=0 (1st Jan 1970).
  pub from_timestamp: Option<i64>,
  /// The Unix time where to stop searching <= 2147483646000 (19th Jan 2038).
  pub to_timestamp: Option<i64>,
  /// The maximum number of the deals to return.
  pub max_rows: Option<i32>,
}

impl From<ProtoOaDealListReq> for DealListReq {
  fn from(value: ProtoOaDealListReq) -> Self {
    DealListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      from_timestamp: value.from_timestamp,
      to_timestamp: value.to_timestamp,
      max_rows: value.max_rows,
    }
  }
}
