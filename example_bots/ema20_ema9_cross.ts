// Runs on every candle close
export default async function handler(ctx: Context) {
  // Calculate EMAs
  const ema20Builder = new ExponentialMovingAverage(20);
  const ema9Builder = new ExponentialMovingAverage(9);

  for (const trendbar of ctx.series) {
    ema20Builder.next(trendbar.close);
    ema9Builder.next(trendbar.close);
  }

  const ema20 = ema20Builder.calculate();
  const ema9 = ema9Builder.calculate();

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
