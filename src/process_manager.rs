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
    start_port: u16,
    port: Option<u16>,
}

impl IFlowProcessManager {
    /// Create a new process manager
    ///
    /// # Arguments
    /// * `start_port` - The port to start the process on
    ///
    /// # Returns
    /// A new IFlowProcessManager instance
    pub fn new(start_port: u16) -> Self {
        Self { 
            process: None,
            start_port,
            port: None,
        }
    }

    /// Check if a port is available for use
///
/// # Arguments
/// * `port` - Port number to check
///
/// # Returns
/// True if the port is available, False otherwise
fn is_port_available(port: u16) -> bool {
    use std::net::TcpListener;
    TcpListener::bind(("localhost", port)).is_ok()
}

/// Find an available port starting from the given port
///
/// # Arguments
/// * `start_port` - Port to start searching from
/// * `max_attempts` - Maximum number of ports to try
///
/// # Returns
/// An available port number
///
/// # Errors
/// Returns an error if no available port is found
fn find_available_port(start_port: u16, max_attempts: u16) -> Result<u16> {
    for i in 0..max_attempts {
        let port = start_port + i;
        if Self::is_port_available(port) {
            tracing::debug!("Found available port: {}", port);
            return Ok(port);
        }
    }
    
    Err(IFlowError::ProcessManager(format!(
        "No available port found in range {}-{}",
        start_port,
        start_port + max_attempts
    )))
}

/// Start the iFlow process
///
/// Starts the iFlow CLI process with ACP support and WebSocket communication.
///
/// # Returns
/// * `Ok(String)` containing the WebSocket URL if the process was started successfully
/// * `Err(IFlowError)` if there was an error starting the process
pub async fn start(&mut self, use_websocket: bool) -> Result<Option<String>> {
        if use_websocket {
            tracing::info!("Starting iFlow process with experimental ACP and WebSocket support");

            // Find an available port
            let port = Self::find_available_port(self.start_port, 100)?;
            self.port = Some(port);

            // Start iFlow process with WebSocket support
            let mut cmd = tokio::process::Command::new("iflow");
            cmd.arg("--experimental-acp");
            cmd.arg("--port");
            cmd.arg(port.to_string());
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.stdin(Stdio::null()); // No stdin needed for WebSocket

            let child = cmd
                .spawn()
                .map_err(|e| IFlowError::ProcessManager(format!("Failed to start iflow: {}", e)))?;

            self.process = Some(child);

            // Wait for process to start
            sleep(Duration::from_secs(3)).await;

            tracing::info!("iFlow process started with WebSocket support on port {}", port);

            // Return the WebSocket URL
            Ok(Some(format!("ws://localhost:{}/acp", port)))
        } else {
            tracing::info!("Starting iFlow process with experimental ACP and stdio support");

            // Start iFlow process with stdio support
            let mut cmd = tokio::process::Command::new("iflow");
            cmd.arg("--experimental-acp");
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.stdin(Stdio::piped()); // stdin needed for stdio

            let child = cmd
                .spawn()
                .map_err(|e| IFlowError::ProcessManager(format!("Failed to start iflow: {}", e)))?;

            self.process = Some(child);

            // Wait for process to start
            sleep(Duration::from_secs(3)).await;

            tracing::info!("iFlow process started with stdio support");

            // No WebSocket URL for stdio
            Ok(None)
        }
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

    /// Get the port the iFlow process is running on
    ///
    /// # Returns
    /// The port number, or None if not running
    pub fn port(&self) -> Option<u16> {
        self.port
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
