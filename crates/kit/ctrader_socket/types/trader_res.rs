use kit_ctrader_proto::ProtoOaTraderRes;

use super::Trader;

/// * Response to the ProtoOATraderReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct TraderRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The Trader account information.
  pub trader: Trader,
}

impl From<ProtoOaTraderRes> for TraderRes {
  fn from(value: ProtoOaTraderRes) -> Self {
    TraderRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      trader: Trader::from(value.trader),
    }
  }
}
