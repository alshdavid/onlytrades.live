use uhttp::Request;
use uhttp::Response;
use uhttp::StatusCode;

use crate::ctx::Ctx;

pub async fn with_secret(
  req: Request,
  res: Response,
  ctx: Ctx,
) -> uhttp::Result<Option<(Request, Response, Ctx)>> {
  let Ctx { env, .. } = &ctx;

  if let Some(api_secret) = &env.api_secret {
    let Some(header) = req.headers().get("X-Secret-6BoVt8Tkj5MW") else {
      res.write_head(StatusCode::BAD_GATEWAY).await?;
      return Ok(None);
    };

    if header.to_str().unwrap_or_default() != api_secret.as_str() {
      res.write_head(StatusCode::BAD_GATEWAY).await?;
      return Ok(None);
    }
  }

  Ok(Some((req, res, ctx)))
}
