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
pub enum ErrorCode {
  /// Generic error.
  UnknownError = 1,
  /// Message is not supported. Wrong message.
  UnsupportedMessage = 2,
  /// Generic error.  Usually used when input value is not correct.
  InvalidRequest = 3,
  /// Deal execution is reached timeout and rejected.
  TimeoutError = 5,
  /// Generic error for requests by id.
  EntityNotFound = 6,
  /// Connection to Server is lost or not supported.
  CantRouteRequest = 7,
  /// Message is too large.
  FrameTooLong = 8,
  /// Market is closed.
  MarketClosed = 9,
  /// Order is blocked (e.g. under execution) and change cannot be applied.
  ConcurrentModification = 10,
  /// Message is blocked by server or rate limit is reached.
  BlockedPayloadType = 11,
}
impl ErrorCode {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::UnknownError => "UNKNOWN_ERROR",
      Self::UnsupportedMessage => "UNSUPPORTED_MESSAGE",
      Self::InvalidRequest => "INVALID_REQUEST",
      Self::TimeoutError => "TIMEOUT_ERROR",
      Self::EntityNotFound => "ENTITY_NOT_FOUND",
      Self::CantRouteRequest => "CANT_ROUTE_REQUEST",
      Self::FrameTooLong => "FRAME_TOO_LONG",
      Self::MarketClosed => "MARKET_CLOSED",
      Self::ConcurrentModification => "CONCURRENT_MODIFICATION",
      Self::BlockedPayloadType => "BLOCKED_PAYLOAD_TYPE",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "UNKNOWN_ERROR" => Some(Self::UnknownError),
      "UNSUPPORTED_MESSAGE" => Some(Self::UnsupportedMessage),
      "INVALID_REQUEST" => Some(Self::InvalidRequest),
      "TIMEOUT_ERROR" => Some(Self::TimeoutError),
      "ENTITY_NOT_FOUND" => Some(Self::EntityNotFound),
      "CANT_ROUTE_REQUEST" => Some(Self::CantRouteRequest),
      "FRAME_TOO_LONG" => Some(Self::FrameTooLong),
      "MARKET_CLOSED" => Some(Self::MarketClosed),
      "CONCURRENT_MODIFICATION" => Some(Self::ConcurrentModification),
      "BLOCKED_PAYLOAD_TYPE" => Some(Self::BlockedPayloadType),
      _ => None,
    }
  }
}
