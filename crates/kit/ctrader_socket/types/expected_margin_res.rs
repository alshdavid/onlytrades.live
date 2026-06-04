use kit_ctrader_proto::ProtoOaExpectedMarginRes;

use super::ExpectedMargin;

/// * The response to the ProtoOAExpectedMarginReq request.
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub struct ExpectedMarginRes {
  pub client_msg_id: Option<String>,
  /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
  pub ctid_trader_account_id: i64,
  /// The buy and sell margin estimate.
  pub margin: Vec<ExpectedMargin>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects margin.buyMargin, margin.sellMargin.
  pub money_digits: Option<u32>,
}

impl From<ProtoOaExpectedMarginRes> for ExpectedMarginRes {
  fn from(value: ProtoOaExpectedMarginRes) -> Self {
    ExpectedMarginRes {
      client_msg_id: None,
      ctid_trader_account_id: value.ctid_trader_account_id,
      margin: value.margin.into_iter().map(ExpectedMargin::from).collect(),
      money_digits: value.money_digits,
    }
  }
}
