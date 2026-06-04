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
pub enum OrderTriggerMethod {
  /// Stop Order: buy is triggered by ask, sell by bid; Stop Loss Order: for buy position is triggered by bid and for sell position by ask.
  Trade = 1,
  /// Stop Order: buy is triggered by bid, sell by ask; Stop Loss Order: for buy position is triggered by ask and for sell position by bid.
  Opposite = 2,
  /// The same as TRADE, but trigger is checked after the second consecutive tick.
  DoubleTrade = 3,
  /// The same as OPPOSITE, but trigger is checked after the second consecutive tick.
  DoubleOpposite = 4,
}
impl OrderTriggerMethod {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Trade => "TRADE",
      Self::Opposite => "OPPOSITE",
      Self::DoubleTrade => "DOUBLE_TRADE",
      Self::DoubleOpposite => "DOUBLE_OPPOSITE",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "TRADE" => Some(Self::Trade),
      "OPPOSITE" => Some(Self::Opposite),
      "DOUBLE_TRADE" => Some(Self::DoubleTrade),
      "DOUBLE_OPPOSITE" => Some(Self::DoubleOpposite),
      _ => None,
    }
  }
}
