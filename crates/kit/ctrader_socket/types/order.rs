use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
  /// The unique ID of the order. Note: trader might have two orders with the same id if orders are taken from accounts from different brokers.
  pub order_id: i64,
  /// Detailed trader data.
  pub trade_data: TradeData,
  /// Order type.
  pub order_type: OrderType,
  /// Order status.
  pub order_status: OrderStatus,
  /// The Unix time in milliseconds of expiration if the order has time in force GTD.
  pub expiration_timestamp: Option<i64>,
  /// Price at which an order was executed. For order with FILLED status.
  pub execution_price: Option<f64>,
  /// Part of the volume that was filled in cents (e.g. 1000 in protocol means 10.00 units).
  pub executed_volume: Option<i64>,
  /// The Unix time in milliseconds of the last update of the order.
  pub utc_last_update_timestamp: Option<i64>,
  /// Used for Market Range order with combination of slippageInPoints to specify price range were order can be executed.
  pub base_slippage_price: Option<f64>,
  /// Used for Market Range and STOP_LIMIT orders to to specify price range were order can be executed.
  pub slippage_in_points: Option<i64>,
  /// If TRUE then the order is closing part of whole position. Must have specified positionId.
  pub closing_order: Option<bool>,
  /// Valid only for LIMIT orders.
  pub limit_price: Option<f64>,
  /// Valid only for STOP and STOP_LIMIT orders.
  pub stop_price: Option<f64>,
  /// Absolute stopLoss price.
  pub stop_loss: Option<f64>,
  /// Absolute takeProfit price.
  pub take_profit: Option<f64>,
  /// Optional ClientOrderId. Max Length = 50 chars.
  pub client_order_id: Option<String>,
  /// Order's time in force. Depends on order type.
  pub time_in_force: Option<TimeInForce>,
  /// ID of the position linked to the order (e.g. closing order, order that increase volume of a specific position, etc.).
  pub position_id: Option<i64>,
  /// Relative stopLoss that can be specified instead of absolute as one. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782) For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
  pub relative_stop_loss: Option<i64>,
  /// Relative takeProfit that can be specified instead of absolute one. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782) ForBUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
  pub relative_take_profit: Option<i64>,
  /// If TRUE then order was stopped out from server side.
  pub is_stop_out: Option<bool>,
  /// If TRUE then order is trailingStopLoss. Valid for STOP_LOSS_TAKE_PROFIT order.
  pub trailing_stop_loss: Option<bool>,
  /// Trigger method for the order. Valid only for STOP and STOP_LIMIT orders.
  pub stop_trigger_method: Option<TriggerMethod>,
}

impl TryFrom<kit_ctrader_proto::ProtoOaOrder> for Order {
  type Error = anyhow::Error;

  fn try_from(order: kit_ctrader_proto::ProtoOaOrder) -> Result<Self, Self::Error> {
    Ok(Order {
      order_id: order.order_id,
      trade_data: TradeData {
        symbol_id: order.trade_data.symbol_id,
        volume: order.trade_data.volume,
        trade_side: TradeSide::try_from(order.trade_data.trade_side)?,
        open_timestamp: order.trade_data.open_timestamp,
        label: order.trade_data.label,
        guaranteed_stop_loss: order.trade_data.guaranteed_stop_loss,
        comment: order.trade_data.comment,
        measurement_units: order.trade_data.measurement_units,
        close_timestamp: order.trade_data.close_timestamp,
      },
      order_type: OrderType::try_from(order.order_type)?,
      order_status: OrderStatus::try_from(order.order_status)?,
      expiration_timestamp: order.expiration_timestamp,
      execution_price: order.execution_price,
      executed_volume: order.executed_volume,
      utc_last_update_timestamp: order.utc_last_update_timestamp,
      base_slippage_price: order.base_slippage_price,
      slippage_in_points: order.slippage_in_points,
      closing_order: order.closing_order,
      limit_price: order.limit_price,
      stop_price: order.stop_price,
      stop_loss: order.stop_loss,
      take_profit: order.take_profit,
      client_order_id: order.client_order_id,
      time_in_force: match order.time_in_force {
        Some(time_in_force) => Some(TimeInForce::try_from(time_in_force)?),
        None => None,
      },
      position_id: order.position_id,
      relative_stop_loss: order.relative_stop_loss,
      relative_take_profit: order.relative_take_profit,
      is_stop_out: order.is_stop_out,
      trailing_stop_loss: order.trailing_stop_loss,
      stop_trigger_method: match order.stop_trigger_method {
        Some(stop_trigger_method) => Some(TriggerMethod::try_from(stop_trigger_method)?),
        None => None,
      },
    })
  }
}
