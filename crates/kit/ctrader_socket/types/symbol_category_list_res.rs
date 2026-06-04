use kit_ctrader_proto::ProtoOaSymbolCategoryListRes;

use super::SymbolCategory;

/// * Response to the ProtoSymbolCategoryListReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolCategoryListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of symbol categories.
  pub symbol_category: Vec<SymbolCategory>,
}

impl From<ProtoOaSymbolCategoryListRes> for SymbolCategoryListRes {
  fn from(value: ProtoOaSymbolCategoryListRes) -> Self {
    SymbolCategoryListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_category: value
        .symbol_category
        .into_iter()
        .map(SymbolCategory::from)
        .collect(),
    }
  }
}
