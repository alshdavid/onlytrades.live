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
pub enum TradingMode {
  Enabled = 0,
  DisabledWithoutPendingsExecution = 1,
  DisabledWithPendingsExecution = 2,
  CloseOnlyMode = 3,
}
impl TradingMode {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Enabled => "ENABLED",
      Self::DisabledWithoutPendingsExecution => "DISABLED_WITHOUT_PENDINGS_EXECUTION",
      Self::DisabledWithPendingsExecution => "DISABLED_WITH_PENDINGS_EXECUTION",
      Self::CloseOnlyMode => "CLOSE_ONLY_MODE",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "ENABLED" => Some(Self::Enabled),
      "DISABLED_WITHOUT_PENDINGS_EXECUTION" => Some(Self::DisabledWithoutPendingsExecution),
      "DISABLED_WITH_PENDINGS_EXECUTION" => Some(Self::DisabledWithPendingsExecution),
      "CLOSE_ONLY_MODE" => Some(Self::CloseOnlyMode),
      _ => None,
    }
  }
}
