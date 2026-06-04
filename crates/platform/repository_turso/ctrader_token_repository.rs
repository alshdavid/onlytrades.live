use chrono::DateTime;
use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::CtraderTokenModel;
use uuid::Uuid;

pub struct CtraderTokenRepository {
  db: Connection,
}

impl CtraderTokenRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    let sql = r#"
      CREATE TABLE IF NOT EXISTS ctrader_tokens (
        token_id                  TEXT PRIMARY KEY,
        profile_id                TEXT NOT NULL UNIQUE,
        refresh_token             TEXT NOT NULL,
        access_token              TEXT NOT NULL,
        access_token_expires_at   INTEGER NOT NULL,
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;
    Ok(())
  }

  /// Creates or updates a cTrader OAuth token set for a profile.
  pub async fn upsert(
    &self,
    model: &CtraderTokenModel,
    profile_id: &uuid::Uuid,
  ) -> anyhow::Result<()> {
    let sql = r#"
      INSERT INTO ctrader_tokens (
        token_id, profile_id, refresh_token, access_token, access_token_expires_at
      ) VALUES (?1, ?2, ?3, ?4, ?5)
      ON CONFLICT(profile_id) DO UPDATE SET
        token_id = EXCLUDED.token_id,
        refresh_token = EXCLUDED.refresh_token,
        access_token = EXCLUDED.access_token,
        access_token_expires_at = EXCLUDED.access_token_expires_at;
    "#;

    self
      .db
      .execute(
        sql,
        params![
          model.token_id.to_string(),
          profile_id.to_string(),
          model.refresh_token.clone(),
          model.access_token.clone(),
          model.access_token_expires_at.timestamp(), // DateTime<Utc> -> i64 timestamp
        ],
      )
      .await?;

    Ok(())
  }

  /// Retrieves the cTrader tokens associated with a single profile, if they exist.
  pub async fn get_tokens_for_profile(
    &self,
    profile_id: &Uuid,
  ) -> anyhow::Result<Option<CtraderTokenModel>> {
    let sql = "SELECT * FROM ctrader_tokens WHERE profile_id = ?1;";
    let mut rows = self.db.query(sql, params![profile_id.to_string()]).await?;

    if let Some(row) = rows.next().await? {
      let model = self.map_row(row)?;
      Ok(Some(model))
    } else {
      Ok(None)
    }
  }

  /// Performs a hard delete on a token block (e.g., when disconnecting an integration).
  pub async fn delete(
    &self,
    token_id: &str,
  ) -> anyhow::Result<()> {
    let sql = "DELETE FROM ctrader_tokens WHERE token_id = ?1;";
    self.db.execute(sql, params![token_id]).await?;
    Ok(())
  }

  /// Helper to map a database row to the CtraderTokenModel struct.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<CtraderTokenModel> {
    Ok(CtraderTokenModel {
      token_id: Uuid::parse_str(&row.get::<String>(0)?)?,
      refresh_token: row.get(2)?,
      access_token: row.get(3)?,
      access_token_expires_at: {
        let expires_at_raw: i64 = row.get(4)?;
        // Safely parse the timestamp back into a Chrono DateTime object
        DateTime::from_timestamp(expires_at_raw, 0)
          .ok_or_else(|| anyhow::anyhow!("Invalid timestamp integer found in database row"))?
      },
    })
  }
}
