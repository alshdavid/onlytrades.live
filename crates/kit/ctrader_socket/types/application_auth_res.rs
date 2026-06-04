use kit_ctrader_proto::ProtoOaApplicationAuthRes;

/// * Response to the ProtoOAApplicationAuthReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ApplicationAuthRes {
  pub client_msg_id: Option<String>,
}

impl From<ProtoOaApplicationAuthRes> for ApplicationAuthRes {
  fn from(_value: ProtoOaApplicationAuthRes) -> Self {
    ApplicationAuthRes {
      client_msg_id: None,
    }
  }
}
