use kit_ctrader::*;
use uuid::Uuid;

use super::CTraderService;

impl CTraderService {
  pub async fn new_order(
    &self,
    connection_name: &str,
    profile_id: &Uuid,
    account_id: &i64,
    options: CTraderNewOrderOptions,
  ) -> anyhow::Result<ExecutionEvent> {
    let (conn, _account) = self
      .get_or_connect_socket(connection_name, profile_id, account_id)
      .await?;
    conn.new_order(options).await
  }

  pub async fn reconcile(
    &self,
    connection_name: &str,
    profile_id: &Uuid,
    account_id: &i64,
    options: CTraderReconcileOptions,
  ) -> anyhow::Result<CTraderReconcileResult> {
    let (conn, _account) = self
      .get_or_connect_socket(connection_name, profile_id, account_id)
      .await?;
    conn.reconcile(options).await
  }

  pub async fn close_position(
    &self,
    connection_name: &str,
    profile_id: &Uuid,
    account_id: &i64,
    options: CTraderClosePositionOptions,
  ) -> anyhow::Result<ExecutionEvent> {
    let (conn, _account) = self
      .get_or_connect_socket(connection_name, profile_id, account_id)
      .await?;
    conn.close_position(options).await
  }

  pub async fn subscribe_spots(
    &self,
    connection_name: &str,
    profile_id: &Uuid,
    account_id: &i64,
    options: CTraderSubscribeSpotsOptions,
  ) -> anyhow::Result<()> {
    let (conn, _account) = self
      .get_or_connect_socket(connection_name, profile_id, account_id)
      .await?;
    conn.subscribe_spots(options).await?;
    Ok(())
  }

  pub async fn subscribe(
    &self,
    connection_name: &str,
    profile_id: &Uuid,
    account_id: &i64,
  ) -> anyhow::Result<ProtoReceiver> {
    let (conn, _account) = self
      .get_or_connect_socket(connection_name, profile_id, account_id)
      .await?;
    Ok(conn.subscribe_fallback_proto().await?)
  }
}
