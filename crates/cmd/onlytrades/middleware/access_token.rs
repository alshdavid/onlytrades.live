use cookie::Cookie;
use kit_auth0::{self};
use uhttp::Request;
use uhttp::Response;
use uhttp::StatusCode;

use crate::ctx::Ctx;

pub async fn with_access_token(
  req: Request,
  res: Response,
  mut ctx: Ctx,
) -> uhttp::Result<Option<(Request, Response, Ctx)>> {
  let Ctx {
    env, access_token, ..
  } = &mut ctx;

  let Some(cookie_header) = req.headers().get("Cookie") else {
    res.write_head(uhttp::StatusCode::FORBIDDEN).await?;
    return Ok(None);
  };

  let cookie_str = cookie_header.to_str().unwrap_or("");

  if cookie_str.is_empty() {
    return Ok(None);
  }

  for c in Cookie::split_parse(cookie_str) {
    match c {
      Ok(cookie) => {
        if cookie.name() == "access_token" {
          let Ok(claims) = kit_auth0::validate_token(&env.auth_domain, cookie.value()).await else {
            break;
          };
          *access_token = claims;
          return Ok(Some((req, res, ctx)));
        }
      }
      _ => continue,
    }
  }

  // If validation fails, clear the cookies in the browser
  let mut access_cookie = Cookie::build("access_token").path("/api").build();
  let mut id_cookie = Cookie::build("id_token").path("/api/auth/me").build();
  let mut refresh_cookie = Cookie::build("refresh_token")
    .path("/api/auth/refresh") // Scope it to only the refresh endpoint for security
    .build();

  access_cookie.make_removal();
  id_cookie.make_removal();
  refresh_cookie.make_removal();

  res
    .header()
    .add("Set-Cookie", &access_cookie.to_string())
    .await?;
  res
    .header()
    .add("Set-Cookie", &id_cookie.to_string())
    .await?;
  res
    .header()
    .add("Set-Cookie", &refresh_cookie.to_string())
    .await?;

  res.write_head(StatusCode::FORBIDDEN).await?;

  Ok(None)
}
