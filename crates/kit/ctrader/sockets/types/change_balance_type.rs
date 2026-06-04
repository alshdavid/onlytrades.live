/// * Balance operation entity. Covers all cash movement operations related to account, trading, IB operations, mirroring, etc.
use num_enum::IntoPrimitive;
/// * Balance operation entity. Covers all cash movement operations related to account, trading, IB operations, mirroring, etc.
use num_enum::TryFromPrimitive;
use serde::Deserialize;
use serde::Serialize;

#[derive(
  Clone,
  Copy,
  Debug,
  PartialEq,
  Eq,
  Hash,
  PartialOrd,
  Ord,
  TryFromPrimitive,
  IntoPrimitive,
  Serialize,
  Deserialize,
)]
#[repr(i32)]
pub enum ChangeBalanceType {
  /// Cash deposit.
  BalanceDeposit = 0,
  /// Cash withdrawal.
  BalanceWithdraw = 1,
  /// Received mirroring commission.
  BalanceDepositStrategyCommissionInner = 3,
  /// Paid mirroring commission.
  BalanceWithdrawStrategyCommissionInner = 4,
  /// For IB account. Commissions paid by trader.
  BalanceDepositIbCommissions = 5,
  /// For IB account. Withdrawal of commissions shared with broker.
  BalanceWithdrawIbSharedPercentage = 6,
  /// For IB account. Commissions paid by sub-ibs.
  BalanceDepositIbSharedPercentageFromSubIb = 7,
  /// For IB account. Commissions paid by broker.
  BalanceDepositIbSharedPercentageFromBroker = 8,
  /// Deposit rebate for trading volume for period.
  BalanceDepositRebate = 9,
  /// Withdrawal of rebate.
  BalanceWithdrawRebate = 10,
  /// Mirroring commission.
  BalanceDepositStrategyCommissionOuter = 11,
  /// Mirroring commission.
  BalanceWithdrawStrategyCommissionOuter = 12,
  /// For IB account. Share commission with the Broker.
  BalanceWithdrawBonusCompensation = 13,
  /// IB commissions.
  BalanceWithdrawIbSharedPercentageToBroker = 14,
  /// Deposit dividends payments.
  BalanceDepositDividends = 15,
  /// Negative dividend charge for short position.
  BalanceWithdrawDividends = 16,
  /// Charge for guaranteedStopLoss.
  BalanceWithdrawGslCharge = 17,
  /// Charge of rollover fee for Shariah compliant accounts.
  BalanceWithdrawRollover = 18,
  /// Broker's operation to deposit bonus.
  BalanceDepositNonwithdrawableBonus = 19,
  /// Broker's operation to withdrawal bonus.
  BalanceWithdrawNonwithdrawableBonus = 20,
  /// Deposits of negative SWAP.
  BalanceDepositSwap = 21,
  /// SWAP charges.
  BalanceWithdrawSwap = 22,
  /// Mirroring commission.
  BalanceDepositManagementFee = 27,
  /// Mirroring commission. Deprecated since 7.1 in favor of BALANCE_WITHDRAW_COPY_FEE (34).
  BalanceWithdrawManagementFee = 28,
  /// Mirroring commission.
  BalanceDepositPerformanceFee = 29,
  /// Withdraw for subaccount creation (cTrader Copy).
  BalanceWithdrawForSubaccount = 30,
  /// Deposit to subaccount on creation (cTrader Copy).
  BalanceDepositToSubaccount = 31,
  /// Manual user's withdraw from subaccount (cTrader Copy), to parent account.
  BalanceWithdrawFromSubaccount = 32,
  /// Manual user's deposit to subaccount (cTrader Copy), from parent account.
  BalanceDepositFromSubaccount = 33,
  /// Withdrawal fees to Strategy Provider.
  BalanceWithdrawCopyFee = 34,
  /// Withdraw of inactivity fee from the balance.
  BalanceWithdrawInactivityFee = 35,
  /// Deposit within the same server (from another account).
  BalanceDepositTransfer = 36,
  /// Withdraw within the same server (to another account).
  BalanceWithdrawTransfer = 37,
  /// Bonus being converted from virtual bonus to real deposit.
  BalanceDepositConvertedBonus = 38,
  /// Applies if negative balance protection is configured by broker, should make balance = 0.
  BalanceDepositNegativeBalanceProtection = 39,
}

impl ChangeBalanceType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
    match self {
      Self::BalanceDeposit => "BALANCE_DEPOSIT",
      Self::BalanceWithdraw => "BALANCE_WITHDRAW",
      Self::BalanceDepositStrategyCommissionInner => "BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER",
      Self::BalanceWithdrawStrategyCommissionInner => "BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER",
      Self::BalanceDepositIbCommissions => "BALANCE_DEPOSIT_IB_COMMISSIONS",
      Self::BalanceWithdrawIbSharedPercentage => "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE",
      Self::BalanceDepositIbSharedPercentageFromSubIb => {
        "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB"
      }
      Self::BalanceDepositIbSharedPercentageFromBroker => {
        "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER"
      }
      Self::BalanceDepositRebate => "BALANCE_DEPOSIT_REBATE",
      Self::BalanceWithdrawRebate => "BALANCE_WITHDRAW_REBATE",
      Self::BalanceDepositStrategyCommissionOuter => "BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER",
      Self::BalanceWithdrawStrategyCommissionOuter => "BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER",
      Self::BalanceWithdrawBonusCompensation => "BALANCE_WITHDRAW_BONUS_COMPENSATION",
      Self::BalanceWithdrawIbSharedPercentageToBroker => {
        "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER"
      }
      Self::BalanceDepositDividends => "BALANCE_DEPOSIT_DIVIDENDS",
      Self::BalanceWithdrawDividends => "BALANCE_WITHDRAW_DIVIDENDS",
      Self::BalanceWithdrawGslCharge => "BALANCE_WITHDRAW_GSL_CHARGE",
      Self::BalanceWithdrawRollover => "BALANCE_WITHDRAW_ROLLOVER",
      Self::BalanceDepositNonwithdrawableBonus => "BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS",
      Self::BalanceWithdrawNonwithdrawableBonus => "BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS",
      Self::BalanceDepositSwap => "BALANCE_DEPOSIT_SWAP",
      Self::BalanceWithdrawSwap => "BALANCE_WITHDRAW_SWAP",
      Self::BalanceDepositManagementFee => "BALANCE_DEPOSIT_MANAGEMENT_FEE",
      Self::BalanceWithdrawManagementFee => "BALANCE_WITHDRAW_MANAGEMENT_FEE",
      Self::BalanceDepositPerformanceFee => "BALANCE_DEPOSIT_PERFORMANCE_FEE",
      Self::BalanceWithdrawForSubaccount => "BALANCE_WITHDRAW_FOR_SUBACCOUNT",
      Self::BalanceDepositToSubaccount => "BALANCE_DEPOSIT_TO_SUBACCOUNT",
      Self::BalanceWithdrawFromSubaccount => "BALANCE_WITHDRAW_FROM_SUBACCOUNT",
      Self::BalanceDepositFromSubaccount => "BALANCE_DEPOSIT_FROM_SUBACCOUNT",
      Self::BalanceWithdrawCopyFee => "BALANCE_WITHDRAW_COPY_FEE",
      Self::BalanceWithdrawInactivityFee => "BALANCE_WITHDRAW_INACTIVITY_FEE",
      Self::BalanceDepositTransfer => "BALANCE_DEPOSIT_TRANSFER",
      Self::BalanceWithdrawTransfer => "BALANCE_WITHDRAW_TRANSFER",
      Self::BalanceDepositConvertedBonus => "BALANCE_DEPOSIT_CONVERTED_BONUS",
      Self::BalanceDepositNegativeBalanceProtection => {
        "BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION"
      }
    }
  }

  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
    match value {
      "BALANCE_DEPOSIT" => Some(Self::BalanceDeposit),
      "BALANCE_WITHDRAW" => Some(Self::BalanceWithdraw),
      "BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER" => {
        Some(Self::BalanceDepositStrategyCommissionInner)
      }
      "BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER" => {
        Some(Self::BalanceWithdrawStrategyCommissionInner)
      }
      "BALANCE_DEPOSIT_IB_COMMISSIONS" => Some(Self::BalanceDepositIbCommissions),
      "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE" => Some(Self::BalanceWithdrawIbSharedPercentage),
      "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB" => {
        Some(Self::BalanceDepositIbSharedPercentageFromSubIb)
      }
      "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER" => {
        Some(Self::BalanceDepositIbSharedPercentageFromBroker)
      }
      "BALANCE_DEPOSIT_REBATE" => Some(Self::BalanceDepositRebate),
      "BALANCE_WITHDRAW_REBATE" => Some(Self::BalanceWithdrawRebate),
      "BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER" => {
        Some(Self::BalanceDepositStrategyCommissionOuter)
      }
      "BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER" => {
        Some(Self::BalanceWithdrawStrategyCommissionOuter)
      }
      "BALANCE_WITHDRAW_BONUS_COMPENSATION" => Some(Self::BalanceWithdrawBonusCompensation),
      "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER" => {
        Some(Self::BalanceWithdrawIbSharedPercentageToBroker)
      }
      "BALANCE_DEPOSIT_DIVIDENDS" => Some(Self::BalanceDepositDividends),
      "BALANCE_WITHDRAW_DIVIDENDS" => Some(Self::BalanceWithdrawDividends),
      "BALANCE_WITHDRAW_GSL_CHARGE" => Some(Self::BalanceWithdrawGslCharge),
      "BALANCE_WITHDRAW_ROLLOVER" => Some(Self::BalanceWithdrawRollover),
      "BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS" => Some(Self::BalanceDepositNonwithdrawableBonus),
      "BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS" => Some(Self::BalanceWithdrawNonwithdrawableBonus),
      "BALANCE_DEPOSIT_SWAP" => Some(Self::BalanceDepositSwap),
      "BALANCE_WITHDRAW_SWAP" => Some(Self::BalanceWithdrawSwap),
      "BALANCE_DEPOSIT_MANAGEMENT_FEE" => Some(Self::BalanceDepositManagementFee),
      "BALANCE_WITHDRAW_MANAGEMENT_FEE" => Some(Self::BalanceWithdrawManagementFee),
      "BALANCE_DEPOSIT_PERFORMANCE_FEE" => Some(Self::BalanceDepositPerformanceFee),
      "BALANCE_WITHDRAW_FOR_SUBACCOUNT" => Some(Self::BalanceWithdrawForSubaccount),
      "BALANCE_DEPOSIT_TO_SUBACCOUNT" => Some(Self::BalanceDepositToSubaccount),
      "BALANCE_WITHDRAW_FROM_SUBACCOUNT" => Some(Self::BalanceWithdrawFromSubaccount),
      "BALANCE_DEPOSIT_FROM_SUBACCOUNT" => Some(Self::BalanceDepositFromSubaccount),
      "BALANCE_WITHDRAW_COPY_FEE" => Some(Self::BalanceWithdrawCopyFee),
      "BALANCE_WITHDRAW_INACTIVITY_FEE" => Some(Self::BalanceWithdrawInactivityFee),
      "BALANCE_DEPOSIT_TRANSFER" => Some(Self::BalanceDepositTransfer),
      "BALANCE_WITHDRAW_TRANSFER" => Some(Self::BalanceWithdrawTransfer),
      "BALANCE_DEPOSIT_CONVERTED_BONUS" => Some(Self::BalanceDepositConvertedBonus),
      "BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION" => {
        Some(Self::BalanceDepositNegativeBalanceProtection)
      }
      _ => None,
    }
  }
}
