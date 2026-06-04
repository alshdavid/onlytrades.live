use platform_models::IdentityModel;
use serde::Serialize;
use uhttp::AsyncWriteExt;
use uuid::Uuid;

use crate::ctx::Ctx;

#[derive(Debug, Default, Serialize)]
struct ApiAuthMeGetResponse {
  id: Uuid,
  email: String,
  identities: Vec<IdentityModel>,
}

pub async fn api_auth_me_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  Ctx {
    profile_data_view,
    access_token,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(profile) = profile_data_view
    .find_by_identity(&access_token.sub)
    .await?
  else {
    res.write_head(uhttp::StatusCode::UNAUTHORIZED).await?;
    return Ok(());
  };

  let response = ApiAuthMeGetResponse {
    id: profile.id,
    email: profile.email,
    identities: profile.identities,
  };

  let body = serde_json::to_string_pretty(&response).unwrap();

  res.header().add("Content-Type", "application/json").await?;
  res.write_head(uhttp::StatusCode::OK).await?;
  res.write_all(body.as_bytes()).await?;

  Ok(())
}
