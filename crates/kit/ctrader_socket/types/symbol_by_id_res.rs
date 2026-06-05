use kit_ctrader_proto::ProtoOaSymbolByIdRes;

use super::ArchivedSymbol;
use super::Symbol;

/// * Response to the ProtoOASymbolByIdReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolByIdRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Symbol entity with the full set of fields.
  pub symbol: Vec<Symbol>,
  /// Archived symbols.
  pub archived_symbol: Vec<ArchivedSymbol>,
}

impl TryFrom<ProtoOaSymbolByIdRes> for SymbolByIdRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaSymbolByIdRes) -> Result<Self, Self::Error> {
    Ok(SymbolByIdRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol: {
        let mut symbols = Vec::<Symbol>::new();
        for symbol in value.symbol {
          symbols.push(Symbol::try_from(symbol)?);
        }
        symbols
      },
      archived_symbol: {
        let mut archived_symbols = Vec::<ArchivedSymbol>::new();
        for archived_symbol in value.archived_symbol {
          archived_symbols.push(ArchivedSymbol::from(archived_symbol));
        }
        archived_symbols
      },
    })
  }
}
