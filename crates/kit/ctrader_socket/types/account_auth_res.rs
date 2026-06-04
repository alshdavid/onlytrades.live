use kit_ctrader_proto::ProtoOaAccountAuthRes;

/// * Response to the ProtoOAApplicationAuthRes request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AccountAuthRes {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaAccountAuthRes> for AccountAuthRes {
  fn from(value: ProtoOaAccountAuthRes) -> Self {
    AccountAuthRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
