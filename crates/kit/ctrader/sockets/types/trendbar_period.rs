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
pub enum TrendbarPeriod {
  M1 = 1,
  M2 = 2,
  M3 = 3,
  M4 = 4,
  M5 = 5,
  M10 = 6,
  M15 = 7,
  M30 = 8,
  H1 = 9,
  H4 = 10,
  H12 = 11,
  D1 = 12,
  W1 = 13,
  Mn1 = 14,
}
impl TrendbarPeriod {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::M1 => "M1",
      Self::M2 => "M2",
      Self::M3 => "M3",
      Self::M4 => "M4",
      Self::M5 => "M5",
      Self::M10 => "M10",
      Self::M15 => "M15",
      Self::M30 => "M30",
      Self::H1 => "H1",
      Self::H4 => "H4",
      Self::H12 => "H12",
      Self::D1 => "D1",
      Self::W1 => "W1",
      Self::Mn1 => "MN1",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "M1" => Some(Self::M1),
      "M2" => Some(Self::M2),
      "M3" => Some(Self::M3),
      "M4" => Some(Self::M4),
      "M5" => Some(Self::M5),
      "M10" => Some(Self::M10),
      "M15" => Some(Self::M15),
      "M30" => Some(Self::M30),
      "H1" => Some(Self::H1),
      "H4" => Some(Self::H4),
      "H12" => Some(Self::H12),
      "D1" => Some(Self::D1),
      "W1" => Some(Self::W1),
      "MN1" => Some(Self::Mn1),
      _ => None,
    }
  }
}
