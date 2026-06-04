use kit_ctrader_proto::ProtoOaGetDynamicLeverageByIdReq;

/// * Request for getting a dynamic leverage entity referenced in ProtoOASymbol.leverageId.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetDynamicLeverageByIdReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  pub leverage_id: i64,
}

impl From<ProtoOaGetDynamicLeverageByIdReq> for GetDynamicLeverageByIdReq {
  fn from(value: ProtoOaGetDynamicLeverageByIdReq) -> Self {
    GetDynamicLeverageByIdReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      leverage_id: value.leverage_id,
    }
  }
}
