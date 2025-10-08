//! Main client implementation for iFlow SDK
//!
//! This module provides the core client functionality for communicating with iFlow
//! using the Agent Communication Protocol (ACP) over stdio or WebSocket.

use crate::acp_protocol::ACPProtocol;
use crate::error::{IFlowError, Result};
use crate::logger::MessageLogger;
use crate::process_manager::IFlowProcessManager;
use crate::types::*;
use serde_json;
use crate::websocket_transport::WebSocketTransport;
use agent_client_protocol::{
    Agent, Client, ClientSideConnection, ContentBlock, SessionId, SessionUpdate,
};
use futures::{FutureExt, pin_mut, stream::Stream};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

// ChildStdin import moved to where it's used
use tokio::sync::{Mutex, mpsc};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tracing::debug;

/// Connection type for iFlow client
enum Connection {
    /// Stdio connection using agent-client-protocol
    Stdio {
        acp_client: ClientSideConnection,
        process_manager: Option<IFlowProcessManager>,
        session_id: Option<SessionId>,
        initialized: bool,
    },
    /// WebSocket connection using custom implementation
    WebSocket {
        acp_protocol: ACPProtocol,
        session_id: Option<String>,
        process_manager: Option<IFlowProcessManager>,
    },
}

/// Main client for bidirectional communication with iFlow
///
/// This client handles the full lifecycle of communication with iFlow,
/// including process management, connection handling, and message passing.
pub struct IFlowClient {
    options: IFlowOptions,
    message_receiver: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
    message_sender: mpsc::UnboundedSender<Message>,
    connected: Arc<Mutex<bool>>,
    connection: Option<Connection>,
    logger: Option<MessageLogger>,
}

/// Stream of messages from iFlow
///
/// This stream provides asynchronous access to messages received from iFlow.
/// It implements the `futures::Stream` trait for easy integration with async code.
pub struct MessageStream {
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
}

impl Stream for MessageStream {
    type Item = Message;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut receiver = match self.receiver.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        // Use asynchronous receiving
        match receiver.try_recv() {
            Ok(msg) => Poll::Ready(Some(msg)),
            Err(mpsc::error::TryRecvError::Empty) => {
                // Register a waker to be notified when new messages arrive
                let recv_future = receiver.recv();
                pin_mut!(recv_future);
                match recv_future.poll_unpin(cx) {
                    Poll::Ready(msg) => Poll::Ready(msg),
                    Poll::Pending => Poll::Pending,
                }
            }
            Err(mpsc::error::TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}

// Implement the Client trait for handling ACP messages
struct IFlowClientHandler {
    message_sender: mpsc::UnboundedSender<Message>,
    logger: Option<MessageLogger>,
}

#[async_trait::async_trait(?Send)]
impl Client for IFlowClientHandler {
    async fn request_permission(
        &self,
        _args: agent_client_protocol::RequestPermissionRequest,
    ) -> anyhow::Result<
        agent_client_protocol::RequestPermissionResponse,
        agent_client_protocol::Error,
    > {
        // For now, cancel all permissions
        Ok(agent_client_protocol::RequestPermissionResponse {
            outcome: agent_client_protocol::RequestPermissionOutcome::Cancelled,
            meta: None,
        })
    }

    async fn write_text_file(
        &self,
        _args: agent_client_protocol::WriteTextFileRequest,
    ) -> anyhow::Result<agent_client_protocol::WriteTextFileResponse, agent_client_protocol::Error>
    {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn read_text_file(
        &self,
        _args: agent_client_protocol::ReadTextFileRequest,
    ) -> anyhow::Result<agent_client_protocol::ReadTextFileResponse, agent_client_protocol::Error>
    {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn create_terminal(
        &self,
        _args: agent_client_protocol::CreateTerminalRequest,
    ) -> anyhow::Result<agent_client_protocol::CreateTerminalResponse, agent_client_protocol::Error>
    {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn terminal_output(
        &self,
        _args: agent_client_protocol::TerminalOutputRequest,
    ) -> anyhow::Result<agent_client_protocol::TerminalOutputResponse, agent_client_protocol::Error>
    {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn release_terminal(
        &self,
        _args: agent_client_protocol::ReleaseTerminalRequest,
    ) -> anyhow::Result<agent_client_protocol::ReleaseTerminalResponse, agent_client_protocol::Error>
    {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn wait_for_terminal_exit(
        &self,
        _args: agent_client_protocol::WaitForTerminalExitRequest,
    ) -> anyhow::Result<
        agent_client_protocol::WaitForTerminalExitResponse,
        agent_client_protocol::Error,
    > {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn kill_terminal_command(
        &self,
        _args: agent_client_protocol::KillTerminalCommandRequest,
    ) -> anyhow::Result<
        agent_client_protocol::KillTerminalCommandResponse,
        agent_client_protocol::Error,
    > {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn session_notification(
        &self,
        args: agent_client_protocol::SessionNotification,
    ) -> anyhow::Result<(), agent_client_protocol::Error> {
        match args.update {
            SessionUpdate::AgentMessageChunk { content } => {
                let text = match content {
                    ContentBlock::Text(text_content) => text_content.text,
                    ContentBlock::Image(_) => "<image>".into(),
                    ContentBlock::Audio(_) => "<audio>".into(),
                    ContentBlock::ResourceLink(resource_link) => resource_link.uri,
                    ContentBlock::Resource(_) => "<resource>".into(),
                };
                let msg = Message::Assistant { content: text };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::UserMessageChunk { content } => {
                let text = match content {
                    ContentBlock::Text(text_content) => text_content.text,
                    ContentBlock::Image(_) => "<image>".into(),
                    ContentBlock::Audio(_) => "<audio>".into(),
                    ContentBlock::ResourceLink(resource_link) => resource_link.uri,
                    ContentBlock::Resource(_) => "<resource>".into(),
                };
                let msg = Message::User { content: text };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::ToolCall(tool_call) => {
                let msg = Message::ToolCall {
                    id: tool_call.id.0.to_string(),
                    name: tool_call.title.clone(),
                    status: format!("{:?}", tool_call.status),
                };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::Plan(plan) => {
                let entries = plan
                    .entries
                    .into_iter()
                    .map(|entry| {
                        // Convert agent-client-protocol PlanEntry to our PlanEntry
                        super::types::PlanEntry {
                            content: entry.content,
                            priority: match entry.priority {
                                agent_client_protocol::PlanEntryPriority::High => {
                                    super::types::PlanPriority::High
                                }
                                agent_client_protocol::PlanEntryPriority::Medium => {
                                    super::types::PlanPriority::Medium
                                }
                                agent_client_protocol::PlanEntryPriority::Low => {
                                    super::types::PlanPriority::Low
                                }
                            },
                            status: match entry.status {
                                agent_client_protocol::PlanEntryStatus::Pending => {
                                    super::types::PlanStatus::Pending
                                }
                                agent_client_protocol::PlanEntryStatus::InProgress => {
                                    super::types::PlanStatus::InProgress
                                }
                                agent_client_protocol::PlanEntryStatus::Completed => {
                                    super::types::PlanStatus::Completed
                                }
                            },
                        }
                    })
                    .collect();

                let msg = Message::Plan { entries };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::AgentThoughtChunk { .. }
            | SessionUpdate::ToolCallUpdate(_)
            | SessionUpdate::CurrentModeUpdate { .. }
            | SessionUpdate::AvailableCommandsUpdate { .. } => {
                // Ignore these for now
            }
        }
        Ok(())
    }

    async fn ext_method(
        &self,
        _args: agent_client_protocol::ExtRequest,
    ) -> anyhow::Result<agent_client_protocol::ExtResponse, agent_client_protocol::Error> {
        Err(agent_client_protocol::Error::method_not_found())
    }

    async fn ext_notification(
        &self,
        _args: agent_client_protocol::ExtNotification,
    ) -> anyhow::Result<(), agent_client_protocol::Error> {
        Err(agent_client_protocol::Error::method_not_found())
    }
}

impl IFlowClient {
    /// Create a new iFlow client
    ///
    /// # Arguments
    /// * `options` - Optional configuration for the client. If None, defaults will be used.
    ///
    /// # Returns
    /// A new IFlowClient instance
    pub fn new(options: Option<IFlowOptions>) -> Self {
        let options = options.unwrap_or_default();
        let (sender, receiver) = mpsc::unbounded_channel();

        // Initialize logger if enabled
        let logger = if options.logging.enabled {
            MessageLogger::new(options.logging.logger_config.clone()).ok()
        } else {
            None
        };

        Self {
            options,
            message_receiver: Arc::new(Mutex::new(receiver)),
            message_sender: sender,
            connected: Arc::new(Mutex::new(false)),
            connection: None,
            logger,
        }
    }

    /// Connect to iFlow
    ///
    /// Establishes a connection to iFlow, starting the process if auto_start_process is enabled.
    /// This method handles all the necessary setup for communication via stdio or WebSocket.
    ///
    /// # Returns
    /// * `Ok(())` if the connection was successful
    /// * `Err(IFlowError)` if the connection failed
    pub async fn connect(&mut self) -> Result<()> {
        if *self.connected.lock().await {
            tracing::warn!("Already connected to iFlow");
            return Ok(());
        }

        // Check if we should use WebSocket or stdio
        if self.options.websocket.is_some() {
            self.connect_websocket().await
        } else {
            self.connect_stdio().await
        }
    }

    /// Connect to iFlow via stdio
    async fn connect_stdio(&mut self) -> Result<()> {
        debug!("Connecting to iFlow via stdio");

        // Start iFlow process if auto_start is enabled
        let mut process_manager = if self.options.process.auto_start {
            // For stdio mode, we don't need a port
            let port = self.options.process.start_port.unwrap_or(8090);
            let mut pm = IFlowProcessManager::new(port, self.options.process.debug);
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
            .ok_or_else(|| IFlowError::Connection("Failed to get stdin".to_string()))?;

        let stdout = process_manager
            .as_mut()
            .and_then(|pm| pm.take_stdout())
            .ok_or_else(|| IFlowError::Connection("Failed to get stdout".to_string()))?;

        // Create ACP client connection
        let handler = IFlowClientHandler {
            message_sender: self.message_sender.clone(),
            logger: self.logger.clone(),
        };

        let (conn, handle_io) =
            ClientSideConnection::new(handler, stdin.compat_write(), stdout.compat(), |fut| {
                tokio::task::spawn_local(fut);
            });

        // Handle I/O in the background
        tokio::task::spawn_local(handle_io);

        // Store the client
        self.connection = Some(Connection::Stdio {
            acp_client: conn,
            process_manager,
            session_id: None,
            initialized: false,
        });

        *self.connected.lock().await = true;
        debug!("Connected to iFlow via stdio");

        Ok(())
    }

    /// Connect to iFlow via WebSocket
    async fn connect_websocket(&mut self) -> Result<()> {
        debug!("Connecting to iFlow via WebSocket");

        let websocket_config = self.options.websocket.as_ref().ok_or_else(|| {
            IFlowError::Connection("WebSocket configuration not provided".to_string())
        })?;

        // Keep the process manager when auto-start is needed
        let mut process_manager_to_keep: Option<IFlowProcessManager> = None;

        // For manual start mode, directly use the provided WebSocket URL
        // For auto start mode, try to connect first and start process if needed
        let final_url = if self.options.process.auto_start {
            if let Some(url) = &websocket_config.url {
                // If URL is provided, check if it's a local URL and try to connect first
                if url.starts_with("ws://localhost:") {
                    debug!(
                        "iFlow auto-start enabled with provided URL, checking if iFlow is already running..."
                    );

                    // Try to connect first to see if iFlow is already running
                    let mut test_transport =
                        WebSocketTransport::new(url.clone(), self.options.timeout);
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
                            "iFlow appears to be running on port {}, but connection failed: {}", port, e
                        );
                                debug!(
                                    "Since iFlow is running on the specified port, we won't start a new process. Please check if the existing iFlow instance is configured correctly for WebSocket connections."
                                );
                                return Err(IFlowError::Connection(format!(
                                    "Failed to connect to existing iFlow process at {}: {}. iFlow appears to be running on port {}, but connection could not be established.",
                                    url, e, port
                                )));
                            } else {
                                // Port is not listening, iFlow is not running, start it
                                debug!(
                                    "iFlow not running on port {}, starting process", port
                                );
                                let mut pm = IFlowProcessManager::new(port, self.options.process.debug);
                                let iflow_url = pm.start(true).await?.ok_or_else(|| {
                                    IFlowError::Connection(
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
                let port = self.options.process.start_port.unwrap_or(8090);
                let mut pm = IFlowProcessManager::new(port, self.options.process.debug);
                let iflow_url = pm.start(true).await?.ok_or_else(|| {
                    IFlowError::Connection("Failed to start iFlow with WebSocket".to_string())
                })?;
                debug!("Started iFlow process at {}", iflow_url);

                // Keep the process manager to avoid early handle drop causing child process exit due to stdout/stderr pipe issues
                process_manager_to_keep = Some(pm);

                iflow_url
            }
        } else {
            // Manual start mode, URL must be provided
            let url = websocket_config.url.as_ref().ok_or_else(|| {
                IFlowError::Connection(
                    "WebSocket URL must be provided in manual start mode".to_string(),
                )
            })?;
            debug!("Using manual start mode with WebSocket URL: {}", url);
            url.clone()
        };

        // Create WebSocket transport with increased timeout
        let mut transport = WebSocketTransport::new(final_url.clone(), self.options.timeout);

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
                        return Err(IFlowError::Connection(format!(
                            "Failed to connect to WebSocket after {} attempts: {}",
                            websocket_config.reconnect_attempts, e
                        )));
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
        let mut acp_protocol =
            ACPProtocol::new(transport, self.message_sender.clone(), self.options.timeout);
        acp_protocol.set_permission_mode(self.options.permission_mode);

        // Store the connection (now also holds process_manager)
        self.connection = Some(Connection::WebSocket {
            acp_protocol,
            session_id: None,
            process_manager: process_manager_to_keep,
        });

        *self.connected.lock().await = true;
        debug!("Connected to iFlow via WebSocket");

        Ok(())
    }

    /// Send a message to iFlow
    ///
    /// Sends a text message to iFlow and handles the complete request-response cycle.
    /// This method initializes the connection, creates a new session, sends the prompt,
    /// and waits for completion before returning.
    ///
    /// # Arguments
    /// * `text` - The text message to send to iFlow
    /// * `_files` - Optional files to send (currently not implemented)
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(IFlowError)` if there was an error
    pub async fn send_message(&mut self, text: &str, _files: Option<Vec<&Path>>) -> Result<()> {
        if !*self.connected.lock().await {
            return Err(IFlowError::NotConnected);
        }

        let is_websocket = matches!(self.connection, Some(Connection::WebSocket { .. }));

        if is_websocket {
            // Adapt to the newly added process_manager field
            if let Some(Connection::WebSocket {
                mut acp_protocol,
                mut session_id,
                process_manager,
            }) = self.connection.take()
            {
                let pm = process_manager;
                let result = self
                    .send_message_websocket(&mut acp_protocol, &mut session_id, text)
                    .await;
                self.connection = Some(Connection::WebSocket {
                    acp_protocol,
                    session_id,
                    process_manager: pm,
                });
                result
            } else {
                Err(IFlowError::NotConnected)
            }
        } else {
            // Handle Stdio connection by temporarily taking ownership
            if let Some(Connection::Stdio {
                acp_client,
                process_manager,
                mut session_id,
                mut initialized,
            }) = self.connection.take()
            {
                let result = self
                    .send_message_stdio(&acp_client, &mut session_id, &mut initialized, text)
                    .await;
                self.connection = Some(Connection::Stdio {
                    acp_client,
                    process_manager,
                    session_id,
                    initialized,
                });
                result
            } else {
                Err(IFlowError::NotConnected)
            }
        }
    }

    /// Send a message via stdio connection
    async fn send_message_stdio(
        &self,
        client: &ClientSideConnection,
        session_id: &mut Option<SessionId>,
        initialized: &mut bool,
        text: &str,
    ) -> Result<()> {
        tracing::debug!("send_message_stdio called with text: {}", text);

        // Initialize the connection if not already done
        if !*initialized {
            tracing::debug!("Initializing connection...");
            client
                .initialize(agent_client_protocol::InitializeRequest {
                    protocol_version: agent_client_protocol::V1,
                    client_capabilities: agent_client_protocol::ClientCapabilities::default(),
                    meta: None,
                })
                .await
                .map_err(|e| IFlowError::Connection(format!("Failed to initialize: {}", e)))?;

            *initialized = true;
            debug!("Initialized stdio connection");
        }

        // Create a new session if we don't have one
        if session_id.is_none() {
            tracing::debug!("Creating new session...");
            let session_request = agent_client_protocol::NewSessionRequest {
                mcp_servers: self.options.mcp_servers.clone(),
                cwd: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                meta: None,
            };
            tracing::debug!("Session request: {:?}", session_request);

            let session_response = client.new_session(session_request).await.map_err(|e| {
                tracing::error!("Failed to create session: {}", e);
                IFlowError::Connection(format!("Failed to create session: {}", e))
            })?;

            *session_id = Some(session_response.session_id);
            debug!("Created new session: {:?}", session_id);
        }

        // Use the existing session
        let current_session_id = session_id.as_ref().unwrap();

        // Send the prompt and wait for completion
        tracing::debug!("Sending prompt to session: {:?}", current_session_id);
        let prompt_response = client
            .prompt(agent_client_protocol::PromptRequest {
                session_id: current_session_id.clone(),
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
                tracing::error!("Failed to send message: {}", e);
                IFlowError::Connection(format!("Failed to send message: {}", e))
            })?;

        tracing::debug!(
                    "Prompt response received, stop reason: {:?}", prompt_response.stop_reason
                );

        // Send task finish message with the actual stop reason
        let message = Message::TaskFinish {
            reason: Some(format!("{:?}", prompt_response.stop_reason)),
        };

        self.message_sender.send(message).map_err(|e| {
            tracing::error!("Failed to send task finish message: {}", e);
            IFlowError::Connection("Message channel closed".to_string())
        })?;

        debug!("Sent message to iFlow via stdio: {}", text);
        Ok(())
    }

    /// Send a message via WebSocket connection
    async fn send_message_websocket(
        &mut self,
        protocol: &mut ACPProtocol,
        session_id: &mut Option<String>,
        text: &str,
    ) -> Result<()> {
        // Initialize the protocol if not already done
        if !protocol.is_initialized() {
            tracing::debug!("Initializing WebSocket protocol...");
            protocol.initialize(&self.options).await.map_err(|e| {
                tracing::error!("Failed to initialize protocol: {}", e);
                e
            })?;

            // Authenticate if needed
            if !protocol.is_authenticated() {
                tracing::debug!("Authenticating...");
                if let Some(method_id) = &self.options.auth_method_id {
                    protocol.authenticate(method_id, None).await.map_err(|e| {
                        tracing::error!("Authentication failed with method {}: {}", method_id, e);
                        e
                    })?;
                } else {
                    // Try default authentication
                    protocol.authenticate("iflow", None).await.map_err(|e| {
                        tracing::error!("Default authentication failed: {}", e);
                        e
                    })?;
                }
            }

            // Create a new session
            tracing::debug!("Creating new session...");
            let current_dir = std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string();
            
            // Convert McpServer objects to JSON-compatible format
            let mcp_servers: Vec<serde_json::Value> = self.options
                .mcp_servers
                .iter()
                .map(|server| {
                    // Since McpServer is an enum, we need to serialize it directly
                    // The agent-client-protocol crate handles the serialization
                    serde_json::json!(server)
                })
                .collect();
            
            let new_session_id = protocol.create_session(&current_dir, mcp_servers).await.map_err(|e| {
                tracing::error!("Failed to create session: {}", e);
                e
            })?;
            *session_id = Some(new_session_id);
            tracing::debug!("Session created successfully");
        }

        // Make sure we have a session
        let current_session_id = session_id
            .as_ref()
            .ok_or_else(|| IFlowError::Connection("No session available".to_string()))?;

        // Send the prompt and get the request ID
        tracing::debug!("Sending prompt to session: {}", current_session_id);
        let _request_id = protocol
            .send_prompt(current_session_id, text)
            .await
            .map_err(|e| {
                tracing::error!("Failed to send prompt: {}", e);
                e
            })?;

        debug!("Sent message to iFlow: {}", text);
        Ok(())
    }

    /// Interrupt the current message generation
    ///
    /// Sends an interrupt signal to stop the current message generation.
    /// This is useful for canceling long-running requests.
    ///
    /// # Returns
    /// * `Ok(())` if the interrupt was sent successfully
    /// * `Err(IFlowError)` if there was an error
    pub async fn interrupt(&self) -> Result<()> {
        if !*self.connected.lock().await {
            return Err(IFlowError::NotConnected);
        }

        let message = Message::TaskFinish {
            reason: Some("interrupted".to_string()),
        };

        self.message_sender
            .send(message)
            .map_err(|_| IFlowError::Connection("Message channel closed".to_string()))?;
        Ok(())
    }

    /// Receive messages from iFlow
    ///
    /// Returns a stream of messages from iFlow that can be used with async iteration.
    ///
    /// # Returns
    /// A `MessageStream` that implements `futures::Stream`
    pub fn messages(&self) -> MessageStream {
        MessageStream {
            receiver: self.message_receiver.clone(),
        }
    }

    /// Receive a single message (convenience method)
    ///
    /// Waits for and returns the next message from iFlow.
    ///
    /// # Returns
    /// * `Ok(Some(Message))` if a message was received
    /// * `Ok(None)` if the channel is closed
    /// * `Err(IFlowError)` if there was an error
    pub async fn receive_message(&self) -> Result<Option<Message>> {
        let mut receiver = self.message_receiver.lock().await;
        Ok(receiver.recv().await)
    }

    /// Disconnect from iFlow
    ///
    /// Cleans up the connection to iFlow and stops the process if it was started by this client.
    /// This method ensures proper cleanup of resources.
    ///
    /// # Returns
    /// * `Ok(())` if the disconnection was successful
    /// * `Err(IFlowError)` if there was an error
    pub async fn disconnect(&mut self) -> Result<()> {
        *self.connected.lock().await = false;

        // Take ownership of the connection to ensure proper cleanup
        if let Some(connection) = self.connection.take() {
            match connection {
                Connection::Stdio {
                    acp_client,
                    mut process_manager,
                    session_id: _,
                    initialized: _,
                } => {
                    // Drop the ACP client connection to stop background tasks
                    drop(acp_client);

                    // Stop the process if we started it
                    if let Some(mut pm) = process_manager.take() {
                        pm.stop().await?;
                    }

                    // Add a small delay to allow background tasks to finish
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                Connection::WebSocket {
                    mut acp_protocol,
                    mut process_manager,
                    session_id: _,
                } => {
                    let _ = acp_protocol.close().await;
                    // if we started the process, stop it
                    if let Some(mut pm) = process_manager.take() {
                        pm.stop().await?;
                    }
                }
            }
        }

        debug!("Disconnected from iFlow");
        Ok(())
    }
}

impl Drop for IFlowClient {
    fn drop(&mut self) {
        // Ensure we're marked as disconnected
        if let Ok(mut connected) = self.connected.try_lock() {
            *connected = false;
        }
    }
}

// Type alias for JSON values from serde_json
pub use serde_json::Value as JsonValue;
