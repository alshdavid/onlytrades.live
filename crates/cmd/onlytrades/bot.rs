use anyhow::Context;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::connection::CTraderConnectionExt;
use kit_ctrader_socket::utils::CTraderConnectableExt;
use kit_ctrader_socket::*;

pub async fn handler(
  conn: CTraderConnection,
  account_id: i64,
) -> anyhow::Result<()> {
  let symbols = conn.get_symbol_list(account_id).await?;
  let target: &LightSymbol = symbols.get("BTCUSD").context("BTCUSD symbol not found")?;
  let symbol_id: i64 = target.symbol_id;
  let symbol_name: &str = target.symbol_name.as_deref().unwrap_or("BTCUSD");
  let timeframe = TrendbarPeriod::M1;

  conn.subscribe_spots(account_id, symbol_id).await?;
  conn
    .subscribe_live_trendbars(account_id, symbol_id, timeframe)
    .await?;
  let historical = conn
    .get_historical_trendbars(account_id, symbol_id, timeframe, 30)
    .await?;

  // Compute initial EMA from historical bars
  let k = 2.0 / (20.0 + 1.0); // EMA20 smoothing factor: 2 / (N + 1)
  let mut ema: f64 = 0.0;
  for (i, bar) in historical.iter().enumerate() {
    let close = bar.close_price();
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

  let mut rx_ctrader = conn.subscribe().await;
  let mut prev_minutes: Option<u32> = None;
  let mut prev_close: Option<f64> = None;

  while let Some(msg) = rx_ctrader.recv().await {
    match msg {
      Ok(CTraderResponseType::SpotEvent(event)) => {
        if event.symbol_id != symbol_id {
          continue;
        }

        if let Some(tb) = event.trendbar.first() {
          if let Some(current_minutes) = tb.utc_timestamp_in_minutes {
            if let Some(prev_m) = prev_minutes {
              if current_minutes != prev_m {
                if let Some(close) = prev_close {
                  ema = close * k + ema * (1.0 - k);
                  println!(
                    "[{symbol_name}] M1 candle closed at minute {prev_m} — Close: {close:.5}, EMA20: {ema:.5}",
                  );
                }
              }
            }

            prev_minutes = Some(current_minutes);
            prev_close = Some(tb.close_price());
          }
        }
      }
      Ok(_) => {}
      Err(_) => panic!("Connection error"),
    }
  }

  Ok(())
}
