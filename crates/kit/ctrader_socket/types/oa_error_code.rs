use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::Deserialize;
use serde::Serialize;

#[derive(
  Clone,
  Copy,
  Debug,
  PartialEq,
  Eq,
  Hash,
  PartialOrd,
  Ord,
  TryFromPrimitive,
  IntoPrimitive,
  Serialize,
  Deserialize,
)]
#[repr(i32)]
pub enum OaErrorCode {
  /// Authorization
  ///
  /// When token used for account authorization is expired.
  OaAuthTokenExpired = 1,
  /// When account is not authorized.
  AccountNotAuthorized = 2,
  /// When such account no longer exists.
  RetNoSuchLogin = 12,
  /// When client tries to authorize after it was already authorized.
  AlreadyLoggedIn = 14,
  /// When account is disabled.
  RetAccountDisabled = 64,
  /// Open API client is not activated or wrong client credentials.
  ChClientAuthFailure = 101,
  /// When a command is sent for not authorized Open API client.
  ChClientNotAuthenticated = 102,
  /// Client is trying to authenticate twice.
  ChClientAlreadyAuthenticated = 103,
  /// Access token is invalid.
  ChAccessTokenInvalid = 104,
  /// Trading service is not available.
  ChServerNotReachable = 105,
  /// Trading account is not found.
  ChCtidTraderAccountNotFound = 106,
  /// Could not find this client id.
  ChOaClientNotFound = 107,
  /// General
  ///
  /// Request frequency is reached.
  RequestFrequencyExceeded = 108,
  /// Server is under maintenance.
  ServerIsUnderMaintenance = 109,
  /// Operations are not allowed for this account.
  ChannelIsBlocked = 110,
  /// Limit of connections is reached for this Open API client.
  ConnectionsLimitExceeded = 67,
  /// Not allowed to increase risk for Positions with Guaranteed Stop Loss.
  WorseGslNotAllowed = 68,
  /// Trading disabled because symbol has holiday.
  SymbolHasHoliday = 69,
  /// Pricing
  ///
  /// When trying to subscribe to depth, trendbars, etc. without spot subscription.
  NotSubscribedToSpots = 112,
  /// When subscription is requested for an active.
  AlreadySubscribed = 113,
  /// Symbol not found.
  SymbolNotFound = 114,
  /// Note: to be merged with SYMBOL_NOT_FOUND.
  UnknownSymbol = 115,
  /// When requested period (from,to) is too large or invalid values are set to from/to.
  IncorrectBoundaries = 35,
  /// Trading
  ///
  /// Trading cannot be done as not quotes are available. Applicable for Book B.
  NoQuotes = 117,
  /// Not enough funds to allocate margin.
  NotEnoughMoney = 118,
  /// Max exposure limit is reached for a {trader, symbol, side}.
  MaxExposureReached = 119,
  /// Position not found.
  PositionNotFound = 120,
  /// Order not found.
  OrderNotFound = 121,
  /// When trying to close a position that it is not open.
  PositionNotOpen = 122,
  /// Position in the state that does not allow to perform an operation.
  PositionLocked = 123,
  /// Trading account reached its limit for max number of open positions and orders.
  TooManyPositions = 124,
  /// Invalid volume.
  TradingBadVolume = 125,
  /// Invalid stop price.
  TradingBadStops = 126,
  /// Invalid price (e.g. negative).
  TradingBadPrices = 127,
  /// Invalid stake volume (e.g. negative).
  TradingBadStake = 128,
  /// Invalid protection prices.
  ProtectionIsTooCloseToMarket = 129,
  /// Invalid expiration.
  TradingBadExpirationDate = 130,
  /// Unable to apply changes as position has an order under execution.
  PendingExecution = 131,
  /// Trading is blocked for the symbol.
  TradingDisabled = 132,
  /// Trading account is in read only mode.
  TradingNotAllowed = 133,
  /// Unable to cancel order.
  UnableToCancelOrder = 134,
  /// Unable to amend order.
  UnableToAmendOrder = 135,
  /// Short selling is not allowed.
  ShortSellingNotAllowed = 136,
}
impl OaErrorCode {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::OaAuthTokenExpired => "OA_AUTH_TOKEN_EXPIRED",
      Self::AccountNotAuthorized => "ACCOUNT_NOT_AUTHORIZED",
      Self::RetNoSuchLogin => "RET_NO_SUCH_LOGIN",
      Self::AlreadyLoggedIn => "ALREADY_LOGGED_IN",
      Self::RetAccountDisabled => "RET_ACCOUNT_DISABLED",
      Self::ChClientAuthFailure => "CH_CLIENT_AUTH_FAILURE",
      Self::ChClientNotAuthenticated => "CH_CLIENT_NOT_AUTHENTICATED",
      Self::ChClientAlreadyAuthenticated => "CH_CLIENT_ALREADY_AUTHENTICATED",
      Self::ChAccessTokenInvalid => "CH_ACCESS_TOKEN_INVALID",
      Self::ChServerNotReachable => "CH_SERVER_NOT_REACHABLE",
      Self::ChCtidTraderAccountNotFound => "CH_CTID_TRADER_ACCOUNT_NOT_FOUND",
      Self::ChOaClientNotFound => "CH_OA_CLIENT_NOT_FOUND",
      Self::RequestFrequencyExceeded => "REQUEST_FREQUENCY_EXCEEDED",
      Self::ServerIsUnderMaintenance => "SERVER_IS_UNDER_MAINTENANCE",
      Self::ChannelIsBlocked => "CHANNEL_IS_BLOCKED",
      Self::ConnectionsLimitExceeded => "CONNECTIONS_LIMIT_EXCEEDED",
      Self::WorseGslNotAllowed => "WORSE_GSL_NOT_ALLOWED",
      Self::SymbolHasHoliday => "SYMBOL_HAS_HOLIDAY",
      Self::NotSubscribedToSpots => "NOT_SUBSCRIBED_TO_SPOTS",
      Self::AlreadySubscribed => "ALREADY_SUBSCRIBED",
      Self::SymbolNotFound => "SYMBOL_NOT_FOUND",
      Self::UnknownSymbol => "UNKNOWN_SYMBOL",
      Self::IncorrectBoundaries => "INCORRECT_BOUNDARIES",
      Self::NoQuotes => "NO_QUOTES",
      Self::NotEnoughMoney => "NOT_ENOUGH_MONEY",
      Self::MaxExposureReached => "MAX_EXPOSURE_REACHED",
      Self::PositionNotFound => "POSITION_NOT_FOUND",
      Self::OrderNotFound => "ORDER_NOT_FOUND",
      Self::PositionNotOpen => "POSITION_NOT_OPEN",
      Self::PositionLocked => "POSITION_LOCKED",
      Self::TooManyPositions => "TOO_MANY_POSITIONS",
      Self::TradingBadVolume => "TRADING_BAD_VOLUME",
      Self::TradingBadStops => "TRADING_BAD_STOPS",
      Self::TradingBadPrices => "TRADING_BAD_PRICES",
      Self::TradingBadStake => "TRADING_BAD_STAKE",
      Self::ProtectionIsTooCloseToMarket => "PROTECTION_IS_TOO_CLOSE_TO_MARKET",
      Self::TradingBadExpirationDate => "TRADING_BAD_EXPIRATION_DATE",
      Self::PendingExecution => "PENDING_EXECUTION",
      Self::TradingDisabled => "TRADING_DISABLED",
      Self::TradingNotAllowed => "TRADING_NOT_ALLOWED",
      Self::UnableToCancelOrder => "UNABLE_TO_CANCEL_ORDER",
      Self::UnableToAmendOrder => "UNABLE_TO_AMEND_ORDER",
      Self::ShortSellingNotAllowed => "SHORT_SELLING_NOT_ALLOWED",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "OA_AUTH_TOKEN_EXPIRED" => Some(Self::OaAuthTokenExpired),
      "ACCOUNT_NOT_AUTHORIZED" => Some(Self::AccountNotAuthorized),
      "RET_NO_SUCH_LOGIN" => Some(Self::RetNoSuchLogin),
      "ALREADY_LOGGED_IN" => Some(Self::AlreadyLoggedIn),
      "RET_ACCOUNT_DISABLED" => Some(Self::RetAccountDisabled),
      "CH_CLIENT_AUTH_FAILURE" => Some(Self::ChClientAuthFailure),
      "CH_CLIENT_NOT_AUTHENTICATED" => Some(Self::ChClientNotAuthenticated),
      "CH_CLIENT_ALREADY_AUTHENTICATED" => Some(Self::ChClientAlreadyAuthenticated),
      "CH_ACCESS_TOKEN_INVALID" => Some(Self::ChAccessTokenInvalid),
      "CH_SERVER_NOT_REACHABLE" => Some(Self::ChServerNotReachable),
      "CH_CTID_TRADER_ACCOUNT_NOT_FOUND" => Some(Self::ChCtidTraderAccountNotFound),
      "CH_OA_CLIENT_NOT_FOUND" => Some(Self::ChOaClientNotFound),
      "REQUEST_FREQUENCY_EXCEEDED" => Some(Self::RequestFrequencyExceeded),
      "SERVER_IS_UNDER_MAINTENANCE" => Some(Self::ServerIsUnderMaintenance),
      "CHANNEL_IS_BLOCKED" => Some(Self::ChannelIsBlocked),
      "CONNECTIONS_LIMIT_EXCEEDED" => Some(Self::ConnectionsLimitExceeded),
      "WORSE_GSL_NOT_ALLOWED" => Some(Self::WorseGslNotAllowed),
      "SYMBOL_HAS_HOLIDAY" => Some(Self::SymbolHasHoliday),
      "NOT_SUBSCRIBED_TO_SPOTS" => Some(Self::NotSubscribedToSpots),
      "ALREADY_SUBSCRIBED" => Some(Self::AlreadySubscribed),
      "SYMBOL_NOT_FOUND" => Some(Self::SymbolNotFound),
      "UNKNOWN_SYMBOL" => Some(Self::UnknownSymbol),
      "INCORRECT_BOUNDARIES" => Some(Self::IncorrectBoundaries),
      "NO_QUOTES" => Some(Self::NoQuotes),
      "NOT_ENOUGH_MONEY" => Some(Self::NotEnoughMoney),
      "MAX_EXPOSURE_REACHED" => Some(Self::MaxExposureReached),
      "POSITION_NOT_FOUND" => Some(Self::PositionNotFound),
      "ORDER_NOT_FOUND" => Some(Self::OrderNotFound),
      "POSITION_NOT_OPEN" => Some(Self::PositionNotOpen),
      "POSITION_LOCKED" => Some(Self::PositionLocked),
      "TOO_MANY_POSITIONS" => Some(Self::TooManyPositions),
      "TRADING_BAD_VOLUME" => Some(Self::TradingBadVolume),
      "TRADING_BAD_STOPS" => Some(Self::TradingBadStops),
      "TRADING_BAD_PRICES" => Some(Self::TradingBadPrices),
      "TRADING_BAD_STAKE" => Some(Self::TradingBadStake),
      "PROTECTION_IS_TOO_CLOSE_TO_MARKET" => Some(Self::ProtectionIsTooCloseToMarket),
      "TRADING_BAD_EXPIRATION_DATE" => Some(Self::TradingBadExpirationDate),
      "PENDING_EXECUTION" => Some(Self::PendingExecution),
      "TRADING_DISABLED" => Some(Self::TradingDisabled),
      "TRADING_NOT_ALLOWED" => Some(Self::TradingNotAllowed),
      "UNABLE_TO_CANCEL_ORDER" => Some(Self::UnableToCancelOrder),
      "UNABLE_TO_AMEND_ORDER" => Some(Self::UnableToAmendOrder),
      "SHORT_SELLING_NOT_ALLOWED" => Some(Self::ShortSellingNotAllowed),
      _ => None,
    }
  }
}
