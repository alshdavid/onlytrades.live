pub use kit_ctrader::sockets::types;
pub use onlytrades_macros::*;
use tokio::sync::mpsc::UnboundedReceiver;

pub fn bootstrap<F, Fut, R>(func: F) -> R
where
  F: 'static + Fn(Context) -> Fut,
  Fut: 'static + Future<Output = R>,
{
  let ctx = Context {};

  tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Failed to build the runtime")
    .block_on(func(ctx))
}

pub enum CTreaderResponseType {
  ExecutionEvent(types::ExecutionEvent),
  SymbolsList(types::SymbolsListRes),
}

pub struct Context {}

impl Context {
  pub fn subscribe(&self) -> UnboundedReceiver<CTreaderResponseType> {
    todo!()
  }
}
