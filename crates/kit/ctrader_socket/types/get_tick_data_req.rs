use kit_ctrader_proto::ProtoOaGetTickDataReq;
use num_enum::TryFromPrimitive;

use super::QuoteType;

/// * Request for getting historical tick data for the symbol.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetTickDataReq {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// Unique identifier of the Symbol in cTrader platform.
  pub symbol_id: i64,
  /// Bid/Ask (1/2).
  pub r#type: QuoteType,
  /// The Unix time in milliseconds of starting the search. Must be bigger or equal to zero (1st Jan 1970).
  pub from_timestamp: Option<i64>,
  /// The Unix time in milliseconds of finishing the search. <= 2147483646000 (19th Jan 2038).
  pub to_timestamp: Option<i64>,
}

impl TryFrom<ProtoOaGetTickDataReq> for GetTickDataReq {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaGetTickDataReq) -> Result<Self, Self::Error> {
    Ok(GetTickDataReq {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      symbol_id: value.symbol_id,
      r#type: QuoteType::try_from_primitive(value.r#type)?,
      from_timestamp: value.from_timestamp,
      to_timestamp: value.to_timestamp,
    })
  }
}
