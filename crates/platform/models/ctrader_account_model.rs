use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtraderAccountModel {
  pub account_id: i64,
  pub account_number: u64,
  pub live: bool,
  pub broker_name: String,
  pub broker_title: String,
  pub deposit_currency: String,
  pub trader_account_type: String,
  pub leverage: u32,
  pub leverage_in_cents: u64,
  pub deleted: bool,
  pub account_status: String,
  pub swap_free: bool,
  pub money_digits: u32,
}
