use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::Row;
use libsql::params;
// Assuming BotModel looks something like this based on your schema.
// Ensure your actual BotModel matches these types or convert them in map_row!
use platform_models::BotModel;
use uuid::Uuid;

pub struct BotRepository {
  db: Connection,
}

impl BotRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    // Fixed the trailing comma syntax error right after the FOREIGN KEY constraint
    let sql = r#"
      CREATE TABLE IF NOT EXISTS bots (
        id                  TEXT      PRIMARY KEY,
        profile_id          TEXT      NOT NULL,
        name                TEXT      NOT NULL,
        language            TEXT      NOT NULL,
        kind                TEXT      NOT NULL,
        handler             BLOB      NOT NULL,
        created_at          DATETIME  DEFAULT CURRENT_TIMESTAMP,

        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;
    Ok(())
  }

  /// Creates or updates a bot configuration using an UPSERT strategy.
  pub async fn upsert(
    &self,
    model: BotModel,
  ) -> anyhow::Result<()> {
    let sql = r#"
      INSERT INTO bots (id, profile_id, name, language, kind, handler, created_at)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
      ON CONFLICT(id) DO UPDATE SET
        profile_id = excluded.profile_id,
        name = excluded.name,
        language = excluded.language,
        kind = excluded.kind,
        handler = excluded.handler,
        created_at = excluded.created_at;
    "#;

    // Convert variables to text/bytes for the DB driver mapping
    let id_str = model.id.to_string();
    let profile_id_str = model.profile_id.to_string();
    let created_at_str = model.created_at.to_rfc3339();

    self
      .db
      .execute(
        sql,
        params![
          id_str,
          profile_id_str,
          model.name,
          model.language,
          model.kind,
          model.handler, // Passes a Vec<u8> natively into the BLOB column
          created_at_str,
        ],
      )
      .await?;

    Ok(())
  }

  pub async fn delete(
    &self,
    bot_id: &Uuid,
  ) -> anyhow::Result<()> {
    let sql = "DELETE FROM bots WHERE id = ?1";
    self.db.execute(sql, params![bot_id.to_string()]).await?;
    Ok(())
  }

  pub async fn get_all(&self) -> anyhow::Result<Vec<BotModel>> {
    let sql = "SELECT id, profile_id, name, language, kind, handler, created_at FROM bots";

    let mut rows = self.db.query(sql, params![]).await?;
    let mut results = Vec::new();

    if let Some(row) = rows.next().await? {
      results.push(self.map_row(row)?);
    }

    Ok(results)
  }

  pub async fn get_by_id(
    &self,
    bot_id: &Uuid,
  ) -> anyhow::Result<Option<BotModel>> {
    let sql =
      "SELECT id, profile_id, name, language, kind, handler, created_at FROM bots WHERE id = ?1";
    let mut rows = self.db.query(sql, params![bot_id.to_string()]).await?;

    if let Some(row) = rows.next().await? {
      let bot = self.map_row(row)?;
      Ok(Some(bot))
    } else {
      Ok(None)
    }
  }

  pub async fn get_by_profile_id(
    &self,
    profile_id: &Uuid,
  ) -> anyhow::Result<Vec<BotModel>> {
    let sql = "SELECT id, profile_id, name, language, kind, handler, created_at FROM bots WHERE profile_id = ?1";
    let mut rows = self.db.query(sql, params![profile_id.to_string()]).await?;

    let mut bots = Vec::new();
    while let Some(row) = rows.next().await? {
      bots.push(self.map_row(row)?);
    }

    Ok(bots)
  }

  /// Maps a database row back into our clean BotModel.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<BotModel> {
    Ok(BotModel {
      id: {
        let id_str: String = row.get(0)?;
        Uuid::parse_str(&id_str)?
      },
      profile_id: {
        let profile_id_str: String = row.get(1)?;
        Uuid::parse_str(&profile_id_str)?
      },
      name: row.get(2)?,
      language: row.get(3)?,
      kind: row.get(4)?,
      handler: row.get(5)?,
      created_at: {
        let created_at_str: String = row.get(6)?;
        DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc)
      },
    })
  }
}
