use std::collections::HashMap;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use dashmap::DashMap;
use platform_ctrader_service::CTraderService;
use platform_log_service::LogService;
use platform_models::BotModel;
use platform_models::DeploymentModel;
use platform_plugins::deno::DenoPlugin;
use platform_process::DenoProcessOptions;
use platform_process::SandboxType;
use platform_repository_turso::BotRepository;
use platform_repository_turso::DeploymentRepository;
use uuid::Uuid;

#[derive(Debug)]
struct RunningDeployment {
  created_at: DateTime<Utc>,
  profile_id: Uuid,
  deployment_id: Uuid,
  bot_id: Uuid,
  plugin: DenoPlugin,
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
  sandbox_type: SandboxType,
  bot_repository: Arc<BotRepository>,
  deployments_repository: Arc<DeploymentRepository>,
  ctrader_service: Arc<CTraderService>,
  log_service: Arc<LogService>,
  running: Arc<DashMap<Uuid, RunningDeployment>>,
}

impl BotService {
  pub fn new(
    plugin_sandbox: &SandboxType,
    bot_repository: &Arc<BotRepository>,
    deployments_repository: &Arc<DeploymentRepository>,
    ctrader_service: &Arc<CTraderService>,
    log_service: &Arc<LogService>,
  ) -> anyhow::Result<Self> {
    Ok(Self {
      sandbox_type: plugin_sandbox.clone(),
      bot_repository: Arc::clone(bot_repository),
      deployments_repository: Arc::clone(deployments_repository),
      ctrader_service: Arc::clone(ctrader_service),
      log_service: Arc::clone(log_service),
      running: Arc::new(DashMap::new()),
    })
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
    let code = String::from_utf8(bot.handler.clone())?;
    let plugin = DenoPlugin::new(
      code.as_str(),
      DenoProcessOptions {
        network: true,
        read_only_fs: true,
        memory: 1024 * 128,
        cpus: 1,
        volumes: std::collections::HashMap::new(),
        env: HashMap::new(),
        sandbox: self.sandbox_type.clone(),
      },
      &bot.profile_id,
      &deployment.account_id,
      &self.ctrader_service,
    )
    .await?;

    let log_key = format!("deployments:{}", deployment.id);

    tokio::task::spawn({
      let mut rx = plugin.instance.stdout().await;
      let log_service = Arc::clone(&self.log_service);
      let log_key = log_key.clone();

      async move {
        while let Some(line) = rx.recv().await {
          log_service.info(&log_key, &line);
        }
      }
    });

    tokio::task::spawn({
      let mut rx = plugin.instance.stderr().await;
      let log_service = Arc::clone(&self.log_service);
      let log_key = log_key.clone();

      async move {
        while let Some(line) = rx.recv().await {
          log_service.error(&log_key, &line);
        }
      }
    });

    tokio::task::spawn({
      let mut rx = plugin.instance.exited().await;
      let log_service = Arc::clone(&self.log_service);
      let log_key = log_key.clone();
      let running = Arc::clone(&self.running);
      let deployment_id = deployment.id;

      async move {
        while rx.recv().await.is_some() {
          log_service.info(&log_key, "Process Exited");

          if let Some(mut dep) = running.get_mut(&deployment_id) {
            (dep.alive) = false;
          };
        }
      }
    });

    self.running.insert(
      deployment.id,
      RunningDeployment {
        created_at: Utc::now(),
        profile_id: bot.profile_id,
        plugin,
        deployment_id: deployment.id,
        bot_id: bot.id,
        alive: true,
      },
    );

    self.log_service.info(&log_key, "Process Started");

    Ok(())
  }

  pub fn stop_deployment(
    &self,
    deployment_id: &Uuid,
  ) {
    if let Some((_id, meta)) = self.running.remove(deployment_id) {
      meta.plugin.instance.send_kill();
    }
  }
}
