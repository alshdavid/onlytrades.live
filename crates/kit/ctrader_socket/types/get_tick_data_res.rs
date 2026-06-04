use kit_ctrader_proto::ProtoOaGetTickDataRes;

use super::TickData;

/// * Response to the ProtoOAGetTickDataReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct GetTickDataRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of ticks is in chronological order (newest first). The first tick contains Unix time in milliseconds while all subsequent ticks have the time difference in milliseconds between the previous and the current one.
  pub tick_data: Vec<TickData>,
  /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
  pub has_more: bool,
}

impl From<ProtoOaGetTickDataRes> for GetTickDataRes {
  fn from(value: ProtoOaGetTickDataRes) -> Self {
    GetTickDataRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      tick_data: value.tick_data.into_iter().map(TickData::from).collect(),
      has_more: value.has_more,
    }
  }
}
