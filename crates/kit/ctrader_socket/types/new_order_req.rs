use kit_ctrader_proto::ProtoOaNewOrderReq;
use num_enum::TryFromPrimitive;

use super::OrderTriggerMethod;
use super::OrderType;
use super::TimeInForce;
use super::TradeSide;

/// * Request for sending a new trading order. Allowed only if the accessToken has the "trade" permissions for the trading account.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct NewOrderReq {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
  /// The unique identifier of a symbol in cTrader platform.
  pub symbol_id: i64,
  /// The type of an order - MARKET, LIMIT, STOP, MARKET_RANGE, STOP_LIMIT.
  pub order_type: OrderType,
  /// The trade direction - BUY or SELL.
  pub trade_side: TradeSide,
  /// The volume represented in 0.01 of a unit (e.g. 1000 in protocol means 10.00 units).
  pub volume: i64,
  /// The limit price, can be specified for the LIMIT order only.
  pub limit_price: Option<f64>,
  /// Stop Price, can be specified for the STOP and the STOP_LIMIT orders only.
  pub stop_price: Option<f64>,
  /// The specific order execution or expiration instruction - GOOD_TILL_DATE, GOOD_TILL_CANCEL, IMMEDIATE_OR_CANCEL, FILL_OR_KILL, MARKET_ON_OPEN.
  pub time_in_force: Option<TimeInForce>,
  /// The Unix time in milliseconds of Order expiration. Should be set for the Good Till Date orders.
  pub expiration_timestamp: Option<i64>,
  /// The absolute Stop Loss price (1.23456 for example). Not supported for MARKET orders.
  pub stop_loss: Option<f64>,
  /// The absolute Take Profit price (1.23456 for example). Unsupported for MARKET orders.
  pub take_profit: Option<f64>,
  /// User-specified comment. MaxLength = 512.
  pub comment: Option<String>,
  /// Base price to calculate relative slippage price for MARKET_RANGE order.
  pub base_slippage_price: Option<f64>,
  /// Slippage distance for MARKET_RANGE and STOP_LIMIT order.
  pub slippage_in_points: Option<i32>,
  /// User-specified label. MaxLength = 100.
  pub label: Option<String>,
  /// Reference to the existing position if the Order is intended to modify it.
  pub position_id: Option<i64>,
  /// Optional user-specific clientOrderId (similar to FIX ClOrderID). MaxLength = 50.
  pub client_order_id: Option<String>,
  /// Relative Stop Loss that can be specified instead of the absolute as one. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782) For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
  pub relative_stop_loss: Option<i64>,
  /// Relative Take Profit that can be specified instead of the absolute one. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782) For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
  pub relative_take_profit: Option<i64>,
  /// If TRUE then stopLoss is guaranteed. Required to be set to TRUE for the Limited Risk accounts (ProtoOATrader.isLimitedRisk=true).
  pub guaranteed_stop_loss: Option<bool>,
  /// If TRUE then the Stop Loss is Trailing.
  pub trailing_stop_loss: Option<bool>,
  /// Trigger method for the STOP or the STOP_LIMIT pending order.
  pub stop_trigger_method: Option<OrderTriggerMethod>,
}

impl TryFrom<ProtoOaNewOrderReq> for NewOrderReq {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaNewOrderReq) -> Result<Self, Self::Error> {
    Ok(NewOrderReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
      order_type: OrderType::try_from_primitive(value.order_type)?,
      trade_side: TradeSide::try_from_primitive(value.trade_side)?,
      volume: value.volume,
      limit_price: value.limit_price,
      stop_price: value.stop_price,
      time_in_force: value
        .time_in_force
        .map(TimeInForce::try_from_primitive)
        .transpose()?,
      expiration_timestamp: value.expiration_timestamp,
      stop_loss: value.stop_loss,
      take_profit: value.take_profit,
      comment: value.comment,
      base_slippage_price: value.base_slippage_price,
      slippage_in_points: value.slippage_in_points,
      label: value.label,
      position_id: value.position_id,
      client_order_id: value.client_order_id,
      relative_stop_loss: value.relative_stop_loss,
      relative_take_profit: value.relative_take_profit,
      guaranteed_stop_loss: value.guaranteed_stop_loss,
      trailing_stop_loss: value.trailing_stop_loss,
      stop_trigger_method: value
        .stop_trigger_method
        .map(OrderTriggerMethod::try_from_primitive)
        .transpose()?,
    })
  }
}
