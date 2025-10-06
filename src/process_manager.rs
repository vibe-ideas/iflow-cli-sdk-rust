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

    /// Check if a port is listening (has a server running)
    ///
    /// # Arguments
    /// * `port` - Port number to check
    ///
    /// # Returns
    /// True if the port is listening, False otherwise
    pub fn is_port_listening(port: u16) -> bool {
        use std::net::TcpStream;
        use std::time::Duration;
        TcpStream::connect_timeout(
            &format!("127.0.0.1:{}", port).parse().unwrap(),
            Duration::from_millis(100),
        )
        .is_ok()
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
            tracing::debug!("Starting iFlow process with experimental ACP and WebSocket support");

            // Find an available port
            let port = Self::find_available_port(self.start_port, 100)?;
            self.port = Some(port);

            // Start iFlow process with WebSocket support
            let mut cmd = tokio::process::Command::new("iflow");
            cmd.arg("--experimental-acp");
            cmd.arg("--port");
            cmd.arg(port.to_string());
            // In WebSocket mode, set stdout/stderr to inherit to avoid blocking/exit when pipes are not consumed
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
            cmd.stdin(Stdio::null()); // No stdin needed for WebSocket

            let child = cmd
                .spawn()
                .map_err(|e| IFlowError::ProcessManager(format!("Failed to start iflow: {}", e)))?;

            self.process = Some(child);

            // Wait longer for process to start and WebSocket server to be ready
            tracing::debug!("Waiting for iFlow process to start...");
            sleep(Duration::from_secs(8)).await;

            // Verify the port is actually listening with more retries and longer timeout
            let mut attempts = 0;
            let max_attempts = 30; // 30 attempts * 1 second = 30 seconds total

            while attempts < max_attempts {
                if Self::is_port_listening(port) {
                    tracing::debug!("iFlow WebSocket server is ready on port {}", port);
                    break;
                }

                attempts += 1;
                if attempts % 5 == 0 {
                    tracing::debug!(
                "Still waiting for iFlow to be ready... (attempt {}/{})",
                attempts,
                max_attempts
            );
                }

                sleep(Duration::from_secs(1)).await;
            }

            if attempts >= max_attempts {
                return Err(IFlowError::ProcessManager(format!(
                    "iFlow process failed to start WebSocket server on port {} after {} seconds",
                    port, max_attempts
                )));
            }

            tracing::debug!(
            "iFlow process started with WebSocket support on port {}",
            port
        );

            // Return the WebSocket URL with peer parameter
            Ok(Some(format!("ws://localhost:{}/acp?peer=iflow", port)))
        } else {
            tracing::debug!("Starting iFlow process with experimental ACP and stdio support");

            // Start iFlow process with stdio support
            let mut cmd = tokio::process::Command::new("iflow");
            cmd.arg("--experimental-acp");
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.stdin(Stdio::piped()); // stdin needed for stdio

            tracing::debug!("Starting iFlow process with command: {:?}", cmd);

            let child = cmd
                .spawn()
                .map_err(|e| IFlowError::ProcessManager(format!("Failed to start iflow: {}", e)))?;

            self.process = Some(child);

            // Wait for process to start
            tracing::debug!("Waiting for iFlow process to start...");
            sleep(Duration::from_secs(5)).await;
            tracing::debug!("iFlow process should be started by now");

            tracing::debug!("iFlow process started with stdio support");

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
            tracing::debug!("Stopping iFlow process");

            // Try graceful shutdown first
            match tokio::time::timeout(Duration::from_secs(5), process.kill()).await {
                Ok(Ok(_)) => {
                    // Wait for the process to actually exit with a timeout
                    match tokio::time::timeout(Duration::from_secs(5), process.wait()).await {
                        Ok(Ok(_)) => tracing::debug!("iFlow process stopped gracefully"),
                        Ok(Err(e)) => tracing::warn!("Error waiting for iFlow process: {}", e),
                        Err(_) => {
                            tracing::warn!(
                                "Timeout waiting for iFlow process to exit, forcing termination"
                            );
                            // Force kill if it didn't exit in time
                            let _ = process.start_kill();
                        }
                    }
                }
                Ok(Err(e)) => {
                    tracing::warn!("Failed to kill iFlow process: {}, forcing termination", e);
                    let _ = process.start_kill();
                }
                Err(_) => {
                    tracing::warn!("Timeout killing iFlow process, forcing termination");
                    let _ = process.start_kill();
                }
            }

            // Add a small delay to ensure all resources are released
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            tracing::debug!("iFlow process stopped");
        }

        // Clear the port when stopping
        self.port = None;

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
