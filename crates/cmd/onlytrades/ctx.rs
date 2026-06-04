use std::sync::Arc;

use kit_auth0::AccessTokenClaims;
use kit_ctrader::CTraderRestClient;
use platform_bot_service::BotService;
use platform_ctrader_service::CTraderService;
use platform_data_view_turso::ProfileDataView;
use platform_log_service::LogService;
use platform_models::ProfileModel;
use platform_repository_turso::BotRepository;
use platform_repository_turso::CtraderAccountRepository;
use platform_repository_turso::CtraderTokenRepository;
use platform_repository_turso::DeploymentRepository;
use platform_repository_turso::IdentityRepository;
use platform_repository_turso::ProfileRepository;
use platform_repository_turso::TriggerRepository;

use super::env::Env;

#[derive(Clone)]
pub struct Ctx {
  pub env: Arc<Env>,
  pub log_service: Arc<LogService>,
  pub ctrader_rest_client: Arc<CTraderRestClient>,
  pub ctrader_service: Arc<CTraderService>,
  pub bot_service: Arc<BotService>,
  // Database
  pub profile_repository: Arc<ProfileRepository>,
  pub trigger_repository: Arc<TriggerRepository>,
  pub identity_repository: Arc<IdentityRepository>,
  pub ctrader_token_repository: Arc<CtraderTokenRepository>,
  pub ctrader_account_repository: Arc<CtraderAccountRepository>,
  pub bot_repository: Arc<BotRepository>,
  pub deployments_repository: Arc<DeploymentRepository>,
  pub profile_data_view: Arc<ProfileDataView>,
  // Request specific
  pub access_token: AccessTokenClaims,
  pub profile: ProfileModel,
}
