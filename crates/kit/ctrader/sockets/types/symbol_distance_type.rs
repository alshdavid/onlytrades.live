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
pub enum SymbolDistanceType {
  SymbolDistanceInPoints = 1,
  SymbolDistanceInPercentage = 2,
}
impl SymbolDistanceType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::SymbolDistanceInPoints => "SYMBOL_DISTANCE_IN_POINTS",
      Self::SymbolDistanceInPercentage => "SYMBOL_DISTANCE_IN_PERCENTAGE",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "SYMBOL_DISTANCE_IN_POINTS" => Some(Self::SymbolDistanceInPoints),
      "SYMBOL_DISTANCE_IN_PERCENTAGE" => Some(Self::SymbolDistanceInPercentage),
      _ => None,
    }
  }
}
