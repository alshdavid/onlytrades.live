use serde::Deserialize;
use serde::Serialize;

use super::Trendbar;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotEvent {
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: i64,
  /// Bid price. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782)
  pub bid: Option<u64>,
  /// Ask price. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782)
  pub ask: Option<u64>,
  /// Returns live trend bar. Requires subscription on the trend bars.
  pub trendbar: Vec<Trendbar>,
  /// Last session close. Specified in 1/100000 of unit of a price. (e.g. 123000 in protocol means 1.23, 53423782 means 534.23782)
  pub session_close: Option<u64>,
  /// The Unix time for spot.
  pub timestamp: Option<i64>,
}

impl TryFrom<kit_ctrader_proto::ProtoOaSpotEvent> for SpotEvent {
  type Error = anyhow::Error;

  fn try_from(value: kit_ctrader_proto::ProtoOaSpotEvent) -> Result<Self, Self::Error> {
    Ok(SpotEvent {
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
      bid: value.bid,
      ask: value.ask,
      trendbar: value.trendbar.into_iter().map(Trendbar::from).collect(),
      session_close: value.session_close,
      timestamp: value.timestamp,
    })
  }
}
