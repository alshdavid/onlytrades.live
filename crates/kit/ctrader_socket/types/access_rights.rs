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
pub enum AccessRights {
  /// Enable all trading.
  FullAccess = 0,
  /// Only closing trading request are enabled.
  CloseOnly = 1,
  /// View only access.
  NoTrading = 2,
  /// No access.
  NoLogin = 3,
}
impl AccessRights {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::FullAccess => "FULL_ACCESS",
      Self::CloseOnly => "CLOSE_ONLY",
      Self::NoTrading => "NO_TRADING",
      Self::NoLogin => "NO_LOGIN",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "FULL_ACCESS" => Some(Self::FullAccess),
      "CLOSE_ONLY" => Some(Self::CloseOnly),
      "NO_TRADING" => Some(Self::NoTrading),
      "NO_LOGIN" => Some(Self::NoLogin),
      _ => None,
    }
  }
}
