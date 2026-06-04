use serde::Serialize;
use uhttp::AsyncWriteExt;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiCtraderAccountsGetResponse {
  ctrader_accounts: Option<Vec<ApiCtraderAccountsGetResponseCtraderAccount>>,
}

#[derive(Debug, Serialize)]
struct ApiCtraderAccountsGetResponseCtraderAccount {
  pub account_id: i64,
  pub account_number: u64,
  pub live: bool,
  pub broker_name: String,
  pub broker_title: String,
  pub deposit_currency: String,
  pub trader_account_type: String,
  pub leverage: u32,
  pub leverage_in_cents: u64,
  pub balance: i64,
  pub deleted: bool,
  pub account_status: String,
  pub swap_free: bool,
  pub money_digits: u32,
}

pub async fn api_ctrader_accounts_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    ctrader_token_repository,
    ctrader_rest_client,
    ctrader_account_repository,
    profile,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let mut response = ApiCtraderAccountsGetResponse {
    ctrader_accounts: None,
  };

  let Some(tokens) = ctrader_token_repository
    .get_tokens_for_profile(&profile.id)
    .await?
  else {
    res.header().add("Content-Type", "application/json").await?;
    res.write_all(&serde_json::to_vec(&response)?).await?;
    res.write_head(uhttp::StatusCode::OK).await?;
    return Ok(());
  };

  let local_ctrader_accounts = ctrader_account_repository
    .find_by_profile_id(&profile.id)
    .await?;

  let ctrader_accounts = ctrader_rest_client
    .get_accounts(&tokens.access_token)
    .await?;

  let mut response_accounts = Vec::<ApiCtraderAccountsGetResponseCtraderAccount>::new();

  for account in local_ctrader_accounts {
    let Some(ctrader_account) = ctrader_accounts
      .iter()
      .find(|a| a.account_id == account.account_id)
    else {
      continue;
    };

    response_accounts.push(ApiCtraderAccountsGetResponseCtraderAccount {
      account_id: account.account_id,
      account_number: account.account_number,
      live: account.live,
      broker_name: account.broker_name,
      broker_title: account.broker_title,
      deposit_currency: account.deposit_currency,
      trader_account_type: account.trader_account_type,
      leverage: account.leverage,
      leverage_in_cents: account.leverage_in_cents,
      deleted: account.deleted,
      account_status: account.account_status,
      swap_free: account.swap_free,
      money_digits: account.money_digits,
      balance: ctrader_account.balance,
    });
  }

  response.ctrader_accounts = Some(response_accounts);

  res.header().add("Content-Type", "application/json").await?;
  res.write_all(&serde_json::to_vec(&response)?).await?;
  res.write_head(uhttp::StatusCode::OK).await?;
  Ok(())
}
