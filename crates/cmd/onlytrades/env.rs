// Demo details
static AUTH_DOMAIN: &str = "auth.onlytrades.live";
static AUTH_CALLBACK_URL: &str = "/api/auth/callback";
static APP_ORIGIN: &str = "http://localhost:4200";
static APP_PORT: &str = "4200";
static APP_COMPRESS: bool = false;

use std::env;
use std::sync::Arc;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Env {
  pub auth_zero_client_id: String,
  pub auth_zero_client_secret: String,
  pub auth_domain: String,
  pub auth_callback_url: String,
  pub app_origin: String,
  pub app_port: String,
  pub compress: bool,
  pub ctrader_client_id: String,
  pub ctrader_client_secret: String,
  pub api_secret: Option<Arc<String>>,
  pub plugin_sandbox: Arc<String>,
}

impl Env {
  pub fn from_env() -> anyhow::Result<Self> {
    Ok(Self {
      auth_zero_client_id: env::var("AUTH_ZERO_CLIENT_ID")
        .context("Missing AUTH_ZERO_CLIENT_ID")?,
      auth_zero_client_secret: env::var("AUTH_ZERO_CLIENT_SECRET")
        .context("Missing AUTH_ZERO_CLIENT_SECRET")?,
      auth_domain: env::var("AUTH_DOMAIN").unwrap_or(AUTH_DOMAIN.to_string()),
      auth_callback_url: env::var("AUTH_CALLBACK_URL").unwrap_or(AUTH_CALLBACK_URL.to_string()),
      app_origin: env::var("APP_ORIGIN").unwrap_or(APP_ORIGIN.to_string()),
      app_port: env::var("APP_PORT").unwrap_or(APP_PORT.to_string()),
      compress: env::var("APP_COMPRESS")
        .map(|v| v == "true")
        .unwrap_or(APP_COMPRESS),
      ctrader_client_id: env::var("CTRADER_CLIENT_ID").context("Missing CTRADER_CLIENT_ID")?,
      ctrader_client_secret: env::var("CTRADER_CLIENT_SECRET")
        .context("Missing CTRADER_CLIENT_SECRET")?,
      api_secret: match env::var("API_SECRET") {
        Ok(api_secret) => Some(Arc::new(api_secret)),
        Err(_) => None,
      },
      plugin_sandbox: Arc::new(env::var("PLUGIN_SANDBOX").unwrap_or(String::from("none"))),
    })
  }
}
