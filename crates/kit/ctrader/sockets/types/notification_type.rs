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
pub enum NotificationType {
  /// one of three margin calls, they are all similar.
  MarginLevelThreshold1 = 61,
  /// one of three margin calls, they are all similar.
  MarginLevelThreshold2 = 62,
  /// one of three margin calls, they are all similar.
  MarginLevelThreshold3 = 63,
}
impl NotificationType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::MarginLevelThreshold1 => "MARGIN_LEVEL_THRESHOLD_1",
      Self::MarginLevelThreshold2 => "MARGIN_LEVEL_THRESHOLD_2",
      Self::MarginLevelThreshold3 => "MARGIN_LEVEL_THRESHOLD_3",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "MARGIN_LEVEL_THRESHOLD_1" => Some(Self::MarginLevelThreshold1),
      "MARGIN_LEVEL_THRESHOLD_2" => Some(Self::MarginLevelThreshold2),
      "MARGIN_LEVEL_THRESHOLD_3" => Some(Self::MarginLevelThreshold3),
      _ => None,
    }
  }
}
