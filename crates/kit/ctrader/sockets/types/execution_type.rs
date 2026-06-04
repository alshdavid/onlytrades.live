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
pub enum ExecutionType {
  /// Order passed validation.
  OrderAccepted = 2,
  /// Order filled.
  OrderFilled = 3,
  /// Pending order is changed with a new one.
  OrderReplaced = 4,
  /// Order cancelled.
  OrderCancelled = 5,
  /// Order with GTD time in force is expired.
  OrderExpired = 6,
  /// Order is rejected due to validations.
  OrderRejected = 7,
  /// Cancel order request is rejected.
  OrderCancelRejected = 8,
  /// Type related to SWAP execution events.
  Swap = 9,
  /// Type related to event of deposit or withdrawal cash flow operation.
  DepositWithdraw = 10,
  /// Order is partially filled.
  OrderPartialFill = 11,
  /// Type related to event of bonus deposit or bonus withdrawal.
  BonusDepositWithdraw = 12,
}

impl ExecutionType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::OrderAccepted => "ORDER_ACCEPTED",
      Self::OrderFilled => "ORDER_FILLED",
      Self::OrderReplaced => "ORDER_REPLACED",
      Self::OrderCancelled => "ORDER_CANCELLED",
      Self::OrderExpired => "ORDER_EXPIRED",
      Self::OrderRejected => "ORDER_REJECTED",
      Self::OrderCancelRejected => "ORDER_CANCEL_REJECTED",
      Self::Swap => "SWAP",
      Self::DepositWithdraw => "DEPOSIT_WITHDRAW",
      Self::OrderPartialFill => "ORDER_PARTIAL_FILL",
      Self::BonusDepositWithdraw => "BONUS_DEPOSIT_WITHDRAW",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "ORDER_ACCEPTED" => Some(Self::OrderAccepted),
      "ORDER_FILLED" => Some(Self::OrderFilled),
      "ORDER_REPLACED" => Some(Self::OrderReplaced),
      "ORDER_CANCELLED" => Some(Self::OrderCancelled),
      "ORDER_EXPIRED" => Some(Self::OrderExpired),
      "ORDER_REJECTED" => Some(Self::OrderRejected),
      "ORDER_CANCEL_REJECTED" => Some(Self::OrderCancelRejected),
      "SWAP" => Some(Self::Swap),
      "DEPOSIT_WITHDRAW" => Some(Self::DepositWithdraw),
      "ORDER_PARTIAL_FILL" => Some(Self::OrderPartialFill),
      "BONUS_DEPOSIT_WITHDRAW" => Some(Self::BonusDepositWithdraw),
      _ => None,
    }
  }
}
