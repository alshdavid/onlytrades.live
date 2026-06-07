use kit_ctrader_socket::connection::CTraderConnectionExt;
use kit_ctrader_socket::*;

use crate::Ctx;
use crate::utils::ema::EmaEngine;

pub async fn strategy(ctx: Ctx) -> anyhow::Result<()> {
  // 1. Need at least 21 bars to have a valid previous EMA20 and current EMA20
  if ctx.series.len() < 21 {
    return Ok(());
  }

  // 2. Prepare the full price set (Current State: t)
  let prices_curr = ctx
    .series
    .iter()
    .map(|v| v.close_price())
    .collect::<Vec<_>>();

  // 3. Prepare the previous price set (Previous State: t-1)
  // We slice the series to exclude the last bar
  let prices_prev = &prices_curr[..prices_curr.len() - 1];

  // 4. Calculate EMAs for both states
  let Some(curr9) = EmaEngine::calculate(&prices_curr, 9) else {
    return Ok(());
  };
  let Some(curr20) = EmaEngine::calculate(&prices_curr, 20) else {
    return Ok(());
  };
  let Some(prev9) = EmaEngine::calculate(prices_prev, 9) else {
    return Ok(());
  };
  let Some(prev20) = EmaEngine::calculate(prices_prev, 20) else {
    return Ok(());
  };

  // 5. Detect Crossover
  let crossed_above = prev9 <= prev20 && curr9 > curr20;
  let crossed_below = prev9 >= prev20 && curr9 < curr20;

  if !ctx.live {
    return Ok(());
  }

  if crossed_above {
    println!(
      "🚀 BULLISH CROSSOVER detected! EMA9: {:.5} > EMA20: {:.5}",
      curr9, curr20
    );

    let reconciled = ctx.conn.reconcile(&ctx.account_id).await?;
    for position in reconciled.position {
      ctx
        .conn
        .close_position(ClosePositionReq {
          client_msg_id: None,
          ctid_trader_account_id: ctx.account_id,
          position_id: position.position_id,
          volume: 10,
        })
        .await?;
    }

    ctx
      .conn
      .new_order(NewOrderReq {
        client_msg_id: None,
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Buy,
        volume: 10,
        limit_price: None,
        stop_price: None,
        time_in_force: None,
        expiration_timestamp: None,
        stop_loss: None,
        take_profit: None,
        comment: None,
        base_slippage_price: None,
        slippage_in_points: None,
        label: None,
        position_id: None,
        client_order_id: None,
        relative_stop_loss: None,
        relative_take_profit: None,
        guaranteed_stop_loss: None,
        trailing_stop_loss: None,
        stop_trigger_method: None,
      })
      .await?;
  } else if crossed_below {
    println!(
      "📉 BEARISH CROSSOVER detected! EMA9: {:.5} < EMA20: {:.5}",
      curr9, curr20
    );

    let reconciled = ctx.conn.reconcile(&ctx.account_id).await?;
    for position in reconciled.position {
      ctx
        .conn
        .close_position(ClosePositionReq {
          client_msg_id: None,
          ctid_trader_account_id: ctx.account_id,
          position_id: position.position_id,
          volume: 10,
        })
        .await?;
    }

    ctx
      .conn
      .new_order(NewOrderReq {
        client_msg_id: None,
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Sell,
        volume: 10,
        limit_price: None,
        stop_price: None,
        time_in_force: None,
        expiration_timestamp: None,
        stop_loss: None,
        take_profit: None,
        comment: None,
        base_slippage_price: None,
        slippage_in_points: None,
        label: None,
        position_id: None,
        client_order_id: None,
        relative_stop_loss: None,
        relative_take_profit: None,
        guaranteed_stop_loss: None,
        trailing_stop_loss: None,
        stop_trigger_method: None,
      })
      .await?;
  }

  // println!("EMA20: {}  EMA9: {}", ema20, ema9);
  Ok(())
}

// println!("Executing order");
// ctx.conn.send(CTraderRequestType::NewOrderReq(NewOrderReq {
//   client_msg_id: None,
//   ctid_trader_account_id: ctx.account_id,
//   symbol_id: ctx.symbol.symbol_id,
//   order_type: OrderType::Market,
//   trade_side: TradeSide::Buy,
//   volume: 10,
//   limit_price: None,
//   stop_price: None,
//   time_in_force: None,
//   expiration_timestamp: None,
//   stop_loss: None,
//   take_profit: None,
//   comment: None,
//   base_slippage_price: None,
//   slippage_in_points: None,
//   label: None,
//   position_id: None,
//   client_order_id: None,
//   relative_stop_loss: None,
//   relative_take_profit: None,
//   guaranteed_stop_loss: None,
//   trailing_stop_loss: None,
//   stop_trigger_method: None,
// })).await?;
