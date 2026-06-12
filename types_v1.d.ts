// ENUMS

declare const OrderType: {
  readonly MARKET: "Market";
  readonly LIMIT: "Limit";
  readonly STOP: "Stop";
  readonly STOP_LOSS_TAKE_PROFIT: "StopLossTakeProfit";
  readonly MARKET_RANGE: "MarketRange";
  readonly STOP_LIMIT: "StopLimit";
};
type OrderType = (typeof OrderType)[keyof typeof OrderType];

declare const TradeSide: {
  readonly BUY: "Buy";
  readonly SELL: "Sell";
};
type TradeSide = (typeof TradeSide)[keyof typeof TradeSide];

declare const TrendbarPeriod: {
  readonly M1: 1;
  readonly M2: 2;
  readonly M3: 3;
  readonly M4: 4;
  readonly M5: 5;
  readonly M10: 6;
  readonly M15: 7;
  readonly M30: 8;
  readonly H1: 9;
  readonly H4: 10;
  readonly H12: 11;
  readonly D1: 12;
  readonly W1: 13;
  readonly MN1: 14;
};
type TrendbarPeriod = (typeof TrendbarPeriod)[keyof typeof TrendbarPeriod];

declare const DayOfWeek: {
  readonly NONE: 0;
  readonly MONDAY: 1;
  readonly TUESDAY: 2;
  readonly WEDNESDAY: 3;
  readonly THURSDAY: 4;
  readonly FRIDAY: 5;
  readonly SATURDAY: 6;
  readonly SUNDAY: 7;
};
type DayOfWeek = (typeof DayOfWeek)[keyof typeof DayOfWeek];

/**
 * Enum for specifying stop loss and take profit distances.
 */
declare const SymbolDistanceType: {
  SYMBOL_DISTANCE_IN_POINTS: 1;
  SYMBOL_DISTANCE_IN_PERCENTAGE: 2;
};
type SymbolDistanceType =
  (typeof SymbolDistanceType)[keyof typeof SymbolDistanceType];

/**
 * Enum for specifying symbol trading mode.
 */
declare const TradingMode: {
  ENABLED: 0;
  DISABLED_WITHOUT_PENDINGS_EXECUTION: 1;
  DISABLED_WITH_PENDINGS_EXECUTION: 2;
  CLOSE_ONLY_MODE: 3;
};
type TradingMode = (typeof TradingMode)[keyof typeof TradingMode];

/**
 * Enum for specifying SWAP calculation type for symbol.
 */
declare const SwapCalculationType: {
  /** Specifies type of SWAP computation as PIPS (0) */
  PIPS: 0;
  /** Specifies type of SWAP computation as PERCENTAGE (1, annual, in percent) */
  PERCENTAGE: 1;
  /** Specifies type of SWAP computation as POINTS (2) */
  POINTS: 2;
};
type SwapCalculationType =
  (typeof SwapCalculationType)[keyof typeof SwapCalculationType];

/**
 * Order's time in force ENUM.
 */
declare const TimeInForce: {
  GOOD_TILL_DATE: 1;
  GOOD_TILL_CANCEL: 2;
  IMMEDIATE_OR_CANCEL: 3;
  FILL_OR_KILL: 4;
  MARKET_ON_OPEN: 5;
};
type TimeInForce = (typeof TimeInForce)[keyof typeof TimeInForce];

/**
 * Stop Order and Stop Loss triggering method ENUM.
 */
declare const OrderTriggerMethod: {
  /** Stop Order: buy is triggered by ask, sell by bid; Stop Loss Order: for buy position is triggered by bid and for sell position by ask. */
  TRADE: 1;
  /** Stop Order: buy is triggered by bid, sell by ask; Stop Loss Order: for buy position is triggered by ask and for sell position by bid. */
  OPPOSITE: 2;
  /** The same as TRADE, but trigger is checked after the second consecutive tick. */
  DOUBLE_TRADE: 3;
  /** The same as OPPOSITE, but trigger is checked after the second consecutive tick. */
  DOUBLE_OPPOSITE: 4;
};
type OrderTriggerMethod =
  (typeof OrderTriggerMethod)[keyof typeof OrderTriggerMethod];

/**
 * Order status ENUM.
 */
declare const OrderStatus: {
  /** Order request validated and accepted for execution. */
  ORDER_STATUS_ACCEPTED: 1;
  /** Order is fully filled. */
  ORDER_STATUS_FILLED: 2;
  /** Order is rejected due to validation. */
  ORDER_STATUS_REJECTED: 3;
  /** Order expired. Might be valid for orders with partially filled volume that were expired on LP. */
  ORDER_STATUS_EXPIRED: 4;
  /** Order is cancelled. Might be valid for orders with partially filled volume that were cancelled by LP. */
  ORDER_STATUS_CANCELLED: 5;
};
type OrderStatus = (typeof OrderStatus)[keyof typeof OrderStatus];

/**
 * Position status ENUM.
 */
declare const PositionStatus: {
  POSITION_STATUS_OPEN: 1;
  POSITION_STATUS_CLOSED: 2;
  /** Empty position is created for pending order. */
  POSITION_STATUS_CREATED: 3;
  POSITION_STATUS_ERROR: 4;
};
type PositionStatus = (typeof PositionStatus)[keyof typeof PositionStatus];

// TYPES

/**
 * Symbol trading session entity.
 */
type TradeInterval = {
  /** Interval start, specified in seconds starting from SUNDAY 00:00 in specified time zone (inclusive to the interval). */
  start_second: number;
  /** Interval end, specified in seconds starting from SUNDAY 00:00 in specified time zone (exclusive from the interval). */
  end_second: number;
};

/**
 * Holiday entity.
 */
type Holiday = {
  /** Unique ID of holiday. */
  holiday_id: number;
  /** Name of holiday. */
  name: string;
  /** Description of holiday. */
  description?: string;
  /** Timezone used for holiday. */
  schedule_time_zone: string;
  /** Amount of days from 1st Jan 1970, multiply it by 86400000 to get Unix time in milliseconds. */
  holiday_date: number;
  /** If TRUE, then the holiday happens each year. */
  is_recurring: boolean;
  /** Amount of seconds from 00:00:00 of the holiday day when holiday actually starts. */
  start_second?: number;
  /** Amount of seconds from 00:00:00 of the holiday day when holiday actually finishes. */
  end_second?: number;
};

/**
 * Historical Trendbar entity.
 */
type Trendbar = {
  volume: number;
  period?: TrendbarPeriod;
  low: number;
  high: number;
  open: number;
  close: number;
  utc_timestamp_in_minutes?: number;
};

/**
 * Trading symbol entity.
 */
type TradeSymbol = {
  /** The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs. */
  symbol_id: number;
  /** Number of price digits to be displayed. */
  digits: number;
  /** Pip position on digits. */
  pip_position: number;
  /** If TRUE then the short selling with the symbol is enabled. */
  enable_short_selling?: boolean;
  /** If TRUE then setting of guaranteedStopLoss is available for limited risk accounts. */
  guaranteed_stop_loss?: boolean;
  /** Day of the week when SWAP charge amount will be tripled. */
  swap_rollover_3_days?: DayOfWeek;
  /** SWAP charge for long positions. */
  swap_long?: number;
  /** SWAP charge for short positions. */
  swap_short?: number;
  /** Maximum allowed volume in cents for an order with a symbol. */
  max_volume?: number;
  /** Minimum allowed volume in cents for an order with a symbol. */
  min_volume?: number;
  /** Step of the volume in cents for an order. */
  step_volume?: number;
  /** Value of max exposure per symbol, per account. */
  max_exposure?: number;
  /** Symbol trading interval, specified in seconds starting from SUNDAY 00:00 in specified time zone. */
  schedule: TradeInterval[];
  /** Commission base amount. Use preciseTradingCommissionRate. @deprecated */
  commission?: number;
  /** Commission type. */
  // commission_type?: CommissionType;
  /** Minimum allowed distance between stop loss and current market price. */
  sl_distance?: number;
  /** Minimum allowed distance between take profit and current market price. */
  tp_distance?: number;
  /** Minimum allowed distance between guaranteed stop loss and current market price. */
  gsl_distance?: number;
  /** Guaranteed stop loss fee. */
  gsl_charge?: number;
  /** Unit of distance measure for slDistance, tpDistance, gslDistance. */
  distance_set_in?: SymbolDistanceType;
  /** Minimum commission amount per trade. Use preciseMinCommission. @deprecated */
  min_commission?: number;
  /** Minimum commission Type. */
  // min_commission_type?: MinCommissionType;
  /** Currency for minimum commission. */
  min_commission_asset?: string;
  /** Administrative Fee, charged instead of Swaps. */
  rollover_commission?: number;
  /** Initial period before the first rolloverCommission will be charged. */
  skip_rollover_days?: number;
  /** Time zone for the symbol trading intervals. */
  schedule_time_zone?: string;
  /** Rules for trading with the symbol. */
  trading_mode?: TradingMode;
  /** Day of the week when Administrative Fee charge amount will be tripled. */
  rollover_commission_3_days?: DayOfWeek;
  /** Specifies type of SWAP computation. */
  swap_calculation_type?: SwapCalculationType;
  /** Lot size of the Symbol (in cents). */
  lot_size?: number;
  /** Commission base amount multiplied by 10^8. */
  precise_trading_commission_rate?: number;
  /** Minimum commission amount per trade multiplied by 10^8. */
  precise_min_commission?: number;
  /** List of holidays for this symbol specified by broker. */
  holiday: Holiday[];
  /** Percentage (1 = 0.01%) of the realized Gross Profit. */
  pnl_conversion_fee_rate?: number;
  /** The unique identifier of dynamic leverage entity. */
  leverage_id?: number;
  /** Period of charging swaps in hours. */
  swap_period?: number;
  /** Time in minutes from 00:00 (UTC) when intraday swaps are charged. */
  swap_time?: number;
  /** Count of swapPeriods before the first SWAP charge. */
  skip_swap_periods?: number;
  /** If enabled, SWAP will be charged for all days of the week. */
  charge_swap_at_weekends?: boolean;
  /** Specifies the units in which the base Asset of the Symbol is denominated. */
  measurement_units?: string;
};

/**
 * Request for sending a new trading order. Allowed only if the accessToken has the "trade" permissions for the trading account.
 */
type NewOrderReq = {
  /** The unique identifier of the trader's account in cTrader platform. */
  ctid_trader_account_id: number;
  /** The unique identifier of a symbol in cTrader platform. */
  symbol_id: number;
  /** The type of an order - MARKET, LIMIT, STOP, MARKET_RANGE, STOP_LIMIT. */
  order_type: OrderType;
  /** The trade direction - BUY or SELL. */
  trade_side: TradeSide;
  /** The volume represented in 0.01 of a unit (e.g. 1000 in protocol means 10.00 units). */
  volume: number;
  /** The limit price, can be specified for the LIMIT order only. */
  limit_price?: number;
  /** Stop Price, can be specified for the STOP and the STOP_LIMIT orders only. */
  stop_price?: number;
  /** The specific order execution or expiration instruction. */
  time_in_force?: TimeInForce;
  /** The Unix time in milliseconds of Order expiration. Should be set for the Good Till Date orders. */
  expiration_timestamp?: number;
  /** The absolute Stop Loss price (1.23456 for example). Not supported for MARKET orders. */
  stop_loss?: number;
  /** The absolute Take Profit price (1.23456 for example). Unsupported for MARKET orders. */
  take_profit?: number;
  /** User-specified comment. MaxLength = 512. */
  comment?: string;
  /** Base price to calculate relative slippage price for MARKET_RANGE order. */
  base_slippage_price?: number;
  /** Slippage distance for MARKET_RANGE and STOP_LIMIT order. */
  slippage_in_points?: number;
  /** User-specified label. MaxLength = 100. */
  label?: string;
  /** Reference to the existing position if the Order is intended to modify it. */
  position_id?: number;
  /** Optional user-specific clientOrderId (similar to FIX ClOrderID). MaxLength = 50. */
  client_order_id?: string;
  /** Relative Stop Loss. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss. */
  relative_stop_loss?: number;
  /** Relative Take Profit. For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit. */
  relative_take_profit?: number;
  /** If TRUE then stopLoss is guaranteed. Required for Limited Risk accounts. */
  guaranteed_stop_loss?: boolean;
  /** If TRUE then the Stop Loss is Trailing. */
  trailing_stop_loss?: boolean;
  /** Trigger method for the STOP or the STOP_LIMIT pending order. */
  stop_trigger_method?: OrderTriggerMethod;
};

/**
 * Position/order trading details entity.
 */
type TradeData = {
  /** The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs. */
  symbol_id: number;
  /** Volume in cents (e.g. 1000 in protocol means 10.00 units). */
  volume: number;
  /** Buy, Sell. */
  trade_side: TradeSide;
  /** The Unix time in milliseconds when position was opened or order was created. */
  open_timestamp?: number;
  /** Text label specified during order request. */
  label?: string;
  /** If TRUE then position/order stop loss is guaranteedStopLoss. */
  guaranteed_stop_loss?: boolean;
  /** User-specified comment. */
  comment?: string;
  /** Specifies the units in which the Symbol is denominated. */
  measurement_units?: string;
  /** The Unix time in milliseconds when a Position was closed. */
  close_timestamp?: number;
};

/**
 * Trade order entity.
 */
type Order = {
  /** The unique ID of the order. Note: trader might have two orders with the same id if orders are taken from accounts from different brokers. */
  order_id: number;
  /** Detailed trader data. */
  trade_data: TradeData;
  /** Order type. */
  order_type: OrderType;
  /** Order status. */
  order_status: OrderStatus;
  /** The Unix time in milliseconds of expiration if the order has time in force GTD. */
  expiration_timestamp?: number;
  /** Price at which an order was executed. */
  execution_price?: number;
  /** Part of the volume that was filled in cents (e.g. 1000 in protocol means 10.00 units). */
  executed_volume?: number;
  /** The Unix time in milliseconds of the last update of the order. */
  utc_last_update_timestamp?: number;
  /** Used for Market Range order with combination of slippageInPoints. */
  base_slippage_price?: number;
  /** Used for Market Range and STOP_LIMIT orders. */
  slippage_in_points?: number;
  /** If TRUE then the order is closing part of whole position. */
  closing_order?: boolean;
  /** Valid only for LIMIT orders. */
  limit_price?: number;
  /** Valid only for STOP and STOP_LIMIT orders. */
  stop_price?: number;
  /** Absolute stopLoss price. */
  stop_loss?: number;
  /** Absolute takeProfit price. */
  take_profit?: number;
  /** Optional ClientOrderId. Max Length = 50 chars. */
  client_order_id?: string;
  /** Order's time in force. */
  time_in_force?: TimeInForce;
  /** ID of the position linked to the order. */
  position_id?: number;
  /** Relative stopLoss. */
  relative_stop_loss?: number;
  /** Relative takeProfit. */
  relative_take_profit?: number;
  /** If TRUE then order was stopped out from server side. */
  is_stop_out?: boolean;
  /** If TRUE then order is trailingStopLoss. */
  trailing_stop_loss?: boolean;
  /** Trigger method for the order. Valid only for STOP and STOP_LIMIT orders. */
  stop_trigger_method?: OrderTriggerMethod;
};

/**
 * Trade position entity.
 */
type Position = {
  /** The unique ID of the position. Note: trader might have two positions with the same id if positions are taken from accounts from different brokers. */
  position_id: number;
  /** Position details. */
  trade_data: TradeData;
  /** Current status of the position. */
  position_status: PositionStatus;
  /** Total amount of charged swap on open position. */
  swap: number;
  /** VWAP price of the position based on all executions (orders) linked to the position. */
  price?: number;
  /** Current stop loss price. */
  stop_loss?: number;
  /** Current take profit price. */
  take_profit?: number;
  /** The Unix time in milliseconds of the last change of the position. */
  utc_last_update_timestamp?: number;
  /** Current unrealized commission related to the position. */
  commission?: number;
  /** Rate for used margin computation. Represented as Base/Deposit. */
  margin_rate?: number;
  /** Amount of unrealized commission related to following of strategy provider. */
  mirroring_commission?: number;
  /** If TRUE then position's stop loss is guaranteedStopLoss. */
  guaranteed_stop_loss?: boolean;
  /** Amount of margin used for the position in deposit currency. */
  used_margin?: number;
  /** Stop trigger method for SL/TP of the position. */
  stop_loss_trigger_method?: OrderTriggerMethod;
  /** Specifies the exponent of the monetary values. */
  money_digits?: number;
  /** If TRUE then the Trailing Stop Loss is applied. */
  trailing_stop_loss?: boolean;
};

/**
 * Request for closing or partially closing of an existing position. Allowed only if the accessToken has "trade" permissions for the trading account.
 */
type ClosePositionReq = {
  /** Unique identifier of the trader's account. Used to match responses to trader's accounts. */
  ctid_trader_account_id: number;
  /** The unique ID of the position to close. */
  position_id: number;
  /** Volume to close, represented in 0.01 of a unit (e.g. 1000 in protocol means 10.00 units). */
  volume: number;
};

/**
 * The response to the ReconcileReq request.
 */
type ReconcileRes = {
  /** Unique identifier of the trader's account. Used to match responses to trader's accounts. */
  ctid_trader_account_id: number;
  /** The list of trader's account open positions. */
  position: Position[];
  /** The list of trader's account pending orders. */
  order: Order[];
};

type JsonValue = string | number | boolean | null | JsonArray | JsonObject;

interface JsonObject {
  [key: string]: JsonValue;
}

interface JsonArray extends Array<JsonValue> { }

// CORE
declare class Context<
  State extends Record<string, JsonValue> = {},
  Env extends JsonObject = {},
> {
  readonly account_id: number;
  readonly name: string;
  readonly live: boolean;
  readonly symbol_name: string;
  readonly symbol: TradeSymbol;
  readonly series: Array<Trendbar>;
  state: Partial<State>;
  env: Env;
  close_all_positions(): Promise<void>;
  close_position(options: ClosePositionReq): Promise<void>;
  new_order(options: NewOrderReq): Promise<void>;
  reconcile(): Promise<ReconcileRes>;
}

// UTILS
declare class Console {
  log(...args: any[]): void;
  error(...args: any[]): void;
  info(...args: any[]): void;
  warn(...args: any[]): void;
}
declare var console: Console;

declare function lots_to_volume(symbol: TradeSymbol, lots: number): number
declare function price_to_volume(symbol: TradeSymbol, amount: number): number

// Technical Analysis Utilities

/**
 * **Exponential Moving Average (EMA)** — A widely used technical analysis
 * indicator that calculates the average of a price series over a specified
 * period, giving more weight to recent data points.
 *
 * Unlike the Simple Moving Average (SMA), which weights all values equally,
 * the EMA reacts more quickly to price changes, making it especially useful
 * for identifying trends, entry/exit signals, and momentum shifts in
 * algorithmic trading strategies.
 *
 * The EMA is computed recursively:
 * ```
 * multiplier = 2 / (period + 1)
 * EMA = (close - EMA_prev) * multiplier + EMA_prev
 * ```
 *
 * Common use cases:
 * - **Trend identification**: A rising EMA suggests an uptrend; a falling
 *   EMA suggests a downtrend.
 * - **Crossover strategies**: Combining a fast EMA (e.g., 9-period) with a
 *   slow EMA (e.g., 21-period) generates buy/sell signals when they cross.
 * - **Dynamic support/resistance**: The EMA line often acts as a moving
 *   support or resistance level.
 *
 * @template State Generic state type for the trading context.
 * @template Env Generic environment type.
 */
declare class ExponentialMovingAverage {
  constructor(period: number);
  next(num: number): void;
  calculate(): Array<number>;
  static calculate(period: number, series: Array<Trendbar>): Array<number>;
}

/**
 * **Relative Strength Index (RSI)** — A momentum oscillator that measures the
 * speed and magnitude of recent price changes to evaluate overbought or
 * oversold conditions in a market.
 *
 * RSI ranges from 0 to 100 and is calculated by comparing the average
 * magnitude of recent gains to recent losses over a specified period:
 * ```
 * RS = average_gain / average_loss
 * RSI = 100 - (100 / (1 + RS))
 * ```
 *
 * Common use cases:
 * - **Overbought/Oversold**: Readings above 70 suggest overbought conditions
 *   (potential sell signal); below 30 suggest oversold (potential buy signal).
 * - **Divergence**: Price making new highs while RSI makes lower highs can
 *   signal trend weakness and a potential reversal.
 * - **Centerline cross**: RSI crossing above/below 50 can confirm trend direction.
 */
declare class RelativeStrengthIndex {
  constructor(period: number);
  next(num: number): void;
  calculate(): Array<number>;
  static calculate(period: number, series: Array<Trendbar>): Array<number>;
}

/**
 * Bollinger Bands result containing lower, average, and upper band values.
 */
type BollingerBandsResult = {
  lower: number,
  average: number,
  upper: number
}

/**
 * **Bollinger Bands (BB)** — A volatility-based indicator consisting of a
 * middle SMA band and upper/lower bands placed at a specified number of
 * standard deviations away from the middle.
 *
 * The bands widen during high volatility and contract during low volatility,
 * making them useful for identifying breakout opportunities and market regimes.
 * ```
 * middle = SMA(close, period)
 * upper = middle + multiplier * StdDev(close, period)
 * lower = middle - multiplier * StdDev(close, period)
 * ```
 *
 * Common use cases:
 * - **Volatility squeeze**: Narrowing bands often precede strong breakouts.
 * - **Overbought/Oversold**: Price touching the upper/lower bands may indicate
 *   extreme conditions (in trending markets, price can "walk the band").
 * - **Mean reversion**: Price returning to the middle band from an extreme.
 */
declare class BollingerBands {
  constructor(period: number, multiplier: number);
  next(num: number): void;
  calculate(): Array<BollingerBandsResult>;
  static calculate(period: number, multiplier: number, trendbars: Array<{ close: number }>): Array<BollingerBandsResult>;
}

/**
 * **Average True Range (ATR)** — A volatility indicator that measures the
 * average range between high and low prices over a specified period.
 *
 * ATR does not indicate price direction — it only measures the degree of
 * price movement, making it invaluable for position sizing and stop-loss
 * placement.
 *
 * True Range is the greatest of:
 * - current high - current low
 * - |current high - previous close|
 * - |current low - previous close|
 *
 * ATR is then calculated as an EMA of True Range values.
 *
 * Common use cases:
 * - **Stop-loss placement**: Setting stops at a multiple of ATR accounts
 *   for current market volatility.
 * - **Position sizing**: Reduce size when ATR is high (volatile) and
 *   increase when ATR is low (calm).
 * - **Volatility filter**: Avoid trading in low-ATR (choppy) conditions.
 */
declare class AverageTrueRange {
  constructor(period: number);
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, trendbars: Array<Trendbar>): Array<number>
}

/**
 * Chandelier Exit result containing long and short exit levels.
 */
type ChandelierExitResult = {
  long: number,
  short: number,
}

/**
 * **Chandelier Exit (CE)** — A volatility-based trailing stop indicator
 * designed to keep traders in trends as long as they continue.
 *
 * It sets a trailing stop based on the Average True Range (ATR) multiplied
 * by a specified multiplier, attached to the highest high (for long exits)
 * or lowest low (for short exits) over the lookback period.
 * ```
 * long_stop = max(high, period) - multiplier * ATR
 * short_stop = min(low, period) + multiplier * ATR
 * ```
 *
 * Common use cases:
 * - **Trend following**: Provides a dynamic exit strategy that adapts to
 *   volatility, allowing winners to run while protecting profits.
 * - **Trailing stops**: Can be used to automatically trail the stop as the
 *   price moves favorably.
 */
declare class ChandelierExit {
  constructor(period: number, multiplier: number);
  next(num: number): void
  calculate(): Array<ChandelierExitResult>;
  static calculate(period: number, multiplier: number, trendbars: Array<Trendbar>): Array<ChandelierExitResult>;
}

/**
 * **Commodity Channel Index (CCI)** — A versatile oscillator that measures
 * the current price level relative to an average price over a given period.
 *
 * CCI identifies cyclical trends and can be used across asset classes
 * despite its name. It is calculated as the difference between the
 * typical price and its SMA, divided by the mean absolute deviation:
 * ```
 * typical_price = (high + low + close) / 3
 * CCI = (typical_price - SMA(typical_price, period)) / (0.015 * MAD)
 * ```
 *
 * Common use cases:
 * - **Overbought/Oversold**: Readings above +100 suggest overbought;
 *   below -100 suggest oversold.
 * - **Trend confirmation**: Sustained readings above/below zero confirm
 *   bullish/bearish trends.
 * - **Divergence**: Price diverging from CCI can signal reversals.
 */
declare class CommodityChannelIndex {
  constructor(period: number)
  next(high: number, low: number, close: number): void
  calculate(): Array<number>
  static calculate(period: number, trendbars: Array<{ high: number; low: number; close: number }>): Array<number>
}

/**
 * **Efficiency Ratio (ER)** — An indicator that measures the efficiency of
 * price movement by comparing the net directional change to the total
 * price movement over a period.
 *
 * Developed by Perry Kaufman, ER ranges from 0 to 1 (or 0% to 100%):
 * ```
 * ER = |close - close[n]| / sum(|close - close[1]| for each bar)
 * ```
 * A value near 1 indicates strong trending conditions; near 0 indicates
 * choppy, sideways movement.
 *
 * Common use cases:
 * - **Trend strength filter**: Only trade when ER is above a threshold.
 * - **Adaptive indicators**: Used in Kaufman's Adaptive Moving Average
 *   (KAMA) to adjust sensitivity based on market conditions.
 * - **Regime detection**: Distinguish between trending and ranging markets.
 */
declare class EfficiencyRatio {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ close: number }>): Array<number>
}

/**
 * **Fast Stochastic (%K)** — A momentum oscillator that compares a closing
 * price to the price range over a given period.
 *
 * The fast stochastic is calculated as:
 * ```
 * %K = (close - lowest_low) / (highest_high - lowest_low) * 100
 * ```
 * where lowest_low and highest_high are over the specified period.
 *
 * Common use cases:
 * - **Overbought/Oversold**: Values above 80 suggest overbought; below 20
 *   suggest oversold.
 * - **Crossovers**: The fast %K line crossing above/below the slow %D line.
 * - **Divergence**: Price-stochastic divergence can signal trend reversals.
 */
declare class FastStochastic {
  constructor(period: number)
  next(high: number, low: number, close: number): void
  calculate(): Array<number>
  static calculate(period: number, trendbars: Array<{ high: number; low: number; close: number }>): Array<number>
}

/**
 * Keltner Channel result containing average, upper, and lower channel values.
 */
type KeltnerChannelResult = {
  average: number,
  upper: number,
  lower: number
}

/**
 * **Keltner Channel (KC)** — A volatility-based envelope indicator plotted
 * around an exponential moving average (EMA) of the price.
 *
 * The upper and lower bands are set by adding/subtracting the Average True
 * Range (ATR) multiplied by a specified multiplier from the EMA line.
 * ```
 * middle = EMA(close, period)
 * upper = middle + multiplier * ATR(period)
 * lower = middle - multiplier * ATR(period)
 * ```
 *
 * Common use cases:
 * - **Trend direction**: Price above the upper band signals strong uptrend;
 *   below the lower band signals strong downtrend.
 * - **Volatility breakouts**: A candle closing outside the bands can signal
 *   the start of a new trend.
 * - **Mean reversion**: Price touching or exceeding the bands may revert
 *   toward the middle EMA.
 */
declare class KeltnerChannel {
  constructor(period: number, multiplier: number);
  next(num: number): void
  calculate(): Array<KeltnerChannelResult>
  static calculate(period: number, multiplier: number, trendbars: Array<{ close: number }>): Array<KeltnerChannelResult>
}

/**
 * **Maximum** — A simple rolling-window indicator that tracks the highest
 * value in a series over a specified period.
 *
 * Common use cases:
 * - **Resistance levels**: Identify recent price highs for dynamic resistance.
 * - **Trailing stop reference**: Used as a component in trailing stop
 *   calculations (e.g., Chandelier Exit).
 * - **Breakout detection**: A new maximum can signal bullish momentum.
 */
declare class Maximum {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ high: number }>): Array<number>
}

/**
 * **Mean Absolute Deviation (MAD)** — A statistical measure of dispersion
 * that calculates the average absolute difference between each data point
 * and the mean over a specified period.
 * ```
 * MAD = sum(|close_i - mean|) / period
 * ```
 *
 * Common use cases:
 * - **Volatility measurement**: An alternative to standard deviation that
 *   is less sensitive to outliers.
 * - **Component indicator**: Used internally by indicators like CCI to
 *   normalize price movements.
 * - **Market regime filter**: Higher MAD values indicate increased volatility.
 */
declare class MeanAbsoluteDeviation {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ close: number }>): Array<number>
}

/**
 * **Minimum** — A simple rolling-window indicator that tracks the lowest
 * value in a series over a specified period.
 *
 * Common use cases:
 * - **Support levels**: Identify recent price lows for dynamic support.
 * - **Trailing stop reference**: Used as a component in trailing stop
 *   calculations (e.g., Chandelier Exit for short positions).
 * - **Breakdown detection**: A new minimum can signal bearish momentum.
 */
declare class Minimum {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ low: number }>): Array<number>
}

/**
 * **Money Flow Index (MFI)** — A volume-weighted momentum oscillator that
 * measures buying and selling pressure by incorporating both price and
 * volume data.
 *
 * MFI is often called "volume-weighted RSI" and ranges from 0 to 100:
 * ```
 * typical_price = (high + low + close) / 3
 * raw_money_flow = typical_price * volume
 * money_ratio = positive_money_flow / negative_money_flow
 * MFI = 100 - (100 / (1 + money_ratio))
 * ```
 *
 * Common use cases:
 * - **Overbought/Oversold**: Above 80 suggests overbought; below 20
 *   suggests oversold (with volume confirmation).
 * - **Divergence**: Price rising while MFI falling warns of weak buying
 *   pressure and potential reversal.
 * - **Volume confirmation**: Confirms price trends with volume participation.
 */
declare class MoneyFlowIndex {
  constructor(period: number)
  next(high: number, low: number, close: number, volume: number): void
  calculate(): Array<number>
  static calculate(period: number, trendbars: Array<{ high: number; low: number; close: number; volume: number }>): Array<number>
}

/**
 * MACD result containing macd, signal, and histogram values.
 */
type MacdResult = {
  macd: number,
  signal: number,
  histogram: number
}

/**
 * **Moving Average Convergence Divergence (MACD)** — A trend-following
 * momentum indicator that shows the relationship between two exponential
 * moving averages of a price series.
 *
 * The MACD line is the difference between a fast-period EMA and a
 * slow-period EMA. The signal line is an EMA of the MACD line, and the
 * histogram is the difference between the two:
 * ```
 * MACD = EMA(fast) - EMA(slow)
 * signal = EMA(MACD, signal_period)
 * histogram = MACD - signal
 * ```
 *
 * Common use cases:
 * - **Crossover signals**: MACD crossing above/below the signal line
 *   generates bullish/bearish signals.
 * - **Centerline cross**: MACD crossing above/below zero signals trend
 *   direction change.
 * - **Divergence**: Price diverging from MACD can indicate trend exhaustion.
 */
declare class MovingAverageConvergenceDivergence {
  constructor(fastPeriod: number, slowPeriod: number, signalPeriod: number)
  next(num: number): void
  calculate(): Array<MacdResult>
  static calculate(fastPeriod: number, slowPeriod: number, signalPeriod: number, trendbars: Array<{ close: number }>): Array<MacdResult>
}

/**
 * **On-Balance Volume (OBV)** — A volume-based momentum indicator that
 * relates volume flow to price changes.
 *
 * OBV adds volume on up days and subtracts volume on down days, forming a
 * cumulative line that can confirm or diverge from price:
 * ```
 * if close > close[1]: OBV += volume
 * if close < close[1]: OBV -= volume
 * if close = close[1]: OBV unchanged
 * ```
 *
 * Common use cases:
 * - **Trend confirmation**: OBV rising alongside price confirms the trend.
 * - **Divergence**: Price making new highs while OBV lags warns of weak
 *   buying interest and potential reversal.
 * - **Breakout validation**: OBV breaking out before price can signal
 *   impending price movement.
 */
declare class OnBalanceVolume {
  next(close: number, volume: number): void
  calculate(): Array<number>
  static calculate(trendbars: Array<{ close: number; volume: number }>): Array<number>
}

/**
 * Percentage Price Oscillator result containing ppo, signal, and histogram values.
 */
type PpoResult = {
  ppo: number,
  signal: number,
  histogram: number
}

/**
 * **Percentage Price Oscillator (PPO)** — A momentum indicator similar to
 * MACD but expressed as a percentage of the slower moving average, making
 * it comparable across different price levels and instruments.
 *
 * The PPO line is the percentage difference between a fast and slow EMA.
 * A signal line (EMA of PPO) and histogram are derived from it:
 * ```
 * PPO = ((EMA(fast) - EMA(slow)) / EMA(slow)) * 100
 * signal = EMA(PPO, signal_period)
 * histogram = PPO - signal
 * ```
 *
 * Common use cases:
 * - **Crossover signals**: PPO crossing above/below the signal line.
 * - **Percentage-based analysis**: Useful for comparing momentum across
 *   instruments with different price scales.
 * - **Centerline cross**: Above zero signals upward momentum; below zero
 *   signals downward momentum.
 */
declare class PercentagePriceOscillator {
  constructor(fastPeriod: number, slowPeriod: number, signalPeriod: number)
  next(num: number): void
  calculate(): Array<PpoResult>
  static calculate(fastPeriod: number, slowPeriod: number, signalPeriod: number, trendbars: Array<{ close: number }>): Array<PpoResult>
}

/**
 * **Rate of Change (ROC)** — A momentum oscillator that measures the
 * percentage change in price between the current price and the price n
 * periods ago.
 * ```
 * ROC = ((close - close[n]) / close[n]) * 100
 * ```
 *
 * Common use cases:
 * - **Momentum measurement**: Positive values indicate upward momentum;
 *   negative values indicate downward momentum.
 * - **Zero-line cross**: Crossing above/below zero signals trend changes.
 * - **Overbought/Oversold**: Extreme ROC values (positive or negative)
 *   can indicate exhausted moves.
 */
declare class RateOfChange {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ close: number }>): Array<number>
}

/**
 * **Simple Moving Average (SMA)** — A widely used technical indicator that
 * calculates the arithmetic mean of a price series over a specified period.
 *
 * Each data point in the series has equal weight:
 * ```
 * SMA = sum(close[1..n]) / period
 * ```
 *
 * Common use cases:
 * - **Trend identification**: Price above SMA suggests uptrend; below SMA
 *   suggests downtrend.
 * - **Dynamic support/resistance**: SMA often acts as a moving floor or
 *   ceiling for price action.
 * - **Crossover systems**: Used in combination with shorter/longer SMAs
 *   or EMAs for trade signals.
 * - **Smoothing**: Reduces noise in price data for clearer trend analysis.
 */
declare class SimpleMovingAverage {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ close: number }>): Array<number>
}

/**
 * **Slow Stochastic (%D)** — A smoothed version of the Fast Stochastic
 * oscillator that applies an additional EMA to the %K line to reduce
 * noise and generate fewer false signals.
 * ```
 * %K = Fast Stochastic (raw K)
 * %D = EMA(%K, ema_period)   ← the Slow Stochastic
 * ```
 *
 * The Slow Stochastic tends to be more reliable than the Fast Stochastic
 * because the additional smoothing filters out minor price fluctuations.
 *
 * Common use cases:
 * - **Overbought/Oversold**: Values above 80 suggest overbought; below 20
 *   suggest oversold.
 * - **Crossover signals**: The slow %K crossing above/below the signal
 *   line (%D) generates trade signals.
 * - **Divergence**: More reliable divergence signals due to smoothing.
 */
declare class SlowStochastic {
  constructor(stochasticPeriod: number, emaPeriod: number)
  next(high: number, low: number, close: number): void
  calculate(): Array<number>
  static calculate(stochasticPeriod: number, emaPeriod: number, trendbars: Array<{ high: number; low: number; close: number }>): Array<number>
}

/**
 * **Standard Deviation (StdDev)** — A statistical measure of market
 * volatility that quantifies the dispersion of a price series around
 * its mean over a specified period.
 * ```
 * StdDev = sqrt(sum((close_i - mean)^2) / period)
 * ```
 *
 * Common use cases:
 * - **Volatility measurement**: Higher values indicate greater price
 *   variability and market uncertainty.
 * - **Component indicator**: Used internally by Bollinger Bands to
 *   calculate upper/lower bands.
 * - **Market regime detection**: Low StdDev indicates consolidation;
 *   high StdDev indicates expansion.
 */
declare class StandardDeviation {
  constructor(period: number)
  next(num: number): void
  calculate(): Array<number>
  static calculate(period: number, series: Array<{ close: number }>): Array<number>
}

/**
 * **True Range (TR)** — A volatility measure that captures the full range
 * of price movement for a period, accounting for gaps and limit moves.
 *
 * True Range is the greatest of:
 * - current high - current low
 * - |current high - previous close|
 * - |current low - previous close|
 *
 * TR is the building block of the Average True Range (ATR) indicator and
 * is useful on its own for gauging raw price volatility.
 *
 * Common use cases:
 * - **Volatility analysis**: Higher TR values indicate more volatile
 *   market conditions.
 * - **Component indicator**: Used internally by ATR and other
 *   volatility-based indicators.
 * - **Stop placement**: Individual TR values can inform per-bar stop distances.
 */
declare class TrueRange {
  next(high: number, low: number, close: number): void
  calculate(): Array<number>
  static calculate(trendbars: Array<{ high: number; low: number; close: number }>): Array<number>
}
