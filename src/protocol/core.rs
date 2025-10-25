//! Core ACP protocol implementation for iFlow SDK
//!
//! This module implements the core ACP protocol for communication
//! between the SDK and iFlow. It handles the JSON-RPC based messaging
//! and protocol flow.

use crate::error::{IFlowError, Result};
use crate::config::options::IFlowOptions;
use crate::message::types::{Message, PermissionMode};
use crate::websocket_transport::WebSocketTransport;
use serde_json::json;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::timeout;
use tracing::debug;

/// ACP protocol handler for iFlow communication
///
/// Implements the [Agent Client Protocol (ACP)](https://github.com/agentclientprotocol/agent-client-protocol) which
/// defines the interaction between GUI applications and AI agents.
pub struct ACPProtocol {
    /// WebSocket transport for communication
    pub transport: WebSocketTransport,
    /// Whether the protocol has been initialized
    pub initialized: bool,
    /// Whether authentication has been completed
    pub authenticated: bool,
    /// Request ID counter
    request_id: u32,
    /// Sender for messages to be processed by the client
    pub message_sender: UnboundedSender<Message>,
    /// Protocol version
    protocol_version: u32,
    /// Permission mode for tool calls
    pub permission_mode: PermissionMode,
    /// Configurable timeout in seconds
    pub timeout_secs: f64,
}

impl ACPProtocol {
    /// Initialize ACP protocol handler
    ///
    /// # Arguments
    /// * `transport` - WebSocket transport for communication
    /// * `message_sender` - Sender for messages to be processed by the client
    /// * `timeout_secs` - Timeout in seconds for protocol operations
    pub fn new(
        transport: WebSocketTransport,
        message_sender: UnboundedSender<Message>,
        timeout_secs: f64,
    ) -> Self {
        Self {
            transport,
            initialized: false,
            authenticated: false,
            request_id: 0,
            message_sender,
            protocol_version: 1,
            permission_mode: PermissionMode::Auto,
            timeout_secs,
        }
    }

    /// Check if the protocol has been initialized
    ///
    /// # Returns
    /// True if initialized, False otherwise
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Check if the protocol has been authenticated
    ///
    /// # Returns
    /// True if authenticated, False otherwise
    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    /// Set the permission mode for tool calls
    ///
    /// # Arguments
    /// * `mode` - The permission mode to use
    pub fn set_permission_mode(&mut self, mode: PermissionMode) {
        self.permission_mode = mode;
    }

    /// Generate next request ID
    ///
    /// # Returns
    /// Unique request ID
    pub fn next_request_id(&mut self) -> u32 {
        self.request_id += 1;
        self.request_id
    }

    /// Initialize the protocol connection
    ///
    /// Performs the ACP initialization handshake:
    /// 1. Wait for //ready signal
    /// 2. Send initialize request with optional configs
    /// 3. Process initialize response
    ///
    /// # Arguments
    /// * `options` - Configuration options
    ///
    /// # Returns
    /// * `Ok(())` if initialization was successful
    /// * `Err(IFlowError)` if initialization failed
    pub async fn initialize(&mut self, options: &IFlowOptions) -> Result<()> {
        if self.initialized {
            tracing::warn!("Protocol already initialized");
            return Ok(());
        }

        debug!("Initializing ACP protocol");

        // Wait for //ready signal with timeout and better error handling
        debug!("Waiting for //ready signal...");
        let ready_timeout = Duration::from_secs_f64(self.timeout_secs);
        let start_time = std::time::Instant::now();

        loop {
            if start_time.elapsed() > ready_timeout {
                return Err(IFlowError::Timeout(
                    "Timeout waiting for //ready signal".to_string(),
                ));
            }

            let msg = match timeout(
                Duration::from_secs_f64(self.timeout_secs.min(10.0)),
                self.transport.receive(),
            )
            .await
            {
                Ok(Ok(msg)) => msg,
                Ok(Err(e)) => {
                    tracing::error!("Transport error while waiting for //ready: {}", e);
                    // Don't immediately fail, try to reconnect or continue
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    continue;
                }
                Err(_) => {
                    tracing::debug!("No message received, continuing to wait for //ready...");
                    continue;
                }
            };

            let trimmed_msg = msg.trim();
            if trimmed_msg == "//ready" {
                debug!("Received //ready signal");
                break;
            } else if trimmed_msg.starts_with("//") {
                // Log other control messages
                tracing::debug!("Control message: {}", trimmed_msg);
                continue;
            } else if !trimmed_msg.is_empty() {
                // Not a control message, continue waiting for //ready
                tracing::debug!(
                    "Non-control message received while waiting for //ready: {}",
                    trimmed_msg
                );
                continue;
            }
        }

        // Add a small delay to ensure the server is fully ready
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Send initialize request
        let request_id = self.next_request_id();
        let mut params = json!({
            "protocolVersion": self.protocol_version,
            "clientCapabilities": {
                "fs": {
                    "readTextFile": true,
                    "writeTextFile": true
                }
            }
        });

        // Add optional configurations from options
        if !options.mcp_servers.is_empty() {
            // Convert McpServer objects to JSON-compatible format
            let mcp_servers: Vec<serde_json::Value> = options
                .mcp_servers
                .iter()
                .map(|server| {
                    // Since McpServer is an enum, we need to serialize it directly
                    // The agent-client-protocol crate handles the serialization
                    json!(server)
                })
                .collect();
            params["mcpServers"] = json!(mcp_servers);
        }

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "initialize",
            "params": params,
        });

        // Send with retry logic
        let mut send_attempts = 0;
        let max_send_attempts = 3;

        while send_attempts < max_send_attempts {
            match self.transport.send(&request).await {
                Ok(_) => {
                    debug!("Sent initialize request (attempt {})", send_attempts + 1);
                    break;
                }
                Err(e) => {
                    send_attempts += 1;
                    tracing::warn!(
                        "Failed to send initialize request (attempt {}): {}",
                        send_attempts,
                        e
                    );
                    if send_attempts >= max_send_attempts {
                        return Err(IFlowError::Protocol(format!(
                            "Failed to send initialize request after {} attempts: {}",
                            max_send_attempts, e
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }

        // Wait for initialize response with timeout
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response(request_id))
            .await
            .map_err(|_| {
                IFlowError::Timeout("Timeout waiting for initialize response".to_string())
            })?
            .map_err(|e| IFlowError::Protocol(format!("Failed to initialize: {}", e)))?;

        if let Some(result) = response.get("result") {
            self.authenticated = result
                .get("isAuthenticated")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            self.initialized = true;
            debug!(
                "Initialized with protocol version: {:?}, authenticated: {}",
                result.get("protocolVersion"),
                self.authenticated
            );
        } else if let Some(error) = response.get("error") {
            return Err(IFlowError::Protocol(format!(
                "Initialize failed: {:?}",
                error
            )));
        } else {
            return Err(IFlowError::Protocol(
                "Invalid initialize response".to_string(),
            ));
        }

        Ok(())
    }

    /// Close the protocol connection
    pub async fn close(&mut self) -> Result<()> {
        self.transport.close().await?;
        Ok(())
    }

    /// Check if the protocol is connected
    ///
    /// # Returns
    /// True if connected, False otherwise
    pub fn is_connected(&self) -> bool {
        self.transport.is_connected()
    }
}