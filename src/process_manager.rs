//! Process manager for iFlow CLI

use crate::error::{IFlowError, Result};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Child;
use tokio::time::sleep;

/// Manages iFlow CLI process lifecycle
pub struct IFlowProcessManager {
    pub process: Option<Child>, // Made public for access in Drop
}

impl IFlowProcessManager {
    pub fn new(_start_port: u16) -> Self {
        Self { process: None }
    }

    pub async fn start(&mut self) -> Result<String> {
        tracing::info!("Starting iFlow process with experimental ACP support");

        // Start iFlow process with stdio support
        let mut cmd = tokio::process::Command::new("iflow");
        cmd.arg("--experimental-acp");
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());

        let child = cmd
            .spawn()
            .map_err(|e| IFlowError::ProcessManager(format!("Failed to start iflow: {}", e)))?;

        self.process = Some(child);

        // Wait for process to start
        sleep(Duration::from_secs(2)).await;

        tracing::info!("iFlow process started with stdio support");

        // For stdio connection, we don't need a URL
        Ok("stdio".to_string())
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            tracing::info!("Stopping iFlow process");

            // Try graceful shutdown first
            let _ = process.kill().await;
            let _ = process.wait().await;

            tracing::info!("iFlow process stopped");
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    /// Take ownership of the process's stdin
    pub fn take_stdin(&mut self) -> Option<tokio::process::ChildStdin> {
        self.process.as_mut().and_then(|p| p.stdin.take())
    }

    /// Take ownership of the process's stdout
    pub fn take_stdout(&mut self) -> Option<tokio::process::ChildStdout> {
        self.process.as_mut().and_then(|p| p.stdout.take())
    }
}
