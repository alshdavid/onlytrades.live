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
pub enum ClientPermissionScope {
  /// Allows to use only view commends. Trade is prohibited.
  ScopeView = 0,
  /// Allows to use all commands.
  ScopeTrade = 1,
}
impl ClientPermissionScope {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::ScopeView => "SCOPE_VIEW",
      Self::ScopeTrade => "SCOPE_TRADE",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "SCOPE_VIEW" => Some(Self::ScopeView),
      "SCOPE_TRADE" => Some(Self::ScopeTrade),
      _ => None,
    }
  }
}
