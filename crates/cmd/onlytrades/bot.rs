use std::collections::HashMap;

use anyhow::Context;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::*;
use tokio::sync::mpsc::unbounded_channel;
use uuid::Uuid;

pub async fn handler(
  conn: CTraderConnection,
  account_id: i64,
) -> anyhow::Result<()> {
  let symbols: HashMap<String, LightSymbol> = get_symbol_list(&conn, account_id).await?;
  let target: &LightSymbol = symbols.get("BTCUSD").context("BTCUSD symbol not found")?;
  let symbol_id: i64 = target.symbol_id;
  let symbol_name: &str = target.symbol_name.as_deref().unwrap_or("BTCUSD");
  let timeframe = TrendbarPeriod::M1;

  // Subscribe to spot events so we receive price updates
  subscribe_spots(&conn, account_id, symbol_id).await?;

  // Subscribe to live M1 trendbars (attached to SpotEvent messages)
  subscribe_live_trendbars(&conn, account_id, symbol_id, timeframe).await?;

  // Fetch recent historical M1 bars to seed the EMA calculation
  let historical = get_historical_trendbars(&conn, account_id, symbol_id, timeframe, 30).await?;

  // Compute initial EMA from historical bars
  let k = 2.0 / (20.0 + 1.0); // EMA20 smoothing factor: 2 / (N + 1)
  let mut ema: f64 = 0.0;
  for (i, bar) in historical.iter().enumerate() {
    let close = close_price(bar);
    if i == 0 {
      ema = close; // seed with the first close
    } else {
      ema = close * k + ema * (1.0 - k);
    }
  }
  println!(
    "[{symbol_name}] Seeded EMA20 from {} historical M1 bars: {ema:.5}",
    historical.len(),
  );

  // Listen for live spot events — each carries the current forming M1 trendbar
  let mut rx_ctrader = conn.subscribe().await;
  let mut prev_minutes: Option<u32> = None;
  let mut prev_close: Option<f64> = None;

  while let Some(msg) = rx_ctrader.recv().await {
    match msg {
      Ok(CTraderResponseType::SpotEvent(event)) => {
        if event.symbol_id != symbol_id {
          continue;
        }

        // SpotEvent.trendbar contains the current live forming bar
        if let Some(tb) = event.trendbar.first() {
          if let Some(current_minutes) = tb.utc_timestamp_in_minutes {
            // Detect candle close: utc_timestamp_in_minutes changed since last event
            if let Some(prev_m) = prev_minutes {
              if current_minutes != prev_m {
                // The previous candle completed — use its last known close for EMA
                if let Some(close) = prev_close {
                  ema = close * k + ema * (1.0 - k);
                  println!(
                    "[{symbol_name}] M1 candle closed at minute {prev_m} — Close: {close:.5}, EMA20: {ema:.5}",
                  );
                }
              }
            }

            prev_minutes = Some(current_minutes);
            prev_close = Some(close_price(tb));
          }
        }
      }
      Ok(_) => {}
      Err(_) => panic!("Connection error"),
    }
  }

  Ok(())
}

/// Extract the close price from a completed trendbar.
/// Prices are in 1/100 000 units; this returns a human-readable decimal.
fn close_price(bar: &Trendbar) -> f64 {
  let low = bar.low.unwrap_or(0) as f64;
  let delta = bar.delta_close.unwrap_or(0) as f64;
  (low + delta) / 100_000.0
}

// ---------------------------------------------------------------------------
// Helper: request / response matching pattern
// ---------------------------------------------------------------------------

/// Subscribe to spot events for a symbol so we receive price updates.
async fn subscribe_spots(
  conn: &CTraderConnection,
  account_id: i64,
  symbol_id: i64,
) -> anyhow::Result<()> {
  let mut rx = conn.subscribe().await;
  let id = Uuid::now_v7().to_string();
  let (tx, mut rx_done) = unbounded_channel::<anyhow::Result<SubscribeSpotsRes>>();

  tokio::task::spawn({
    let id = id.clone();
    async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Ok(CTraderResponseType::SubscribeSpotsRes(res)) => {
            if res.client_msg_id.as_deref() == Some(&id) {
              let _ = tx.send(Ok(res));
              break;
            }
          }
          Ok(_) => {}
          Err(_) => break,
        }
      }
    }
  });

  conn
    .send(CTraderRequestType::SubscribeSpotsReq(SubscribeSpotsReq {
      client_msg_id: Some(id),
      ctid_trader_account_id: account_id,
      symbol_id: vec![symbol_id],
      subscribe_to_spot_timestamp: Some(true),
    }))
    .await?;

  rx_done
    .recv()
    .await
    .context("No SubscribeSpotsRes received")??;
  Ok(())
}

/// Subscribe to live M1 trendbars so SpotEvent messages carry trendbar data.
async fn subscribe_live_trendbars(
  conn: &CTraderConnection,
  account_id: i64,
  symbol_id: i64,
  period: TrendbarPeriod,
) -> anyhow::Result<()> {
  let mut rx = conn.subscribe().await;
  let id = Uuid::now_v7().to_string();
  let (tx, mut rx_done) = unbounded_channel::<anyhow::Result<SubscribeLiveTrendbarRes>>();

  tokio::task::spawn({
    let id = id.clone();
    async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Ok(CTraderResponseType::SubscribeLiveTrendbarRes(res)) => {
            if res.client_msg_id.as_deref() == Some(&id) {
              let _ = tx.send(Ok(res));
              break;
            }
          }
          Ok(_) => {}
          Err(_) => break,
        }
      }
    }
  });

  conn
    .send(CTraderRequestType::SubscribeLiveTrendbarReq(
      SubscribeLiveTrendbarReq {
        client_msg_id: Some(id),
        ctid_trader_account_id: account_id,
        period,
        symbol_id,
      },
    ))
    .await?;

  rx_done
    .recv()
    .await
    .context("No SubscribeLiveTrendbarRes received")??;
  Ok(())
}

/// Fetch the most recent historical trendbars to seed the EMA calculation.
async fn get_historical_trendbars(
  conn: &CTraderConnection,
  account_id: i64,
  symbol_id: i64,
  period: TrendbarPeriod,
  count: u32,
) -> anyhow::Result<Vec<Trendbar>> {
  let mut rx = conn.subscribe().await;
  let id = Uuid::now_v7().to_string();
  let (tx, mut rx_done) = unbounded_channel::<anyhow::Result<GetTrendbarsRes>>();

  tokio::task::spawn({
    let id = id.clone();
    async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Ok(CTraderResponseType::GetTrendbarsRes(res)) => {
            if res.client_msg_id.as_deref() == Some(&id) {
              let _ = tx.send(Ok(res));
              break;
            }
          }
          Ok(_) => {}
          Err(_) => break,
        }
      }
    }
  });

  conn
    .send(CTraderRequestType::GetTrendbarsReq(GetTrendbarsReq {
      client_msg_id: Some(id),
      ctid_trader_account_id: account_id,
      from_timestamp: None,
      to_timestamp: None,
      period,
      symbol_id,
      count: Some(count),
    }))
    .await?;

  let res = rx_done
    .recv()
    .await
    .context("No GetTrendbarsRes received")??;
  Ok(res.trendbar)
}

async fn get_symbol_list(
  conn: &CTraderConnection,
  account_id: i64,
) -> anyhow::Result<HashMap<String, LightSymbol>> {
  let mut rx_ctrader = conn.subscribe().await;
  let id = Uuid::now_v7().to_string();
  let (tx, mut rx) = unbounded_channel::<anyhow::Result<SymbolsListRes>>();

  tokio::task::spawn({
    let id = id.clone();

    async move {
      while let Some(msg) = rx_ctrader.recv().await {
        match msg {
          Ok(CTraderResponseType::SymbolsListRes(res)) => {
            if let Some(msg_id) = &res.client_msg_id
              && msg_id == &id
            {
              let _ = tx.send(Ok(res));
              break;
            }
          }
          Ok(_) => {}
          Err(_) => panic!(),
        }
      }
    }
  });

  conn
    .send(CTraderRequestType::SymbolsListReq(SymbolsListReq {
      client_msg_id: Some(id),
      ctid_trader_account_id: account_id,
      include_archived_symbols: None,
    }))
    .await?;

  let res = rx.recv().await.context("")??;

  let mut symbols = HashMap::new();

  for sym in res.symbol {
    let Some(name) = &sym.symbol_name else {
      continue;
    };

    symbols.insert(name.clone(), sym);
  }

  Ok(symbols)
}
