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
pub enum DayOfWeek {
  None = 0,
  Monday = 1,
  Tuesday = 2,
  Wednesday = 3,
  Thursday = 4,
  Friday = 5,
  Saturday = 6,
  Sunday = 7,
}
impl DayOfWeek {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::None => "NONE",
      Self::Monday => "MONDAY",
      Self::Tuesday => "TUESDAY",
      Self::Wednesday => "WEDNESDAY",
      Self::Thursday => "THURSDAY",
      Self::Friday => "FRIDAY",
      Self::Saturday => "SATURDAY",
      Self::Sunday => "SUNDAY",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "NONE" => Some(Self::None),
      "MONDAY" => Some(Self::Monday),
      "TUESDAY" => Some(Self::Tuesday),
      "WEDNESDAY" => Some(Self::Wednesday),
      "THURSDAY" => Some(Self::Thursday),
      "FRIDAY" => Some(Self::Friday),
      "SATURDAY" => Some(Self::Saturday),
      "SUNDAY" => Some(Self::Sunday),
      _ => None,
    }
  }
}
