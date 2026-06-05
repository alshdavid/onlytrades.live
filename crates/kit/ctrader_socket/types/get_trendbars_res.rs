use kit_ctrader_proto::ProtoOaGetTrendbarsRes;
use num_enum::TryFromPrimitive;

use super::Trendbar;
use super::TrendbarPeriod;

/// * Response to the ProtoOAGetTrendbarsReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetTrendbarsRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Specifies period of trend bar series (e.g. M1, M10, etc.).
  pub period: TrendbarPeriod,
  /// The list of trend bars.
  pub trendbar: Vec<Trendbar>,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: Option<i64>,
  /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
  pub has_more: Option<bool>,
}

impl TryFrom<ProtoOaGetTrendbarsRes> for GetTrendbarsRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaGetTrendbarsRes) -> Result<Self, Self::Error> {
    Ok(GetTrendbarsRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      period: TrendbarPeriod::try_from_primitive(value.period)?,
      trendbar: value.trendbar.into_iter().map(Trendbar::from).collect(),
      symbol_id: value.symbol_id,
      has_more: value.has_more,
    })
  }
}
