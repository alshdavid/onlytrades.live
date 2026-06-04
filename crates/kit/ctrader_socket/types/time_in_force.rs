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
pub enum TimeInForce {
  GoodTillDate = 1,
  GoodTillCancel = 2,
  ImmediateOrCancel = 3,
  FillOrKill = 4,
  MarketOnOpen = 5,
}
impl TimeInForce {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::GoodTillDate => "GOOD_TILL_DATE",
      Self::GoodTillCancel => "GOOD_TILL_CANCEL",
      Self::ImmediateOrCancel => "IMMEDIATE_OR_CANCEL",
      Self::FillOrKill => "FILL_OR_KILL",
      Self::MarketOnOpen => "MARKET_ON_OPEN",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "GOOD_TILL_DATE" => Some(Self::GoodTillDate),
      "GOOD_TILL_CANCEL" => Some(Self::GoodTillCancel),
      "IMMEDIATE_OR_CANCEL" => Some(Self::ImmediateOrCancel),
      "FILL_OR_KILL" => Some(Self::FillOrKill),
      "MARKET_ON_OPEN" => Some(Self::MarketOnOpen),
      _ => None,
    }
  }
}
