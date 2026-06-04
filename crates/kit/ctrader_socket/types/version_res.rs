use kit_ctrader_proto::ProtoOaVersionRes;

/// * Response to the ProtoOAVersionReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct VersionRes {
  pub client_msg_id: Option<String>,
  /// The current version of the server application.
  pub version: String,
}

impl From<ProtoOaVersionRes> for VersionRes {
  fn from(value: ProtoOaVersionRes) -> Self {
    VersionRes {
      client_msg_id: None,
      version: value.version,
    }
  }
}
