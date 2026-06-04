use kit_ctrader_proto::ProtoOaAssetClassListReq;

/// * Request for a list of asset classes available for the trader's account.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AssetClassListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaAssetClassListReq> for AssetClassListReq {
  fn from(value: ProtoOaAssetClassListReq) -> Self {
    AssetClassListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
