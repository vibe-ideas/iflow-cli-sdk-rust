//! Process manager for iFlow CLI
//!
//! This module handles the lifecycle of the iFlow CLI process,
//! including starting, stopping, and managing stdio communication.

use crate::error::{IFlowError, Result};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Child;
use tokio::time::sleep;

/// Manages iFlow CLI process lifecycle
///
/// Handles starting and stopping the iFlow CLI process, as well as
/// providing access to its stdio streams for communication.
pub struct IFlowProcessManager {
    pub process: Option<Child>, // Made public for access in Drop
}

impl IFlowProcessManager {
    /// Create a new process manager
    ///
    /// # Arguments
    /// * `_start_port` - The port to start the process on (deprecated, no longer used)
    ///
    /// # Returns
    /// A new IFlowProcessManager instance
    pub fn new(_start_port: u16) -> Self {
        Self { process: None }
    }

    /// Start the iFlow process
    ///
    /// Starts the iFlow CLI process with ACP support and stdio communication.
    ///
    /// # Returns
    /// * `Ok("stdio")` if the process was started successfully
    /// * `Err(IFlowError)` if there was an error starting the process
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

    /// Stop the iFlow process
    ///
    /// Attempts to gracefully stop the iFlow process if it's running.
    ///
    /// # Returns
    /// * `Ok(())` if the process was stopped successfully or wasn't running
    /// * `Err(IFlowError)` if there was an error stopping the process
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

    /// Check if the iFlow process is running
    ///
    /// # Returns
    /// `true` if the process is running, `false` otherwise
    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    /// Take ownership of the process's stdin
    ///
    /// Takes ownership of the process's stdin stream for communication.
    /// This method can only be called once, as it consumes the stream.
    ///
    /// # Returns
    /// `Some(ChildStdin)` if the process is running and has a stdin stream, `None` otherwise
    pub fn take_stdin(&mut self) -> Option<tokio::process::ChildStdin> {
        self.process.as_mut().and_then(|p| p.stdin.take())
    }

    /// Take ownership of the process's stdout
    ///
    /// Takes ownership of the process's stdout stream for communication.
    /// This method can only be called once, as it consumes the stream.
    ///
    /// # Returns
    /// `Some(ChildStdout)` if the process is running and has a stdout stream, `None` otherwise
    pub fn take_stdout(&mut self) -> Option<tokio::process::ChildStdout> {
        self.process.as_mut().and_then(|p| p.stdout.take())
    }
}
