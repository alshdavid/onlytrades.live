use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct Deal {
  /// The unique ID of the execution deal.
  pub deal_id: i64,
  /// Source order of the deal.
  pub order_id: i64,
  /// Source position of the deal.
  pub position_id: i64,
  /// Volume sent for execution, in cents.
  pub volume: i64,
  /// Filled volume, in cents.
  pub filled_volume: i64,
  /// The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs.
  pub symbol_id: i64,
  /// The Unix time in milliseconds when the deal was sent for execution.
  pub create_timestamp: i64,
  /// The Unix time in milliseconds when the deal was executed.
  pub execution_timestamp: i64,
  /// The Unix time in milliseconds when the deal was created, executed or rejected.
  pub utc_last_update_timestamp: Option<i64>,
  /// Execution price.
  pub execution_price: Option<f64>,
  /// Buy/Sell.
  pub trade_side: TradeSide,
  /// Status of the deal.
  pub deal_status: DealStatus,
  /// Rate for used margin computation. Represented as Base/Deposit.
  pub margin_rate: Option<f64>,
  /// Amount of trading commission associated with the deal.
  pub commission: Option<i64>,
  /// Base to USD conversion rate on the time of deal execution.
  pub base_to_usd_conversion_rate: Option<f64>,
  /// Closing position detail. Valid only for closing deal.
  pub close_position_detail: Option<ClosePositionDetail>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects commission.
  pub money_digits: Option<u32>,
}

impl TryFrom<kit_ctrader_proto::ProtoOaDeal> for Deal {
  type Error = anyhow::Error;

  fn try_from(deal: kit_ctrader_proto::ProtoOaDeal) -> Result<Self, Self::Error> {
    Ok(Deal {
      deal_id: deal.deal_id,
      order_id: deal.order_id,
      position_id: deal.position_id,
      volume: deal.volume,
      filled_volume: deal.filled_volume,
      symbol_id: deal.symbol_id,
      create_timestamp: deal.create_timestamp,
      execution_timestamp: deal.execution_timestamp,
      utc_last_update_timestamp: deal.utc_last_update_timestamp,
      execution_price: deal.execution_price,
      trade_side: TradeSide::try_from(deal.trade_side)?,
      deal_status: DealStatus::try_from(deal.deal_status)?,
      margin_rate: deal.margin_rate,
      commission: deal.commission,
      base_to_usd_conversion_rate: deal.base_to_usd_conversion_rate,
      close_position_detail: deal.close_position_detail.map(ClosePositionDetail::from),
      money_digits: deal.money_digits,
    })
  }
}
