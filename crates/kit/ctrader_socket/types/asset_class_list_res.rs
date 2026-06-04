use kit_ctrader_proto::ProtoOaAssetClassListRes;

use super::AssetClass;

/// * Response to the ProtoOAAssetListReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct AssetClassListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// List of the asset classes.
  pub asset_class: Vec<AssetClass>,
}

impl From<ProtoOaAssetClassListRes> for AssetClassListRes {
  fn from(value: ProtoOaAssetClassListRes) -> Self {
    AssetClassListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      asset_class: value
        .asset_class
        .into_iter()
        .map(AssetClass::from)
        .collect(),
    }
  }
}
