use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct ClosePositionDetail {
  /// Position price at the moment of filling the closing order.
  pub entry_price: f64,
  /// Amount of realized gross profit after closing deal execution.
  pub gross_profit: i64,
  /// Amount of realized swap related to closed volume.
  pub swap: i64,
  /// Amount of realized commission related to closed volume.
  pub commission: i64,
  /// Account balance after closing deal execution.
  pub balance: i64,
  /// Quote/Deposit currency conversion rate on the time of closing deal execution.
  pub quote_to_deposit_conversion_rate: Option<f64>,
  /// Closed volume in cents.
  pub closed_volume: Option<i64>,
  /// Balance version of the account related to closing deal operation.
  pub balance_version: Option<i64>,
  /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects grossProfit, swap, commission, balance, pnlConversionFee.
  pub money_digits: Option<u32>,
  /// Fee for conversion applied to the Deal in account's ccy when trader symbol's quote asset id <> ProtoOATrader.depositAssetId.
  pub pnl_conversion_fee: Option<i64>,
}

impl From<super::super::messages::ProtoOaClosePositionDetail> for ClosePositionDetail {
  fn from(close_position_detail: super::super::messages::ProtoOaClosePositionDetail) -> Self {
    ClosePositionDetail {
      entry_price: close_position_detail.entry_price,
      gross_profit: close_position_detail.gross_profit,
      swap: close_position_detail.swap,
      commission: close_position_detail.commission,
      balance: close_position_detail.balance,
      quote_to_deposit_conversion_rate: close_position_detail.quote_to_deposit_conversion_rate,
      closed_volume: close_position_detail.closed_volume,
      balance_version: close_position_detail.balance_version,
      money_digits: close_position_detail.money_digits,
      pnl_conversion_fee: close_position_detail.pnl_conversion_fee,
    }
  }
}
