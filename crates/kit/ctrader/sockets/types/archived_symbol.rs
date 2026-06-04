use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArchivedSymbol {
  /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
  pub symbol_id: i64,
  /// Name of the symbol (e.g. EUR/USD).
  pub name: String,
  /// The Unix time in milliseconds of the last update of the symbol.
  pub utc_last_update_timestamp: i64,
  /// Description of the symbol.
  pub description: Option<String>,
}

impl From<kit_ctrader_proto::ProtoOaArchivedSymbol> for ArchivedSymbol {
  fn from(symbol: kit_ctrader_proto::ProtoOaArchivedSymbol) -> Self {
    ArchivedSymbol {
      symbol_id: symbol.symbol_id,
      name: symbol.name,
      utc_last_update_timestamp: symbol.utc_last_update_timestamp,
      description: symbol.description,
    }
  }
}
