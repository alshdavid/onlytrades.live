use std::collections::VecDeque;
use std::time::Duration;

use anyhow::Context;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::connection::CTraderConnectionExt;
use kit_ctrader_socket::utils::CTraderConnectionUtils;
use kit_ctrader_socket::*;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Clone, Debug)]
pub struct Ctx<'a> {
  series: &'a VecDeque<Trendbar>,
}

pub async fn strategy<'a>(ctx: Ctx<'a>) {
  let prices = ctx.series.iter().map(|v| v.close_price()).collect::<Vec<_>>();
  
  if let Some(raw_ema20) = EmaEngine::calculate(&prices, 20) {
    let real_ema20 = raw_ema20 / 100_000.0;
    println!("Current EMA20: {:.2}", real_ema20);
  }
}

pub async fn handler(
  conn: CTraderConnection,
  account_id: i64,
) -> anyhow::Result<()> {
  let mut symbols = conn.get_symbol_list(account_id).await?;
  let symbol = symbols
    .remove("BTCUSD")
    .context("BTCUSD symbol not found")?;

  let timeframe = TrendbarPeriod::M1;

  let mut rx = conn.subscribe().await;

  conn.subscribe_spots(account_id, symbol.symbol_id).await?;
  conn.subscribe_live_trendbars(account_id, symbol.symbol_id, timeframe).await?;

  let historical = conn
    .get_historical_trendbars(&account_id, &symbol.symbol_id, timeframe, 500)
    .await?;


  let mut series = VecDeque::with_capacity(1000);
  for trendbar in historical {
    series.push_back(trendbar);

    let ctx = Ctx {
      series: &series,
    };

    strategy(ctx).await;
  }

  println!("LIVE");

  while let Some(msg) = rx.recv().await {
    match msg {
      Ok(CTraderResponseType::SpotEvent(event)) => {
        if event.symbol_id != symbol.symbol_id {
          continue;
        }

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
            series.push_back(new_bar.clone());
            println!("next candle");
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
        let ctx = Ctx { series: &series };
        strategy(ctx).await;
      }
      Ok(_) => {}
      Err(_) => panic!("Connection error"),
    }
  }

  Ok(())
}

pub struct EmaEngine;

impl EmaEngine {
  pub fn calculate(
    prices: &[i64], // Assuming close_price() returns raw i64 values
    period: usize,
  ) -> Option<f64> {
    if prices.len() < period {
      return None;
    }

    // FIX 2: Perform calculations using float math so fractions work
    let k = 2.0 / (period as f64 + 1.0);

    // FIX 3: Seed SMA using floats
    let initial_sma: f64 = prices.iter()
        .take(period)
        .map(|&p| p as f64)
        .sum::<f64>() / (period as f64);

    let mut ema = initial_sma;
    
    // Iterates forward chronologically from the 21st bar onward
    for &price in prices.iter().skip(period) {
        ema = (price as f64) * k + ema * (1.0 - k);
    }

    Some(ema)
  }
}


  // dbg!(historical);

  // let period = 20;
  // let k = 2.0 / (period as f64 + 1.0);
  // let mut ema: f64 = 0.0;

  // let initial_sma: f64 = historical
  //   .iter()
  //   .take(period)
  //   .map(|bar| bar.close_price())
  //   .sum::<f64>()
  //   / (period as f64);

  // ema = initial_sma;

  // for bar in historical.iter().skip(period) {
  //   let close = bar.close_price();
  //   ema = close * k + ema * (1.0 - k);
  // }
  // println!(
  //   "[] Seeded EMA20 from {} historical M1 bars: {ema:.5}",
  //   historical.len(),
  // );

  // let mut rx_ctrader = conn.subscribe().await;
  // let mut prev_minutes: Option<u32> = None;
  // let mut prev_close: Option<f64> = None;

  // while let Some(msg) = rx_ctrader.recv().await {
  //   match msg {
  //     Ok(CTraderResponseType::SpotEvent(event)) => {
  //       if event.symbol_id != symbol.symbol_id {
  //         continue;
  //       }

  //       event.

  //       if let Some(tb) = event.trendbar.first() {
  //         if let Some(current_minutes) = tb.utc_timestamp_in_minutes {
  //           if let Some(prev_m) = prev_minutes {
  //             if current_minutes != prev_m {
  //               if let Some(close) = prev_close {
  //                 ema = close * k + ema * (1.0 - k);
  //                 println!(
  //                   "[] M1 candle closed at minute {prev_m} — Close: {close:.5}, EMA20: {ema:.5}",
  //                 );
  //               }
  //             }
  //           }

  //           prev_minutes = Some(current_minutes);
  //           prev_close = Some(tb.close_price());
  //         }
  //       }
  //     }
  //     Ok(_) => {}
  //     Err(_) => panic!("Connection error"),
  //   }
  // }

  // tokio::time::sleep(Duration::from_secs(1000)).await;

// struct EmaIndicator {}

// struct EmaEvent {
//   closed_at: u128,
//   close_value: f64,
//   ema: f64,
// }

// impl EmaIndicator {
//   pub async fn new(
//     conn: &CTraderConnection,
//     account_id: &i64,
//     symbol: LightSymbol,
//     timeframe: TrendbarPeriod,
//     period: u32,
//   ) -> anyhow::Result<Self> {
//     let mut rx_ctrader = conn.subscribe().await;
//     let mut prev_minutes: Option<u32> = None;
//     let mut prev_close: Option<f64> = None;

//     let historical = conn
//       .get_historical_trendbars(account_id, &symbol.symbol_id, timeframe, period * 5)
//       .await?;

//     let k = 2.0 / (period as f64 + 1.0);
//     let mut ema: f64 = 0.0;

//     let initial_sma: f64 = historical
//       .iter()
//       .take(period as usize)
//       .map(|bar| bar.close_price())
//       .sum::<f64>()
//       / (period as f64);

//     ema = initial_sma;

//     for bar in historical.iter().skip(period as usize) {
//       let close = bar.close_price();
//       ema = close * k + ema * (1.0 - k);
//     }

//   println!(
//     "[] Seeded EMA20 from {} historical M1 bars: {ema:.5}",
//     historical.len(),
//   );
//     tokio::spawn(async move {
//       while let Some(msg) = rx_ctrader.recv().await {
//         match msg {
//           Ok(CTraderResponseType::SpotEvent(event)) => {
//             if event.symbol_id != symbol.symbol_id {
//               continue;
//             }

//             if let Some(tb) = event.trendbar.first() {
//               if let Some(current_minutes) = tb.utc_timestamp_in_minutes {
//                 if let Some(prev_m) = prev_minutes {
//                   if current_minutes != prev_m {
//                     if let Some(close) = prev_close {
//                       ema = close * k + ema * (1.0 - k);
//                       println!(
//                         "[2] M1 candle closed at minute {prev_m} — Close: {close:.5}, EMA20: {ema:.5}",
//                       );
//                     }
//                   }
//                 }

//                 prev_minutes = Some(current_minutes);
//                 prev_close = Some(tb.close_price());
//               }
//             }
//           }
//           Ok(_) => {}
//           Err(_) => panic!("Connection error"),
//         }
//       }

//     });

//     Ok(Self {})
//   }

//   pub fn subscribe(&self) -> UnboundedReceiver<EmaEvent> {
//     todo!()
//   }
// }
