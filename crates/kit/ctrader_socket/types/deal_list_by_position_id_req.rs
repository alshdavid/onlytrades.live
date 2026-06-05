use kit_ctrader_proto::ProtoOaDealListByPositionIdReq;

/// * Request for retrieving the deals related to a position.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct DealListByPositionIdReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the position.
  pub position_id: i64,
  /// The Unix time in milliseconds of starting the search. Must be bigger or equal to zero (1st Jan 1970).
  pub from_timestamp: Option<i64>,
  /// The Unix time in milliseconds of finishing the search. <= 2147483646000 (19th Jan 2038).
  pub to_timestamp: Option<i64>,
}

impl From<ProtoOaDealListByPositionIdReq> for DealListByPositionIdReq {
  fn from(value: ProtoOaDealListByPositionIdReq) -> Self {
    DealListByPositionIdReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      position_id: value.position_id,
      from_timestamp: value.from_timestamp,
      to_timestamp: value.to_timestamp,
    }
  }
}
