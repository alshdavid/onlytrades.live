use uhttp::*;

use crate::ctx::Ctx;

pub async fn api_auth_refresh_get(
  _req: uhttp::Request,
  mut res: uhttp::Response,
  _ctx: Ctx,
) -> uhttp::Result<()> {
  res.header().set("Content-Type", "application/json").await?;

  res.write_all(b"{ \"status\": \"HEALTHY\" }").await?;
  Ok(())
}
