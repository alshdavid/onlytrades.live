use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicLeverage {
  /// Unique ID of dynamic leverage.
  pub leverage_id: i64,
  /// Tiers sorted by volume. Last tier's leverage is applied also to volume above specified.
  pub tiers: Vec<DynamicLeverageTier>,
}

impl From<super::super::messages::ProtoOaDynamicLeverage> for DynamicLeverage {
  fn from(leverage: super::super::messages::ProtoOaDynamicLeverage) -> Self {
    DynamicLeverage {
      leverage_id: leverage.leverage_id,
      tiers: leverage
        .tiers
        .into_iter()
        .map(DynamicLeverageTier::from)
        .collect(),
    }
  }
}
