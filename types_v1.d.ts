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

// CORE
declare class Context {
  readonly account_id: number;
  readonly name: string;
  readonly live: boolean;
  readonly symbol_name: string;
  readonly symbol: Readonly<TradeSymbol>;
  readonly series: Readonly<Array<Readonly<Trendbar>>>;
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

declare class ExponentialMovingAverage {
  constructor(period: number);
  next(num: number): void;
  calculate(): Array<number>;
  static calculate(period: number, series: Array<number>): Array<number>;
}

declare class RelativeStrengthIndex {
  constructor(period: number);
  next(num: number): void;
  calculate(): Array<number>;
  static calculate(period: number, series: Array<number>): Array<number>;
}
