use kit_ctrader_proto::ProtoOaClosePositionReq;

/// * Request for closing or partially closing of an existing position. Allowed only if the accessToken has "trade" permissions for the trading account.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ClosePositionReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the position to close.
  pub position_id: i64,
  /// Volume to close, represented in 0.01 of a unit (e.g. 1000 in protocol means 10.00 units).
  pub volume: i64,
}

impl From<ProtoOaClosePositionReq> for ClosePositionReq {
  fn from(value: ProtoOaClosePositionReq) -> Self {
    ClosePositionReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      position_id: value.position_id,
      volume: value.volume,
    }
  }
}
