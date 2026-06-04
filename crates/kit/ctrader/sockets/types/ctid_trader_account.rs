use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CtidTraderAccount {
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.cTrader platform. Different brokers might have different ids
  pub ctid_trader_account_id: u64,
  /// If TRUE then the account is belong to Live environment and live host must be used to authorize it
  pub is_live: Option<bool>,
  /// TraderLogin for a specific account. Value is displayed on Client App UI
  pub trader_login: Option<i64>,
  /// The Unix time in milliseconds of the last ProtoOAClosePositionDetail happened to this account.
  pub last_closing_deal_timestamp: Option<i64>,
  /// The Unix time in milliseconds of the last ProtoOADepositWithdraw happened to this account.
  pub last_balance_update_timestamp: Option<i64>,
  /// The name of the broker to which the account belongs to. Shortened to be displayed in the UI.
  pub broker_title_short: Option<String>,
}

impl From<super::super::messages::ProtoOaCtidTraderAccount> for CtidTraderAccount {
  fn from(account: super::super::messages::ProtoOaCtidTraderAccount) -> Self {
    CtidTraderAccount {
      ctid_trader_account_id: account.ctid_trader_account_id,
      is_live: account.is_live,
      trader_login: account.trader_login,
      last_closing_deal_timestamp: account.last_closing_deal_timestamp,
      last_balance_update_timestamp: account.last_balance_update_timestamp,
      broker_title_short: account.broker_title_short,
    }
  }
}
