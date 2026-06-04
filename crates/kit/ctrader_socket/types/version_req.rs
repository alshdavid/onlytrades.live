use kit_ctrader_proto::ProtoOaVersionReq;

/// * Request for getting the proxy version. Can be used to check the current version of the Open API scheme.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct VersionReq {
  pub client_msg_id: Option<String>,
}

impl From<ProtoOaVersionReq> for VersionReq {
  fn from(_value: ProtoOaVersionReq) -> Self {
    VersionReq {
      client_msg_id: None,
    }
  }
}
