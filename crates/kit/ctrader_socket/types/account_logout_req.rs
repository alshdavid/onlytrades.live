use kit_ctrader_proto::ProtoOaAccountLogoutReq;

/// * Request for logout of trading account session.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AccountLogoutReq {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaAccountLogoutReq> for AccountLogoutReq {
  fn from(value: ProtoOaAccountLogoutReq) -> Self {
    AccountLogoutReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
