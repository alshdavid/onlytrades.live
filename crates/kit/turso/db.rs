use std::path::Path;
use std::path::PathBuf;

use libsql::Builder;
use libsql::Connection;
use libsql::Database;
use std_ext::PathExt;

pub struct TursoDb {
  db: Database,
}

impl TursoDb {
  pub async fn new_remote(
    url: &str,
    token: &str,
  ) -> anyhow::Result<Self> {
    let db = Builder::new_remote(url.to_string(), token.to_string())
      .build()
      .await?;
    Ok(Self { db })
  }

  pub async fn new_local(filepath: &Path) -> anyhow::Result<Self> {
    let path_str = filepath.to_str().unwrap_or("local.db");
    let db = Builder::new_local(path_str).build().await?;
    Ok(Self { db })
  }

  pub async fn new_auto(
    name: &str,
    url: &str,
    token: &str,
  ) -> anyhow::Result<Self> {
    if url.is_empty() || url.starts_with('.') || url.starts_with('/') {
      let target = if url.is_empty() {
        let exe = std::env::current_exe()?;
        exe.try_parent()?.join(name)
      } else {
        PathBuf::from(url)
      };
      Self::new_local(&target).await
    } else {
      Self::new_remote(url, token).await
    }
  }

  pub fn connect(&self) -> anyhow::Result<Connection> {
    Ok(self.db.connect()?)
  }
}
