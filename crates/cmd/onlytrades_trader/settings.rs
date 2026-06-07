use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct SettingsData {
  ctrader_refresh_token: String,
  #[serde(default)]
  ctrader_access_token: String,
  #[serde(default)]
  expires_at: u64,
}

#[derive(Debug)]
pub struct Settings {
  path: PathBuf,
  inner: Mutex<SettingsData>,
}

impl Settings {
  pub fn from_file(path: &Path) -> anyhow::Result<Self> {
    let content = fs::read_to_string(path)?;
    let data: SettingsData = serde_json::from_str(&content)?;
    Ok(Self {
      path: path.to_owned(),
      inner: Mutex::new(data),
    })
  }

  pub fn get_access_token(&self) -> String {
    let data = self.inner.lock().unwrap();
    data.ctrader_access_token.clone()
  }

  pub async fn set_access_token(
    &self,
    token: String,
  ) {
    let mut data = self.inner.lock().unwrap();
    data.ctrader_access_token = token;
    Self::persist(&self.path, &data).await;
  }

  pub fn get_refresh_token(&self) -> String {
    let data = self.inner.lock().unwrap();
    data.ctrader_refresh_token.clone()
  }

  pub async fn set_refresh_token(
    &self,
    token: String,
  ) {
    let mut data = self.inner.lock().unwrap();
    data.ctrader_refresh_token = token;
    Self::persist(&self.path, &data).await;
  }

  pub fn get_expires_at(&self) -> u64 {
    let data = self.inner.lock().unwrap();
    data.expires_at.clone()
  }

  pub async fn set_expires_at(
    &self,
    expires_at: u64,
  ) {
    let mut data = self.inner.lock().unwrap();
    data.expires_at = expires_at;
    Self::persist(&self.path, &data).await;
  }

  async fn persist(
    path: &Path,
    data: &SettingsData,
  ) {
    let json = serde_json::to_string_pretty(data).expect("serialization should not fail");
    let _ = fs::write(path, &json);
  }
}
