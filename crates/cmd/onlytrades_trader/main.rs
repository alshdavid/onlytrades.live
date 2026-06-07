mod bot;
mod bots;
mod ctx;
mod env;
mod settings;
mod utils;

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use env::Env;
use kit_ctrader_rest::client::CTraderRestClient;
use kit_ctrader_rest::client::CTraderRestClientOptions;
use kit_ctrader_rest::client::TradingAccount;
use kit_ctrader_socket::AccountAuthReq;
use kit_ctrader_socket::ApplicationAuthReq;
use kit_ctrader_socket::CTraderConnectionUtils;
use kit_ctrader_socket::CTraderRequestType;
use kit_ctrader_socket::CTraderResponseType;
use kit_ctrader_socket::LightSymbol;
use kit_ctrader_socket::Trendbar;
use kit_ctrader_socket::TrendbarPeriod;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::connection::CTraderConnectionExt;
use kit_ctrader_socket::connection::CTraderConnectionOptions;
use kit_std_ext::PathExt;

use crate::settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let env = Arc::new(Env::from_env()?);
  let settings =
    Settings::from_file(&std::env::current_exe()?.try_parent()?.join("settings.json"))?;

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

  // dbg!(&accounts);

  let account = accounts
    .into_iter()
    .find(|a| a.account_id == 46436103)
    .context("No account")?;

  let conn = connect_to_ctrader_socket(&env, &settings.get_access_token(), &account).await?;

  tokio::spawn({
    let conn = conn.clone();

    async move {
      loop {
        let Ok(latency) = conn.latency().await else {
          continue;
        };
        println!("LATENCY: {}", latency);
        tokio::time::sleep(Duration::from_secs(60)).await;
      }
    }
  });

  executor(bots::ema20_9_cross::strategy, conn, account.account_id).await?;

  Ok(())
}

#[derive(Clone, Debug)]
pub struct Ctx {
  series: VecDeque<Trendbar>,
  live: bool,
  conn: CTraderConnection,
  account_id: i64,
  symbol: LightSymbol,
}

pub async fn executor<F, Fut>(
  strategy_func: F,
  conn: CTraderConnection,
  account_id: i64,
) -> anyhow::Result<()>
where
  F: 'static + Fn(Ctx) -> Fut,
  Fut: Send + Sync + Future<Output = anyhow::Result<()>>,
{
  let mut symbols = conn.get_symbol_list(account_id).await?;
  let symbol = symbols
    .remove("BTCUSD")
    .context("BTCUSD symbol not found")?;

  let timeframe = TrendbarPeriod::M1;

  let mut rx = conn.subscribe().await;

  conn.subscribe_spots(account_id, symbol.symbol_id).await?;
  conn
    .subscribe_live_trendbars(account_id, symbol.symbol_id, timeframe)
    .await?;

  let historical = conn
    .get_historical_trendbars(&account_id, &symbol.symbol_id, timeframe, 500)
    .await?;

  let mut series = VecDeque::with_capacity(1000);
  for trendbar in historical {
    series.push_back(trendbar);

    let ctx = Ctx {
      series: series.clone(),
      live: false,
      conn: conn.clone(),
      account_id: account_id.clone(),
      symbol: symbol.clone(),
    };

    strategy_func(ctx).await?;
  }

  while let Some(msg) = rx.recv().await {
    match msg {
      Ok(CTraderResponseType::SpotEvent(event)) => {
        if event.symbol_id != symbol.symbol_id {
          continue;
        }

        dbg!(&event.trendbar);
        let Some(new_bar) = event.trendbar.first() else {
          continue;
        };

        // 1. Manage the series (Mutate forming or Push new)
        if let Some(last_bar) = series.back_mut() {
          if last_bar.utc_timestamp_in_minutes == new_bar.utc_timestamp_in_minutes {
            // It's the same minute: Mutate the forming candle
            *last_bar = new_bar.clone();
          } else {
            // It's a new minute: Push the new bar
            dbg!(&new_bar);
            series.push_back(new_bar.clone());

            let ctx = Ctx {
              series: series.clone(),
              live: true,
              conn: conn.clone(),
              account_id: account_id.clone(),
              symbol: symbol.clone(),
            };
            strategy_func(ctx).await?;
          }
        } else {
          // Fallback if series was empty for some reason
          series.push_back(new_bar.clone());
        }

        // 2. Keep the series at max capacity (e.g., 1000)
        while series.len() > 1000 {
          series.pop_front();
        }

        // 3. Update the strategy with the new state
        // let ctx = Ctx { series: series.clone() };
        // strategy_func(ctx).await?;
      }
      Ok(_) => {}
      Err(_) => {}
    }
  }

  Ok(())
}

async fn connect_to_ctrader_socket(
  env: &Env,
  ctrader_access_token: &str,
  account: &TradingAccount,
) -> anyhow::Result<CTraderConnection> {
  let conn = CTraderConnection::connect(CTraderConnectionOptions { live: account.live }).await?;

  let mut rx = conn.subscribe().await;

  conn
    .send(CTraderRequestType::ApplicationAuthReq(ApplicationAuthReq {
      client_msg_id: None,
      client_id: env.ctrader_client_id.clone(),
      client_secret: env.ctrader_client_secret.clone(),
    }))
    .await?;

  match rx.recv().await.context("connection ended unexpectedly")? {
    Ok(CTraderResponseType::ApplicationAuthRes(_)) => {}
    Ok(_) => return Err(anyhow::anyhow!("Invalid response")),
    Err(err) => return Err(anyhow::anyhow!(err)),
  };

  conn
    .send(CTraderRequestType::AccountAuthReq(AccountAuthReq {
      client_msg_id: None,
      ctid_trader_account_id: account.account_id.clone(),
      access_token: ctrader_access_token.to_string(),
    }))
    .await?;

  match rx.recv().await.context("connection ended unexpectedly")? {
    Ok(CTraderResponseType::AccountAuthRes(_)) => {}
    Ok(_) => return Err(anyhow::anyhow!("Invalid response")),
    Err(err) => return Err(anyhow::anyhow!(err)),
  };

  Ok(conn)
}
