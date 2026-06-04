// https://auth.onlytrades.live/authorize
//  ?response_type=code
//  &client_id=ChY4kwQq9QcA7VXlMJN9kQpGFlOo9vXv
//  &redirect_uri=http://localhost:9000/api/auth/callback
//  &scope=openid profile email
//  &state=RANDOM_STATE_STRING

use url::Url;

pub fn generate_logout_url(
  app_origin: &str,
  path: Option<String>,
  auth_domain: &str,
  auth_zero_client_id: &str,
) -> String {
  let mut url = Url::parse(&format!("https://{}/v2/logout", auth_domain))
    .expect("Failed to parse logout base URL");

  let mut return_to = app_origin.to_string();
  if let Some(path) = path {
    return_to = format!("{}{}", return_to, path);
  }

  url
    .query_pairs_mut()
    .append_pair("client_id", auth_zero_client_id)
    .append_pair("returnTo", &return_to);

  url.to_string()
}
