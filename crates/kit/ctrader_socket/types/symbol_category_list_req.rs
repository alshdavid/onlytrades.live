use kit_ctrader_proto::ProtoOaSymbolCategoryListReq;

/// * Request for a list of symbol categories available for a trading account.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolCategoryListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaSymbolCategoryListReq> for SymbolCategoryListReq {
  fn from(value: ProtoOaSymbolCategoryListReq) -> Self {
    SymbolCategoryListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
