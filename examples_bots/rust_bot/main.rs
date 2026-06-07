use onlytrades_bot::CTraderRequestType;
use onlytrades_bot::CTraderResponseType;
use onlytrades_bot::Context;
use onlytrades_bot::NewOrderReq;
use onlytrades_bot::OrderType;
use onlytrades_bot::SymbolsListReq;
use onlytrades_bot::TimeInForce;
use onlytrades_bot::TradeSide;
use tokio::sync::mpsc::unbounded_channel;

#[onlytrades_bot::main]
async fn main(ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
  let (tx_symbols, mut rx_symbols) = unbounded_channel();

  let handle = tokio::task::spawn_local({
    let mut rx = ctx.subscribe().await;

    async move {
      while let Some(msg) = rx.recv().await {
        match msg {
          Ok(CTraderResponseType::SymbolsListRes(res)) => {
            let sym = res
              .symbol
              .into_iter()
              .find(|s| s.symbol_name.as_ref().is_some_and(|name| name == "US500"))
              .unwrap();
            let _ = tx_symbols.send(sym);
          }
          Ok(CTraderResponseType::ExecutionEvent(_ev)) => {
            println!("ExecutionEvent")
          }
          Ok(_ev) => {
            panic!()
          }
          Err(_) => panic!(),
        };
        println!();
      }
    }
  });

  ctx.send(CTraderRequestType::SymbolsListReq(SymbolsListReq {
    client_msg_id: None,
    ctid_trader_account_id: ctx.account_id(),
    include_archived_symbols: None,
  }))?;

  let sym = rx_symbols.recv().await.unwrap();
  dbg!(&sym);

  ctx.send(CTraderRequestType::NewOrderReq(NewOrderReq {
    client_msg_id: None,
    ctid_trader_account_id: ctx.account_id(),
    symbol_id: sym.symbol_id,
    order_type: OrderType::Market,
    trade_side: TradeSide::Buy,
    volume: 10,
    limit_price: None,
    stop_price: None,
    time_in_force: Some(TimeInForce::ImmediateOrCancel),
    expiration_timestamp: None,
    stop_loss: None,
    take_profit: None,
    comment: None,
    base_slippage_price: None,
    slippage_in_points: None,
    label: None,
    position_id: None,
    client_order_id: None,
    relative_stop_loss: None,
    relative_take_profit: None,
    guaranteed_stop_loss: None,
    trailing_stop_loss: None,
    stop_trigger_method: None,
  }))?;

  handle.await.unwrap();
  Ok(())
}
