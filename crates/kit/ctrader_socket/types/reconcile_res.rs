use kit_ctrader_proto::ProtoOaReconcileRes;

use super::Order;
use super::Position;

/// * The response to the ProtoOAReconcileReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ReconcileRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of trader's account open positions.
  pub position: Vec<Position>,
  /// The list of trader's account pending orders.
  pub order: Vec<Order>,
}

impl TryFrom<ProtoOaReconcileRes> for ReconcileRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaReconcileRes) -> Result<Self, Self::Error> {
    Ok(ReconcileRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      position: {
        let mut positions = Vec::<Position>::new();
        for position in value.position {
          positions.push(Position::try_from(position)?);
        }
        positions
      },
      order: {
        let mut orders = Vec::<Order>::new();
        for order in value.order {
          orders.push(Order::try_from(order)?);
        }
        orders
      },
    })
  }
}
