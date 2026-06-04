use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpectedMargin {
  /// Volume in cents used for computation of expected margin.
  pub volume: i64,
  /// Buy margin amount.
  pub buy_margin: i64,
  /// Sell margin amount.
  pub sell_margin: i64,
}

impl From<super::super::messages::ProtoOaExpectedMargin> for ExpectedMargin {
  fn from(margin: super::super::messages::ProtoOaExpectedMargin) -> Self {
    ExpectedMargin {
      volume: margin.volume,
      buy_margin: margin.buy_margin,
      sell_margin: margin.sell_margin,
    }
  }
}
