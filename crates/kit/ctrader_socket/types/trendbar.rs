use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trendbar {
  /// Bar volume in ticks.
  pub volume: i64,
  /// Bar period.
  pub period: Option<i32>,
  /// Low price of the bar.
  pub low: Option<i64>,
  /// Delta between open and low price. open = low + deltaOpen.
  pub delta_open: Option<u64>,
  /// Delta between close and low price. close = low + deltaClose.
  pub delta_close: Option<u64>,
  /// Delta between high and low price. high = low + deltaHigh.
  pub delta_high: Option<u64>,
  /// The Unix time in minutes of the bar, equal to the timestamp of the open tick.
  pub utc_timestamp_in_minutes: Option<u32>,
}

impl From<kit_ctrader_proto::ProtoOaTrendbar> for Trendbar {
  fn from(value: kit_ctrader_proto::ProtoOaTrendbar) -> Self {
    Trendbar {
      volume: value.volume,
      period: value.period,
      low: value.low,
      delta_open: value.delta_open,
      delta_close: value.delta_close,
      delta_high: value.delta_high,
      utc_timestamp_in_minutes: value.utc_timestamp_in_minutes,
    }
  }
}
