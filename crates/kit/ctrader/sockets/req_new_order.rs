use super::CTraderSocketClient;
use super::ProtoMessageParse;
use kit_ctrader_proto::*;
use super::types::*;

pub struct CTraderNewOrderOptions {
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

impl CTraderSocketClient {
  pub async fn new_order(
    &self,
    options: CTraderNewOrderOptions,
  ) -> anyhow::Result<ExecutionEvent> {
    let mut rx = self
      .send_and_subscribe(
        ProtoOaPayloadType::ProtoOaNewOrderReq,
        ProtoOaNewOrderReq {
          payload_type: None,
          ctid_trader_account_id: options.ctid_trader_account_id,
          symbol_id: options.symbol_id,
          order_type: options.order_type.into(),
          trade_side: options.trade_side.into(),
          volume: options.volume,
          limit_price: options.limit_price,
          stop_price: options.stop_price,
          time_in_force: options.time_in_force.map(|v| v.into()),
          expiration_timestamp: options.expiration_timestamp,
          stop_loss: options.stop_loss,
          take_profit: options.take_profit,
          comment: options.comment,
          base_slippage_price: options.base_slippage_price,
          slippage_in_points: options.slippage_in_points,
          label: options.label,
          position_id: options.position_id,
          client_order_id: options.client_order_id,
          relative_stop_loss: options.relative_stop_loss,
          relative_take_profit: options.relative_take_profit,
          guaranteed_stop_loss: options.guaranteed_stop_loss,
          trailing_stop_loss: options.trailing_stop_loss,
          stop_trigger_method: options.stop_trigger_method.map(|v| v.into()),
        },
      )
      .await?;

    while let Some(result) = rx.recv().await {
      match result {
        Err(err) => return Err(err.into()),
        Ok(msg) => match msg.payload_type {
          2126 => {
            let ev =
              ExecutionEvent::try_from(msg.try_decode_body::<ProtoOaExecutionEvent>()?)?;
            match ev.execution_type {
              ExecutionType::OrderFilled => {
                return Ok(ev);
              }
              _ => continue,
            }
          }
          _ => {
            anyhow::bail!("Invalid message type for request")
          }
        },
      };
    }

    anyhow::bail!("request terminated early")
  }
}
