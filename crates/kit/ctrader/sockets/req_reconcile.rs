use serde::Serialize;

use super::CTraderSocketClient;
use kit_ctrader_proto::*;
use super::types::*;

pub struct CTraderReconcileOptions {
  pub account_id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CTraderReconcileResult {
  /// The list of trader's account open positions.
  pub position: Vec<Position>,
  /// The list of trader's account pending orders.
  pub order: Vec<Order>,
}

impl CTraderSocketClient {
  pub async fn reconcile(
    &self,
    options: CTraderReconcileOptions,
  ) -> anyhow::Result<CTraderReconcileResult> {
    let res = self
      .send_and_receive_oneshot::<ProtoOaReconcileRes>(
        ProtoOaPayloadType::ProtoOaReconcileReq,
        ProtoOaReconcileReq {
          payload_type: None,
          ctid_trader_account_id: options.account_id,
          return_protection_orders: None,
        },
      )
      .await?;

    Ok(CTraderReconcileResult {
      position: {
        let mut positions = Vec::<Position>::new();
        for position in res.position {
          positions.push(Position::try_from(position)?);
        }
        positions
      },
      order: {
        let mut orders = Vec::<Order>::new();
        for order in res.order {
          orders.push(Order::try_from(order)?);
        }
        orders
      },
    })
  }
}
