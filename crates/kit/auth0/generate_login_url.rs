// https://auth.onlytrades.live/authorize
//  ?response_type=code
//  &client_id=ChY4kwQq9QcA7VXlMJN9kQpGFlOo9vXv
//  &redirect_uri=http://localhost:9000/api/auth/callback
//  &scope=openid profile email
//  &state=RANDOM_STATE_STRING

use url::Url;

pub fn generate_login_url(
  host: &str,
  state: &str,
  auth_domain: &str,
  auth_zero_client_id: &str,
  auth_callback_url: &str,
) -> String {
  let mut url =
    Url::parse(&format!("https://{}/authorize", auth_domain)).expect("Failed to parse base URL");

  url
    .query_pairs_mut()
    .append_pair("response_type", "code")
    .append_pair("client_id", auth_zero_client_id)
    .append_pair("redirect_uri", &format!("{}{}", host, auth_callback_url))
    .append_pair("scope", "openid profile email offline_access")
    .append_pair("audience", "onlytrades.api")
    .append_pair("state", state);

  url.to_string()
}
