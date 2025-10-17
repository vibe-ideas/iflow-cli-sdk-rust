//! ACP (Agent Communication Protocol) implementation for iFlow SDK
//!
//! This module implements the ACP protocol for communication
//! between the SDK and iFlow. It handles the JSON-RPC based messaging
//! and protocol flow.

use crate::error::{IFlowError, Result};
use crate::types::{IFlowOptions, Message, PermissionMode};
use crate::websocket_transport::WebSocketTransport;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::timeout;
use tracing::debug;

/// ACP protocol handler for iFlow communication
///
/// Implements the Agent Communication Protocol (ACP) which
/// defines the interaction between GUI applications and AI agents.
pub struct ACPProtocol {
    /// WebSocket transport for communication
    transport: WebSocketTransport,
    /// Whether the protocol has been initialized
    initialized: bool,
    /// Whether authentication has been completed
    authenticated: bool,
    /// Request ID counter
    request_id: u32,
    /// Sender for messages to be processed by the client
    message_sender: UnboundedSender<Message>,
    /// Protocol version
    protocol_version: u32,
    /// Permission mode for tool calls
    permission_mode: PermissionMode,
    /// Configurable timeout in seconds
    timeout_secs: f64,
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
    fn next_request_id(&mut self) -> u32 {
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

    /// Perform authentication if required
    ///
    /// This method should be called if initialize() indicates
    /// that authentication is needed (isAuthenticated = False).
    ///
    /// # Arguments
    /// * `method_id` - Authentication method ID
    /// * `method_info` - Optional authentication info
    ///
    /// # Returns
    /// * `Ok(())` if authentication was successful
    /// * `Err(IFlowError)` if authentication failed
    pub async fn authenticate(
        &mut self,
        method_id: &str,
        method_info: Option<HashMap<String, String>>,
    ) -> Result<()> {
        if self.authenticated {
            debug!("Already authenticated");
            return Ok(());
        }

        let request_id = self.next_request_id();
        let mut params = json!({
            "methodId": method_id,
        });

        if let Some(info) = method_info {
            params["methodInfo"] = json!(info);
        }

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "authenticate",
            "params": params,
        });

        self.transport.send(&request).await?;
        debug!("Sent authenticate request with method: {}", method_id);

        // Wait for authentication response with timeout
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response(request_id))
            .await
            .map_err(|_| {
                IFlowError::Timeout("Timeout waiting for authentication response".to_string())
            })?
            .map_err(|e| IFlowError::Protocol(format!("Failed to authenticate: {}", e)))?;

        if let Some(result) = response.get("result") {
            if let Some(response_method) = result.get("methodId").and_then(|v| v.as_str()) {
                if response_method == method_id {
                    self.authenticated = true;
                    debug!("Authentication successful with method: {}", response_method);
                } else {
                    tracing::warn!(
                        "Unexpected methodId in response: {} (expected {})",
                        response_method,
                        method_id
                    );
                    // Still mark as authenticated if we got a response
                    self.authenticated = true;
                }
            } else {
                self.authenticated = true;
            }
        } else if let Some(error) = response.get("error") {
            return Err(IFlowError::Authentication(format!(
                "Authentication failed: {:?}",
                error
            )));
        } else {
            return Err(IFlowError::Protocol(
                "Invalid authenticate response".to_string(),
            ));
        }

        Ok(())
    }

    /// Create a new session
    ///
    /// # Arguments
    /// * `cwd` - Working directory for the session
    /// * `mcp_servers` - MCP servers to connect to
    ///
    /// # Returns
    /// * `Ok(String)` containing the session ID
    /// * `Err(IFlowError)` if session creation failed
    pub async fn create_session(
        &mut self,
        cwd: &str,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<String> {
        if !self.initialized {
            return Err(IFlowError::Protocol(
                "Protocol not initialized. Call initialize() first.".to_string(),
            ));
        }

        if !self.authenticated {
            return Err(IFlowError::Protocol(
                "Not authenticated. Call authenticate() first.".to_string(),
            ));
        }

        let request_id = self.next_request_id();
        let params = json!({
            "cwd": cwd,
            "mcpServers": mcp_servers,
        });

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "session/new",
            "params": params,
        });

        self.transport.send(&request).await?;
        debug!(
            "Sent session/new request with cwd: {} and mcpServers: {:?}",
            cwd, mcp_servers
        );

        // Wait for response with timeout
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response(request_id))
            .await
            .map_err(|_| {
                IFlowError::Timeout("Timeout waiting for session creation response".to_string())
            })?
            .map_err(|e| IFlowError::Protocol(format!("Failed to create session: {}", e)))?;

        if let Some(result) = response.get("result") {
            if let Some(session_id) = result.get("sessionId").and_then(|v| v.as_str()) {
                debug!("Created session: {}", session_id);
                Ok(session_id.to_string())
            } else {
                debug!(
                    "Invalid session/new response, using fallback ID: session_{}",
                    request_id
                );
                Ok(format!("session_{}", request_id))
            }
        } else if let Some(error) = response.get("error") {
            Err(IFlowError::Protocol(format!(
                "session/new failed: {:?}",
                error
            )))
        } else {
            Err(IFlowError::Protocol(
                "Invalid session/new response".to_string(),
            ))
        }
    }

    /// Send a prompt to the session and wait for response
    ///
    /// # Arguments
    /// * `session_id` - The session ID from create_session()
    /// * `prompt` - The prompt text to send
    ///
    /// # Returns
    /// * `Ok(u32)` containing the request ID for tracking the message
    /// * `Err(IFlowError)` if sending failed
    pub async fn send_prompt(&mut self, session_id: &str, prompt: &str) -> Result<u32> {
        if !self.initialized {
            return Err(IFlowError::Protocol(
                "Protocol not initialized. Call initialize() first.".to_string(),
            ));
        }

        if !self.authenticated {
            return Err(IFlowError::Protocol(
                "Not authenticated. Call authenticate() first.".to_string(),
            ));
        }

        let request_id = self.next_request_id();
        // Create prompt as a list of content blocks
        let prompt_blocks = vec![json!({
            "type": "text",
            "text": prompt
        })];

        let params = json!({
            "sessionId": session_id,
            "prompt": prompt_blocks,
        });

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "session/prompt",
            "params": params,
        });

        self.transport.send(&request).await?;
        debug!("Sent session/prompt");

        // Wait for response
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response_with_notifications(request_id))
            .await
            .map_err(|_| IFlowError::Timeout("Timeout waiting for prompt response".to_string()))?
            .map_err(|e| IFlowError::Protocol(format!("Failed to send prompt: {}", e)))?;

        // Check for errors in response
        if let Some(error) = response.get("error") {
            return Err(IFlowError::Protocol(format!("Prompt failed: {:?}", error)));
        }

        // Send task finish message to indicate completion
        let msg = Message::TaskFinish {
            reason: Some("completed".to_string()),
        };
        let _ = self.message_sender.send(msg);

        Ok(request_id)
    }

    /// Wait for a response to a specific request
    ///
    /// # Arguments
    /// * `request_id` - The ID of the request to wait for
    ///
    /// # Returns
    /// * `Ok(Value)` containing the response
    /// * `Err(IFlowError)` if waiting failed
    async fn wait_for_response(&mut self, request_id: u32) -> Result<Value> {
        let timeout_duration = Duration::from_secs_f64(self.timeout_secs);
        let start_time = std::time::Instant::now();

        loop {
            if start_time.elapsed() > timeout_duration {
                return Err(IFlowError::Timeout(format!(
                    "Timeout waiting for response to request {}",
                    request_id
                )));
            }

            // Use a shorter timeout for receiving messages to allow for periodic checks
            let receive_timeout = Duration::from_secs_f64(self.timeout_secs.min(1.0));
            let msg = match timeout(receive_timeout, self.transport.receive()).await {
                Ok(Ok(msg)) => msg,
                Ok(Err(e)) => {
                    tracing::error!("Transport error while waiting for response: {}", e);
                    return Err(e);
                }
                Err(_) => {
                    // Timeout is expected, continue waiting
                    tracing::debug!(
                        "No message received, continuing to wait for response to request {}...",
                        request_id
                    );
                    continue;
                }
            };

            // Skip control messages
            if msg.starts_with("//") {
                tracing::debug!("Control message: {}", msg);
                continue;
            }

            // Try to parse as JSON
            let data: Value = match serde_json::from_str(&msg) {
                Ok(data) => data,
                Err(e) => {
                    tracing::debug!("Failed to parse message as JSON: {}, message: {}", e, msg);
                    continue;
                }
            };

            // Check if this is the response we're waiting for
            if let Some(id) = data.get("id").and_then(|v| v.as_u64()) {
                if id == request_id as u64 {
                    return Ok(data);
                }
            }

            // If not our response, process as a notification
            if let Err(e) = self.handle_notification(data).await {
                tracing::warn!("Failed to handle notification: {}", e);
                // Don't fail the entire wait, just log and continue
            }
        }
    }

    /// Wait for a response to a specific request while handling notifications
    ///
    /// # Arguments
    /// * `request_id` - The ID of the request to wait for
    ///
    /// # Returns
    /// * `Ok(Value)` containing the response
    /// * `Err(IFlowError)` if waiting failed
    async fn wait_for_response_with_notifications(&mut self, request_id: u32) -> Result<Value> {
        let timeout_duration = Duration::from_secs_f64(self.timeout_secs);
        let start_time = std::time::Instant::now();

        loop {
            if start_time.elapsed() > timeout_duration {
                return Err(IFlowError::Timeout(format!(
                    "Timeout waiting for response to request {}",
                    request_id
                )));
            }

            // Use a shorter timeout for receiving messages to allow for periodic checks
            let receive_timeout = Duration::from_secs_f64(self.timeout_secs.min(1.0));
            let msg = match timeout(receive_timeout, self.transport.receive()).await {
                Ok(Ok(msg)) => msg,
                Ok(Err(e)) => {
                    tracing::error!("Transport error while waiting for response: {}", e);
                    return Err(e);
                }
                Err(_) => {
                    // Timeout is expected, continue waiting
                    tracing::debug!(
                        "No message received, continuing to wait for response to request {}...",
                        request_id
                    );
                    continue;
                }
            };

            // Skip control messages
            if msg.starts_with("//") {
                tracing::debug!("Control message: {}", msg);
                continue;
            }

            // Try to parse as JSON
            let data: Value = match serde_json::from_str(&msg) {
                Ok(data) => data,
                Err(e) => {
                    tracing::debug!("Failed to parse message as JSON: {}, message: {}", e, msg);
                    continue;
                }
            };

            // Check if this is the response we're waiting for
            if let Some(id) = data.get("id").and_then(|v| v.as_u64()) {
                // Handle permission requests that come with an ID
                if let Some(method) = data.get("method").and_then(|v| v.as_str()) {
                    if method == "session/request_permission" {
                        tracing::debug!("Handling session/request_permission with ID: {}", id);
                        // Process the permission request immediately
                        if let Err(e) = self.handle_client_method(method, data.clone()).await {
                            tracing::warn!("Failed to handle permission request: {}", e);
                        }
                        // Continue waiting for the main response
                        continue;
                    }
                }
                
                // If this is the response we're waiting for, return it
                if id == request_id as u64 {
                    return Ok(data);
                }
            }

            // If not our response, process as a notification
            if let Err(e) = self.handle_notification(data).await {
                tracing::warn!("Failed to handle notification: {}", e);
                // Don't fail the entire wait, just log and continue
            }
        }
    }

    /// Handle incoming notifications from the server
    ///
    /// # Arguments
    /// * `data` - The JSON data of the notification
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_notification(&mut self, data: Value) -> Result<()> {
        // Handle method calls from server (client interface)
        if let Some(method) = data.get("method").and_then(|v| v.as_str()) {
            if data.get("result").is_none() && data.get("error").is_none() {
                self.handle_client_method(method, data.clone()).await?;
            }
        }

        Ok(())
    }

    /// Handle client method calls from the server
    ///
    /// # Arguments
    /// * `method` - The method name
    /// * `data` - The JSON data of the method call
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_client_method(&mut self, method: &str, data: Value) -> Result<()> {
        let params = data.get("params").cloned().unwrap_or(Value::Null);
        let request_id = data.get("id").and_then(|v| v.as_u64());

        match method {
            "session/update" => {
                if let Some(update_obj) = params.get("update").and_then(|v| v.as_object()) {
                    if let Some(session_update) =
                        update_obj.get("sessionUpdate").and_then(|v| v.as_str())
                    {
                        self.handle_session_update(session_update, update_obj, request_id)
                            .await?;
                    }
                }
            }
            "session/request_permission" => {
                // Handle permission request from CLI
                tracing::debug!("Handling session/request_permission");
                self.handle_permission_request(params, request_id).await?;
            }
            _ => {
                tracing::warn!("Unknown method: {}", method);
                // Send error response for unknown methods
                if let Some(id) = request_id {
                    let error_response = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32601,
                            "message": "Method not found"
                        }
                    });
                    self.transport.send(&error_response).await?;
                }
            }
        }

        Ok(())
    }

    /// Handle permission request from the CLI
    ///
    /// # Arguments
    /// * `params` - The parameters of the permission request
    /// * `request_id` - Optional request ID for responses
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_permission_request(
        &mut self,
        params: Value,
        request_id: Option<u64>,
    ) -> Result<()> {
        // Extract tool call information from params
        let tool_call = params.get("toolCall").unwrap_or(&Value::Null);
        let options = params.get("options").unwrap_or(&Value::Null);
        let _session_id = params.get("sessionId").and_then(|v| v.as_str());

        // Log the tool call information
        let tool_title = tool_call
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let tool_type = tool_call
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        tracing::debug!(
            "Permission request for tool '{}' (type: {})",
            tool_title,
            tool_type
        );

        // Determine response based on permission_mode
        let auto_approve = match self.permission_mode {
            PermissionMode::Auto => {
                // Auto-approve all tool calls
                true
            }
            PermissionMode::Manual => {
                // Require manual confirmation for all
                false
            }
            PermissionMode::Selective => {
                // Auto-approve based on tool type
                // For now, we'll auto-approve read/fetch operations
                tool_type == "read" || tool_type == "fetch" || tool_type == "list"
            }
        };

        use agent_client_protocol::{RequestPermissionOutcome, RequestPermissionResponse};
        let permission_response = if auto_approve {
            // Find the appropriate option from the provided options
            let mut selected_option = "proceed_once".to_string();
            if let Some(options_array) = options.as_array() {
                for option in options_array {
                    if let Some(option_id) = option.get("optionId").and_then(|v| v.as_str()) {
                        if option_id == "proceed_once" {
                            selected_option = option_id.to_string();
                            break;
                        } else if option_id == "proceed_always" {
                            selected_option = option_id.to_string();
                        }
                    }
                }
                // Fallback to first option's optionId if no specific option found
                if selected_option == "proceed_once" && !options_array.is_empty() {
                    if let Some(first_option_id) = options_array[0].get("optionId").and_then(|v| v.as_str()) {
                        selected_option = first_option_id.to_string();
                    }
                }
            }
            RequestPermissionResponse {
                outcome: RequestPermissionOutcome::Selected {
                    option_id: agent_client_protocol::PermissionOptionId(std::sync::Arc::from(selected_option)),
                },
                meta: None,
            }
        } else {
            RequestPermissionResponse {
                outcome: RequestPermissionOutcome::Cancelled,
                meta: None,
            }
        };

        // Send response if request ID is provided
        if let Some(id) = request_id {
            let response_message = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": permission_response
            });
            self.transport.send(&response_message).await?;
        }

        let outcome_str = match &permission_response.outcome {
            RequestPermissionOutcome::Cancelled => "cancelled",
            RequestPermissionOutcome::Selected { option_id } => &*option_id.0,
        };
        tracing::debug!("Permission request for tool '{}': {}", tool_title, outcome_str);
        
        Ok(())
    }

    /// Handle session update notifications
    ///
    /// # Arguments
    /// * `update_type` - The type of update
    /// * `update` - The update data
    /// * `request_id` - Optional request ID for responses
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_session_update(
        &mut self,
        update_type: &str,
        update: &serde_json::Map<String, Value>,
        request_id: Option<u64>,
    ) -> Result<()> {
        match update_type {
            "agent_message_chunk" => {
                if let Some(content) = update.get("content") {
                    let text = match content {
                        Value::Object(obj) => {
                            if let Some(text_content) = obj.get("text").and_then(|v| v.as_str()) {
                                text_content.to_string()
                            } else {
                                "<unknown>".to_string()
                            }
                        }
                        _ => "<unknown>".to_string(),
                    };

                    let msg = Message::Assistant { content: text };
                    let _ = self.message_sender.send(msg);
                }
            }
            "user_message_chunk" => {
                if let Some(content) = update.get("content") {
                    let text = match content {
                        Value::Object(obj) => {
                            if let Some(text_content) = obj.get("text").and_then(|v| v.as_str()) {
                                text_content.to_string()
                            } else {
                                "<unknown>".to_string()
                            }
                        }
                        _ => "<unknown>".to_string(),
                    };

                    let msg = Message::User { content: text };
                    let _ = self.message_sender.send(msg);
                }
            }
            "tool_call" => {
                if let Some(tool_call) = update.get("toolCall") {
                    let id = tool_call
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let name = tool_call
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let status = tool_call
                        .get("status")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let msg = Message::ToolCall { id, name, status };
                    let _ = self.message_sender.send(msg);
                }
            }
            "plan" => {
                if let Some(entries) = update.get("entries").and_then(|v| v.as_array()) {
                    let entries: Vec<super::types::PlanEntry> = entries
                        .iter()
                        .filter_map(|entry| {
                            let content =
                                entry.get("content").and_then(|v| v.as_str())?.to_string();
                            let priority_str = entry
                                .get("priority")
                                .and_then(|v| v.as_str())
                                .unwrap_or("medium");
                            let status_str = entry
                                .get("status")
                                .and_then(|v| v.as_str())
                                .unwrap_or("pending");

                            let priority = match priority_str {
                                "high" => super::types::PlanPriority::High,
                                "medium" => super::types::PlanPriority::Medium,
                                "low" => super::types::PlanPriority::Low,
                                _ => super::types::PlanPriority::Medium,
                            };

                            let status = match status_str {
                                "pending" => super::types::PlanStatus::Pending,
                                "in_progress" => super::types::PlanStatus::InProgress,
                                "completed" => super::types::PlanStatus::Completed,
                                _ => super::types::PlanStatus::Pending,
                            };

                            Some(super::types::PlanEntry {
                                content,
                                priority,
                                status,
                            })
                        })
                        .collect();

                    let msg = Message::Plan { entries };
                    let _ = self.message_sender.send(msg);
                }
            }
            "tool_call_update" => {
                // For now, we'll just acknowledge the update if there's a request ID
                if let Some(id) = request_id {
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": null
                    });
                    self.transport.send(&response).await?;
                }
            }
            "agent_thought_chunk" | "current_mode_update" | "available_commands_update" => {
                // Ignore these for now
            }
            _ => {
                tracing::debug!("Unhandled session update type: {}", update_type);
            }
        }

        // Send acknowledgment for notifications that require it
        if let Some(id) = request_id {
            match update_type {
                "tool_call_update" | "notifyTaskFinish" => {
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": null
                    });
                    self.transport.send(&response).await?;
                }
                _ => {}
            }
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