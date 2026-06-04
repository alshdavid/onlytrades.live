use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DealOffset {
  /// The unique ID of the execution Deal.
  pub deal_id: i64,
  /// Matched volume, in cents.
  pub volume: i64,
  /// The Unix time in milliseconds when the offset Deal was executed.
  pub execution_timestamp: Option<i64>,
  ///   Execution price of the offset Deal.
  pub execution_price: Option<f64>,
}

impl From<kit_ctrader_proto::ProtoOaDealOffset> for DealOffset {
  fn from(deal_offset: kit_ctrader_proto::ProtoOaDealOffset) -> Self {
    DealOffset {
      deal_id: deal_offset.deal_id,
      volume: deal_offset.volume,
      execution_timestamp: deal_offset.execution_timestamp,
      execution_price: deal_offset.execution_price,
    }
  }
}
