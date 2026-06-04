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
pub enum OrderType {
  Market = 1,
  Limit = 2,
  Stop = 3,
  StopLossTakeProfit = 4,
  MarketRange = 5,
  StopLimit = 6,
}
impl OrderType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Market => "MARKET",
      Self::Limit => "LIMIT",
      Self::Stop => "STOP",
      Self::StopLossTakeProfit => "STOP_LOSS_TAKE_PROFIT",
      Self::MarketRange => "MARKET_RANGE",
      Self::StopLimit => "STOP_LIMIT",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "MARKET" => Some(Self::Market),
      "LIMIT" => Some(Self::Limit),
      "STOP" => Some(Self::Stop),
      "STOP_LOSS_TAKE_PROFIT" => Some(Self::StopLossTakeProfit),
      "MARKET_RANGE" => Some(Self::MarketRange),
      "STOP_LIMIT" => Some(Self::StopLimit),
      _ => None,
    }
  }
}
