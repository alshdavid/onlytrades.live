use kit_ctrader_proto::ProtoOaOrderListByPositionIdReq;

/// * Request for retrieving Orders related to a Position by using Position ID. Filtered by utcLastUpdateTimestamp.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct OrderListByPositionIdReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the Position.
  pub position_id: i64,
  /// The Unix time from which the search starts >=0 (1st Jan 1970). Search by utcLastUpdateTimestamp of the Order.
  pub from_timestamp: Option<i64>,
  /// The Unix time where to stop searching <= 2147483646000 (19th Jan 2038). Search by utcLastUpdateTimestamp of the Order.
  pub to_timestamp: Option<i64>,
}

impl From<ProtoOaOrderListByPositionIdReq> for OrderListByPositionIdReq {
  fn from(value: ProtoOaOrderListByPositionIdReq) -> Self {
    OrderListByPositionIdReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      position_id: value.position_id,
      from_timestamp: value.from_timestamp,
      to_timestamp: value.to_timestamp,
    }
  }
}
