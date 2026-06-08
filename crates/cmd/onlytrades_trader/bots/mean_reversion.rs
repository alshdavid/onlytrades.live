use anyhow::Context;
use kit_ctrader_socket::*;

use crate::run_strategy::Ctx;
use crate::run_strategy::StrategyPosition;
use crate::run_strategy::StrategyState;
use crate::utils::ema::ExponentialMovingAverageExt;

const BB_LENGTH: usize = 20;
const BB_MULT: f64 = 2.0;
const RSI_LENGTH: usize = 14;
const RSI_OVERSOLD: f64 = 30.0;
const RSI_OVERBOUGHT: f64 = 70.0;
const STOP_LOSS_PCT: f64 = 0.025; // 2.5%

fn bollinger_bands(
  prices: &[f64],
  length: usize,
  mult: f64,
) -> Option<(f64, f64, f64)> {
  if prices.len() < length {
    return None;
  }
  let window = &prices[prices.len() - length..];
  let basis = window.iter().sum::<f64>() / length as f64;
  let variance = window.iter().map(|p| (p - basis).powi(2)).sum::<f64>() / length as f64;
  let std_dev = variance.sqrt();
  Some((basis, basis + mult * std_dev, basis - mult * std_dev))
}

pub async fn strategy(ctx: Ctx) -> anyhow::Result<()> {
  let prices: Vec<f64> = ctx
    .series
    .iter()
    .map(|bar| bar.close_price() as f64)
    .collect();

  if prices.len() < BB_LENGTH.max(RSI_LENGTH + 1) {
    // Not enough data yet — skip this tick
    return Ok(());
  }

  let current_close = *prices.last().context("Empty price series")?;

  // BOLLINGER BANDS
  let (bb_basis, bb_upper, bb_lower) =
    bollinger_bands(&prices, BB_LENGTH, BB_MULT).context("BB calculation failed")?;

  // RSI
  let rsi_values = ta::indicators::RelativeStrengthIndex::new(RSI_LENGTH)?.calculate(&ctx.series);
  let rsi_val = rsi_values.last().context("No RSI value")?;

  let mut state = ctx.state.lock().await;

  let long_stop_price = state.long_stop_price; // Option<f64>
  let short_stop_price = state.short_stop_price; // Option<f64>

  let long_condition = current_close < bb_lower && *rsi_val < RSI_OVERSOLD;
  let short_condition = current_close > bb_upper && *rsi_val > RSI_OVERBOUGHT;

  // ENTRY CONDITIONS
  if long_condition && state.position == StrategyPosition::Flat {
    println!(
      "📈 LONG ENTRY — close {:.0} < BB lower {:.0}, RSI {:.1}",
      current_close, bb_lower, rsi_val
    );

    // Convert stop loss % to relative points (broker units are 1/100000 of price)
    let sl_relative = (current_close * STOP_LOSS_PCT * 100_000.0) as i64;

    ctx
      .new_order(NewOrderReq {
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Buy,
        relative_stop_loss: Some(sl_relative), // broker-side backstop only
        volume: ctx.volume,
        ..NewOrderReq::default()
      })
      .await?;

    state.long_stop_price = Some(current_close * (1.0 - STOP_LOSS_PCT));
    state.position = StrategyPosition::Long;
  } else if short_condition && state.position == StrategyPosition::Flat {
    println!(
      "📉 SHORT ENTRY — close {:.0} > BB upper {:.0}, RSI {:.1}",
      current_close, bb_upper, rsi_val
    );

    let sl_relative = (current_close * STOP_LOSS_PCT * 100_000.0) as i64;

    ctx
      .new_order(NewOrderReq {
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Sell,
        relative_stop_loss: Some(sl_relative), // broker-side backstop only
        volume: ctx.volume,
        ..NewOrderReq::default()
      })
      .await?;
    state.position = StrategyPosition::Short;
    state.short_stop_price = Some(current_close * (1.0 + STOP_LOSS_PCT));
  }

  // EXIT CONDITIONS
  if state.position == StrategyPosition::Long {
    let hit_take_profit = current_close >= bb_basis;
    let hit_stop_loss = long_stop_price.map_or(false, |sl| current_close <= sl);

    if hit_take_profit {
      println!("✅ LONG TP — price reverted to mean ({:.0})", bb_basis);
      ctx.close_all_positions().await?;
      state.long_stop_price = None;
      state.position = StrategyPosition::Flat;
    } else if hit_stop_loss {
      println!(
        "🛑 LONG SL — stop loss hit at {:.0}",
        long_stop_price.unwrap()
      );
      ctx.close_all_positions().await?;
      state.long_stop_price = None;
      state.position = StrategyPosition::Flat;
    }
  }

  if state.position == StrategyPosition::Short {
    let hit_take_profit = current_close <= bb_basis;
    let hit_stop_loss = short_stop_price.map_or(false, |sl| current_close >= sl);

    if hit_take_profit {
      println!("✅ SHORT TP — price reverted to mean ({:.0})", bb_basis);
      ctx.close_all_positions().await?;
      state.short_stop_price = None;
      state.position = StrategyPosition::Flat;
    } else if hit_stop_loss {
      println!(
        "🛑 SHORT SL — stop loss hit at {:.0}",
        short_stop_price.unwrap()
      );
      ctx.close_all_positions().await?;
      state.short_stop_price = None;
      state.position = StrategyPosition::Flat;
    }
  }

  Ok(())
}
