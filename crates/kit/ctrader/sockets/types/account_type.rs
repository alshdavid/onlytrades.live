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
pub enum AccountType {
  /// Allows multiple positions on a trading account for a symbol.
  Hedged = 0,
  /// Only one position per symbol is allowed on a trading account.
  Netted = 1,
  /// Spread betting type account.
  SpreadBetting = 2,
}
impl AccountType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Hedged => "HEDGED",
      Self::Netted => "NETTED",
      Self::SpreadBetting => "SPREAD_BETTING",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "HEDGED" => Some(Self::Hedged),
      "NETTED" => Some(Self::Netted),
      "SPREAD_BETTING" => Some(Self::SpreadBetting),
      _ => None,
    }
  }
}
