use kit_ctrader_proto::ProtoOaSymbolByIdReq;

/// * Request for getting a full symbol entity.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolByIdReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the symbol in cTrader platform.
  pub symbol_id: Vec<i64>,
}

impl From<ProtoOaSymbolByIdReq> for SymbolByIdReq {
  fn from(value: ProtoOaSymbolByIdReq) -> Self {
    SymbolByIdReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
    }
  }
}
