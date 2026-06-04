use kit_ctrader_proto::ProtoOaGetPositionUnrealizedPnLRes;

use super::PositionUnrealizedPnL;

/// * Response to ProtoOAGetPositionUnrealizedPnLReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetPositionUnrealizedPnLRes {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
  /// Information about trader's positions' unrealized PnLs.
  pub position_unrealized_pn_l: Vec<PositionUnrealizedPnL>,
  /// Specifies the exponent of various monetary values. E.g., moneyDigits = 8 should be interpreted as the value multiplied by 10^8 with the 'real' value equal to 10053099944 / 10^8 = 100.53099944. Affects positionUnrealizedPnL.grossUnrealizedPnL, positionUnrealizedPnL.netUnrealizedPnL.
  pub money_digits: u32,
}

impl From<ProtoOaGetPositionUnrealizedPnLRes> for GetPositionUnrealizedPnLRes {
  fn from(value: ProtoOaGetPositionUnrealizedPnLRes) -> Self {
    GetPositionUnrealizedPnLRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      position_unrealized_pn_l: value
        .position_unrealized_pn_l
        .into_iter()
        .map(PositionUnrealizedPnL::from)
        .collect(),
      money_digits: value.money_digits,
    }
  }
}
