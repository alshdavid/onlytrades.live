use uhttp::*;

use crate::ctx::Ctx;

#[rustfmt::skip]
pub async fn api_health_any(_req: uhttp::Request, mut res: uhttp::Response, _ctx: Ctx) -> uhttp::Result<()> {
  res.header().set("Content-Type", "application/json").await?;
  res.write_all(b"{ \"status\": \"HEALTHY\" }").await?;
  Ok(())
}
