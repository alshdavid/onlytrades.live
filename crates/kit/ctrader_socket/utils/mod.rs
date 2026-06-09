use std::collections::HashMap;

use anyhow::Context;
use tokio::sync::mpsc::unbounded_channel;
use uuid::Uuid;

use super::types::*;
use crate::CTraderRequestType;
use crate::CTraderResponseType;
use crate::LightSymbol;
use crate::NewOrderReq;
use crate::connection::CTraderConnectionExt;

pub trait CTraderConnectionUtils {
  fn get_symbol_list(
    &self,
    account_id: i64,
  ) -> impl Future<Output = anyhow::Result<HashMap<String, LightSymbol>>>;
  fn get_symbol(
    &self,
    account_id: i64,
    symbol_ids: &[i64],
  ) -> impl Future<Output = anyhow::Result<HashMap<i64, Symbol>>>;
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
  fn reconcile(
    &self,
    account_id: &i64,
  ) -> impl Future<Output = anyhow::Result<ReconcileRes>>;

  fn new_order(
    &self,
    options: NewOrderReq,
  ) -> impl Future<Output = anyhow::Result<ExecutionEvent>>;

  fn close_position(
    &self,
    options: ClosePositionReq,
  ) -> impl Future<Output = anyhow::Result<ExecutionEvent>>;
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
              Ok(CTraderResponseType::ErrorRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Err(anyhow::anyhow!("{:?}", res)));
                  break;
                }
              }
              Ok(_) => {}
              Err(err) => {
                let _ = tx.send(Err(anyhow::anyhow!("{:?}", err)));
                break;
              }
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
              Ok(CTraderResponseType::ErrorRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Err(anyhow::anyhow!("{:?}", res)));
                  break;
                }
              }
              Ok(_) => {}
              Err(err) => {
                let _ = tx.send(Err(anyhow::anyhow!("{:?}", err)));
                break;
              }
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
              Ok(CTraderResponseType::ErrorRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Err(anyhow::anyhow!("{:?}", res)));
                  break;
                }
              }
              Ok(_) => {}
              Err(err) => {
                let _ = tx.send(Err(anyhow::anyhow!("{:?}", err)));
                break;
              }
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
              Ok(CTraderResponseType::ErrorRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Err(anyhow::anyhow!("{:?}", res)));
                  break;
                }
              }
              Ok(_) => {}
              Err(err) => {
                let _ = tx.send(Err(anyhow::anyhow!("{:?}", err)));
                break;
              }
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

  fn reconcile(
    &self,
    account_id: &i64,
  ) -> impl Future<Output = anyhow::Result<ReconcileRes>> {
    async move {
      let mut rx = self.subscribe().await;
      let id = Uuid::now_v7().to_string();
      let (tx, mut rx_done) = unbounded_channel::<anyhow::Result<ReconcileRes>>();

      tokio::task::spawn({
        let id = id.clone();
        async move {
          while let Some(msg) = rx.recv().await {
            match msg {
              Ok(CTraderResponseType::ReconcileRes(res)) => {
                if res.client_msg_id.as_deref() == Some(&id) {
                  let _ = tx.send(Ok(res));
                  break;
                }
              }
              Ok(CTraderResponseType::ErrorRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Err(anyhow::anyhow!("{:?}", res)));
                  break;
                }
              }
              Ok(_) => {}
              Err(err) => {
                let _ = tx.send(Err(anyhow::anyhow!("{:?}", err)));
                break;
              }
            }
          }
        }
      });

      self
        .send(CTraderRequestType::ReconcileReq(ReconcileReq {
          client_msg_id: Some(id),
          ctid_trader_account_id: *account_id,
          return_protection_orders: None,
        }))
        .await?;

      let res = rx_done.recv().await.context("No ReconcileRes received")??;

      Ok(res)
    }
  }

  fn new_order(
    &self,
    options: NewOrderReq,
  ) -> impl Future<Output = anyhow::Result<ExecutionEvent>> {
    async move {
      let (tx, mut rx_done) = unbounded_channel::<anyhow::Result<ExecutionEvent>>();
      let mut rx = self.subscribe().await;

      self.send(CTraderRequestType::NewOrderReq(options)).await?;

      tokio::task::spawn({
        async move {
          while let Some(msg) = rx.recv().await {
            match msg {
              Ok(CTraderResponseType::ExecutionEvent(res)) => {
                if res.execution_type == ExecutionType::OrderFilled {
                  let _ = tx.send(Ok(*res));
                  break;
                }
              }
              Ok(_) => {}
              Err(_) => break,
            }
          }
        }
      });

      rx_done.recv().await.context("No ReconcileRes received")?
    }
  }

  fn close_position(
    &self,
    options: ClosePositionReq,
  ) -> impl Future<Output = anyhow::Result<ExecutionEvent>> {
    async move {
      let (tx, mut rx_done) = unbounded_channel::<anyhow::Result<ExecutionEvent>>();
      let mut rx = self.subscribe().await;

      self
        .send(CTraderRequestType::ClosePositionReq(options))
        .await?;

      tokio::task::spawn({
        async move {
          while let Some(msg) = rx.recv().await {
            match msg {
              Ok(CTraderResponseType::ExecutionEvent(res)) => {
                if res.execution_type == ExecutionType::OrderFilled {
                  let _ = tx.send(Ok(*res));
                  break;
                }
              }
              Ok(_) => {}
              Err(_) => {}
            }
          }
        }
      });

      rx_done.recv().await.context("No ReconcileRes received")?
    }
  }

  fn get_symbol(
    &self,
    account_id: i64,
    symbol_ids: &[i64],
  ) -> impl Future<Output = anyhow::Result<HashMap<i64, Symbol>>> {
    async move {
      let mut rx_ctrader = self.subscribe().await;
      let id = Uuid::new_v4().to_string();
      let (tx, mut rx) = unbounded_channel::<anyhow::Result<SymbolByIdRes>>();

      tokio::task::spawn({
        let id = id.clone();

        async move {
          while let Some(msg) = rx_ctrader.recv().await {
            match msg {
              Ok(CTraderResponseType::SymbolByIdRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Ok(res));
                  break;
                }
              }
              Ok(CTraderResponseType::ErrorRes(res)) => {
                if let Some(msg_id) = &res.client_msg_id
                  && msg_id == &id
                {
                  let _ = tx.send(Err(anyhow::anyhow!("{:?}", res)));
                  break;
                }
              }
              Ok(_) => {}
              Err(err) => {
                let _ = tx.send(Err(anyhow::anyhow!("{:?}", err)));
                break;
              }
            }
          }
        }
      });

      self
        .send(CTraderRequestType::SymbolByIdReq(SymbolByIdReq {
          client_msg_id: Some(id),
          ctid_trader_account_id: account_id,
          symbol_id: symbol_ids.to_vec(),
        }))
        .await?;

      let res = rx.recv().await.context("")??;

      let mut symbols = HashMap::new();

      for sym in res.symbol {
        symbols.insert(sym.symbol_id.clone(), sym);
      }

      Ok(symbols)
    }
  }
}
