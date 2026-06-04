use kit_ctrader_proto::ProtoOaDealOffsetListReq;

/// * Request for getting sets of Deals that were offset by a specific Deal and that are offsetting the Deal.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct DealOffsetListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The unique ID of the Deal.
  pub deal_id: i64,
}

impl From<ProtoOaDealOffsetListReq> for DealOffsetListReq {
  fn from(value: ProtoOaDealOffsetListReq) -> Self {
    DealOffsetListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      deal_id: value.deal_id,
    }
  }
}
