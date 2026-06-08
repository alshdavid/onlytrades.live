use num_enum::TryFromPrimitive;
use serde::Deserialize;
use serde::Serialize;

use crate::TrendbarPeriod;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trendbar {
  /// Bar volume in ticks.
  pub volume: i64,
  /// Bar period.
  pub period: Option<TrendbarPeriod>,
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

impl std::fmt::Debug for Trendbar {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Trendbar")
      .field("volume", &self.volume)
      .field("period", &self.period)
      .field("open", &self.open_price())
      .field("close", &self.close_price())
      .field("high", &self.high_price())
      .field("low", &self.low_price())
      .field("delta_open", &self.delta_open)
      .field("delta_close", &self.delta_close)
      .field("delta_high", &self.delta_high)
      .field("utc_timestamp_in_minutes", &self.utc_timestamp_in_minutes)
      .field("time", &self.timestamp_locale())
      .finish()
  }
}

impl From<kit_ctrader_proto::ProtoOaTrendbar> for Trendbar {
  fn from(value: kit_ctrader_proto::ProtoOaTrendbar) -> Self {
    Trendbar {
      volume: value.volume,
      period: match value.period {
        Some(period) => Some(TrendbarPeriod::try_from_primitive(period).unwrap()),
        None => None,
      },
      low: value.low,
      delta_open: value.delta_open,
      delta_close: value.delta_close,
      delta_high: value.delta_high,
      utc_timestamp_in_minutes: value.utc_timestamp_in_minutes,
    }
  }
}

impl Trendbar {
  /// Returns the absolute opening price as a raw i64 integer.
  pub fn open_price(&self) -> i64 {
    let low = self.low.unwrap_or(0);
    let delta = self.delta_open.unwrap_or(0) as i64;
    low + delta
  }

  /// Returns the absolute highest price as a raw i64 integer.
  pub fn high_price(&self) -> i64 {
    let low = self.low.unwrap_or(0);
    let delta = self.delta_high.unwrap_or(0) as i64;
    low + delta
  }

  /// Returns the absolute lowest price as a raw i64 integer.
  pub fn low_price(&self) -> i64 {
    self.low.unwrap_or(0)
  }

  /// Returns the absolute closing price as a raw i64 integer.
  pub fn close_price(&self) -> i64 {
    let low = self.low.unwrap_or(0);
    let delta = self.delta_close.unwrap_or(0) as i64;
    low + delta
  }

  pub fn timestamp_locale(&self) -> Option<String> {
    self.utc_timestamp_in_minutes.map(|mins| {
      let secs = mins as i64 * 60;
      chrono::DateTime::from_timestamp(secs, 0)
        .map(|dt| {
          dt.with_timezone(&chrono::Local)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
        })
        .unwrap_or_else(|| "Invalid timestamp".to_string())
    })
  }
}
