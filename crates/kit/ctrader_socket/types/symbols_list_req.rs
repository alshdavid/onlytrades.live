use kit_ctrader_proto::ProtoOaSymbolsListReq;

/// * Request for a list of symbols available for a trading account. Symbol entries are returned with the limited set of fields.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolsListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Whether to include old archived symbols into response.
  pub include_archived_symbols: Option<bool>,
}

impl From<ProtoOaSymbolsListReq> for SymbolsListReq {
  fn from(value: ProtoOaSymbolsListReq) -> Self {
    SymbolsListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      include_archived_symbols: value.include_archived_symbols,
    }
  }
}
