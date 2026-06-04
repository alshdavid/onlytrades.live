use kit_ctrader_proto::ProtoOaMarginCallUpdateRes;

/// * If this response received, it means that margin call was successfully updated.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct MarginCallUpdateRes {
  pub client_msg_id: Option<String>,
}

impl From<ProtoOaMarginCallUpdateRes> for MarginCallUpdateRes {
  fn from(_value: ProtoOaMarginCallUpdateRes) -> Self {
    MarginCallUpdateRes {
      client_msg_id: None,
    }
  }
}
