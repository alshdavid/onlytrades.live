use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeData {
  /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
  pub symbol_id: i64,
  /// Volume in cents (e.g. 1000 in protocol means 10.00 units).
  pub volume: i64,
  /// Buy, Sell.
  pub trade_side: TradeSide,
  /// The Unix time in milliseconds when position was opened or order was created.
  pub open_timestamp: Option<i64>,
  /// Text label specified during order request.
  pub label: Option<String>,
  /// If TRUE then position/order stop loss is guaranteedStopLoss.
  pub guaranteed_stop_loss: Option<bool>,
  /// User-specified comment.
  pub comment: Option<String>,
  /// Specifies the units in which the Symbol is denominated.
  pub measurement_units: Option<String>,
  /// The Unix time in milliseconds when a Position was closed
  pub close_timestamp: Option<u64>,
}

impl TryFrom<kit_ctrader_proto::ProtoOaTradeData> for TradeData {
  type Error = anyhow::Error;

  fn try_from(trade_data: kit_ctrader_proto::ProtoOaTradeData) -> Result<Self, Self::Error> {
    Ok(TradeData {
      symbol_id: trade_data.symbol_id,
      volume: trade_data.volume,
      trade_side: TradeSide::try_from(trade_data.trade_side)?,
      open_timestamp: trade_data.open_timestamp,
      label: trade_data.label,
      guaranteed_stop_loss: trade_data.guaranteed_stop_loss,
      comment: trade_data.comment,
      measurement_units: trade_data.measurement_units,
      close_timestamp: trade_data.close_timestamp,
    })
  }
}
