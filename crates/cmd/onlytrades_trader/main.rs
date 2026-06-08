mod bots;
mod ctx;
mod env;
mod measure_latency;
mod run_strategy;
mod settings;
mod utils;

use std::sync::Arc;

use env::Env;
use kit_ctrader_rest::client::CTraderRestClient;
use kit_ctrader_rest::client::CTraderRestClientOptions;
use kit_ctrader_socket::*;
use kit_std_ext::PathExt;

use crate::measure_latency::measure_latency;
use crate::run_strategy::run_strategy;
use crate::settings::Settings;

static GENERAL: i64 = 46436103; // 5227748 
static BOT_01: i64 = 46435450; // 5227678 
static BOT_02: i64 = 45386651; // 5154571 
static BOT_03: i64 = 46426095; // 5226942 
static BOT_04: i64 = 47550871; // 5297225 
static BOT_05: i64 = 47550876; // 5297226 
static BOT_06: i64 = 47550881; // 5297227 
static BOT_07: i64 = 47550886; // 5297229 
static BOT_08: i64 = 47550901; // 5297230 
static BOT_09: i64 = 47550907; // 5297231 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let env = Arc::new(Env::from_env()?);
  let settings_json = std::env::current_exe()?.try_parent()?.join("settings.json");
  let settings = Settings::from_file(&settings_json)?;

  let ctrader_rest_client = Arc::new(CTraderRestClient::new(CTraderRestClientOptions {
    hostname: env.app_origin.clone(),
    client_id: env.ctrader_client_id.clone(),
    client_secret: env.ctrader_client_secret.clone(),
  }));

  if settings.get_access_token() == ""
    || ctrader_rest_client.has_expired_raw(settings.get_expires_at())
  {
    let ctrader_tokens = ctrader_rest_client
      .oauth_refresh(&settings.get_refresh_token())
      .await?;

    settings.set_access_token(ctrader_tokens.access_token).await;
    settings
      .set_refresh_token(ctrader_tokens.refresh_token)
      .await;
    settings.set_expires_at(ctrader_tokens.expires_at).await;
  }

  let accounts = ctrader_rest_client
    .get_accounts(&settings.get_access_token())
    .await?;

  tokio::task::spawn(measure_latency(
    env.clone(),
    settings.get_access_token(),
    accounts.clone(),
    GENERAL,
  ));

  let mut h = vec![];

  // ALGOS
  // h.push(tokio::task::spawn(run_strategy(
  //   env.clone(),
  //   settings.get_access_token(),
  //   bots::macd_us500::strategy,
  //   accounts.clone(),
  //   DEMO_0,
  //   "US500",
  //   TrendbarPeriod::M1,
  // )));

  h.push(tokio::task::spawn(run_strategy(
    env.clone(),
    settings.get_access_token(),
    bots::ema20_9_cross::strategy,
    accounts.clone(),
    BOT_01,
    "US500",
    TrendbarPeriod::M1,
  )));

  h.push(tokio::task::spawn(run_strategy(
    env.clone(),
    settings.get_access_token(),
    bots::mean_reversion::strategy,
    accounts.clone(),
    BOT_02,
    "US500",
    TrendbarPeriod::M1,
  )));

  h.push(tokio::task::spawn(run_strategy(
    env.clone(),
    settings.get_access_token(),
    bots::mean_reversion::strategy,
    accounts.clone(),
    BOT_03,
    "US500",
    TrendbarPeriod::H1,
  )));

  h.push(tokio::task::spawn(run_strategy(
    env.clone(),
    settings.get_access_token(),
    bots::mean_reversion::strategy,
    accounts.clone(),
    BOT_04,
    "XAUUSD",
    TrendbarPeriod::M1,
  )));

  h.push(tokio::task::spawn(run_strategy(
    env.clone(),
    settings.get_access_token(),
    bots::mean_reversion::strategy,
    accounts.clone(),
    BOT_05,
    "XAUUSD",
    TrendbarPeriod::H1,
  )));

  // Algos should never finish
  for h in h {
    h.await??;
  }

  Ok(())
}
