use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetClass {
  /// Unique asset ID.
  pub id: Option<i64>,
  /// Asset class name.
  pub name: Option<String>,
  /// The number used for sorting Asset Classes in the UI (lowest number should appear at the top).
  pub sorting_number: Option<f64>,
}

impl From<super::super::messages::ProtoOaAssetClass> for AssetClass {
  fn from(asset_class: super::super::messages::ProtoOaAssetClass) -> Self {
    AssetClass {
      id: asset_class.id,
      name: asset_class.name,
      sorting_number: asset_class.sorting_number,
    }
  }
}
