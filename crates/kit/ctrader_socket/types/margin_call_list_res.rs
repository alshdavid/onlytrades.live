use kit_ctrader_proto::ProtoOaMarginCallListRes;

use super::MarginCall;

/// * Response with a list of existing user Margin Calls, usually contains 3 items.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct MarginCallListRes {
  pub client_msg_id: Option<String>,
  pub margin_call: Vec<MarginCall>,
}

impl From<ProtoOaMarginCallListRes> for MarginCallListRes {
  fn from(value: ProtoOaMarginCallListRes) -> Self {
    MarginCallListRes {
      client_msg_id: None,
      margin_call: value
        .margin_call
        .into_iter()
        .map(MarginCall::from)
        .collect(),
    }
  }
}
