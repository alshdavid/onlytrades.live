// Bollinger Bands + RSI Mean Reversion Strategy
// Enters long when price < BB lower & RSI oversold
// Enters short when price > BB upper & RSI overbought
// Exits when price reverts to BB middle (TP) or hits stop loss

const BB_LENGTH = 20;
const BB_MULT = 2.0;
const RSI_LENGTH = 14;
const RSI_OVERSOLD = 30;
const RSI_OVERBOUGHT = 70;
const STOP_LOSS_PCT = 0.025; // 2.5%
const VOLUME = 100;

class State {
  position: "flat" | "long" | "short" = "flat";
  long_stop_price: number | undefined = undefined;
  short_stop_price: number | undefined = undefined;
}

// Runs on every candle close
export default async function handler(ctx: Context<State>) {
  const prices = ctx.series.map((bar) => bar.close);

  if (prices.length < Math.max(BB_LENGTH, RSI_LENGTH + 1)) {
    // Not enough data yet — skip this tick
    return;
  }

  const currentClose = prices[prices.length - 1];

  // Bollinger Bands
  const bb = bollingerBands(prices, BB_LENGTH, BB_MULT);
  if (!bb) return;
  const { basis, upper, lower } = bb;

  // RSI
  const rsiValues = RelativeStrengthIndex.calculate(RSI_LENGTH, ctx.series);
  const rsiValue = rsiValues[rsiValues.length - 1];

  // ENTRY CONDITIONS
  if (currentClose < lower && rsiValue < RSI_OVERSOLD && ctx.state.position === "flat") {
    console.log(
      `[${ctx.name}] 📈 LONG ENTRY — close ${currentClose.toFixed(0)} < BB lower ${lower.toFixed(0)}, RSI ${rsiValue.toFixed(1)}`,
    );

    const relativeStopLoss = currentClose * STOP_LOSS_PCT;

    await ctx.new_order({
      ctid_trader_account_id: ctx.account_id,
      symbol_id: ctx.symbol.symbol_id,
      order_type: OrderType.MARKET,
      trade_side: TradeSide.BUY,
      relative_stop_loss: relativeStopLoss,
      volume: VOLUME,
    });

    ctx.state.long_stop_price = currentClose * (1 - STOP_LOSS_PCT);
    ctx.state.position = "long";
  } else if (
    currentClose > upper &&
    rsiValue > RSI_OVERBOUGHT &&
    ctx.state.position === "flat"
  ) {
    console.log(
      `[${ctx.name}] 📉 SHORT ENTRY — close ${currentClose.toFixed(0)} > BB upper ${upper.toFixed(0)}, RSI ${rsiValue.toFixed(1)}`,
    );

    const relativeStopLoss = currentClose * STOP_LOSS_PCT;

    await ctx.new_order({
      ctid_trader_account_id: ctx.account_id,
      symbol_id: ctx.symbol.symbol_id,
      order_type: OrderType.MARKET,
      trade_side: TradeSide.SELL,
      relative_stop_loss: relativeStopLoss,
      volume: VOLUME,
    });

    ctx.state.short_stop_price = currentClose * (1 + STOP_LOSS_PCT);
    ctx.state.position = "short";
  }

  // EXIT CONDITIONS — LONG
  if (ctx.state.position === "long") {
    const hitTakeProfit = currentClose >= basis;
    const hitStopLoss =
      ctx.state.long_stop_price !== undefined && currentClose <= ctx.state.long_stop_price;

    if (hitTakeProfit) {
      console.log(
        `[${ctx.name}] ✅ LONG TP — price reverted to mean (${basis.toFixed(0)})`,
      );
      await ctx.close_all_positions();
      ctx.state.long_stop_price = undefined;
      ctx.state.position = "flat";
    } else if (hitStopLoss) {
      console.log(
        `[${ctx.name}] 🛑 LONG SL — stop loss hit at ${ctx.state.long_stop_price!.toFixed(0)}`,
      );
      await ctx.close_all_positions();
      ctx.state.long_stop_price = undefined;
      ctx.state.position = "flat";
    }
  }

  // EXIT CONDITIONS — SHORT
  if (ctx.state.position === "short") {
    const hitTakeProfit = currentClose <= basis;
    const hitStopLoss =
      ctx.state.short_stop_price !== undefined && currentClose >= ctx.state.short_stop_price;

    if (hitTakeProfit) {
      console.log(
        `[${ctx.name}] ✅ SHORT TP — price reverted to mean (${basis.toFixed(0)})`,
      );
      await ctx.close_all_positions();
      ctx.state.short_stop_price = undefined;
      ctx.state.position = "flat";
    } else if (hitStopLoss) {
      console.log(
        `[${ctx.name}] 🛑 SHORT SL — stop loss hit at ${ctx.state.short_stop_price!.toFixed(0)}`,
      );
      await ctx.close_all_positions();
      ctx.state.short_stop_price = undefined;
      ctx.state.position = "flat";
    }
  }
}

function bollingerBands(
  prices: number[],
  length: number,
  mult: number,
): { basis: number; upper: number; lower: number } | undefined {
  if (prices.length < length) return undefined;

  const window = prices.slice(prices.length - length);
  const basis = window.reduce((a, b) => a + b, 0) / length;
  const variance =
    window.reduce((acc, p) => acc + (p - basis) ** 2, 0) / length;
  const stdDev = Math.sqrt(variance);

  return { basis, upper: basis + mult * stdDev, lower: basis - mult * stdDev };
}
