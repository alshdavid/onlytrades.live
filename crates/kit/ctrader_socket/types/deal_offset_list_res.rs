use kit_ctrader_proto::ProtoOaDealOffsetListRes;

use super::DealOffset;

/// * Response for ProtoOADealOffsetListReq.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct DealOffsetListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Deals which closed the specified deal.
  pub offset_by: Vec<DealOffset>,
  /// Deals which were closed by the specified deal.
  pub offsetting: Vec<DealOffset>,
}

impl From<ProtoOaDealOffsetListRes> for DealOffsetListRes {
  fn from(value: ProtoOaDealOffsetListRes) -> Self {
    DealOffsetListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      offset_by: value.offset_by.into_iter().map(DealOffset::from).collect(),
      offsetting: value.offsetting.into_iter().map(DealOffset::from).collect(),
    }
  }
}
