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
  Deserialize,
  Serialize,
)]
#[repr(i32)]
pub enum TradeSide {
  Buy = 1,
  Sell = 2,
}
impl TradeSide {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Buy => "BUY",
      Self::Sell => "SELL",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "BUY" => Some(Self::Buy),
      "SELL" => Some(Self::Sell),
      "buy" => Some(Self::Buy),
      "sell" => Some(Self::Sell),
      _ => None,
    }
  }
}
