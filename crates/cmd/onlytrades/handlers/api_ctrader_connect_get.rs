use uhttp::*;

use crate::ctx::Ctx;

pub async fn api_ctrader_connect_get(
  _req: uhttp::Request,
  res: uhttp::Response,
  Ctx {
    ctrader_rest_client,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  res
    .header()
    .set("Location", &ctrader_rest_client.oauth_login())
    .await?;
  res.write_head(StatusCode::TEMPORARY_REDIRECT).await?;
  Ok(())
}
