use kit_ctrader_proto::ProtoOaOrderDetailsRes;

use super::Deal;
use super::Order;

/// * Response to the ProtoOAOrderDetailsReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct OrderDetailsRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Order details.
  pub order: Order,
  /// All Deals created by filling the specified Order.
  pub deal: Vec<Deal>,
}

impl TryFrom<ProtoOaOrderDetailsRes> for OrderDetailsRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaOrderDetailsRes) -> Result<Self, Self::Error> {
    Ok(OrderDetailsRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      order: Order::try_from(value.order)?,
      deal: {
        let mut deals = Vec::<Deal>::new();
        for deal in value.deal {
          deals.push(Deal::try_from(deal)?);
        }
        deals
      },
    })
  }
}
