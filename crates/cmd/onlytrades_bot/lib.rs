mod context;
mod error;

pub use kit_ctrader_socket::CTraderRequestType;
pub use kit_ctrader_socket::CTraderResponseType;
pub use kit_ctrader_socket::types::*;
pub use onlytrades_macros::*;

pub use self::context::*;
pub use self::error::*;

pub fn bootstrap<F, Fut, R>(func: F) -> R
where
  F: 'static + Fn(Context) -> Fut,
  Fut: 'static + Future<Output = R>,
{
  tokio::runtime::LocalRuntime::new()
    .expect("Failed to build the runtime")
    .block_on(async move {
      let ctx = Context::new()
        .await
        // This must connect or exit the process
        .unwrap();

      func(ctx).await
    })
}
