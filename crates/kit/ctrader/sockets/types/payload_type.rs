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
pub enum PayloadType {
  /// common intensive
  ProtoMessage = 5,
  /// common commands
  ErrorRes = 50,
  HeartbeatEvent = 51,
}
impl PayloadType {
  /// String value of the enum field names used in the ProtoBuf definition.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::ProtoMessage => "PROTO_MESSAGE",
      Self::ErrorRes => "ERROR_RES",
      Self::HeartbeatEvent => "HEARTBEAT_EVENT",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "PROTO_MESSAGE" => Some(Self::ProtoMessage),
      "ERROR_RES" => Some(Self::ErrorRes),
      "HEARTBEAT_EVENT" => Some(Self::HeartbeatEvent),
      _ => None,
    }
  }
}
