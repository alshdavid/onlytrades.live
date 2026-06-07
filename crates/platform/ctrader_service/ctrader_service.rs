use std::sync::Arc;

use chrono::TimeDelta;
use chrono::Utc;
use kit_ctrader_rest::client::CTraderRestClient;
use kit_ctrader_rest::client::CTraderRestClientOptions;
use platform_models::CtraderAccountModel;
use platform_models::CtraderTokenModel;
use platform_repository_turso::CtraderAccountRepository;
use platform_repository_turso::CtraderTokenRepository;
use uuid::Uuid;

// use super::CTraderConnectionManager;

pub struct CTraderService {
  // pub(super) manager: CTraderConnectionManager,
  pub(super) rest_client: CTraderRestClient,
  pub(super) ctrader_account_repository: Arc<CtraderAccountRepository>,
  pub(super) ctrader_token_repository: Arc<CtraderTokenRepository>,
}

impl CTraderService {
  pub fn new(
    service_hostname: &str,
    ctrader_client_id: &str,
    ctrader_client_secret: &str,
    ctrader_account_repository: &Arc<CtraderAccountRepository>,
    ctrader_token_repository: &Arc<CtraderTokenRepository>,
  ) -> Self {
    Self {
      // manager: CTraderConnectionManager::new(ctrader_client_id, ctrader_client_secret),
      rest_client: CTraderRestClient::new(CTraderRestClientOptions {
        hostname: service_hostname.to_string(),
        client_id: ctrader_client_id.to_string(),
        client_secret: ctrader_client_secret.to_string(),
      }),
      ctrader_account_repository: Arc::clone(ctrader_account_repository),
      ctrader_token_repository: Arc::clone(ctrader_token_repository),
    }
  }

  // /// Gets an existing socket connection or creates a new connection.
  // /// Will validate that the profile owns the account and
  // /// will refresh the access_token if it has expired
  // pub(super) async fn get_or_connect_socket(
  //   &self,
  //   connection_name: &str,
  //   profile_id: &Uuid,
  //   account_id: &i64,
  // ) -> anyhow::Result<(Arc<CTraderSocketClient>, CtraderAccountModel)> {
  //   let accounts = self
  //     .ctrader_account_repository
  //     .find_by_profile_id(profile_id)
  //     .await?;

  //   // Validate that the account belongs to the profile
  //   let Some(account) = accounts.into_iter().find(|a| &a.account_id == account_id) else {
  //     anyhow::bail!("Account {} not in profile {}", account_id, profile_id)
  //   };

  //   let Some(tokens) = self
  //     .ctrader_token_repository
  //     .get_tokens_for_profile(profile_id)
  //     .await?
  //   else {
  //     anyhow::bail!("No ctrader tokens for {}", profile_id)
  //   };

  //   let mut ctrader_access_token = tokens.access_token;

  //   // Refresh the token if it has expired
  //   if self
  //     .rest_client
  //     .has_expired(&tokens.access_token_expires_at)
  //   {
  //     let ctrader_tokens = self
  //       .rest_client
  //       .oauth_refresh(&tokens.refresh_token)
  //       .await?;

  //     self
  //       .ctrader_token_repository
  //       .upsert(
  //         &CtraderTokenModel {
  //           token_id: Uuid::now_v7(),
  //           refresh_token: ctrader_tokens.refresh_token.clone(),
  //           access_token: ctrader_tokens.access_token.clone(),
  //           access_token_expires_at: Utc::now()
  //             + TimeDelta::seconds(ctrader_tokens.expires_in as i64),
  //         },
  //         profile_id,
  //       )
  //       .await?;

  //     ctrader_access_token = ctrader_tokens.access_token;
  //   }

  //   let conn = self
  //     .manager
  //     .get_or_connect(
  //       connection_name,
  //       &account.account_id,
  //       account.live,
  //       &ctrader_access_token,
  //     )
  //     .await?;

  //   Ok((conn, account))
  // }

  // #[deprecated]
  // #[allow(unused)]
  // pub async fn profile_owns_account(
  //   &self,
  //   profile_id: &Uuid,
  //   account_id: &i64,
  // ) -> anyhow::Result<bool> {
  //   let accounts = self
  //     .ctrader_account_repository
  //     .find_by_profile_id(profile_id)
  //     .await?;

  //   // Validate that the account belongs to the profile
  //   let Some(account) = accounts.into_iter().find(|a| &a.account_id == account_id) else {
  //     anyhow::bail!("Account {} not in profile {}", account_id, profile_id)
  //   };

  //   let Some(tokens) = self
  //     .ctrader_token_repository
  //     .get_tokens_for_profile(profile_id)
  //     .await?
  //   else {
  //     return Ok(false);
  //   };

  //   Ok(true)
  // }
}
