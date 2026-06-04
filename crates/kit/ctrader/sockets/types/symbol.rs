use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
  /// The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs.
  pub symbol_id: i64,
  /// Number of price digits to be displayed.
  pub digits: i32,
  /// Pip position on digits.
  pub pip_position: i32,
  /// If TRUE then the short selling with the symbol is enabled.
  pub enable_short_selling: Option<bool>,
  /// If TRUE then setting of guaranteedStopLoss is available for limited risk accounts.
  pub guaranteed_stop_loss: Option<bool>,
  /// Day of the week when SWAP charge amount will be tripled. Doesn't impact Rollover Commission.
  pub swap_rollover3_days: Option<i32>,
  /// SWAP charge for long positions.
  pub swap_long: Option<f64>,
  /// SWAP charge for short positions.
  pub swap_short: Option<f64>,
  /// Maximum allowed volume in cents for an order with a symbol.
  pub max_volume: Option<i64>,
  /// Minimum allowed volume in cents for an order with a symbol.
  pub min_volume: Option<i64>,
  /// Step of the volume in cents for an order.
  pub step_volume: Option<i64>,
  /// Value of max exposure per symbol, per account. Blocks execution if breached.
  pub max_exposure: Option<u64>,
  /// Symbol trading interval, specified in seconds starting from SUNDAY 00:00 in specified time zone.
  pub schedule: Vec<Interval>,
  /// Commission type. See ProtoOACommissionType for details.
  pub commission_type: Option<i32>,
  /// Minimum allowed distance between stop loss and current market price.
  pub sl_distance: Option<u32>,
  /// Minimum allowed distance between take profit and current market price.
  pub tp_distance: Option<u32>,
  /// Minimum allowed distance between guaranteed stop loss and current market price.
  pub gsl_distance: Option<u32>,
  /// Guaranteed stop loss fee.
  pub gsl_charge: Option<i64>,
  /// Unit of distance measure for slDistance, tpDistance, gslDistance.
  pub distance_set_in: Option<i32>,
  /// Minimum commission Type. See ProtoOAMinCommissionType for details.
  pub min_commission_type: Option<i32>,
  /// Currency for minimum commission. (USD or quote currency).
  pub min_commission_asset: Option<String>,
  /// Administrative Fee, charged instead of Swaps if the Account is marked as a "Shariah Compliant (Swap Free)". The Administrative Fee is charged daily as USD per current open volume of Position in lots. The Account charged in the Deposit currency.
  pub rollover_commission: Option<i64>,
  /// Initial period before the first rolloverCommission will be charged on the account.
  pub skip_rollover_days: Option<i32>,
  /// Time zone for the symbol trading intervals.
  pub schedule_time_zone: Option<String>,
  /// Rules for trading with the symbol. See ProtoOATradingMode for details.
  pub trading_mode: Option<i32>,
  /// Day of the week (in UTC) when Administrative Fee charge amount will be tripled. Applied only if RolloverChargePeriod = 0 or 1.
  pub rollover_commission3_days: Option<i32>,
  /// Specifies type of SWAP computation as PIPS (0) or PERCENTAGE (1, annual, in percent).
  pub swap_calculation_type: Option<i32>,
  /// Lot size of the Symbol (in cents).
  pub lot_size: Option<i64>,
  /// Commission base amount. Total commission depends on commissionType: for non-percentage types it is multiplied by 10^8, for percentage of value commission type it is multiplied by 10^5.
  pub precise_trading_commission_rate: Option<i64>,
  /// Minimum commission amount per trade multiplied by 10^8.
  pub precise_min_commission: Option<i64>,
  /// List of holidays for this symbol specified by broker.
  pub holiday: Vec<Holiday>,
  /// Percentage (1 = 0.01%) of the realized Gross Profit, which will be paid by the Trader for any trade if the Quote Asset of the traded Symbol is not matched with the Deposit Asset.
  pub pnl_conversion_fee_rate: Option<i32>,
  /// The unique identifier of dynamic leverage entity. <https://help.ctrader.com/ctrader/trading/dynamic-leverage>
  pub leverage_id: Option<i64>,
  /// Period of charging swaps in hours. 24 means swaps will be charged 1 time per day, 12 - every 12 hours, 8 - every 8 hours, etc.
  pub swap_period: Option<i32>,
  /// Time in minutes from 00:00 (UTC) when intraday swaps are charged for the first time.
  pub swap_time: Option<i32>,
  /// Count of swapPeriods before the first SWAP charge.
  pub skip_swap_periods: Option<i32>,
  /// If enabled, SWAP will be charged for all days of the week, including Saturday and Sunday.
  pub charge_swap_at_weekends: Option<bool>,
  /// Specifies the units in which the base Asset of the Symbol is denominated.
  pub measurement_units: Option<String>,
}

impl TryFrom<super::super::messages::ProtoOaSymbol> for Symbol {
  type Error = anyhow::Error;

  fn try_from(symbol: super::super::messages::ProtoOaSymbol) -> Result<Self, Self::Error> {
    Ok(Symbol {
      symbol_id: symbol.symbol_id,
      digits: symbol.digits,
      pip_position: symbol.pip_position,
      enable_short_selling: symbol.enable_short_selling,
      guaranteed_stop_loss: symbol.guaranteed_stop_loss,
      swap_rollover3_days: symbol.swap_rollover3_days,
      swap_long: symbol.swap_long,
      swap_short: symbol.swap_short,
      max_volume: symbol.max_volume,
      min_volume: symbol.min_volume,
      step_volume: symbol.step_volume,
      max_exposure: symbol.max_exposure,
      schedule: symbol.schedule.into_iter().map(Interval::from).collect(),
      commission_type: symbol.commission_type,
      sl_distance: symbol.sl_distance,
      tp_distance: symbol.tp_distance,
      gsl_distance: symbol.gsl_distance,
      gsl_charge: symbol.gsl_charge,
      distance_set_in: symbol.distance_set_in,
      min_commission_type: symbol.min_commission_type,
      min_commission_asset: symbol.min_commission_asset,
      rollover_commission: symbol.rollover_commission,
      skip_rollover_days: symbol.skip_rollover_days,
      schedule_time_zone: symbol.schedule_time_zone,
      trading_mode: symbol.trading_mode,
      rollover_commission3_days: symbol.rollover_commission3_days,
      swap_calculation_type: symbol.swap_calculation_type,
      lot_size: symbol.lot_size,
      precise_trading_commission_rate: symbol.precise_trading_commission_rate,
      precise_min_commission: symbol.precise_min_commission,
      holiday: symbol.holiday.into_iter().map(Holiday::from).collect(),
      pnl_conversion_fee_rate: symbol.pnl_conversion_fee_rate,
      leverage_id: symbol.leverage_id,
      swap_period: symbol.swap_period,
      swap_time: symbol.swap_time,
      skip_swap_periods: symbol.skip_swap_periods,
      charge_swap_at_weekends: symbol.charge_swap_at_weekends,
      measurement_units: symbol.measurement_units,
    })
  }
}
