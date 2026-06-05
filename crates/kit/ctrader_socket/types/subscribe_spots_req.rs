use kit_ctrader_proto::ProtoOaSubscribeSpotsReq;

/// * Request for subscribing on spot events of the specified symbol. After successful subscription you'll receive technical ProtoOASpotEvent with latest price, after which you'll start receiving updates on prices via consequent ProtoOASpotEvents.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SubscribeSpotsReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: Vec<i64>,
  /// If TRUE you will also receive the timestamp in ProtoOASpotEvent.
  pub subscribe_to_spot_timestamp: Option<bool>,
}

impl From<ProtoOaSubscribeSpotsReq> for SubscribeSpotsReq {
  fn from(value: ProtoOaSubscribeSpotsReq) -> Self {
    SubscribeSpotsReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
      subscribe_to_spot_timestamp: value.subscribe_to_spot_timestamp,
    }
  }
}
