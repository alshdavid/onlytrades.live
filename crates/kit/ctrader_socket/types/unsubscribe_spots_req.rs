use kit_ctrader_proto::ProtoOaUnsubscribeSpotsReq;

/// * Request for unsubscribing from the spot events of the specified symbol. Request to stop receiving ProtoOASpotEvents related to particular symbols. Unsubscription is useful to minimize traffic, especially during high volatility events.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct UnsubscribeSpotsReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: Vec<i64>,
}

impl From<ProtoOaUnsubscribeSpotsReq> for UnsubscribeSpotsReq {
  fn from(value: ProtoOaUnsubscribeSpotsReq) -> Self {
    UnsubscribeSpotsReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
    }
  }
}
