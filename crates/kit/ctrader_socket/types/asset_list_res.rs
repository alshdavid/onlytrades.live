use kit_ctrader_proto::ProtoOaAssetListRes;

use super::Asset;

/// * Response to the ProtoOAAssetListReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AssetListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of assets.
  pub asset: Vec<Asset>,
}

impl From<ProtoOaAssetListRes> for AssetListRes {
  fn from(value: ProtoOaAssetListRes) -> Self {
    AssetListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      asset: value.asset.into_iter().map(Asset::from).collect(),
    }
  }
}
