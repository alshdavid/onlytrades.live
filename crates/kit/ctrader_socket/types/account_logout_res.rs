use kit_ctrader_proto::ProtoOaAccountLogoutRes;

/// * Response to the ProtoOAAccountLogoutReq request. Actual logout of trading account will be completed on ProtoOAAccountDisconnectEvent.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AccountLogoutRes {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaAccountLogoutRes> for AccountLogoutRes {
  fn from(value: ProtoOaAccountLogoutRes) -> Self {
    AccountLogoutRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
