use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymbolCategory {
  /// The unique identifier of the symbol category.
  pub id: i64,
  /// Link to the asset class. One asset class can have many symbol categories.
  pub asset_class_id: i64,
  /// Category name.
  pub name: String,
  /// The number used for sorting Symbol Categories in the UI (lowest number should appear at the top).
  pub sorting_number: Option<f64>,
}

impl From<kit_ctrader_proto::ProtoOaSymbolCategory> for SymbolCategory {
  fn from(category: kit_ctrader_proto::ProtoOaSymbolCategory) -> Self {
    SymbolCategory {
      id: category.id,
      asset_class_id: category.asset_class_id,
      name: category.name,
      sorting_number: category.sorting_number,
    }
  }
}
