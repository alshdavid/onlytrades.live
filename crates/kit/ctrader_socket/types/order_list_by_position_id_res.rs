use kit_ctrader_proto::ProtoOaOrderListByPositionIdRes;

use super::Order;

/// * Response to ProtoOAOrderListByPositionIdReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct OrderListByPositionIdRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Orders related to the specified Position, sorted by utcLastUpdateTimestamp in descending order (newest first).
  pub order: Vec<Order>,
  /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
  pub has_more: bool,
}

impl TryFrom<ProtoOaOrderListByPositionIdRes> for OrderListByPositionIdRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaOrderListByPositionIdRes) -> Result<Self, Self::Error> {
    Ok(OrderListByPositionIdRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      order: {
        let mut orders = Vec::<Order>::new();
        for order in value.order {
          orders.push(Order::try_from(order)?);
        }
        orders
      },
      has_more: value.has_more,
    })
  }
}
