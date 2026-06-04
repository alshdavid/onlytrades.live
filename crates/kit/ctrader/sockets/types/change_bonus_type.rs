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
pub enum ChangeBonusType {
  BonusDeposit = 0,
  BonusWithdraw = 1,
}
impl ChangeBonusType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::BonusDeposit => "BONUS_DEPOSIT",
      Self::BonusWithdraw => "BONUS_WITHDRAW",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "BONUS_DEPOSIT" => Some(Self::BonusDeposit),
      "BONUS_WITHDRAW" => Some(Self::BonusWithdraw),
      _ => None,
    }
  }
}
