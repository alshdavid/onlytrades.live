use kit_ctrader_proto::ProtoOaDealListRes;

use super::Deal;

/// * The response to the ProtoOADealListRes request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct DealListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of the deals.
  pub deal: Vec<Deal>,
  /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
  pub has_more: bool,
}

impl TryFrom<ProtoOaDealListRes> for DealListRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaDealListRes) -> Result<Self, Self::Error> {
    Ok(DealListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      deal: {
        let mut deals = Vec::<Deal>::new();
        for deal in value.deal {
          deals.push(Deal::try_from(deal)?);
        }
        deals
      },
      has_more: value.has_more,
    })
  }
}
