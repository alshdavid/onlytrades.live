use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DepositWithdraw {
  /// Type of the operation. Deposit/Withdrawal.
  pub operation_type: ChangeBalanceType,
  /// The unique ID of the deposit/withdrawal operation.
  pub balance_history_id: i64,
  /// Account balance after the operation was executed.
  pub balance: i64,
  /// Amount of deposit/withdrawal operation.
  pub delta: i64,
  /// The Unix time in milliseconds when deposit/withdrawal operation was executed.
  pub change_balance_timestamp: i64,
  /// Note added to operation. Visible to the trader.
  pub external_note: Option<String>,
  /// Balance version used to identify the final balance. Increments each time when the trader's account balance is changed.
  pub balance_version: Option<i64>,
  /// Total account's equity after balance operation was executed.
  pub equity: Option<i64>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects balance, delta, equity.
  pub money_digits: Option<u32>,
}

impl TryFrom<kit_ctrader_proto::ProtoOaDepositWithdraw> for DepositWithdraw {
  type Error = anyhow::Error;

  fn try_from(
    deposit_withdraw: kit_ctrader_proto::ProtoOaDepositWithdraw
  ) -> Result<Self, Self::Error> {
    Ok(DepositWithdraw {
      operation_type: ChangeBalanceType::try_from(deposit_withdraw.operation_type)?,
      balance_history_id: deposit_withdraw.balance_history_id,
      balance: deposit_withdraw.balance,
      delta: deposit_withdraw.delta,
      change_balance_timestamp: deposit_withdraw.change_balance_timestamp,
      external_note: deposit_withdraw.external_note,
      balance_version: deposit_withdraw.balance_version,
      equity: deposit_withdraw.equity,
      money_digits: deposit_withdraw.money_digits,
    })
  }
}
