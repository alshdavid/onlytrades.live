// Demo details
static AUTH_DOMAIN: &str = "auth.onlytrades.live";
static AUTH_CALLBACK_URL: &str = "/api/auth/callback";
static APP_ORIGIN: &str = "http://localhost:4200";
static APP_PORT: &str = "4200";
static APP_COMPRESS: bool = false;

use std::env;

#[derive(Debug, Clone)]
pub struct Env {
  pub app_origin: String,
  pub app_port: String,
  pub ctrader_client_id: String,
  pub ctrader_client_secret: String,
}

impl Env {
  pub fn from_env() -> anyhow::Result<Self> {
    Ok(Self {
      app_origin: env::var("APP_ORIGIN").unwrap_or(APP_ORIGIN.to_string()),
      app_port: env::var("APP_PORT").unwrap_or(APP_PORT.to_string()),
      ctrader_client_id: env::var("CTRADER_CLIENT_ID")?,
      ctrader_client_secret: env::var("CTRADER_CLIENT_SECRET")?,
    })
  }
}
