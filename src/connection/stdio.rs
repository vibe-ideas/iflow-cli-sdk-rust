//! Stdio connection implementation for iFlow SDK
//!
//! This module implements the ConnectionHandler trait for stdio connections
//! using the agent-client-protocol crate.

use crate::connection::handler::{ConnectionError, ConnectionHandler};
use crate::error::Result;
use crate::process_manager::IFlowProcessManager;
use crate::config::options::IFlowOptions;
use crate::message::types::Message;
use agent_client_protocol::{Agent, ClientSideConnection};
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Stdio connection implementation
pub struct StdioConnection {
    /// ACP client connection
    acp_client: Option<ClientSideConnection>,
    /// Process manager for iFlow process
    process_manager: Option<IFlowProcessManager>,
    /// Session ID for the current session
    session_id: Option<String>,
    /// Whether the connection has been initialized
    initialized: bool,
    /// Message sender for notifications
    message_sender: UnboundedSender<Message>,
}

impl StdioConnection {
    /// Create a new StdioConnection
    ///
    /// # Arguments
    /// * `message_sender` - Sender for messages to be processed by the client
    pub fn new(message_sender: UnboundedSender<Message>) -> Self {
        Self {
            acp_client: None,
            process_manager: None,
            session_id: None,
            initialized: false,
            message_sender,
        }
    }

    /// Get a reference to the ACP client
    ///
    /// # Returns
    /// * `Some(&ClientSideConnection)` if the client exists
    /// * `None` if the client is not initialized
    pub fn client(&self) -> Option<&ClientSideConnection> {
        self.acp_client.as_ref()
    }

    /// Get a mutable reference to the ACP client
    ///
    /// # Returns
    /// * `Some(&mut ClientSideConnection)` if the client exists
    /// * `None` if the client is not initialized
    pub fn client_mut(&mut self) -> Option<&mut ClientSideConnection> {
        self.acp_client.as_mut()
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
impl ConnectionHandler for StdioConnection {
    async fn initialize(&mut self, options: &IFlowOptions) -> Result<()> {
        debug!("Initializing stdio connection");

        // Start iFlow process if auto_start is enabled
        let mut process_manager = if options.process.auto_start {
            // For stdio mode, we don't need a port
            let port = options.process.start_port.unwrap_or(8090);
            let mut pm = IFlowProcessManager::new(port, options.process.debug);
            let _url = pm.start(false).await?; // false for stdio
            debug!("iFlow process started");
            Some(pm)
        } else {
            None
        };

        // Get stdin and stdout from the process manager
        let stdin = process_manager
            .as_mut()
            .and_then(|pm| pm.take_stdin())
            .ok_or_else(|| {
                ConnectionError::ConnectionError("Failed to get stdin".to_string())
            })?;

        let stdout = process_manager
            .as_mut()
            .and_then(|pm| pm.take_stdout())
            .ok_or_else(|| {
                ConnectionError::ConnectionError("Failed to get stdout".to_string())
            })?;

        // Create ACP client connection
        let handler = crate::client::acp_handler::IFlowClientHandler::new(
            self.message_sender.clone(),
            None, // logger will be handled at a higher level
        );

        let (conn, handle_io) = ClientSideConnection::new(
            handler,
            stdin.compat_write(),
            stdout.compat(),
            |fut| {
                tokio::task::spawn_local(fut);
            },
        );

        // Handle I/O in the background
        tokio::task::spawn_local(handle_io);

        // Store the client and process manager
        self.acp_client = Some(conn);
        self.process_manager = process_manager;
        self.initialized = true;

        debug!("Initialized stdio connection");
        Ok(())
    }

    async fn create_session(&mut self, options: &IFlowOptions) -> Result<String> {
        if !self.initialized {
            return Err(ConnectionError::NotInitialized.into());
        }

        let client = self.acp_client.as_ref().ok_or_else(|| {
            ConnectionError::ConnectionError("No ACP client available".to_string())
        })?;

        // Initialize the connection if not already done
        if !self.initialized {
            debug!("Initializing connection...");
            client
                .initialize(agent_client_protocol::InitializeRequest {
                    protocol_version: agent_client_protocol::V1,
                    client_capabilities: agent_client_protocol::ClientCapabilities::default(),
                    client_info: Default::default(),
                    meta: None,
                })
                .await
                .map_err(|e| {
                    ConnectionError::ConnectionError(format!("Failed to initialize: {}", e))
                })?;

            self.initialized = true;
            debug!("Initialized stdio connection");
        }

        // Create a new session
        debug!("Creating new session...");
        let session_request = agent_client_protocol::NewSessionRequest {
            mcp_servers: options.mcp_servers.clone(),
            cwd: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
            meta: None,
        };
        debug!("Session request: {:?}", session_request);

        let session_response = client
            .new_session(session_request)
            .await
            .map_err(|e| {
                debug!("Failed to create session: {}", e);
                ConnectionError::ConnectionError(format!("Failed to create session: {}", e))
            })?;

        self.session_id = Some(session_response.session_id.to_string());
        debug!("Created new session: {:?}", self.session_id);

        Ok(session_response.session_id.to_string())
    }

    async fn send_message(&mut self, _session_id: &str, text: &str) -> Result<()> {
        if !self.initialized {
            return Err(ConnectionError::NotInitialized.into());
        }

        let client = self.acp_client.as_ref().ok_or_else(|| {
            ConnectionError::ConnectionError("No ACP client available".to_string())
        })?;

        // Use the existing session
        let current_session_id = self.session_id.as_ref().ok_or_else(|| {
            ConnectionError::NoSession
        })?;

        // Send the prompt and wait for completion
        debug!("Sending prompt to session: {:?}", current_session_id);
        let prompt_response = client
            .prompt(agent_client_protocol::PromptRequest {
                session_id: current_session_id.clone().into(),
                prompt: vec![agent_client_protocol::ContentBlock::Text(
                    agent_client_protocol::TextContent {
                        text: text.to_string(),
                        annotations: None,
                        meta: None,
                    },
                )],
                meta: None,
            })
            .await
            .map_err(|e| {
                debug!("Failed to send message: {}", e);
                ConnectionError::ConnectionError(format!("Failed to send message: {}", e))
            })?;

        debug!(
            "Prompt response received, stop reason: {:?}",
            prompt_response.stop_reason
        );

        // Send task finish message with the actual stop reason
        let message = Message::TaskFinish {
            reason: Some(format!("{:?}", prompt_response.stop_reason)),
        };

        self.message_sender.send(message).map_err(|e| {
            debug!("Failed to send task finish message: {}", e);
            ConnectionError::ConnectionError("Message channel closed".to_string())
        })?;

        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        // Drop the ACP client connection to stop background tasks
        self.acp_client.take();

        // Stop the process if we started it
        if let Some(mut pm) = self.process_manager.take() {
            pm.stop().await?;
        }

        // Add a small delay to allow background tasks to finish
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn is_authenticated(&self) -> bool {
        // For stdio connections, we consider it authenticated if initialized
        self.initialized
    }

    fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
}