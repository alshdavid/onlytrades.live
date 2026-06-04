use serde::Deserialize;
use serde::Serialize;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BonusDepositWithdraw {
  /// Type of the operation. Deposit/Withdrawal.
  pub operation_type: ChangeBonusType,
  /// The unique ID of the bonus deposit/withdrawal operation.
  pub bonus_history_id: i64,
  /// Total amount of broker's bonus after the operation.
  pub manager_bonus: i64,
  /// Amount of bonus deposited/withdrew by manager.
  pub manager_delta: i64,
  /// Total amount of introducing broker's bonus after the operation.
  pub ib_bonus: i64,
  /// Amount of bonus deposited/withdrew by introducing broker.
  pub ib_delta: i64,
  /// The Unix time in milliseconds when the bonus operation was executed.
  pub change_bonus_timestamp: i64,
  /// Note added to operation. Visible to the trader.
  pub external_note: Option<String>,
  /// ID of introducing broker who deposited/withdrew bonus.
  pub introducing_broker_id: Option<i64>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects managerBonus, managerDelta, ibBonus, ibDelta.
  pub money_digits: Option<u32>,
}

impl TryFrom<super::super::messages::ProtoOaBonusDepositWithdraw> for BonusDepositWithdraw {
  type Error = anyhow::Error;

  fn try_from(
    bonus_deposit_withdraw: super::super::messages::ProtoOaBonusDepositWithdraw
  ) -> Result<Self, Self::Error> {
    Ok(BonusDepositWithdraw {
      operation_type: ChangeBonusType::try_from(bonus_deposit_withdraw.operation_type)?,
      bonus_history_id: bonus_deposit_withdraw.bonus_history_id,
      manager_bonus: bonus_deposit_withdraw.manager_bonus,
      manager_delta: bonus_deposit_withdraw.manager_delta,
      ib_bonus: bonus_deposit_withdraw.ib_bonus,
      ib_delta: bonus_deposit_withdraw.ib_delta,
      change_bonus_timestamp: bonus_deposit_withdraw.change_bonus_timestamp,
      external_note: bonus_deposit_withdraw.external_note,
      introducing_broker_id: bonus_deposit_withdraw.introducing_broker_id,
      money_digits: bonus_deposit_withdraw.money_digits,
    })
  }
}
