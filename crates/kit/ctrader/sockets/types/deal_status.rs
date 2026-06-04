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
pub enum DealStatus {
  /// Deal filled.
  Filled = 2,
  /// Deal is partially filled.
  PartiallyFilled = 3,
  /// Deal is correct but was rejected by liquidity provider (e.g. no liquidity).
  Rejected = 4,
  /// Deal rejected by server (e.g. no price quotes).
  InternallyRejected = 5,
  /// Deal is rejected by LP due to error (e.g. symbol is unknown).
  Error = 6,
  /// Liquidity provider did not sent response on the deal during specified execution time period.
  Missed = 7,
}

impl DealStatus {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::Filled => "FILLED",
      Self::PartiallyFilled => "PARTIALLY_FILLED",
      Self::Rejected => "REJECTED",
      Self::InternallyRejected => "INTERNALLY_REJECTED",
      Self::Error => "ERROR",
      Self::Missed => "MISSED",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "FILLED" => Some(Self::Filled),
      "PARTIALLY_FILLED" => Some(Self::PartiallyFilled),
      "REJECTED" => Some(Self::Rejected),
      "INTERNALLY_REJECTED" => Some(Self::InternallyRejected),
      "ERROR" => Some(Self::Error),
      "MISSED" => Some(Self::Missed),
      _ => None,
    }
  }
}
