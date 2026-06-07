use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use dashmap::DashMap;
use kit_ctrader_rest::client::CTraderRestClient;
use kit_ctrader_socket::AccountAuthReq;
use kit_ctrader_socket::ApplicationAuthReq;
use kit_ctrader_socket::CTraderRequestType;
use kit_ctrader_socket::connection::CTraderConnection;
use kit_ctrader_socket::connection::CTraderConnectionOptions;
use platform_ctrader_service::CTraderService;
use platform_log_service::LogService;
use platform_models::BotModel;
use platform_models::DeploymentModel;
use platform_process::GenericProcessOptions;
use platform_process::IpcTcpInstance;
// use platform_plugins::deno::DenoPlugin;
// use platform_process::DenoProcessOptions;
use platform_process::SandboxType;
use platform_process::create_temp_file;
use platform_repository_turso::BotRepository;
use platform_repository_turso::CtraderAccountRepository;
use platform_repository_turso::CtraderTokenRepository;
use platform_repository_turso::DeploymentRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum BotSandboxType {
  None,
  Podman,
}

impl TryFrom<&str> for BotSandboxType {
  type Error = io::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "none" => Ok(Self::None),
      "podman" => Ok(Self::Podman),
      _ => Err(io::Error::other("Unknown error")),
    }
  }
}

#[derive(Debug)]
struct RunningDeployment {
  created_at: DateTime<Utc>,
  profile_id: Uuid,
  deployment_id: Uuid,
  bot_id: Uuid,
  process: IpcTcpInstance,
  alive: bool,
}

pub struct DeploymentMeta {
  pub deployment_id: Uuid,
  pub bot_id: Uuid,
  pub profile_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub alive: bool,
}

pub struct BotService {
  ctrader_client_id: String,
  ctrader_client_secret: String,
  ctrader_token_repository: Arc<CtraderTokenRepository>,
  ctrader_account_repository: Arc<CtraderAccountRepository>,
  ctrader_rest: Arc<CTraderRestClient>,
  sandbox_type: BotSandboxType,
  bot_repository: Arc<BotRepository>,
  deployments_repository: Arc<DeploymentRepository>,
  ctrader_service: Arc<CTraderService>,
  log_service: Arc<LogService>,
  running: Arc<DashMap<Uuid, RunningDeployment>>,
}

impl BotService {
  pub fn new(
    plugin_sandbox: &BotSandboxType,
    bot_repository: &Arc<BotRepository>,
    deployments_repository: &Arc<DeploymentRepository>,
    ctrader_service: &Arc<CTraderService>,
    log_service: &Arc<LogService>,
  ) -> anyhow::Result<Self> {
    todo!()
    // Ok(Self {
    //   sandbox_type: plugin_sandbox.clone(),
    //   bot_repository: Arc::clone(bot_repository),
    //   deployments_repository: Arc::clone(deployments_repository),
    //   ctrader_service: Arc::clone(ctrader_service),
    //   log_service: Arc::clone(log_service),
    //   running: Arc::new(DashMap::new()),
    // })
  }

  pub fn get_all(&self) -> Vec<DeploymentMeta> {
    let mut deployments = Vec::<DeploymentMeta>::new();

    for deployment in self.running.iter() {
      deployments.push(DeploymentMeta {
        deployment_id: deployment.deployment_id,
        bot_id: deployment.bot_id,
        profile_id: deployment.profile_id,
        created_at: deployment.created_at,
        alive: deployment.alive,
      })
    }

    deployments
  }

  pub async fn bootstrap(&self) -> anyhow::Result<()> {
    let bots_all = self.bot_repository.get_all().await?;
    let mut bots = HashMap::<Uuid, BotModel>::new();
    for bot in bots_all {
      bots.insert(bot.id, bot);
    }

    let deployments = self.deployments_repository.get_all().await?;

    for deployment in deployments {
      if !deployment.active {
        continue;
      }

      let Some(bot) = bots.get(&deployment.bot_id) else {
        eprintln!("Cannot find bot for deployment");
        continue;
      };

      self.start_deployment(&deployment, bot).await?;
    }

    Ok(())
  }

  pub async fn restart_deployments(
    &self,
    bot_id: &Uuid,
  ) -> anyhow::Result<()> {
    let mut to_restart = Vec::<(BotModel, DeploymentModel)>::new();

    for deployment in self.running.iter() {
      if &deployment.bot_id != bot_id {
        continue;
      }

      let Some(bot) = self.bot_repository.get_by_id(&deployment.bot_id).await? else {
        continue;
      };

      let Some(deployment) = self
        .deployments_repository
        .get_by_id(&deployment.deployment_id)
        .await?
      else {
        continue;
      };

      to_restart.push((bot, deployment));
    }

    for (bot, deployment) in to_restart {
      self.stop_deployment(&deployment.id);
      self.start_deployment(&deployment, &bot).await?;
    }

    Ok(())
  }

  pub async fn start_deployment(
    &self,
    deployment: &DeploymentModel,
    bot: &BotModel,
  ) -> anyhow::Result<()> {
    match bot.language.as_str() {
      "rust_v1" => {
        // let ctrader_tokens = self
        //   .ctrader_token_repository
        //   .get_tokens_for_profile(&bot.profile_id)
        //   .await?
        //   .unwrap();

        // let account = self
        //   .ctrader_account_repository
        //   .find_by_profile_id(&bot.profile_id)
        //   .await
        //   .unwrap()
        //   .into_iter()
        //   .find(|a| a.account_id == deployment.account_id)
        //   .unwrap();

        // let conn = CTraderConnection::connect(CTraderConnectionOptions { live: account.live })
        //   .await
        //   .unwrap();

        // let mut rx = conn.subscribe().await;

        // conn
        //   .send(CTraderRequestType::ApplicationAuthReq(ApplicationAuthReq {
        //     client_msg_id: None,
        //     client_id: self.ctrader_client_id.clone(),
        //     client_secret: self.ctrader_client_secret.clone(),
        //   }))
        //   .await?;

        // rx.recv().await.unwrap().unwrap();

        // conn
        //   .send(CTraderRequestType::AccountAuthReq(AccountAuthReq {
        //     client_msg_id: None,
        //     ctid_trader_account_id: account.account_id.clone(),
        //     access_token: ctrader_tokens.access_token.clone(),
        //   }))
        //   .await?;

        // rx.recv().await.unwrap().unwrap();
        // drop(rx);

        // let cwd = &std::env::current_exe()
        //   .unwrap()
        //   .parent()
        //   .unwrap()
        //   .to_path_buf();

        // let temp_file = create_temp_file(&cwd, &deployment.id.to_string(), &bot.handler)
        //   .await
        //   .unwrap();

        // let bin_path = temp_file.path().to_str().unwrap().to_string();

        // let mut process = IpcTcpInstance::new(GenericProcessOptions {
        //   network: true,
        //   read_only_fs: true,
        //   memory: 1024 * 32,
        //   cpus: 100,
        //   volumes: HashMap::new(),
        //   env: HashMap::new(),
        //   sandbox: SandboxType::None {
        //     command: vec![bin_path],
        //   },
        // })
        // .await
        // .unwrap();

        // tokio::spawn({
        //   let tx = process.writer();
        //   let mut rx = conn.subscribe().await;
        //   async move {
        //     while let Some(res) = rx.recv().await {
        //       let message = serde_json::to_vec(&res).unwrap();
        //       let _ = tx.send(message);
        //     }
        //   }
        // });

        // tokio::spawn({
        //   let mut rx = process.reader()?;
        //   async move {
        //     while let Some(res) = rx.recv().await {
        //       let message = serde_json::from_slice::<CTraderRequestType>(&res).unwrap();
        //       dbg!(&message);
        //       conn.send(message).await.unwrap();
        //     }
        //   }
        // });

        // let mut stdout = process.stdout().await;
        // tokio::spawn(async move {
        //   while let Some(line) = stdout.recv().await {
        //     println!("STDOUT: {}", line)
        //   }
        // });

        // let mut stderr = process.stderr().await;
        // tokio::spawn(async move {
        //   while let Some(line) = stderr.recv().await {
        //     println!("STDERR: {}", line)
        //   }
        // });

        // self.running.insert(
        //   deployment.id,
        //   RunningDeployment {
        //     created_at: Utc::now(),
        //     profile_id: bot.profile_id,
        //     deployment_id: deployment.id,
        //     bot_id: bot.id,
        //     alive: true,
        //     process,
        //   },
        // );
      }
      _ => panic!(),
    };

    Ok(())
  }

  pub fn stop_deployment(
    &self,
    deployment_id: &Uuid,
  ) {
    // if let Some((_id, meta)) = self.running.remove(deployment_id) {
    //   meta.plugin.instance.send_kill();
    // }
  }
}
