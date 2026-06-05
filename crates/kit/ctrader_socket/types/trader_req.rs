use kit_ctrader_proto::ProtoOaTraderReq;

/// * Request for getting data of Trader's Account.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct TraderReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaTraderReq> for TraderReq {
  fn from(value: ProtoOaTraderReq) -> Self {
    TraderReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
