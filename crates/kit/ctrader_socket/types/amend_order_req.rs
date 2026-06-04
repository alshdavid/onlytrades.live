use kit_ctrader_proto::ProtoOaAmendOrderReq;
use num_enum::TryFromPrimitive;

use super::OrderTriggerMethod;

/// * Request for amending the existing pending order. Allowed only if the Access Token has "trade" permissions for the trading account.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AmendOrderReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the order.
  pub order_id: i64,
  /// Volume, represented in 0.01 of a unit (e.g. 1000 in protocol means 10.00 units).
  pub volume: Option<i64>,
  /// The Limit Price, can be specified for the LIMIT order only.
  pub limit_price: Option<f64>,
  /// The Stop Price, can be specified for the STOP and the STOP_LIMIT orders.
  pub stop_price: Option<f64>,
  /// The Unix timestamp in milliseconds of Order expiration. Should be set for the Good Till Date orders.
  pub expiration_timestamp: Option<i64>,
  /// The absolute Stop Loss price (e.g. 1.23456). Not supported for MARKET orders.
  pub stop_loss: Option<f64>,
  /// The absolute Take Profit price (e.g. 1.23456). Not supported for MARKET orders.
  pub take_profit: Option<f64>,
  /// Slippage distance for the MARKET_RANGE and the STOP_LIMIT orders.
  pub slippage_in_points: Option<i32>,
  /// The relative Stop Loss can be specified instead of the absolute one. Specified in 1/100000 of a unit of price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782) For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
  pub relative_stop_loss: Option<i64>,
  /// The relative Take Profit can be specified instead of the absolute one. Specified in 1/100000 of a unit of price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782) For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
  pub relative_take_profit: Option<i64>,
  /// If TRUE then the Stop Loss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
  pub guaranteed_stop_loss: Option<bool>,
  /// If TRUE then the Trailing Stop Loss is applied.
  pub trailing_stop_loss: Option<bool>,
  /// Trigger method for the STOP or the STOP_LIMIT pending order.
  pub stop_trigger_method: Option<OrderTriggerMethod>,
}

impl TryFrom<ProtoOaAmendOrderReq> for AmendOrderReq {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaAmendOrderReq) -> Result<Self, Self::Error> {
    Ok(AmendOrderReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      order_id: value.order_id,
      volume: value.volume,
      limit_price: value.limit_price,
      stop_price: value.stop_price,
      expiration_timestamp: value.expiration_timestamp,
      stop_loss: value.stop_loss,
      take_profit: value.take_profit,
      slippage_in_points: value.slippage_in_points,
      relative_stop_loss: value.relative_stop_loss,
      relative_take_profit: value.relative_take_profit,
      guaranteed_stop_loss: value.guaranteed_stop_loss,
      trailing_stop_loss: value.trailing_stop_loss,
      stop_trigger_method: match value.stop_trigger_method {
        Some(stop_trigger_method) => {
          Some(OrderTriggerMethod::try_from_primitive(stop_trigger_method)?)
        }
        None => None,
      },
    })
  }
}
