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
pub enum LimitedRiskMarginCalculationStrategy {
  AccordingToLeverage = 0,
  AccordingToGsl = 1,
  AccordingToGslAndLeverage = 2,
}
impl LimitedRiskMarginCalculationStrategy {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::AccordingToLeverage => "ACCORDING_TO_LEVERAGE",
      Self::AccordingToGsl => "ACCORDING_TO_GSL",
      Self::AccordingToGslAndLeverage => "ACCORDING_TO_GSL_AND_LEVERAGE",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "ACCORDING_TO_LEVERAGE" => Some(Self::AccordingToLeverage),
      "ACCORDING_TO_GSL" => Some(Self::AccordingToGsl),
      "ACCORDING_TO_GSL_AND_LEVERAGE" => Some(Self::AccordingToGslAndLeverage),
      _ => None,
    }
  }
}
