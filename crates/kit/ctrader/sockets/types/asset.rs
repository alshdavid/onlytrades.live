use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Asset {
  /// The unique asset ID.
  pub asset_id: i64,
  /// The asset name.
  pub name: String,
  /// User friendly name.
  pub display_name: Option<String>,
  /// Precision of the asset.
  pub digits: Option<i32>,
}

impl From<super::super::messages::ProtoOaAsset> for Asset {
  fn from(asset: super::super::messages::ProtoOaAsset) -> Self {
    Asset {
      asset_id: asset.asset_id,
      name: asset.name,
      display_name: asset.display_name,
      digits: asset.digits,
    }
  }
}
