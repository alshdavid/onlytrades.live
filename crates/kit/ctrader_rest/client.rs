use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::TimeDelta;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
  #[serde(rename = "accessToken")]
  pub access_token: String,
  #[serde(rename = "refreshToken")]
  pub refresh_token: String,
  #[serde(rename = "expiresIn")]
  pub expires_in: u64,
  #[serde(rename = "tokenType")]
  pub token_type: String,
  #[serde(skip)]
  pub expires_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradingAccountsResponse {
  pub data: Vec<TradingAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingAccount {
  pub account_id: i64,
  pub account_number: u64,
  pub live: bool,
  pub broker_name: String,
  pub broker_title: String,
  pub deposit_currency: String,
  pub trader_registration_timestamp: u64,
  pub trader_account_type: String,
  pub leverage: u32,
  pub leverage_in_cents: u64,
  pub balance: i64,
  pub deleted: bool,
  pub account_status: String,
  pub swap_free: bool,
  pub money_digits: u32,
}

pub struct CTraderRestClientOptions {
  pub hostname: String,
  pub client_id: String,
  pub client_secret: String,
}

#[derive(Clone)]
pub struct CTraderRestClient {
  client_id: String,
  client_secret: String,
  hostname: String,
}

impl CTraderRestClient {
  pub fn new(options: CTraderRestClientOptions) -> Self {
    Self {
      hostname: options.hostname,
      client_id: options.client_id,
      client_secret: options.client_secret,
    }
  }

  pub fn oauth_login(&self) -> String {
    self.url_code()
  }

  pub async fn oauth_get_token(
    &self,
    code: &str,
  ) -> anyhow::Result<TokenResponse> {
    let client = reqwest::Client::new();

    let response = client
      .post(self.url_token(code))
      .header("Accept", "application/json")
      .header("Content-Type", "application/json")
      .send()
      .await?;

    let mut response = response.json::<TokenResponse>().await?;
    response.expires_at = expires_in_to_timestamp(response.expires_in);
    Ok(response)
  }

  pub async fn oauth_refresh(
    &self,
    refresh_token: &str,
  ) -> anyhow::Result<TokenResponse> {
    let client = reqwest::Client::new();

    let response = client
      .post(self.refresh_token(refresh_token))
      .header("Accept", "application/json")
      .header("Content-Type", "application/json")
      .send()
      .await?;

    let mut response = response.json::<TokenResponse>().await?;
    response.expires_at = expires_in_to_timestamp(response.expires_in);
    Ok(response)
  }

  pub async fn get_accounts(
    &self,
    access_token: &str,
  ) -> anyhow::Result<Vec<TradingAccount>> {
    let client = reqwest::Client::new();

    let response = client
      .get(format!(
        "https://api.spotware.com/connect/tradingaccounts?access_token={}",
        access_token
      ))
      .header("Accept", "application/json")
      .send()
      .await?;

    Ok(response.json::<TradingAccountsResponse>().await?.data)
  }

  pub fn has_expired(
    &self,
    expires_at: &chrono::DateTime<Utc>,
  ) -> bool {
    let buffer_threshold = Utc::now() + TimeDelta::seconds(300);
    *expires_at <= buffer_threshold
  }

  pub fn has_expired_raw(
    &self,
    timestamp: u64,
  ) -> bool {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();

    now >= timestamp.saturating_sub(300)
  }

  fn redirect_uri(&self) -> String {
    format!("{}/api/ctrader/callback", self.hostname)
  }

  fn url_code(&self) -> String {
    format!(
      "https://id.ctrader.com/my/settings/openapi/grantingaccess/?scope={}&product=web&client_id={}&redirect_uri={}",
      "trading",
      self.client_id,
      self.redirect_uri(),
    )
  }

  fn url_token(
    &self,
    code: &str,
  ) -> String {
    format!(
      "https://openapi.ctrader.com/apps/token?grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
      code,
      self.redirect_uri(),
      self.client_id,
      self.client_secret,
    )
  }

  #[allow(unused)]
  fn url_logout(&self) -> String {
    format!(
      "https://openapi.ctrader.com/apps/logout?&redirect_uri={}&client_id={}&client_secret={}",
      self.redirect_uri(),
      self.client_id,
      self.client_secret,
    )
  }

  fn refresh_token(
    &self,
    refresh_token: &str,
  ) -> String {
    format!(
      "https://openapi.ctrader.com/apps/token?grant_type=refresh_token&refresh_token={}&client_id={}&client_secret={}",
      refresh_token, self.client_id, self.client_secret
    )
  }
}

fn expires_in_to_timestamp(expires_in: u64) -> u64 {
  let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  now + expires_in
}
