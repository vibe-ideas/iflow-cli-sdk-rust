//! Main client implementation for iFlow SDK
//!
//! This module provides the core client functionality for communicating with iFlow
//! using the Agent Communication Protocol (ACP) over stdio.

use crate::error::{IFlowError, Result};
use crate::process_manager::IFlowProcessManager;
use crate::types::*;
use agent_client_protocol::{Client, ClientSideConnection, ContentBlock, SessionUpdate};
use futures::{FutureExt, pin_mut, stream::Stream};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::process::ChildStdin;
use tokio::sync::{Mutex, mpsc};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tracing::info;

/// Main client for bidirectional communication with iFlow
///
/// This client handles the full lifecycle of communication with iFlow,
/// including process management, connection handling, and message passing.
pub struct IFlowClient {
    options: IFlowOptions,
    message_receiver: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
    message_sender: mpsc::UnboundedSender<Message>,
    connected: Arc<Mutex<bool>>,
    acp_client: Option<ClientSideConnection>,
    process_manager: Option<IFlowProcessManager>,
    stdin: Option<ChildStdin>,
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

        // 使用异步接收
        match receiver.try_recv() {
            Ok(msg) => Poll::Ready(Some(msg)),
            Err(mpsc::error::TryRecvError::Empty) => {
                // 注册 waker 以便在有新消息时被唤醒
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
                let _ = self.message_sender.send(msg);
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
                let _ = self.message_sender.send(msg);
            }
            SessionUpdate::ToolCall(tool_call) => {
                let msg = Message::ToolCall {
                    id: tool_call.id.0.to_string(),
                    name: tool_call.title.clone(),
                    status: format!("{:?}", tool_call.status),
                };
                let _ = self.message_sender.send(msg);
            }
            SessionUpdate::Plan(plan) => {
                let msg = Message::Plan {
                    entries: plan
                        .entries
                        .into_iter()
                        .map(|entry| entry.content)
                        .collect(),
                };
                let _ = self.message_sender.send(msg);
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

        Self {
            options,
            message_receiver: Arc::new(Mutex::new(receiver)),
            message_sender: sender,
            connected: Arc::new(Mutex::new(false)),
            acp_client: None,
            process_manager: None,
            stdin: None,
        }
    }

    /// Connect to iFlow
    ///
    /// Establishes a connection to iFlow, starting the process if auto_start_process is enabled.
    /// This method handles all the necessary setup for communication via stdio.
    ///
    /// # Returns
    /// * `Ok(())` if the connection was successful
    /// * `Err(IFlowError)` if the connection failed
    pub async fn connect(&mut self) -> Result<()> {
        if *self.connected.lock().await {
            tracing::warn!("Already connected to iFlow");
            return Ok(());
        }

        info!("Connecting to iFlow via stdio");

        // Start iFlow process if auto_start_process is enabled
        if self.options.auto_start_process {
            let mut process_manager = IFlowProcessManager::new(self.options.process_start_port);
            let _url = process_manager.start().await?;
            self.process_manager = Some(process_manager);
            info!("iFlow process started");
        }

        // Get stdin and stdout from the process manager
        let stdin = self
            .process_manager
            .as_mut()
            .and_then(|pm| pm.take_stdin())
            .ok_or_else(|| IFlowError::Connection("Failed to get stdin".to_string()))?;

        let stdout = self
            .process_manager
            .as_mut()
            .and_then(|pm| pm.take_stdout())
            .ok_or_else(|| IFlowError::Connection("Failed to get stdout".to_string()))?;

        // Create ACP client connection
        let handler = IFlowClientHandler {
            message_sender: self.message_sender.clone(),
        };

        let (conn, handle_io) =
            ClientSideConnection::new(handler, stdin.compat_write(), stdout.compat(), |fut| {
                tokio::task::spawn_local(fut);
            });

        // Handle I/O in the background
        tokio::task::spawn_local(handle_io);

        // Store the client
        self.acp_client = Some(conn);

        *self.connected.lock().await = true;
        info!("Connected to iFlow via stdio");

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
    pub async fn send_message(&self, text: &str, _files: Option<Vec<&Path>>) -> Result<()> {
        if !*self.connected.lock().await {
            return Err(IFlowError::NotConnected);
        }

        // Send via ACP client
        if let Some(client) = &self.acp_client {
            use agent_client_protocol::Agent;

            // First initialize the connection
            client
                .initialize(agent_client_protocol::InitializeRequest {
                    protocol_version: agent_client_protocol::V1,
                    client_capabilities: agent_client_protocol::ClientCapabilities::default(),
                    meta: None,
                })
                .await
                .map_err(|e| IFlowError::Connection(format!("Failed to initialize: {}", e)))?;

            // Create a new session
            let session_response = client
                .new_session(agent_client_protocol::NewSessionRequest {
                    mcp_servers: Vec::new(),
                    cwd: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                    meta: None,
                })
                .await
                .map_err(|e| IFlowError::Connection(format!("Failed to create session: {}", e)))?;

            // Send the prompt and wait for completion
            let prompt_response = client
                .prompt(agent_client_protocol::PromptRequest {
                    session_id: session_response.session_id,
                    prompt: vec![text.into()],
                    meta: None,
                })
                .await
                .map_err(|e| IFlowError::Connection(format!("Failed to send message: {}", e)))?;

            // Send task finish message
            let message = Message::TaskFinish {
                reason: Some(format!("{:?}", prompt_response.stop_reason)),
            };

            self.message_sender
                .send(message)
                .map_err(|_| IFlowError::Connection("Message channel closed".to_string()))?;

            info!("Sent message to iFlow: {}", text);
            Ok(())
        } else {
            Err(IFlowError::Connection("Not connected".to_string()))
        }
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

        // Send interrupt via ACP client
        if let Some(_client) = &self.acp_client {
            // For now, we'll just send a task finish message to our own channel
            let message = Message::TaskFinish {
                reason: Some("interrupted".to_string()),
            };

            self.message_sender
                .send(message)
                .map_err(|_| IFlowError::Connection("Message channel closed".to_string()))?;
            Ok(())
        } else {
            Err(IFlowError::Connection("Not connected".to_string()))
        }
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

        // Stop iFlow process if we started it
        if let Some(mut process_manager) = self.process_manager.take() {
            process_manager.stop().await?;
        }

        // Clear resources
        self.acp_client = None;
        self.stdin = None;

        info!("Disconnected from iFlow");
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
