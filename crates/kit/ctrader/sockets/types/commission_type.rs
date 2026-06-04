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
pub enum CommissionType {
  /// USD per million USD volume - usually used for FX. Example: 50 USD for 1 mil USD of trading volume.
  UsdPerMillionUsd = 1,
  /// USD per 1 lot - usually used for CFDs and futures for commodities, and indices. Example: 15 USD for 1 contract.
  UsdPerLot = 2,
  /// Percentage of trading volume - usually used for Equities. Example: 0.005% of notional trading volume. Multiplied by 100,000.
  PercentageOfValue = 3,
  /// Quote ccy of Symbol per 1 lot - will be used for CFDs and futures for commodities, and indices. Example: 15 EUR for 1 contract of DAX.
  QuoteCcyPerLot = 4,
}
impl CommissionType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::UsdPerMillionUsd => "USD_PER_MILLION_USD",
      Self::UsdPerLot => "USD_PER_LOT",
      Self::PercentageOfValue => "PERCENTAGE_OF_VALUE",
      Self::QuoteCcyPerLot => "QUOTE_CCY_PER_LOT",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "USD_PER_MILLION_USD" => Some(Self::UsdPerMillionUsd),
      "USD_PER_LOT" => Some(Self::UsdPerLot),
      "PERCENTAGE_OF_VALUE" => Some(Self::PercentageOfValue),
      "QUOTE_CCY_PER_LOT" => Some(Self::QuoteCcyPerLot),
      _ => None,
    }
  }
}
