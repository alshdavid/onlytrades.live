use kit_ctrader_proto::ProtoOaOrderDetailsReq;

/// * Request for getting Order and its related Deals.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct OrderDetailsReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the Order.
  pub order_id: i64,
}

impl From<ProtoOaOrderDetailsReq> for OrderDetailsReq {
  fn from(value: ProtoOaOrderDetailsReq) -> Self {
    OrderDetailsReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      order_id: value.order_id,
    }
  }
}
