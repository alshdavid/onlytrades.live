mod bot;
mod client;
mod ctx;
mod env;
// mod handlers;
mod middleware;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use client::client_manifest;
use ctx::Ctx;
use env::Env;
use kit_auth0::AccessTokenClaims;
use kit_ctrader_rest::client::CTraderRestClient;
use kit_ctrader_rest::client::CTraderRestClientOptions;
// use kit_ctrader::CTraderRestClient;
// use kit_ctrader::CTraderRestClientOptions;
use kit_ctrader_socket::AccountAuthReq;
use kit_ctrader_socket::ApplicationAuthReq;
use kit_ctrader_socket::CTraderRequestType;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::connection::CTraderConnectionOptions;
use kit_std_ext::PathExt;
use kit_turso::TursoDb;
use platform_ctrader_service::CTraderService;
// use platform_ctrader_service::CTraderService;
use platform_data_view_turso::ProfileDataView;
use platform_log_service::LogService;
use platform_models::ProfilePermission;
use platform_process::TempFs;
use platform_process::create_temp_file;
use platform_repository_turso::BotRepository;
use platform_repository_turso::CtraderAccountRepository;
use platform_repository_turso::CtraderTokenRepository;
use platform_repository_turso::DeploymentRepository;
use platform_repository_turso::IdentityRepository;
use platform_repository_turso::ProfilePermissionRepository;
use platform_repository_turso::ProfileRepository;
use platform_repository_turso::TriggerRepository;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let env = Arc::new(Env::from_env()?);

  let cwd = &std::env::current_exe()?.try_parent()?.join("tmp");

  let tmp_fs = TempFs::new(&cwd).await?;

  // Database
  let turso_conn = TursoDb::new_auto("onlytrades.db", "", "").await?;
  let turso_logs_conn = TursoDb::new_auto("logs.db", "", "").await?;

  let conn = turso_conn.connect()?;
  let conn_logs = turso_logs_conn.connect()?;

  let profile_repository = Arc::new(ProfileRepository::new(&conn));
  let identity_repository = Arc::new(IdentityRepository::new(&conn));
  let profile_permission_repository = Arc::new(ProfilePermissionRepository::new(&conn));
  let ctrader_token_repository = Arc::new(CtraderTokenRepository::new(&conn));
  let ctrader_account_repository = Arc::new(CtraderAccountRepository::new(&conn));
  let trigger_repository = Arc::new(TriggerRepository::new(&conn));
  let bot_repository = Arc::new(BotRepository::new(&conn));
  let deployments_repository = Arc::new(DeploymentRepository::new(&conn));

  profile_repository.init().await?;
  identity_repository.init().await?;
  profile_permission_repository.init().await?;
  ctrader_account_repository.init().await?;
  ctrader_token_repository.init().await?;
  trigger_repository.init().await?;
  bot_repository.init().await?;
  deployments_repository.init().await?;

  let profile_data_view = Arc::new(ProfileDataView::new(&conn));

  // DEBUG
  {
    if let Some(profile) = profile_repository
      .find_by_email("alshdavid@gmail.com")
      .await?
    {
      profile_permission_repository
        .upsert(&profile.id, ProfilePermission::Admin)
        .await?;
    };
  };

  // Services
  let log_service = Arc::new(LogService::new(conn_logs).await?);

  // CTrader
  let ctrader_rest_client = Arc::new(CTraderRestClient::new(CTraderRestClientOptions {
    hostname: env.app_origin.clone(),
    client_id: env.ctrader_client_id.clone(),
    client_secret: env.ctrader_client_secret.clone(),
  }));

  let ctrader_service = Arc::new(CTraderService::new(
    &env.ctrader_client_id,
    &env.ctrader_client_secret,
    &ctrader_rest_client,
    &ctrader_token_repository,
    &ctrader_account_repository,
  ));

  let profile_id = Uuid::parse_str("019e955c-e10a-7b92-acd2-5bbb9d70c0a8")?;
  let account_id: i64 = std::env::var("CTRADER_ACCOUNT_ID")?.parse()?;

  let conn = ctrader_service
    .new_connection(&profile_id, &account_id)
    .await?;

  // println!("latency: {}", conn.latency().await?);

  bot::handler(conn, account_id).await?;

  tokio::time::sleep(Duration::from_secs(100)).await;

  // let ctrader_service = Arc::new(CTraderService::new(
  //   &env.app_origin,
  //   &env.ctrader_client_id,
  //   &env.ctrader_client_secret,
  //   &ctrader_account_repository,
  //   &ctrader_token_repository,
  // ));

  // let bot_service = Arc::new(BotService::new(
  //   &BotSandboxType::try_from(env.plugin_sandbox.as_str())?,
  //   &bot_repository,
  //   &deployments_repository,
  //   &ctrader_service,
  //   &log_service,
  // )?);
  // bot_service.bootstrap().await?;

  // let ctx = Ctx {
  //   env: env.clone(),
  //   log_service,
  //   ctrader_rest_client,
  //   ctrader_service,
  //   bot_service,
  //   // Database
  //   profile_repository,
  //   identity_repository,
  //   ctrader_token_repository,
  //   ctrader_account_repository,
  //   trigger_repository,
  //   bot_repository: Arc::clone(&bot_repository),
  //   deployments_repository: Arc::clone(&deployments_repository),
  //   // Read Only
  //   profile_data_view,
  //   // Request Specific
  //   access_token: AccessTokenClaims::default(),
  //   profile: ProfileModel::default(),
  // };

  // let mut app = uhttp::router::Router::new(ctx);

  // app.with_all(uhttp::middleware::logger_default);
  // app.with_all(middleware::with_secret);

  // app.any("/api/health", handlers::api_health_any);

  // // Auth
  // app.get("/api/auth/login", handlers::api_auth_login_get);
  // app.get("/api/auth/logout", handlers::api_auth_logout_get);
  // app.get("/api/auth/callback", handlers::api_auth_callback_get);
  // app.get("/api/auth/refresh", handlers::api_auth_refresh_get);
  // app
  //   .with(middleware::with_access_token)
  //   .get("/api/auth/me", handlers::api_auth_me_get);

  // // cTrader endpoints
  // app
  //   .with(middleware::with_access_token)
  //   .get("/api/ctrader/connect", handlers::api_ctrader_connect_get);
  // app
  //   .with(middleware::with_access_token)
  //   .get("/api/ctrader/callback", handlers::api_ctrader_callback_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/ctrader/accounts", handlers::api_ctrader_accounts_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .delete(
  //     "/api/ctrader/account/:id",
  //     handlers::api_ctrader_account_id_delete,
  //   );
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/ctrader/symbols", handlers::api_ctrader_symbols_get);

  // // Logs
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/logs/:id", handlers::api_logs_id_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/logs/stream/:id", handlers::api_logs_stream_id_get);

  // // Triggers
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/triggers", handlers::api_triggers_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/triggers/:id", handlers::api_triggers_get_id);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .delete("/api/triggers/:id", handlers::api_triggers_delete_id);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .post("/api/triggers", handlers::api_triggers_post);

  // // Triggers - Webhooks
  // app.get("/webhooks", handlers::webhooks_get);
  // app.post("/webhooks", handlers::webhooks_post);

  // // Bots
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .post("/api/bots", handlers::api_bots_post);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/bots", handlers::api_bots_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/bots/:id", handlers::api_bots_get_id);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .delete("/api/bots/:id", handlers::api_bots_delete_id);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .patch("/api/bots/:id", handlers::api_bots_patch_id);

  // // Bot deployments
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .post("/api/deployments", handlers::api_deployments_post);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/deployments", handlers::api_deployments_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .get("/api/deployments/:id", handlers::api_deployments_id_get);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .delete("/api/deployments/:id", handlers::api_deployments_id_delete);
  // app
  //   .with(middleware::with_access_token)
  //   .with(middleware::with_profile)
  //   .patch("/api/deployments/:id", handlers::api_deployments_id_patch);

  // // Admin
  // app
  //   .with(middleware::with_access_token)
  //   .get("/api/admin", handlers::api_admin_get);

  // // Client
  // for (path, dest) in client_manifest()? {
  //   app.get(&path, handlers::serve_static_get(&dest, env.compress)?);
  // }

  // app.get("/*", handlers::not_found_any);
  // app.get("/api/*", handlers::not_found_json_any);

  // println!("Listening on http://localhost:{}", env.app_port);
  // uhttp::http1::create_server(app.handler())
  //   .listen(format!("0.0.0.0:{}", env.app_port))
  //   .await?;

  Ok(())
}
