use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
struct TokenRequest {
  grant_type: String,
  client_id: String,
  client_secret: String,
  code: String,
  redirect_uri: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
  pub access_token: String,
  pub refresh_token: Option<String>,
  pub id_token: String,
  pub token_type: String,
  pub expires_in: u64,
}

pub async fn exchange_code_for_tokens(
  code: &str,
  auth_zero_client_id: &str,
  auth_zero_client_secret: &str,
  auth_callback_url: &str,
  auth_domain: &str,
  app_origin: &str,
) -> Result<TokenResponse, reqwest::Error> {
  let client = reqwest::Client::new();

  let params = TokenRequest {
    grant_type: "authorization_code".to_string(),
    client_id: auth_zero_client_id.to_string(),
    client_secret: auth_zero_client_secret.to_string(),
    code: code.to_string(),
    redirect_uri: format!("{}{}", app_origin, auth_callback_url),
  };

  let res = client
    .post(format!("https://{}/oauth/token", auth_domain))
    .json(&params)
    .send()
    .await?
    .json::<TokenResponse>()
    .await?;

  Ok(res)
}
