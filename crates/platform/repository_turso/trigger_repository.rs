use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::TriggerModel;
use uuid::Uuid;

pub struct TriggerRepository {
  db: Connection,
}

impl TriggerRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    // Fixed typo "(code id" and added missing comma between FOREIGN KEY constraints
    let sql = r#"
      CREATE TABLE IF NOT EXISTS triggers (
        id                  TEXT      PRIMARY KEY,
        profile_id          TEXT      NOT NULL,
        ctrader_account_id  INTEGER   NOT NULL,
        name                TEXT      NOT NULL,
        platform            TEXT      NOT NULL,
        status              TEXT      NOT NULL,
        created_at          DATETIME  DEFAULT CURRENT_TIMESTAMP,

        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
        FOREIGN KEY (ctrader_account_id) REFERENCES ctrader_accounts(account_id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;
    Ok(())
  }

  /// Creates or updates a trigger.
  pub async fn upsert(
    &self,
    model: TriggerModel,
  ) -> anyhow::Result<()> {
    let sql = r#"
      INSERT INTO triggers (id, profile_id, ctrader_account_id, name, platform, status, created_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
      ON CONFLICT(id) DO UPDATE SET
        profile_id = excluded.profile_id,
        ctrader_account_id = excluded.ctrader_account_id,
        name = excluded.name,
        platform = excluded.platform,
        status = excluded.status,
        created_at = excluded.created_at;
    "#;

    self
      .db
      .execute(
        sql,
        params![
          model.id.to_string(),
          model.profile_id.to_string(),
          model.ctrader_account_id,
          model.name,
          model.platform,
          model.status,
          model.created_at.to_rfc3339(),
        ],
      )
      .await?;

    Ok(())
  }

  pub async fn delete(
    &self,
    trigger_id: &Uuid,
  ) -> anyhow::Result<()> {
    let sql = "DELETE FROM triggers WHERE id = ?1;";

    self
      .db
      .execute(sql, params![trigger_id.to_string()])
      .await?;

    Ok(())
  }

  pub async fn get_by_id(
    &self,
    trigger_id: &Uuid,
  ) -> anyhow::Result<Option<TriggerModel>> {
    let sql = "SELECT id, profile_id, ctrader_account_id, name, platform, status, created_at FROM triggers WHERE id = ?1;";

    let mut rows = self.db.query(sql, params![trigger_id.to_string()]).await?;

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
  ) -> anyhow::Result<Vec<TriggerModel>> {
    let sql = "SELECT id, profile_id, ctrader_account_id, name, platform, status, created_at FROM triggers WHERE profile_id = ?1;";

    let mut rows = self.db.query(sql, params![profile_id.to_string()]).await?;
    let mut results = Vec::new();

    while let Some(row) = rows.next().await? {
      let model = self.map_row(row)?;
      results.push(model);
    }

    Ok(results)
  }

  /// Maps a database row back into our clean TriggerModel.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<TriggerModel> {
    Ok(TriggerModel {
      id: {
        let id_str: String = row.get(0)?;
        Uuid::parse_str(&id_str)?
      },
      profile_id: {
        let profile_id_str: String = row.get(1)?;
        Uuid::parse_str(&profile_id_str)?
      },
      ctrader_account_id: row.get(2)?,
      name: row.get(3)?,
      platform: row.get(4)?,
      status: row.get(5)?,
      created_at: {
        let created_at: String = row.get(6)?;
        created_at.parse::<DateTime<Utc>>()?
      },
    })
  }
}
