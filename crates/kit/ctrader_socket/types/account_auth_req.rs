use kit_ctrader_proto::ProtoOaAccountAuthReq;

/// * Request for authorizing of the trading account session. Requires established authorized connection with the client application using ProtoOAApplicationAuthReq.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AccountAuthReq {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
  /// The Access Token issued for providing access to the Trader's Account.
  pub access_token: String,
}

impl From<ProtoOaAccountAuthReq> for AccountAuthReq {
  fn from(value: ProtoOaAccountAuthReq) -> Self {
    AccountAuthReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      access_token: value.access_token,
    }
  }
}
