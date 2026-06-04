use kit_ctrader_proto::ProtoOaMarginCallListReq;

/// * Request for a list of existing margin call thresholds configured for a user.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct MarginCallListReq {
  pub client_msg_id: Option<String>,
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaMarginCallListReq> for MarginCallListReq {
  fn from(value: ProtoOaMarginCallListReq) -> Self {
    MarginCallListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
