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
pub enum TotalMarginCalculationType {
  Max = 0,
  Sum = 1,
  Net = 2,
}
impl TotalMarginCalculationType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Max => "MAX",
      Self::Sum => "SUM",
      Self::Net => "NET",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "MAX" => Some(Self::Max),
      "SUM" => Some(Self::Sum),
      "NET" => Some(Self::Net),
      _ => None,
    }
  }
}
