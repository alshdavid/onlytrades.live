use kit_ctrader_proto::ProtoOaAssetListReq;

/// * Request for the list of assets available for a trader's account.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AssetListReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
}

impl From<ProtoOaAssetListReq> for AssetListReq {
  fn from(value: ProtoOaAssetListReq) -> Self {
    AssetListReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
    }
  }
}
