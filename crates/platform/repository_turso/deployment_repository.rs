use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::DeploymentModel;
use uuid::Uuid;

pub struct DeploymentRepository {
  db: Connection,
}

impl DeploymentRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    let sql = r#"
      CREATE TABLE IF NOT EXISTS deployments (
        id              TEXT      PRIMARY KEY,
        bot_id          TEXT      NOT NULL,
        account_id      INTEGER   NOT NULL,
        name            TEXT      NOT NULL,
        environment     BLOB      NOT NULL,
        active          INTEGER   NOT NULL,
        created_at      DATETIME  DEFAULT CURRENT_TIMESTAMP,

        FOREIGN KEY (bot_id) REFERENCES bots(id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;
    Ok(())
  }

  pub async fn upsert(
    &self,
    model: DeploymentModel,
  ) -> anyhow::Result<()> {
    let sql = r#"
      INSERT INTO deployments (id, bot_id, account_id, name, environment, active, created_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
      ON CONFLICT(id) DO UPDATE SET
        bot_id = excluded.bot_id,
        account_id = excluded.account_id,
        name = excluded.name,
        environment = excluded.environment,
        active = excluded.active,
        created_at = excluded.created_at;
    "#;

    // Format timestamps consistently as ISO 8601 strings for SQLite
    let created_at_str = model.created_at.to_rfc3339();

    self
      .db
      .execute(
        sql,
        params![
          model.id.to_string(),
          model.bot_id.to_string(),
          model.account_id,
          model.name,
          model.environment,
          if model.active { 1 } else { 0 },
          created_at_str,
        ],
      )
      .await?;

    Ok(())
  }

  pub async fn delete(
    &self,
    deployment_id: &Uuid,
  ) -> anyhow::Result<()> {
    let sql = "DELETE FROM deployments WHERE id = ?1";

    self
      .db
      .execute(sql, params![deployment_id.to_string()])
      .await?;
    Ok(())
  }

  pub async fn get_all(&self) -> anyhow::Result<Vec<DeploymentModel>> {
    let sql =
      "SELECT id, bot_id, account_id, name, environment, active, created_at FROM deployments";

    let mut rows = self.db.query(sql, params![]).await?;
    let mut results = Vec::new();

    if let Some(row) = rows.next().await? {
      results.push(self.map_row(row)?);
    }

    Ok(results)
  }

  pub async fn get_by_id(
    &self,
    id: &Uuid, // Note: Changed parameter name from bot_id to id to match 'get_by_id' intent
  ) -> anyhow::Result<Option<DeploymentModel>> {
    let sql = "SELECT id, bot_id, account_id, name, environment, active, created_at FROM deployments WHERE id = ?1";

    let mut rows = self.db.query(sql, params![id.to_string()]).await?;

    if let Some(row) = rows.next().await? {
      let model = self.map_row(row)?;
      Ok(Some(model))
    } else {
      Ok(None)
    }
  }

  pub async fn get_by_profile_id(
    &self,
    profile_id: &Uuid,
  ) -> anyhow::Result<Vec<DeploymentModel>> {
    // Note: Assuming your 'bots' table links to a 'profile_id'.
    // We perform a JOIN here since profile_id isn't directly on the deployments table.
    let sql = r#"
      SELECT d.id, d.bot_id, d.account_id, d.name, d.environment, d.active, d.created_at 
      FROM deployments d
      JOIN bots b ON d.bot_id = b.id
      WHERE b.profile_id = ?1
    "#;

    let mut rows = self.db.query(sql, params![profile_id.to_string()]).await?;
    let mut deployments = Vec::new();

    while let Some(row) = rows.next().await? {
      deployments.push(self.map_row(row)?);
    }

    Ok(deployments)
  }

  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<DeploymentModel> {
    Ok(DeploymentModel {
      id: {
        let id_str: String = row.get(0)?;
        Uuid::parse_str(&id_str)?
      },
      bot_id: {
        let bot_id_str: String = row.get(1)?;
        Uuid::parse_str(&bot_id_str)?
      },
      account_id: row.get(2)?,
      name: row.get(3)?,
      environment: row.get(4)?,
      active: {
        let active: i64 = row.get(5)?;
        active == 1
      },
      created_at: {
        let created_at_str: String = row.get(6)?;
        DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc)
      },
    })
  }
}
