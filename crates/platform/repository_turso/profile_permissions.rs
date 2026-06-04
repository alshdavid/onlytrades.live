use libsql::Connection;
use libsql::Row;
use libsql::params;
use platform_models::ProfilePermission;
use uuid::Uuid;

pub struct ProfilePermissionRepository {
  db: Connection,
}

impl ProfilePermissionRepository {
  pub fn new(db: &Connection) -> Self {
    Self { db: db.clone() }
  }

  pub async fn init(&self) -> anyhow::Result<()> {
    let sql = r#"
      CREATE TABLE IF NOT EXISTS profile_permissions (
        profile_id TEXT NOT NULL,
        role       TEXT NOT NULL,
        PRIMARY KEY (profile_id, role),
        FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
      );
    "#;

    self.db.execute(sql, ()).await?;
    Ok(())
  }

  /// Add a permission/role to a profile.
  /// Uses INSERT OR IGNORE since the composite key handles duplicates.
  pub async fn upsert(
    &self,
    profile_id: &Uuid,
    permission: ProfilePermission,
  ) -> anyhow::Result<()> {
    let sql = r#"
      INSERT OR IGNORE INTO profile_permissions (profile_id, role)
      VALUES (?1, ?2);
    "#;

    self
      .db
      .execute(sql, params![profile_id.to_string(), permission.to_string()])
      .await?;

    Ok(())
  }

  /// Finds all permissions for a given profile.
  pub async fn find_by_id(
    &self,
    id: &Uuid,
  ) -> anyhow::Result<Option<Vec<ProfilePermission>>> {
    let sql = "SELECT role FROM profile_permissions WHERE profile_id = ?1;";

    let mut rows = self.db.query(sql, params![id.to_string()]).await?;
    let mut permissions = Vec::new();

    while let Some(row) = rows.next().await? {
      if let Some(perm) = self.map_row(row)? {
        permissions.push(perm);
      }
    }

    if permissions.is_empty() {
      Ok(None)
    } else {
      Ok(Some(permissions))
    }
  }

  /// Delete a permission for a profile.
  pub async fn delete(
    &self,
    profile_id: &Uuid,
    permission: &str,
  ) -> anyhow::Result<()> {
    let sql = "DELETE FROM profile_permissions WHERE profile_id = ?1 AND role = ?2;";

    self
      .db
      .execute(sql, params![profile_id.to_string(), permission])
      .await?;

    Ok(())
  }

  /// Maps a single database row back to a single permission string.
  fn map_row(
    &self,
    row: Row,
  ) -> anyhow::Result<Option<ProfilePermission>> {
    // Index 0 because we only selected the `role` column in the query
    let role: String = row.get(0)?;
    Ok(Some(ProfilePermission::try_from(role)?))
  }
}
