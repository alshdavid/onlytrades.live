use kit_ctrader_proto::ProtoOaReconcileReq;

/// * Request for getting Trader's current open positions and pending orders data.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ReconcileReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// If TRUE, then current protection orders are returned separately, otherwise you can use position.stopLoss and position.takeProfit fields.
  pub return_protection_orders: Option<bool>,
}

impl From<ProtoOaReconcileReq> for ReconcileReq {
  fn from(value: ProtoOaReconcileReq) -> Self {
    ReconcileReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      return_protection_orders: value.return_protection_orders,
    }
  }
}
