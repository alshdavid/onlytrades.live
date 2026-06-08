use std::collections::VecDeque;
use std::sync::Arc;

use anyhow::Context;
use kit_ctrader_rest::client::TradingAccount;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::connection::CTraderConnectionExt;
use kit_ctrader_socket::connection::CTraderConnectionOptions;
use kit_ctrader_socket::*;
use tokio::sync::Mutex;

use crate::Env;

pub async fn run_strategy<F, Fut>(
  name: &str,
  env: Arc<Env>,
  ctrader_access_token: String,
  strategy_func: F,
  accounts: Vec<TradingAccount>,
  account_id: i64,
  symbol_name: &str,
  timeframe: TrendbarPeriod,
  volume: i64,
) -> anyhow::Result<()>
where
  F: 'static + Fn(Ctx) -> Fut,
  Fut: Send + Sync + Future<Output = anyhow::Result<()>>,
{
  let account = accounts
    .into_iter()
    .find(|a| a.account_id == account_id)
    .context("No account")?;

  let conn = connect_to_ctrader_socket(&env, &ctrader_access_token, &account).await?;

  let mut symbols = conn.get_symbol_list(account_id).await?;
  let symbol = symbols.remove(symbol_name).context("symbol not found")?;

  let mut rx = conn.subscribe().await;

  conn.subscribe_spots(account_id, symbol.symbol_id).await?;

  conn
    .subscribe_live_trendbars(account_id, symbol.symbol_id, timeframe)
    .await?;

  let historical = conn
    .get_historical_trendbars(&account_id, &symbol.symbol_id, timeframe, 500)
    .await?;

  println!(
    "Starting Algo: {} / {} -> {}",
    account_id, account.account_number, symbol_name
  );

  let state = Arc::new(Mutex::new(StrategyState::default()));
  let mut series = VecDeque::with_capacity(1000);
  for trendbar in historical {
    series.push_back(trendbar);
    // println!("* Old bar     {:?}", trendbar.timestamp_locale());

    let ctx = Ctx {
      name: name.to_string(),
      series: series.clone(),
      live: false,
      conn: conn.clone(),
      account_id: account_id.clone(),
      symbol: symbol.clone(),
      state: Arc::clone(&state),
      volume,
    };

    strategy_func(ctx).await?;
  }

  let mut forming = None::<Trendbar>;

  while let Some(msg) = rx.recv().await {
    match msg {
      Ok(CTraderResponseType::SpotEvent(event)) => {
        if event.symbol_id != symbol.symbol_id {
          continue;
        }

        let Some(trendbar) = event
          .trendbar
          .into_iter()
          .find(|t| t.period.is_some_and(|p| p == TrendbarPeriod::M1))
        else {
          continue;
        };

        let Some(forming_trendbar) = forming else {
          // println!("* Forming bar {:?}", trendbar.timestamp_locale());
          forming.replace(trendbar);
          continue;
        };

        if trendbar.utc_timestamp_in_minutes != forming_trendbar.utc_timestamp_in_minutes {
          series.push_back(trendbar);
          let _ = forming.take();

          let ctx = Ctx {
            name: name.to_string(),
            series: series.clone(),
            live: true,
            conn: conn.clone(),
            account_id: account_id.clone(),
            symbol: symbol.clone(),
            state: Arc::clone(&state),
            volume,
          };

          strategy_func(ctx).await?;
        }

        while series.len() > 1000 {
          series.pop_front();
        }
      }
      Ok(_) => {}
      Err(_) => {}
    }
  }

  Ok(())
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum StrategyPosition {
  #[default]
  Flat,
  Long,
  Short,
}

#[derive(Default, Clone, Debug)]
pub struct StrategyState {
  pub long_stop_price: Option<f64>,
  pub short_stop_price: Option<f64>,
  pub position: StrategyPosition,
}

#[derive(Clone, Debug)]
pub struct Ctx {
  pub name: String,
  pub series: VecDeque<Trendbar>,
  pub live: bool,
  pub conn: CTraderConnection,
  pub account_id: i64,
  pub symbol: LightSymbol,
  pub volume: i64,
  pub state: Arc<Mutex<StrategyState>>,
}

impl Ctx {
  pub fn prices(&self) -> Vec<i64> {
    self.series.iter().map(|v| v.close_price()).collect()
  }

  pub async fn new_order(
    &self,
    req: NewOrderReq,
  ) -> anyhow::Result<()> {
    if !self.live {
      return Ok(());
    }

    self.conn.new_order(req).await?;

    Ok(())
  }

  pub async fn close_position(
    &self,
    req: ClosePositionReq,
  ) -> anyhow::Result<()> {
    if !self.live {
      return Ok(());
    }

    self.conn.close_position(req).await?;

    Ok(())
  }

  pub async fn close_all_positions(&self) -> anyhow::Result<()> {
    if !self.live {
      return Ok(());
    }

    for position in self.reconcile().await?.position {
      self
        .close_position(ClosePositionReq {
          client_msg_id: None,
          ctid_trader_account_id: self.account_id,
          position_id: position.position_id,
          volume: position.trade_data.volume,
        })
        .await?;
    }

    Ok(())
  }

  pub async fn reconcile(&self) -> anyhow::Result<ReconcileRes> {
    if !self.live {
      return Ok(ReconcileRes::default());
    }

    self.conn.reconcile(&self.account_id).await
  }
}

pub async fn connect_to_ctrader_socket(
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
