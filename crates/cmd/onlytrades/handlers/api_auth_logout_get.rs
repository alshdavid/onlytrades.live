use cookie::Cookie;

use crate::ctx::Ctx;

pub async fn api_auth_logout_get(
  _req: uhttp::Request,
  res: uhttp::Response,
  Ctx { env, .. }: Ctx,
) -> uhttp::Result<()> {
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

  let logout_url = kit_auth0::generate_logout_url(
    &env.app_origin,
    None,
    &env.auth_domain,
    &env.auth_zero_client_id,
  );

  res.header().set("Location", &logout_url).await?;
  res
    .write_head(uhttp::StatusCode::TEMPORARY_REDIRECT)
    .await?;
  Ok(())
}
