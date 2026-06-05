use kit_ctrader_proto::ProtoOaSubscribeLiveTrendbarReq;
use num_enum::TryFromPrimitive;

use super::TrendbarPeriod;

/// * Request for subscribing for live trend bars. Requires subscription on the spot events, see ProtoOASubscribeSpotsReq.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct SubscribeLiveTrendbarReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Specifies period of trend bar series (e.g. M1, M10, etc.).
  pub period: TrendbarPeriod,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: i64,
}

impl TryFrom<ProtoOaSubscribeLiveTrendbarReq> for SubscribeLiveTrendbarReq {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaSubscribeLiveTrendbarReq) -> Result<Self, Self::Error> {
    Ok(SubscribeLiveTrendbarReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      period: TrendbarPeriod::try_from_primitive(value.period)?,
      symbol_id: value.symbol_id,
    })
  }
}
