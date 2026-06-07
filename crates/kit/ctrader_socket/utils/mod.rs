use std::collections::HashMap;

use anyhow::Context;
use tokio::sync::mpsc::unbounded_channel;
use uuid::Uuid;

use super::types::*;
use crate::CTraderRequestType;
use crate::CTraderResponseType;
use crate::LightSymbol;
use crate::connection::CTraderConnectionExt;

pub trait CTraderConnectionUtils {
  fn get_symbol_list(
    &self,
    account_id: i64,
  ) -> impl Future<Output = anyhow::Result<HashMap<String, LightSymbol>>>;
  fn subscribe_spots(
    &self,
    account_id: i64,
    symbol_id: i64,
  ) -> impl Future<Output = anyhow::Result<()>>;
  fn subscribe_live_trendbars(
    &self,
    account_id: i64,
    symbol_id: i64,
    period: TrendbarPeriod,
  ) -> impl Future<Output = anyhow::Result<()>>;
  fn get_historical_trendbars(
    &self,
    account_id: &i64,
    symbol_id: &i64,
    period: TrendbarPeriod,
    count: u32,
  ) -> impl Future<Output = anyhow::Result<Vec<Trendbar>>>;
}

impl<T: CTraderConnectionExt> CTraderConnectionUtils for T {
  fn get_symbol_list(
    &self,
    account_id: i64,
  ) -> impl Future<Output = anyhow::Result<HashMap<String, LightSymbol>>> {
    async move {
      let mut rx_ctrader = self.subscribe().await;
      let id = Uuid::new_v4().to_string();
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

      self
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
  }

  fn subscribe_spots(
    &self,
    account_id: i64,
    symbol_id: i64,
  ) -> impl Future<Output = anyhow::Result<()>> {
    async move {
      let mut rx = self.subscribe().await;
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

      self
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
  }

  fn subscribe_live_trendbars(
    &self,
    account_id: i64,
    symbol_id: i64,
    period: TrendbarPeriod,
  ) -> impl Future<Output = anyhow::Result<()>> {
    async move {
      let mut rx = self.subscribe().await;
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

      self
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
  }

  fn get_historical_trendbars(
    &self,
    account_id: &i64,
    symbol_id: &i64,
    period: TrendbarPeriod,
    count: u32,
  ) -> impl Future<Output = anyhow::Result<Vec<Trendbar>>> {
    async move {
      let mut rx = self.subscribe().await;
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

      self
        .send(CTraderRequestType::GetTrendbarsReq(GetTrendbarsReq {
          client_msg_id: Some(id),
          ctid_trader_account_id: *account_id,
          from_timestamp: None,
          to_timestamp: None,
          period: period,
          symbol_id: *symbol_id,
          count: Some(count),
        }))
        .await?;

      let res = rx_done
        .recv()
        .await
        .context("No GetTrendbarsRes received")??;
      Ok(res.trendbar)
    }
  }
}
