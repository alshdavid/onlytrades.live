use kit_ctrader_proto::ProtoOaOrderListRes;

use super::Order;

/// * The response to the ProtoOAOrderListReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct OrderListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of the orders.
  pub order: Vec<Order>,
  /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
  pub has_more: bool,
}

impl TryFrom<ProtoOaOrderListRes> for OrderListRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaOrderListRes) -> Result<Self, Self::Error> {
    Ok(OrderListRes {
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
