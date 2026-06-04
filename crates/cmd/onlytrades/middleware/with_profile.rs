use uhttp::Request;
use uhttp::Response;
use uhttp::StatusCode;

use crate::ctx::Ctx;

pub async fn with_profile(
  req: Request,
  res: Response,
  mut ctx: Ctx,
) -> uhttp::Result<Option<(Request, Response, Ctx)>> {
  let Ctx {
    profile_repository,
    identity_repository,
    access_token,
    profile,
    ..
  } = &mut ctx;

  let Some(identity) = identity_repository.find_by_sub(&access_token.sub).await? else {
    res.write_head(StatusCode::FORBIDDEN).await?;
    return Ok(None);
  };

  let Some(found_profile) = profile_repository.find_by_id(&identity.profile_id).await? else {
    res.write_head(StatusCode::FORBIDDEN).await?;
    return Ok(None);
  };

  *profile = found_profile;

  Ok(Some((req, res, ctx)))
}
