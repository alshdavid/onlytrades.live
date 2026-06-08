use anyhow::Context;
use kit_ctrader_socket::*;

use crate::Ctx;
use crate::utils::ema::ExponentialMovingAverageExt;

pub async fn strategy(ctx: Ctx) -> anyhow::Result<()> {
  // Calculate EMA
  let ema20 = ta::indicators::ExponentialMovingAverage::new(20)?.calculate(&ctx.series);
  let ema9 = ta::indicators::ExponentialMovingAverage::new(9)?.calculate(&ctx.series);

  // Get current and last ema
  let current_ema20 = ema20.last().context("No current ema20")?;
  let prev_ema20 = ema20
    .get(ema20.len().saturating_sub(2))
    .context("No prev ema20")?;

  let current_ema9 = ema9.last().context("No current ema9")?;
  let prev_ema9 = ema9
    .get(ema9.len().saturating_sub(2))
    .context("No prev ema9")?;

  // 5. Detect Crossover
  let crossed_above = prev_ema9 <= prev_ema20 && current_ema9 > current_ema20;
  let crossed_below = prev_ema9 >= prev_ema20 && current_ema9 < current_ema20;

  // Open long if ema9 crosses over ema20
  // Open short if ema9 crosses below ema20
  if crossed_above {
    println!(
      "🚀 BULLISH CROSSOVER detected! EMA9: {:.5} > EMA20: {:.5}",
      current_ema9, current_ema20
    );

    ctx.close_all_positions().await?;

    ctx
      .new_order(NewOrderReq {
        client_msg_id: None,
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Buy,
        volume: 10,
        ..NewOrderReq::default()
      })
      .await?;
  } else if crossed_below {
    println!(
      "📉 BEARISH CROSSOVER detected! EMA9: {:.5} < EMA20: {:.5}",
      current_ema9, current_ema20
    );

    ctx.close_all_positions().await?;

    ctx
      .new_order(NewOrderReq {
        client_msg_id: None,
        ctid_trader_account_id: ctx.account_id,
        symbol_id: ctx.symbol.symbol_id,
        order_type: OrderType::Market,
        trade_side: TradeSide::Sell,
        volume: 10,
        ..NewOrderReq::default()
      })
      .await?;
  }

  Ok(())
}
