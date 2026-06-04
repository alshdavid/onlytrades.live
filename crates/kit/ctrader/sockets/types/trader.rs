use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trader {
  /// The unique Trader's Account ID used to match the responses to the Trader's Account.
  pub ctid_trader_account_id: i64,
  /// Current account balance.
  pub balance: i64,
  /// Balance version used to identify the final balance. Increments each time when the trader's account balance is changed.
  pub balance_version: Option<i64>,
  /// Amount of broker's bonus allocated to the account.
  pub manager_bonus: Option<i64>,
  /// Amount of introducing broker bonus allocated to the account.
  pub ib_bonus: Option<i64>,
  /// Broker's bonus that cannot be withdrew from the account as cash.
  pub non_withdrawable_bonus: Option<i64>,
  /// Access rights that an owner has to the account in cTrader platform. See ProtoOAAccessRights for details.
  pub access_rights: Option<i32>,
  /// Deposit currency of the account.
  pub deposit_asset_id: i64,
  /// If TRUE then account is Shariah compliant.
  pub swap_free: Option<bool>,
  /// Account leverage (e.g. If leverage = 1:50 then value = 5000).
  pub leverage_in_cents: Option<u32>,
  /// Margin computation type for the account (MAX, SUM, NET).
  pub total_margin_calculation_type: Option<i32>,
  /// Maximum allowed leverage for the account. Used as validation when a Trader can change leverage value.
  pub max_leverage: Option<u32>,
  /// ID of the account that is unique per server (Broker).
  pub trader_login: Option<i64>,
  /// Account type: HEDGED, NETTED, etc.
  pub account_type: Option<i32>,
  /// Some whitelabel assigned to trader by broker at the moment of account creation.
  pub broker_name: Option<String>,
  /// The Unix timestamp in milliseconds of the account registration. Should be used as minimal date in historical data requests.
  pub registration_timestamp: Option<i64>,
  /// If TRUE then account is compliant to use specific margin calculation strategy. Such accounts are require to have guaranteed stop loss on all positions.
  pub is_limited_risk: Option<bool>,
  /// Special strategy used in margin calculations for this account (if account isLimitedRisk).
  pub limited_risk_margin_calculation_strategy: Option<i32>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects balance, managerBonus, ibBonus, nonWithdrawableBonus.
  pub money_digits: Option<u32>,
  /// If TRUE - Position is fully closed on Stop Out, if FALSE - smart (partial closing) Stop Out is applied, if unspecified  - Stop Out format is determined by Broker.
  pub fair_stop_out: Option<bool>,
  /// The Stop Out strategy that is used for this Trader. The Trader can change the value in the cTrader UI if this option is not disabled by the Broker
  pub stop_out_strategy: Option<i32>,
}

impl From<super::super::messages::ProtoOaTrader> for Trader {
  fn from(trader: super::super::messages::ProtoOaTrader) -> Self {
    Trader {
      ctid_trader_account_id: trader.ctid_trader_account_id,
      balance: trader.balance,
      balance_version: trader.balance_version,
      manager_bonus: trader.manager_bonus,
      ib_bonus: trader.ib_bonus,
      non_withdrawable_bonus: trader.non_withdrawable_bonus,
      access_rights: trader.access_rights,
      deposit_asset_id: trader.deposit_asset_id,
      swap_free: trader.swap_free,
      leverage_in_cents: trader.leverage_in_cents,
      total_margin_calculation_type: trader.total_margin_calculation_type,
      max_leverage: trader.max_leverage,
      trader_login: trader.trader_login,
      account_type: trader.account_type,
      broker_name: trader.broker_name,
      registration_timestamp: trader.registration_timestamp,
      is_limited_risk: trader.is_limited_risk,
      limited_risk_margin_calculation_strategy: trader.limited_risk_margin_calculation_strategy,
      money_digits: trader.money_digits,
      fair_stop_out: trader.fair_stop_out,
      stop_out_strategy: trader.stop_out_strategy,
    }
  }
}
