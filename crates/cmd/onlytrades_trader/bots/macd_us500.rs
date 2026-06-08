use kit_ctrader_socket::*;
use ta::Next;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;

use crate::run_strategy::Ctx;

pub async fn strategy(ctx: Ctx) -> anyhow::Result<()> {
  let series = &ctx.series;

  if series.len() < 5 {
    return Ok(());
  }

  // --- Calculate MACD[12,26,9] ---
  let mut macd = Macd::new(12, 26, 9)?;
  let macd_values: Vec<_> = series
    .iter()
    .map(|candle| macd.next(candle.close_price() as f64))
    .collect();

  // [0] = latest, [1] = one bar ago, etc.
  let len = macd_values.len();
  if len < 5 {
    return Ok(());
  }

  // The MACD output is typically a struct with `.macd` (the line) and `.signal`
  let macd_0 = macd_values[len - 1].macd;
  let macd_1 = macd_values[len - 2].macd;
  let macd_2 = macd_values[len - 3].macd;
  let macd_3 = macd_values[len - 4].macd;
  let macd_4 = macd_values[len - 5].macd;

  // --- OHLC values ---
  let close_0 = series[series.len() - 1].close_price() as f64;
  let close_1 = series[series.len() - 2].close_price() as f64;
  let high_0 = series[series.len() - 1].high_price() as f64;
  let high_1 = series[series.len() - 2].high_price() as f64;
  let low_0 = series[series.len() - 1].low_price() as f64;

  // --- Entry Conditions ---
  let entry_condition1 = macd_0 < macd_1 && macd_1 < macd_2 && macd_2 < macd_3 && macd_3 < macd_4;

  let entry_condition2 = macd_0 < 0.0 && close_0 < close_1;

  let bar_range = high_0 - low_0;
  let entry_condition3 = if bar_range > 0.0 {
    (close_0 - low_0) / bar_range < 0.13
  } else {
    false
  };

  // --- Exit Condition ---
  let exit_condition = close_0 > high_1;

  // --- Execution ---
  if exit_condition {
    if ctx.live {
      println!(
        "🚪 EXIT: Close {:.5} broke above prev high {:.5}",
        close_0, high_1
      );
    }
    ctx.close_all_positions().await?;
  } else if entry_condition1 && entry_condition2 && entry_condition3 {
    if ctx.live {
      println!(
        "🚀 ENTRY: MACD falling ({:.5}), negative ({:.5}), bearish close ratio ({:.3})",
        macd_0,
        macd_0,
        (close_0 - low_0) / bar_range
      );
    }
    ctx.close_all_positions().await?;
    ctx
      .new_order(NewOrderReq {
        client_msg_id: None,
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Buy,
        relative_stop_loss: Some(1_000_000),
        relative_take_profit: Some(2_000_000),
        trailing_stop_loss: Some(true),
        volume: 10,
        ..NewOrderReq::default()
      })
      .await?;
  }

  Ok(())
}
