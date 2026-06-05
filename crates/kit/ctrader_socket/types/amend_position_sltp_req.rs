use kit_ctrader_proto::ProtoOaAmendPositionSltpReq;
use num_enum::TryFromPrimitive;

use super::OrderTriggerMethod;

/// * Request for amending StopLoss and TakeProfit of existing position. Allowed only if the accessToken has "trade" permissions for the trading account.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AmendPositionSltpReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the position to amend.
  pub position_id: i64,
  /// Absolute Stop Loss price (1.23456 for example).
  pub stop_loss: Option<f64>,
  /// Absolute Take Profit price (1.26543 for example).
  pub take_profit: Option<f64>,
  /// If TRUE then the Stop Loss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
  pub guaranteed_stop_loss: Option<bool>,
  /// If TRUE then the Trailing Stop Loss is applied.
  pub trailing_stop_loss: Option<bool>,
  /// The Stop trigger method for the Stop Loss/Take Profit order.
  pub stop_loss_trigger_method: Option<OrderTriggerMethod>,
}

impl TryFrom<ProtoOaAmendPositionSltpReq> for AmendPositionSltpReq {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaAmendPositionSltpReq) -> Result<Self, Self::Error> {
    Ok(AmendPositionSltpReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      position_id: value.position_id,
      stop_loss: value.stop_loss,
      take_profit: value.take_profit,
      guaranteed_stop_loss: value.guaranteed_stop_loss,
      trailing_stop_loss: value.trailing_stop_loss,
      stop_loss_trigger_method: match value.stop_loss_trigger_method {
        Some(stop_loss_trigger_method) => Some(OrderTriggerMethod::try_from_primitive(
          stop_loss_trigger_method,
        )?),
        None => None,
      },
    })
  }
}
