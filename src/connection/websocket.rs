//! WebSocket connection implementation for iFlow SDK
//!
//! This module implements the ConnectionHandler trait for WebSocket connections
//! using the custom ACP protocol implementation.

use crate::connection::handler::{ConnectionError, ConnectionHandler};
use crate::error::Result;
use crate::process_manager::IFlowProcessManager;
use crate::config::options::IFlowOptions;
use crate::websocket_transport::WebSocketTransport;
use tracing::debug;

/// WebSocket connection implementation
pub struct WebSocketConnection {
    /// ACP protocol handler
    acp_protocol: Option<crate::acp_protocol::ACPProtocol>,
    /// Session ID for the current session
    session_id: Option<String>,
    /// Process manager for iFlow process
    process_manager: Option<IFlowProcessManager>,
    /// Message sender for notifications
    message_sender: tokio::sync::mpsc::UnboundedSender<crate::message::types::Message>,
    /// Timeout in seconds
    timeout_secs: f64,
}

impl WebSocketConnection {
    /// Create a new WebSocketConnection
    ///
    /// # Arguments
    /// * `message_sender` - Sender for messages to be processed by the client
    /// * `timeout_secs` - Timeout in seconds for protocol operations
    pub fn new(
        message_sender: tokio::sync::mpsc::UnboundedSender<crate::message::types::Message>,
        timeout_secs: f64,
    ) -> Self {
        Self {
            acp_protocol: None,
            session_id: None,
            process_manager: None,
            message_sender,
            timeout_secs,
        }
    }

    /// Get a reference to the ACP protocol
    ///
    /// # Returns
    /// * `Some(&ACPProtocol)` if the protocol exists
    /// * `None` if the protocol is not initialized
    pub fn protocol(&self) -> Option<&crate::acp_protocol::ACPProtocol> {
        self.acp_protocol.as_ref()
    }

    /// Get a mutable reference to the ACP protocol
    ///
    /// # Returns
    /// * `Some(&mut ACPProtocol)` if the protocol exists
    /// * `None` if the protocol is not initialized
    pub fn protocol_mut(&mut self) -> Option<&mut crate::acp_protocol::ACPProtocol> {
        self.acp_protocol.as_mut()
    }

    /// Get the session ID
    ///
    /// # Returns
    /// * `Some(&str)` if a session exists
    /// * `None` if no session exists
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    /// Set the session ID
    ///
    /// # Arguments
    /// * `session_id` - The session ID to set
    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = Some(session_id);
    }

    /// Take the process manager
    ///
    /// This method takes ownership of the process manager, leaving None in its place.
    ///
    /// # Returns
    /// * `Some(IFlowProcessManager)` if a process manager exists
    /// * `None` if no process manager exists
    pub fn take_process_manager(&mut self) -> Option<IFlowProcessManager> {
        self.process_manager.take()
    }

    /// Set the process manager
    ///
    /// # Arguments
    /// * `process_manager` - The process manager to set
    pub fn set_process_manager(&mut self, process_manager: IFlowProcessManager) {
        self.process_manager = Some(process_manager);
    }
}

#[async_trait::async_trait(?Send)]
impl ConnectionHandler for WebSocketConnection {
    async fn initialize(&mut self, options: &IFlowOptions) -> Result<()> {
        debug!("Initializing WebSocket connection");

        let websocket_config = options.websocket.as_ref().ok_or_else(|| {
            ConnectionError::ConnectionError("WebSocket configuration not provided".to_string())
        })?;

        // Keep the process manager when auto-start is needed
        let mut process_manager_to_keep: Option<IFlowProcessManager> = None;

        // For manual start mode, directly use the provided WebSocket URL
        // For auto start mode, try to connect first and start process if needed
        let final_url = if options.process.auto_start {
            if let Some(url) = &websocket_config.url {
                // If URL is provided, check if it's a local URL and try to connect first
                if url.starts_with("ws://localhost:") {
                    debug!(
                        "iFlow auto-start enabled with provided URL, checking if iFlow is already running..."
                    );

                    // Try to connect first to see if iFlow is already running
                    let mut test_transport =
                        WebSocketTransport::new(url.clone(), self.timeout_secs);
                    match test_transport.connect().await {
                        Ok(_) => {
                            // Successfully connected to existing iFlow process
                            let _ = test_transport.close().await;
                            debug!("Connected to existing iFlow process at {}", url);
                            url.clone()
                        }
                        Err(e) => {
                            // Connection failed, check if it's because the port is in use
                            // Extract port from WebSocket URL
                            let port = url
                                .split(':')
                                .nth(2)
                                .and_then(|port_str| port_str.split('/').next())
                                .and_then(|port_str| port_str.parse::<u16>().ok())
                                .unwrap_or(8090);

                            // Check if the port is actually listening
                            if IFlowProcessManager::is_port_listening(port) {
                                // Port is listening, so iFlow is running but we can't connect for some other reason
                                // This could be because:
                                // 1. There's already another WebSocket connection to this iFlow instance
                                // 2. Authentication or other protocol issues
                                // 3. The iFlow instance is busy or not ready
                                debug!(
                                    "iFlow appears to be running on port {}, but connection failed: {}",
                                    port, e
                                );
                                debug!(
                                    "Since iFlow is running on the specified port, we won't start a new process. Please check if the existing iFlow instance is configured correctly for WebSocket connections."
                                );
                                return Err(ConnectionError::ConnectionError(format!(
                                    "Failed to connect to existing iFlow process at {}: {}. iFlow appears to be running on port {}, but connection could not be established.",
                                    url, e, port
                                )).into());
                            } else {
                                // Port is not listening, iFlow is not running, start it
                                debug!("iFlow not running on port {}, starting process", port);
                                let mut pm =
                                    IFlowProcessManager::new(port, options.process.debug);
                                let iflow_url = pm.start(true).await?.ok_or_else(|| {
                                    ConnectionError::ConnectionError(
                                        "Failed to start iFlow with WebSocket".to_string(),
                                    )
                                })?;
                                debug!("Started iFlow process at {}", iflow_url);

                                // Keep the process manager to avoid early handle drop causing child process exit due to stdout/stderr pipe issues
                                process_manager_to_keep = Some(pm);

                                iflow_url
                            }
                        }
                    }
                } else {
                    // For non-local URLs, directly use the provided URL
                    debug!("Using manual start mode or non-local WebSocket URL");
                    url.clone()
                }
            } else {
                // URL is None, auto-generate it by starting iFlow process
                debug!("iFlow auto-start enabled with auto-generated URL...");
                let port = options.process.start_port.unwrap_or(8090);
                let mut pm = IFlowProcessManager::new(port, options.process.debug);
                let iflow_url = pm.start(true).await?.ok_or_else(|| {
                    ConnectionError::ConnectionError(
                        "Failed to start iFlow with WebSocket".to_string(),
                    )
                })?;
                debug!("Started iFlow process at {}", iflow_url);

                // Keep the process manager to avoid early handle drop causing child process exit due to stdout/stderr pipe issues
                process_manager_to_keep = Some(pm);

                iflow_url
            }
        } else {
            // Manual start mode, URL must be provided
            let url = websocket_config.url.as_ref().ok_or_else(|| {
                ConnectionError::ConnectionError(
                    "WebSocket URL must be provided in manual start mode".to_string(),
                )
            })?;
            debug!("Using manual start mode with WebSocket URL: {}", url);
            url.clone()
        };

        // Create WebSocket transport with increased timeout
        let mut transport = WebSocketTransport::new(final_url.clone(), self.timeout_secs);

        // Connect to WebSocket with retry logic
        let mut connect_attempts = 0;

        while connect_attempts < websocket_config.reconnect_attempts {
            match transport.connect().await {
                Ok(_) => {
                    debug!("Successfully connected to WebSocket at {}", final_url);
                    break;
                }
                Err(e) => {
                    connect_attempts += 1;
                    tracing::warn!(
                        "Failed to connect to WebSocket (attempt {}): {}",
                        connect_attempts,
                        e
                    );

                    if connect_attempts >= websocket_config.reconnect_attempts {
                        return Err(ConnectionError::ConnectionError(format!(
                            "Failed to connect to WebSocket after {} attempts: {}",
                            websocket_config.reconnect_attempts, e
                        ))
                        .into());
                    }

                    // Wait before retrying
                    tracing::debug!(
                        "Waiting {:?} before retry...",
                        websocket_config.reconnect_interval
                    );
                    tokio::time::sleep(websocket_config.reconnect_interval).await;
                }
            }
        }

        // Create ACP protocol handler
        let mut acp_protocol = crate::acp_protocol::ACPProtocol::new(
            transport,
            self.message_sender.clone(),
            self.timeout_secs,
        );
        acp_protocol.set_permission_mode(options.permission_mode);

        // Store the protocol and process manager
        self.acp_protocol = Some(acp_protocol);
        self.process_manager = process_manager_to_keep;

        debug!("Initialized WebSocket connection");
        Ok(())
    }

    async fn create_session(&mut self, options: &IFlowOptions) -> Result<String> {
        let protocol = self.acp_protocol.as_mut().ok_or_else(|| {
            ConnectionError::NotInitialized
        })?;

        // Initialize the protocol if not already done
        if !protocol.is_initialized() {
            debug!("Initializing WebSocket protocol...");
            protocol.initialize(options).await.map_err(|e| {
                debug!("Failed to initialize protocol: {}", e);
                e
            })?;

            // Authenticate if needed
            if !protocol.is_authenticated() {
                debug!("Authenticating...");
                if let Some(method_id) = &options.auth_method_id {
                    protocol
                        .authenticate(method_id, None)
                        .await
                        .map_err(|e| {
                            debug!("Authentication failed with method {}: {}", method_id, e);
                            e
                        })?;
                } else {
                    // Try default authentication
                    protocol.authenticate("iflow", None).await.map_err(|e| {
                        debug!("Default authentication failed: {}", e);
                        e
                    })?;
                }
            }
        }

        // Create a new session
        debug!("Creating new session...");
        let current_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string();

        // Convert McpServer objects to JSON-compatible format
        let mcp_servers: Vec<serde_json::Value> = options
            .mcp_servers
            .iter()
            .map(|server| {
                // Since McpServer is an enum, we need to serialize it directly
                // The agent-client-protocol crate handles the serialization
                serde_json::json!(server)
            })
            .collect();

        let new_session_id = protocol
            .create_session(&current_dir, mcp_servers)
            .await
            .map_err(|e| {
                debug!("Failed to create session: {}", e);
                e
            })?;

        self.session_id = Some(new_session_id.clone());
        debug!("Session created successfully");

        Ok(new_session_id)
    }

    async fn send_message(&mut self, _session_id: &str, text: &str) -> Result<()> {
        let protocol = self.acp_protocol.as_mut().ok_or_else(|| {
            ConnectionError::NotInitialized
        })?;

        // Make sure we have a session
        let current_session_id = self.session_id.as_ref().ok_or_else(|| {
            ConnectionError::NoSession
        })?;

        // Send the prompt and get the request ID
        debug!("Sending prompt to session: {}", current_session_id);
        let _request_id = protocol
            .send_prompt(current_session_id, text)
            .await
            .map_err(|e| {
                debug!("Failed to send prompt: {}", e);
                e
            })?;

        debug!("Sent message to iFlow: {}", text);
        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        if let Some(mut protocol) = self.acp_protocol.take() {
            let _ = protocol.close().await;
        }

        // if we started the process, stop it
        if let Some(mut pm) = self.process_manager.take() {
            pm.stop().await?;
        }

        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.acp_protocol
            .as_ref()
            .map(|p| p.is_initialized())
            .unwrap_or(false)
    }

    fn is_authenticated(&self) -> bool {
        self.acp_protocol
            .as_ref()
            .map(|p| p.is_authenticated())
            .unwrap_or(false)
    }

    fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
}