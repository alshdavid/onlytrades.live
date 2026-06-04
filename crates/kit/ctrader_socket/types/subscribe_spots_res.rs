use kit_ctrader_proto::ProtoOaSubscribeSpotsRes;

/// * Response to the ProtoOASubscribeSpotsReq request. Reflects that your request to subscribe for symbol has been added to queue. You'll receive technical ProtoOASpotEvent with current price shortly after this response.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SubscribeSpotsRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaSubscribeSpotsRes> for SubscribeSpotsRes {
  fn from(value: ProtoOaSubscribeSpotsRes) -> Self {
    SubscribeSpotsRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
