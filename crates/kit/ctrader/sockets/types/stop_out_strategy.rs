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
pub enum StopOutStrategy {
  /// A Stop Out strategy that closes a Position with the largest Used Margin
  MostMarginUsedFirst = 0,
  /// A Stop Out strategy that closes a Position with the least PnL
  MostLosingFirst = 1,
}
impl StopOutStrategy {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::MostMarginUsedFirst => "MOST_MARGIN_USED_FIRST",
      Self::MostLosingFirst => "MOST_LOSING_FIRST",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "MOST_MARGIN_USED_FIRST" => Some(Self::MostMarginUsedFirst),
      "MOST_LOSING_FIRST" => Some(Self::MostLosingFirst),
      _ => None,
    }
  }
}
