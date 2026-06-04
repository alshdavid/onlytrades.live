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
pub enum PositionStatus {
  PositionStatusOpen = 1,
  PositionStatusClosed = 2,
  /// Empty position is created for pending order.
  PositionStatusCreated = 3,
  PositionStatusError = 4,
}

impl PositionStatus {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::PositionStatusOpen => "POSITION_STATUS_OPEN",
      Self::PositionStatusClosed => "POSITION_STATUS_CLOSED",
      Self::PositionStatusCreated => "POSITION_STATUS_CREATED",
      Self::PositionStatusError => "POSITION_STATUS_ERROR",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "POSITION_STATUS_OPEN" => Some(Self::PositionStatusOpen),
      "POSITION_STATUS_CLOSED" => Some(Self::PositionStatusClosed),
      "POSITION_STATUS_CREATED" => Some(Self::PositionStatusCreated),
      "POSITION_STATUS_ERROR" => Some(Self::PositionStatusError),
      _ => None,
    }
  }
}
