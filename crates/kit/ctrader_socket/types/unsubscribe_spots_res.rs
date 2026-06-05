use kit_ctrader_proto::ProtoOaUnsubscribeSpotsRes;

/// * Response to the ProtoOASubscribeSpotsRes request. Reflects that your request to unsubscribe will has been added to queue and will be completed shortly. You may still occasionally receive ProtoOASpotEvents until request processing is complete.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct UnsubscribeSpotsRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaUnsubscribeSpotsRes> for UnsubscribeSpotsRes {
  fn from(value: ProtoOaUnsubscribeSpotsRes) -> Self {
    UnsubscribeSpotsRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
