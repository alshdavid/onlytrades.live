#![allow(unused)]

use sysinfo::Pid;
use sysinfo::ProcessRefreshKind;
use sysinfo::RefreshKind;
use sysinfo::System;

#[derive(Debug)]
pub struct CommandChildSystemInfo {
  pub memory_usage_bytes: u64,
  pub memory_usage_kb: u64,
  pub memory_usage_mb: u64,
  pub cpu_usage_percent: usize, // 100 = 1 CPU core
}

pub trait CommandChildExt {
  fn stats(&self) -> anyhow::Result<CommandChildSystemInfo>;
}

impl CommandChildExt for tokio::process::Child {
  fn stats(&self) -> anyhow::Result<CommandChildSystemInfo> {
    let pid = self
      .id()
      .ok_or_else(|| anyhow::anyhow!("Child process has no accessible PID (already reaped?)"))?;

    let refresh_kind =
      RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing().with_memory().with_cpu());

    let mut system = System::new();
    system.refresh_specifics(refresh_kind);

    let process = system
      .process(Pid::from_u32(pid))
      .ok_or_else(|| anyhow::anyhow!("Process {} not found in system info", pid))?;

    let memory_usage_bytes = process.memory();
    let memory_usage_kb = memory_usage_bytes / 1024;
    let memory_usage_mb = memory_usage_kb / 1024;

    Ok(CommandChildSystemInfo {
      memory_usage_mb,
      memory_usage_kb,
      memory_usage_bytes,
      cpu_usage_percent: process.cpu_usage() as usize,
    })
  }
}
