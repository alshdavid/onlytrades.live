use crate::ctx::Ctx;

pub async fn api_auth_login_get(
  _req: uhttp::Request,
  res: uhttp::Response,
  Ctx { env, .. }: Ctx,
) -> uhttp::Result<()> {
  // if let Some(cookie_header) = req.headers().get("Cookie") {
  //   let cookie_str = cookie_header.to_str().unwrap_or("");
  //   let mut access_token = None::<String>;

  //   for c in Cookie::split_parse(cookie_str) {
  //     match c {
  //       Ok(cookie) => {
  //         if cookie.name() == "access_token" {
  //           access_token = Some(cookie.value().to_string());
  //           break;
  //         }
  //       }
  //       _ => continue,
  //     }
  //   }

  //   if let Some(access_token) = access_token
  //     && auth0::validate_token(&access_token).await.is_ok()
  //   {
  //     res.header().set("Location", "/dashboard").await?;
  //     res
  //       .write_head(uhttp::StatusCode::TEMPORARY_REDIRECT)
  //       .await?;
  //     return Ok(());
  //   };
  // }

  let redirect = kit_auth0::generate_login_url(
    &env.app_origin,
    "TEST_STATE",
    &env.auth_domain,
    &env.auth_zero_client_id,
    &env.auth_callback_url,
  );

  res.header().set("Location", &redirect).await?;
  res
    .write_head(uhttp::StatusCode::TEMPORARY_REDIRECT)
    .await?;
  Ok(())
}
