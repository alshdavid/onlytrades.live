use platform_models::ProfilePermission;
use uhttp::AsyncWriteExt;

use crate::ctx::Ctx;

pub async fn api_admin_get(
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

  if !profile.permissions.contains(&ProfilePermission::Admin) {
    res.write_head(uhttp::StatusCode::FORBIDDEN).await?;
    return Ok(());
  }

  res.write_all(b"You are an admin").await?;
  res.write_head(uhttp::StatusCode::OK).await?;
  Ok(())
}
