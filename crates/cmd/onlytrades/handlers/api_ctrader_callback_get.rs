use chrono::TimeDelta;
use chrono::Utc;
use platform_models::CtraderAccountModel;
use platform_models::CtraderTokenModel;
use serde::Deserialize;
use uhttp::AsyncWriteExt;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Deserialize)]
struct Query {
  code: Option<String>,
}

pub async fn api_ctrader_callback_get(
  req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    ctrader_rest_client,
    identity_repository,
    profile_repository,
    ctrader_account_repository,
    ctrader_token_repository,
    access_token,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(identity) = identity_repository.find_by_sub(&access_token.sub).await? else {
    res.write_all(b"missing identity").await?;
    res.write_head(uhttp::StatusCode::FORBIDDEN).await?;
    return Ok(());
  };

  let Some(profile) = profile_repository.find_by_id(&identity.profile_id).await? else {
    res.write_all(b"missing profile").await?;
    res.write_head(uhttp::StatusCode::FORBIDDEN).await?;
    return Ok(());
  };

  let Some(query_str) = req.uri().query() else {
    res.write_all(b"missing querystring").await?;
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let query = serde_urlencoded::from_str::<Query>(query_str).unwrap();

  let Some(code) = query.code else {
    res.write_all(b"missing querystring 'code'").await?;
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let ctrader_tokens = ctrader_rest_client.oauth_get_token(&code).await?;

  let accounts = ctrader_rest_client
    .get_accounts(&ctrader_tokens.access_token)
    .await?;

  ctrader_token_repository
    .upsert(
      &CtraderTokenModel {
        token_id: Uuid::now_v7(),
        refresh_token: ctrader_tokens.refresh_token,
        access_token: ctrader_tokens.access_token,
        access_token_expires_at: Utc::now() + TimeDelta::seconds(ctrader_tokens.expires_in as i64),
      },
      &profile.id,
    )
    .await?;

  for account in accounts {
    ctrader_account_repository
      .upsert(
        &CtraderAccountModel {
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
        },
        &profile.id,
      )
      .await?;
  }

  res.header().set("Location", "/dashboard").await?;
  res
    .write_head(uhttp::StatusCode::TEMPORARY_REDIRECT)
    .await?;

  Ok(())
}
