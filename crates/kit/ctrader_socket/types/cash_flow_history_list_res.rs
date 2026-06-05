use kit_ctrader_proto::ProtoOaCashFlowHistoryListRes;

use super::DepositWithdraw;

/// * Response to the ProtoOACashFlowHistoryListReq request.
#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct CashFlowHistoryListRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The list of deposit and withdrawal operations.
  pub deposit_withdraw: Vec<DepositWithdraw>,
}

impl TryFrom<ProtoOaCashFlowHistoryListRes> for CashFlowHistoryListRes {
  type Error = anyhow::Error;

  fn try_from(value: ProtoOaCashFlowHistoryListRes) -> Result<Self, Self::Error> {
    Ok(CashFlowHistoryListRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      deposit_withdraw: value
        .deposit_withdraw
        .into_iter()
        .map(DepositWithdraw::try_from)
        .collect::<Result<Vec<_>, _>>()?,
    })
  }
}
