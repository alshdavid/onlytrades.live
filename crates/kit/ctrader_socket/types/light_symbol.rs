use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightSymbol {
  /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
  pub symbol_id: i64,
  /// Name of the symbol (e.g. EUR/USD).
  pub symbol_name: Option<String>,
  /// If TRUE then symbol is visible for traders.
  pub enabled: Option<bool>,
  /// Base asset.
  pub base_asset_id: Option<i64>,
  /// Quote asset.
  pub quote_asset_id: Option<i64>,
  /// Id of the symbol category used for symbols grouping.
  pub symbol_category_id: Option<i64>,
  pub description: Option<String>,
  /// The number used for sorting Symbols in the UI (lowest number should appear at the top).
  pub sorting_number: Option<f64>,
}

impl From<kit_ctrader_proto::ProtoOaLightSymbol> for LightSymbol {
  fn from(symbol: kit_ctrader_proto::ProtoOaLightSymbol) -> Self {
    LightSymbol {
      symbol_id: symbol.symbol_id,
      symbol_name: symbol.symbol_name,
      enabled: symbol.enabled,
      base_asset_id: symbol.base_asset_id,
      quote_asset_id: symbol.quote_asset_id,
      symbol_category_id: symbol.symbol_category_id,
      description: symbol.description,
      sorting_number: symbol.sorting_number,
    }
  }
}
