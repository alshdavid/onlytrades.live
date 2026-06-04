use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TickData {
  /// The Unix time in milliseconds of the tick. See ProtoOAGetTickDataRes.tickData for details.
  pub timestamp: i64,
  /// Tick price.
  pub tick: i64,
}

impl From<kit_ctrader_proto::ProtoOaTickData> for TickData {
  fn from(tick_data: kit_ctrader_proto::ProtoOaTickData) -> Self {
    TickData {
      timestamp: tick_data.timestamp,
      tick: tick_data.tick,
    }
  }
}
