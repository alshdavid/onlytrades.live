// TODO

export default async function handler(ctx: IContext): Promise<void> {
  // Stream that emits price data on each tick
  ctx.onSpotEvent((event) => {
    console.log({
      symbol_id: event.symbol_id,   // Ticker
      bid: event.bid,               // Buy price (includes spread)
      ask: event.ask,               // Sell price (includes spread)
      trendbar: event.trendbar,     // Can request candle information
      timestamp: event.timestamp,   // Timestamp for tick event
    })
  })

  const symbols = await ctx.symbolsList()

  // This tells the broker we want to receive 
  // updates for the supplied symbols
  ctx.subscribeSpots({
    symbols: [symbols.get('US500')!]
  }) 
}
