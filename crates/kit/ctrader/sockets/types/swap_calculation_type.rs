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
pub enum SwapCalculationType {
  /// Specifies type of SWAP computation as PIPS (0)
  Pips = 0,
  /// Specifies type of SWAP computation as PERCENTAGE (1, annual, in percent)
  Percentage = 1,
  /// Specifies type of SWAP computation as POINTS (2)
  Points = 2,
}
impl SwapCalculationType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Pips => "PIPS",
      Self::Percentage => "PERCENTAGE",
      Self::Points => "POINTS",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "PIPS" => Some(Self::Pips),
      "PERCENTAGE" => Some(Self::Percentage),
      "POINTS" => Some(Self::Points),
      _ => None,
    }
  }
}
