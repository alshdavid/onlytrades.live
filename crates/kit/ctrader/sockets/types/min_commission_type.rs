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
pub enum MinCommissionType {
  Currency = 1,
  QuoteCurrency = 2,
}
impl MinCommissionType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Currency => "CURRENCY",
      Self::QuoteCurrency => "QUOTE_CURRENCY",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "CURRENCY" => Some(Self::Currency),
      "QUOTE_CURRENCY" => Some(Self::QuoteCurrency),
      _ => None,
    }
  }
}
