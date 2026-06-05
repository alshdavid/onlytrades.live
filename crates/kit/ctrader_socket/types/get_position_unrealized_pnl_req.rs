use kit_ctrader_proto::ProtoOaGetPositionUnrealizedPnLReq;

/// * Request for getting trader's positions' unrealized PnLs.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetPositionUnrealizedPnLReq {
  pub client_msg_id: Option<String>,
  /// The unique identifier of the trader's account in cTrader platform.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaGetPositionUnrealizedPnLReq> for GetPositionUnrealizedPnLReq {
  fn from(value: ProtoOaGetPositionUnrealizedPnLReq) -> Self {
    GetPositionUnrealizedPnLReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
