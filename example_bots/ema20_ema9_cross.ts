// Runs on every candle close
export default async function handler(ctx: Context) {
  // Calculate EMAs
  const ema20 = ExponentialMovingAverage.calculate(20, ctx.series);
  const ema9 = ExponentialMovingAverage.calculate(9, ctx.series);

  // Get EMA for current and last candles
  const current_ema20 = ema20[ema20.length - 1];
  const current_ema9 = ema9[ema9.length - 1];

  const prev_ema20 = ema20[ema20.length - 2];
  const prev_ema9 = ema9[ema9.length - 2];

  console.info(`EMA9: ${current_ema9} > EMA20: ${current_ema20}`);

  // Calculate cross
  const crossed_above = prev_ema9 <= prev_ema20 && current_ema9 > current_ema20;
  const crossed_below = prev_ema9 >= prev_ema20 && current_ema9 < current_ema20;

  // Execute trade
  if (crossed_above) {
    console.log(`🚀 BULLISH CROSSOVER!`);

    await ctx.close_all_positions();
    await ctx.new_order({
      ctid_trader_account_id: ctx.account_id,
      symbol_id: ctx.symbol.symbol_id,
      order_type: OrderType.MARKET,
      trade_side: TradeSide.BUY,
      volume: 100,
    });
  } else if (crossed_below) {
    console.log(`📉 BEARISH CROSSOVER!`);

    await ctx.close_all_positions();
    await ctx.new_order({
      ctid_trader_account_id: ctx.account_id,
      symbol_id: ctx.symbol.symbol_id,
      order_type: OrderType.MARKET,
      trade_side: TradeSide.SELL,
      volume: 100,
    });
  }
}
