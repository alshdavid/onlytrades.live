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
pub enum OrderStatus {
  /// Order request validated and accepted for execution.
  OrderStatusAccepted = 1,
  /// Order is fully filled.
  OrderStatusFilled = 2,
  /// Order is rejected due to validation.
  OrderStatusRejected = 3,
  /// Order expired. Might be valid for orders with partially filled volume that were expired on LP.
  OrderStatusExpired = 4,
  /// Order is cancelled. Might be valid for orders with partially filled volume that were cancelled by LP.
  OrderStatusCancelled = 5,
}

impl OrderStatus {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::OrderStatusAccepted => "ORDER_STATUS_ACCEPTED",
      Self::OrderStatusFilled => "ORDER_STATUS_FILLED",
      Self::OrderStatusRejected => "ORDER_STATUS_REJECTED",
      Self::OrderStatusExpired => "ORDER_STATUS_EXPIRED",
      Self::OrderStatusCancelled => "ORDER_STATUS_CANCELLED",
    }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "ORDER_STATUS_ACCEPTED" => Some(Self::OrderStatusAccepted),
      "ORDER_STATUS_FILLED" => Some(Self::OrderStatusFilled),
      "ORDER_STATUS_REJECTED" => Some(Self::OrderStatusRejected),
      "ORDER_STATUS_EXPIRED" => Some(Self::OrderStatusExpired),
      "ORDER_STATUS_CANCELLED" => Some(Self::OrderStatusCancelled),
      _ => None,
    }
  }
}
