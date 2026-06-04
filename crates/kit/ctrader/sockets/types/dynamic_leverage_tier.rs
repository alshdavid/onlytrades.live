use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DynamicLeverageTier {
  /// Max USD volume (in cents) of the Open Position (per side) to apply specified leverage. Last tier's leverage is applied also to volume above specified.
  pub volume: i64,
  /// Applied leverage.
  pub leverage: i32,
}

impl From<super::super::messages::ProtoOaDynamicLeverageTier> for DynamicLeverageTier {
  fn from(tier: super::super::messages::ProtoOaDynamicLeverageTier) -> Self {
    DynamicLeverageTier {
      volume: tier.volume,
      leverage: tier.leverage,
    }
  }
}
