use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
  /// The unique ID of the position. Note: trader might have two positions with the same id if positions are taken from accounts from different brokers.
  pub position_id: i64,
  /// Position details. See ProtoOATradeData for details.
  pub trade_data: TradeData,
  /// Current status of the position.
  pub position_status: PositionStatus,
  /// Total amount of charged swap on open position.
  pub swap: i64,
  /// VWAP price of the position based on all executions (orders) linked to the position.
  pub price: Option<f64>,
  /// Current stop loss price.
  pub stop_loss: Option<f64>,
  /// Current take profit price.
  pub take_profit: Option<f64>,
  /// The Unix time in milliseconds of the last change of the position, including amend SL/TP of the position, execution of related order, cancel or related order, etc.
  pub utc_last_update_timestamp: Option<i64>,
  /// Current unrealized commission related to the position.
  pub commission: Option<i64>,
  /// Rate for used margin computation. Represented as Base/Deposit.
  pub margin_rate: Option<f64>,
  /// Amount of unrealized commission related to following of strategy provider.
  pub mirroring_commission: Option<i64>,
  /// If TRUE then position's stop loss is guaranteedStopLoss.
  pub guaranteed_stop_loss: Option<bool>,
  /// Amount of margin used for the position in deposit currency.
  pub used_margin: Option<u64>,
  /// Stop trigger method for SL/TP of the position.
  pub stop_loss_trigger_method: Option<TriggerMethod>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects swap, commission, mirroringCommission, usedMargin.
  pub money_digits: Option<u32>,
  /// If TRUE then the Trailing Stop Loss is applied.
  pub trailing_stop_loss: Option<bool>,
}

impl TryFrom<super::super::messages::ProtoOaPosition> for Position {
  type Error = anyhow::Error;

  fn try_from(position: super::super::messages::ProtoOaPosition) -> Result<Self, Self::Error> {
    Ok(Position {
      position_id: position.position_id,
      trade_data: TradeData::try_from(position.trade_data)?,
      position_status: PositionStatus::try_from(position.position_status)?,
      swap: position.swap,
      price: position.price,
      stop_loss: position.stop_loss,
      take_profit: position.take_profit,
      utc_last_update_timestamp: position.utc_last_update_timestamp,
      commission: position.commission,
      margin_rate: position.margin_rate,
      mirroring_commission: position.mirroring_commission,
      guaranteed_stop_loss: position.guaranteed_stop_loss,
      used_margin: position.used_margin,
      stop_loss_trigger_method: match position.stop_loss_trigger_method {
        Some(v) => Some(TriggerMethod::try_from(v)?),
        None => None,
      },
      money_digits: position.money_digits,
      trailing_stop_loss: position.trailing_stop_loss,
    })
  }
}
