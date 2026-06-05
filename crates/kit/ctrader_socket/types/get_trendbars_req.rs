use kit_ctrader_proto::ProtoOaGetTrendbarsReq;
use num_enum::TryFromPrimitive;

use super::TrendbarPeriod;

/// * Request for getting historical trend bars for the symbol.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetTrendbarsReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The Unix time in milliseconds from which the search starts. Must be bigger or equal to zero (1st Jan 1970).
  pub from_timestamp: Option<i64>,
  /// The Unix time in milliseconds of finishing the search. Smaller or equal to 2147483646000 (19th Jan 2038).
  pub to_timestamp: Option<i64>,
  /// Specifies period of trend bar series (e.g. M1, M10, etc.).
  pub period: TrendbarPeriod,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: i64,
  /// Limit number of trend bars in response back from toTimestamp.
  pub count: Option<u32>,
}

impl TryFrom<ProtoOaGetTrendbarsReq> for GetTrendbarsReq {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaGetTrendbarsReq) -> Result<Self, Self::Error> {
    Ok(GetTrendbarsReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      from_timestamp: value.from_timestamp,
      to_timestamp: value.to_timestamp,
      period: TrendbarPeriod::try_from_primitive(value.period)?,
      symbol_id: value.symbol_id,
      count: value.count,
    })
  }
}
