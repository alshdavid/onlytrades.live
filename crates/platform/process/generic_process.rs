use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxType {
  None { command: Vec<String> },
  Podman { image: String, command: Vec<String> },
}

enum ProcessType {
  Standard,
  Podman { name: String },
}

#[derive(Debug, Clone)]
pub struct GenericProcessOptions {
  pub network: bool,
  pub read_only_fs: bool,
  pub memory: usize, // kilobytes
  pub cpus: usize,   // 100 = 1 CPU core
  pub volumes: HashMap<PathBuf, PathBuf>,
  pub env: HashMap<String, String>,
  pub sandbox: SandboxType,
}

pub struct GenericProcess {
  tx_stdout: Arc<Mutex<Vec<UnboundedSender<String>>>>,
  tx_stderr: Arc<Mutex<Vec<UnboundedSender<String>>>>,
  tx_exited: Arc<Mutex<Vec<UnboundedSender<()>>>>,
  tx_kill: UnboundedSender<()>,
  running: Arc<AtomicBool>,
}

impl GenericProcess {
  pub async fn launch(options: GenericProcessOptions) -> anyhow::Result<Self> {
    let (process_type, mut cmd) = Self::command(&options).await?;

    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn()?;

    // if let Some(mut stdin) = child.stdin.take() {
    //   stdin.write_all(code.as_bytes()).await?;
    //   stdin.shutdown().await?;
    // }

    let tx_stdout = Arc::new(Mutex::new(Vec::<UnboundedSender<String>>::new()));

    if let Some(stdout) = child.stdout.take() {
      let tx_stdout = Arc::clone(&tx_stdout);

      tokio::task::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
          tx_stdout
            .lock()
            .await
            .retain(|tx| tx.send(line.clone()).is_ok());
        }
      });
    }

    let tx_stderr = Arc::new(Mutex::new(Vec::<UnboundedSender<String>>::new()));

    if let Some(stderr) = child.stderr.take() {
      let tx_stderr = Arc::clone(&tx_stderr);

      tokio::task::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
          tx_stderr
            .lock()
            .await
            .retain(|tx| tx.send(line.clone()).is_ok());
        }
      });
    }

    let (tx_kill, mut rx_kill) = tokio::sync::mpsc::unbounded_channel();
    let tx_exited = Arc::new(Mutex::new(Vec::<UnboundedSender<()>>::new()));
    let running = Arc::new(AtomicBool::new(true));

    tokio::task::spawn({
      let tx_exited = Arc::clone(&tx_exited);
      let running = Arc::clone(&running);

      async move {
        tokio::select! {
          _ = rx_kill.recv() => {},
          _ = child.wait() => {},
        };

        match process_type {
          ProcessType::Standard => {
            let _ = child.kill().await;
          }
          ProcessType::Podman { name } => {
            let _ = child.kill().await;
            let mut cmd = Command::new("podman");
            cmd.arg("stop").arg(name);
            cmd.stdin(std::process::Stdio::null());
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
            if let Ok(mut child) = cmd.spawn() {
              let _ = child.wait().await;
            };
          }
        }

        tx_exited.lock().await.retain(|tx| tx.send(()).is_ok());
        running.store(false, Ordering::Relaxed);
      }
    });

    Ok(Self {
      tx_stdout,
      tx_stderr,
      tx_kill,
      tx_exited,
      running,
    })
  }

  #[allow(unused)]
  pub fn running(&self) -> bool {
    self.running.load(Ordering::Relaxed)
  }

  pub async fn stdout(&self) -> UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    self.tx_stdout.lock().await.push(tx);
    rx
  }

  pub async fn stderr(&self) -> UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    self.tx_stderr.lock().await.push(tx);
    rx
  }

  pub async fn exited(&self) -> UnboundedReceiver<()> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    self.tx_exited.lock().await.push(tx);
    rx
  }

  pub fn send_kill(&self) {
    let _ = self.tx_kill.send(());
  }

  async fn command(options: &GenericProcessOptions) -> anyhow::Result<(ProcessType, Command)> {
    match &options.sandbox {
      SandboxType::None { command } => {
        let mut command = command.clone();

        let arg0 = command.remove(0);
        let mut cmd = Command::new(arg0);
        cmd.args(command);
        for (k, v) in &options.env {
          cmd.env(k, v);
        }

        Ok((ProcessType::Standard, cmd))
      }
      SandboxType::Podman { image, command } => {
        let name = format!("onlytrades_{}", Uuid::now_v7());
        let mut cmd = Command::new("podman");
        cmd.arg("run").arg("--rm").arg("-i");

        for (k, v) in &options.env {
          cmd.env(k, v);
          cmd.arg("-e").arg(format!("{}={}", k, v));
        }

        cmd.arg("--name").arg(&name);

        if !options.network {
          cmd.arg("--network").arg("none");
        }

        if options.read_only_fs {
          cmd.arg("--read-only");
        }

        if options.memory > 0 {
          cmd.arg("--memory").arg(format!("{}k", options.memory));
        }

        if options.cpus > 0 {
          let cpu_limit = options.cpus as f64 / 100.0;
          cmd.arg("--cpus").arg(format!("{:.2}", cpu_limit));
        }

        cmd.args([
          "--pids-limit",
          "20",
          "--cap-drop",
          "ALL",
          "--security-opt",
          "no-new-privileges",
        ]);

        // Maybe use gVisor later
        // cmd.arg("--runtime").arg("/usr/local/bin/runsc");

        for (host_path, container_path) in &options.volumes {
          let host_str = host_path.to_str().ok_or_else(|| {
            anyhow::anyhow!("Invalid host path character formatting: {:?}", host_path)
          })?;
          let container_str = container_path.to_str().ok_or_else(|| {
            anyhow::anyhow!(
              "Invalid container path character formatting: {:?}",
              container_path
            )
          })?;

          cmd
            .arg("-v")
            .arg(format!("{}:{}:ro,Z", host_str, container_str));
        }

        cmd.arg(image).args(command);
        cmd.arg("--allow-env").arg("--allow-net").arg("-");

        Ok((ProcessType::Podman { name }, cmd))
      }
    }
  }
}

pub struct TempFs {
  root_dir: PathBuf,
}

impl TempFs {
  pub async fn new(root_dir: &Path) -> anyhow::Result<Self> {
    tokio::fs::create_dir_all(&root_dir).await?;
    Ok(Self {
      root_dir: root_dir.to_path_buf(),
    })
  }

  pub fn path(&self) -> PathBuf {
    self.root_dir.clone()
  }
}

impl Drop for TempFs {
  fn drop(&mut self) {
    if !self.root_dir.exists() {
      return;
    }

    if let Err(e) = std::fs::remove_dir_all(&self.root_dir) {
      eprintln!("Warning: Failed to clean up TempFs directory: {}", e);
    }
  }
}

pub async fn create_temp_file(
  cwd: &Path,
  name: &str,
  bytes: &[u8],
) -> anyhow::Result<PathBuf> {
  let bin_name = kit_hash::sha256(bytes);
  let tmp_file = cwd.join(&bin_name);

  tokio::fs::write(&tmp_file, bytes).await?;

  Ok(tmp_file)
}
