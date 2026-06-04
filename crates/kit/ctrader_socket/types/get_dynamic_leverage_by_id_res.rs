use kit_ctrader_proto::ProtoOaGetDynamicLeverageByIdRes;

use super::DynamicLeverage;

/// * Response to the ProtoOAGetDynamicLeverageByIDReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetDynamicLeverageByIdRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  pub leverage: DynamicLeverage,
}

impl From<ProtoOaGetDynamicLeverageByIdRes> for GetDynamicLeverageByIdRes {
  fn from(value: ProtoOaGetDynamicLeverageByIdRes) -> Self {
    GetDynamicLeverageByIdRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      leverage: DynamicLeverage::from(value.leverage),
    }
  }
}
