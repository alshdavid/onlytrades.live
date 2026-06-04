use cookie::Cookie;
use cookie::SameSite;
use cookie::time::Duration;
use serde::Deserialize;

use crate::ctx::Ctx;

#[derive(Debug, Deserialize)]
struct Query {
  code: Option<String>,
  state: Option<String>,
}

pub async fn api_auth_callback_get(
  req: uhttp::Request,
  res: uhttp::Response,
  Ctx {
    env,
    identity_repository,
    profile_repository,
    ..
  }: Ctx,
) -> uhttp::Result<()> {
  let Some(query_str) = req.uri().query() else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let query = serde_urlencoded::from_str::<Query>(query_str).unwrap();

  let Some(_state) = query.state else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let Some(code) = query.code else {
    res.write_head(uhttp::StatusCode::BAD_REQUEST).await?;
    return Ok(());
  };

  let tokens = kit_auth0::exchange_code_for_tokens(
    &code,
    &env.auth_zero_client_id,
    &env.auth_zero_client_secret,
    &env.auth_callback_url,
    &env.auth_domain,
    &env.app_origin,
  )
  .await?;

  let claims = kit_auth0::peek_at_id_token(&tokens.id_token)?;

  if !claims.email_verified {
    let logout_url = kit_auth0::generate_logout_url(
      &env.app_origin,
      Some("/verify-email".to_string()),
      &env.auth_domain,
      &env.auth_zero_client_id,
    );
    res.header().set("Location", &logout_url).await?;
    res
      .write_head(uhttp::StatusCode::TEMPORARY_REDIRECT)
      .await?;
    return Ok(());
  }

  match profile_repository.find_by_email(&claims.email).await? {
    Some(profile) => {
      identity_repository
        .upsert(&claims.sub, &profile.id)
        .await
        .unwrap();
    }
    None => {
      let profile = profile_repository.create(&claims.email).await.unwrap();
      identity_repository
        .upsert(&claims.sub, &profile.id)
        .await
        .unwrap();
    }
  };

  let access_cookie = Cookie::build(("access_token", tokens.access_token))
    .path("/api")
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(60))
    .build();

  let id_cookie = Cookie::build(("id_token", tokens.id_token))
    .path("/api/auth/me")
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Lax)
    .max_age(Duration::minutes(60))
    .build();

  let refresh_cookie = Cookie::build(("refresh_token", tokens.refresh_token.unwrap_or_default()))
    .path("/api/auth/refresh") // Scope it to only the refresh endpoint for security
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Strict) // Stricter for the refresh token
    .max_age(Duration::days(30))
    .build();

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

  res.header().set("Location", "/dashboard").await?;
  res
    .write_head(uhttp::StatusCode::TEMPORARY_REDIRECT)
    .await?;
  Ok(())
}
