use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Interval {
  /// Interval start, specified in seconds starting from SUNDAY 00:00 in specified time zone (inclusive to the interval).
  pub start_second: u32,
  /// Interval end, specified in seconds starting from SUNDAY 00:00 in specified time zone (exclusive from the interval).
  pub end_second: u32,
}

impl From<super::super::messages::ProtoOaInterval> for Interval {
  fn from(interval: super::super::messages::ProtoOaInterval) -> Self {
    Interval {
      start_second: interval.start_second,
      end_second: interval.end_second,
    }
  }
}
