use kit_ctrader_proto::ProtoOaSymbolsListRes;

use super::ArchivedSymbol;
use super::LightSymbol;

/// * Response to the ProtoOASymbolsListReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolsListRes {
  pub client_msg_id: Option<String>,
  pub ctid_trader_account_id: i64,
  pub symbol: Vec<LightSymbol>,
  pub archived_symbol: Vec<ArchivedSymbol>,
}

impl From<ProtoOaSymbolsListRes> for SymbolsListRes {
  fn from(value: ProtoOaSymbolsListRes) -> Self {
    SymbolsListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol: value.symbol.into_iter().map(LightSymbol::from).collect(),
      archived_symbol: value
        .archived_symbol
        .into_iter()
        .map(ArchivedSymbol::from)
        .collect(),
    }
  }
}
