use kit_ctrader_proto::ProtoOaSymbolsForConversionRes;

use super::LightSymbol;

/// * Response to the ProtoOASymbolsForConversionReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SymbolsForConversionRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Conversion chain of the symbols (e.g. EUR/USD, USD/JPY, GBP/JPY -> EUR/GBP).
  pub symbol: Vec<LightSymbol>,
}

impl From<ProtoOaSymbolsForConversionRes> for SymbolsForConversionRes {
  fn from(value: ProtoOaSymbolsForConversionRes) -> Self {
    SymbolsForConversionRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol: value.symbol.into_iter().map(LightSymbol::from).collect(),
    }
  }
}
