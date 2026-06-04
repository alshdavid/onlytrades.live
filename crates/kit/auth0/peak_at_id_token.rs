use base64::Engine;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct IdTokenClaims {
  pub sub: String, // User primary key
  pub aud: String,
  pub email: String,
  pub name: String,
  pub iss: String,
  pub exp: usize,
  pub email_verified: bool,
}

pub fn peek_at_id_token(token: &str) -> anyhow::Result<IdTokenClaims> {
  let parts: Vec<&str> = token.split('.').collect();
  if parts.len() != 3 {
    anyhow::bail!("Invalid JWT format");
  }

  let payload_b64 = parts[1];
  let decoded_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
    .decode(payload_b64)
    .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))?;

  let claims: IdTokenClaims = serde_json::from_slice(&decoded_bytes)
    .map_err(|e| anyhow::anyhow!("JSON parse failed: {}", e))?;

  Ok(claims)
}
