use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::decode_header;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessTokenClaims {
  pub sub: String,
  pub aud: serde_json::Value,
  pub iss: String,
  pub exp: usize,
  pub scope: Option<String>,
  pub email: Option<String>,
  pub email_verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Jwk {
  kid: String,
  n: String,
  e: String,
}

#[derive(Debug, Deserialize)]
struct Jwks {
  keys: Vec<Jwk>,
}

pub async fn validate_token(
  auth_domain: &str,
  token: &str,
) -> anyhow::Result<AccessTokenClaims> {
  let auth0_domain = format!("https://{}/", auth_domain);
  let api_audience = "onlytrades.api";

  let header = decode_header(token)?;
  let kid = header
    .kid
    .ok_or_else(|| anyhow::anyhow!("No 'kid' in token header"))?;

  let jwks_url = format!("{}.well-known/jwks.json", auth0_domain);
  let jwks: Jwks = reqwest::get(jwks_url).await?.json().await?;

  let jwk = jwks
    .keys
    .iter()
    .find(|k| k.kid == kid)
    .ok_or_else(|| anyhow::anyhow!("No matching key found in JWKS"))?;

  let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?;

  let mut validation = Validation::new(Algorithm::RS256);
  validation.set_audience(&[api_audience]);
  validation.set_issuer(&[auth0_domain]);
  // Note: jsonwebtoken handles 'exp' (expiration) check by default

  let token_data = decode::<AccessTokenClaims>(token, &decoding_key, &validation)
    .map_err(|e| anyhow::anyhow!("Validation failed: {}", e))?;

  Ok(token_data.claims)
}
