// CREATE TABLE IF NOT EXISTS profiles (
//     id         TEXT PRIMARY KEY,
//     email      TEXT UNIQUE, -- The primary contact email
//     created_at DATETIME DEFAULT CURRENT_TIMESTAMP
// );

use chrono::DateTime;
use chrono::Utc;
use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::ProfileModel;
use uuid::Uuid;

pub struct ProfileRepository {
  db: Connection,
}

impl ProfileRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    let sql = r#"
      CREATE TABLE IF NOT EXISTS profiles (
        id         TEXT PRIMARY KEY,
        email      TEXT UNIQUE NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      );
    "#;

    self.db.execute(sql, ()).await?;

    Ok(())
  }

  /// Creates a new profile with a fresh UUID.
  pub async fn create(
    &self,
    email: &str,
  ) -> anyhow::Result<ProfileModel> {
    let id = Uuid::now_v7();
    let now = Utc::now();
    let now_str = now.to_rfc3339();

    self
      .db
      .execute(
        "INSERT INTO profiles (id, email, created_at) VALUES (?1, ?2, ?3)",
        params![id.to_string(), email, now_str],
      )
      .await?;

    Ok(ProfileModel {
      id,
      email: email.to_string(),
      created_at: now,
    })
  }

  /// Finds a profile by its GUID.
  pub async fn find_by_id(
    &self,
    id: &Uuid,
  ) -> anyhow::Result<Option<ProfileModel>> {
    let mut rows = self
      .db
      .query(
        "SELECT id, email, created_at FROM profiles WHERE id = ?1",
        params![id.to_string()],
      )
      .await?;

    if let Some(row) = rows.next().await? {
      return Ok(Some(self.map_row(row)?));
    }

    Ok(None)
  }

  /// Finds a profile by email (useful for the login/linking flow).
  pub async fn find_by_email(
    &self,
    email: &str,
  ) -> anyhow::Result<Option<ProfileModel>> {
    let mut rows = self
      .db
      .query(
        "SELECT id, email, created_at FROM profiles WHERE email = ?1",
        params![email],
      )
      .await?;

    if let Some(row) = rows.next().await? {
      return Ok(Some(self.map_row(row)?));
    }

    Ok(None)
  }

  /// Updates the email for a profile.
  pub async fn update_email(
    &self,
    id: &str,
    new_email: &str,
  ) -> anyhow::Result<()> {
    self
      .db
      .execute(
        "UPDATE profiles SET email = ?1 WHERE id = ?2",
        params![new_email, id],
      )
      .await?;

    Ok(())
  }

  /// Maps a database row back into our clean ProfileModel.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<ProfileModel> {
    Ok(ProfileModel {
      id: uuid::Uuid::parse_str(&row.get::<String>(0)?)?,
      email: row.get::<String>(1)?,
      created_at: {
        let created_at_str: String = row.get(2)?;
        created_at_str.parse::<DateTime<Utc>>()?
      },
    })
  }
}
