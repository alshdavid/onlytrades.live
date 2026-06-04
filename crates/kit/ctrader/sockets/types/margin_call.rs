use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarginCall {
  /// Type of margin call. All margin calls are similar, only difference is in marginLevelThreshold.
  pub margin_call_type: i32,
  /// Margin level threshold for margin call.
  pub margin_level_threshold: f64,
  /// The Unix time in milliseconds of the last update of the margin call.
  pub utc_last_update_timestamp: Option<i64>,
}

impl From<super::super::messages::ProtoOaMarginCall> for MarginCall {
  fn from(margin_call: super::super::messages::ProtoOaMarginCall) -> Self {
    MarginCall {
      margin_call_type: margin_call.margin_call_type,
      margin_level_threshold: margin_call.margin_level_threshold,
      utc_last_update_timestamp: margin_call.utc_last_update_timestamp,
    }
  }
}
