use kit_ctrader_proto::ProtoOaMarginCallUpdateReq;

use super::MarginCall;

/// * Request to modify marginLevelThreshold of specified marginCallType for ctidTraderAccountId.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct MarginCallUpdateReq {
  pub client_msg_id: Option<String>,
  pub ctid_trader_account_id: i64,
  pub margin_call: MarginCall,
}

impl From<ProtoOaMarginCallUpdateReq> for MarginCallUpdateReq {
  fn from(value: ProtoOaMarginCallUpdateReq) -> Self {
    MarginCallUpdateReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      margin_call: MarginCall::from(value.margin_call),
    }
  }
}
