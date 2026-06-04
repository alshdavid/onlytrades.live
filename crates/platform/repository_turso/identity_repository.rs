// CREATE TABLE IF NOT EXISTS identities (
//     sub         TEXT PRIMARY KEY, -- The unique Auth0 'sub' (e.g., 'auth0|123')
//     profile_id  TEXT NOT NULL,
//     provider    TEXT,             -- e.g., 'google-oauth2', 'github', 'database'
//     last_login  DATETIME DEFAULT CURRENT_TIMESTAMP,
//     FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
// );

use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::IdentityModel;
use uuid::Uuid;

pub struct IdentityRepository {
  db: Connection,
}

impl IdentityRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    let sql = r#"
      CREATE TABLE IF NOT EXISTS identities (
        sub         TEXT PRIMARY KEY,
        profile_id  TEXT NOT NULL,
        provider    TEXT,
        last_login  DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;

    Ok(())
  }

  /// Creates or updates an identity on login.
  /// This updates the 'last_login' timestamp if the sub already exists.
  pub async fn upsert(
    &self,
    sub: &str,
    profile_id: &Uuid,
  ) -> anyhow::Result<()> {
    let provider = sub.split('|').next().unwrap_or("auth0");
    let now = Utc::now().to_rfc3339();

    self
      .db
      .execute(
        "INSERT INTO identities (sub, profile_id, provider, last_login)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(sub) DO UPDATE SET last_login = excluded.last_login",
        params![sub, profile_id.to_string(), provider, now],
      )
      .await?;

    Ok(())
  }

  /// Retrieves a specific identity by its Auth0 sub.
  pub async fn find_by_sub(
    &self,
    sub: &str,
  ) -> anyhow::Result<Option<IdentityModel>> {
    let mut rows = self
      .db
      .query(
        "SELECT sub, profile_id, provider, last_login FROM identities WHERE sub = ?1",
        params![sub],
      )
      .await?;

    if let Some(row) = rows.next().await? {
      return Ok(Some(self.map_row(row)?));
    }

    Ok(None)
  }

  /// Finds all identities linked to a single profile (e.g., for a settings page).
  pub async fn find_by_profile_id(
    &self,
    profile_id: &Uuid,
  ) -> anyhow::Result<Vec<IdentityModel>> {
    let mut rows = self
      .db
      .query(
        "SELECT sub, profile_id, provider, last_login FROM identities WHERE profile_id = ?1",
        params![profile_id.to_string()],
      )
      .await?;

    let mut identities = Vec::new();
    while let Some(row) = rows.next().await? {
      identities.push(self.map_row(row)?);
    }

    Ok(identities)
  }

  pub async fn delete(
    &self,
    sub: &str,
  ) -> anyhow::Result<()> {
    // We check if this is the user's LAST identity first?
    // (See Safeguards section below)

    self
      .db
      .execute("DELETE FROM identities WHERE sub = ?1", params![sub])
      .await?;

    Ok(())
  }

  /// Helper to map a database row to the IdentityModel struct.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<IdentityModel> {
    Ok(IdentityModel {
      sub: row.get(0)?,
      profile_id: uuid::Uuid::parse_str(&row.get::<String>(1)?)?,
      provider: row.get(2)?,
      last_login: {
        // Parsing the ISO 8601 string back into DateTime<Utc>
        let last_login_str: String = row.get(3)?;
        last_login_str.parse::<DateTime<Utc>>()?
      },
    })
  }
}
