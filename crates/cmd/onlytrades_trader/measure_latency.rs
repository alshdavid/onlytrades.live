use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use kit_ctrader_rest::client::TradingAccount;
use kit_ctrader_socket::connection::CTraderConnectionExt;

use crate::Env;
use crate::run_strategy::connect_to_ctrader_socket;

pub async fn measure_latency(
  env: Arc<Env>,
  ctrader_access_token: String,
  accounts: Vec<TradingAccount>,
  account_id: i64,
) -> anyhow::Result<()> {
  let account = accounts
    .into_iter()
    .find(|a| a.account_id == account_id)
    .context("No account")?;

  let conn = connect_to_ctrader_socket(&env, &ctrader_access_token, &account).await?;

  loop {
    let Ok(latency) = conn.latency().await else {
      continue;
    };
    println!("LATENCY: {}", latency);
    tokio::time::sleep(Duration::from_secs(60)).await;
  }
}
