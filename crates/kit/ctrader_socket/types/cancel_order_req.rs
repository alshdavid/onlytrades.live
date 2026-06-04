use kit_ctrader_proto::ProtoOaCancelOrderReq;

/// * Request for cancelling existing pending order. Allowed only if the accessToken has "trade" permissions for the trading account.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct CancelOrderReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the order.
  pub order_id: i64,
}

impl From<ProtoOaCancelOrderReq> for CancelOrderReq {
  fn from(value: ProtoOaCancelOrderReq) -> Self {
    CancelOrderReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      order_id: value.order_id,
    }
  }
}
