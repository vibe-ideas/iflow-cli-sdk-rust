//! Main client handler implementation for iFlow SDK
//!
//! This module provides the core client functionality for communicating with iFlow
//! using the [Agent Client Protocol (ACP)](https://github.com/agentclientprotocol/agent-client-protocol) over stdio or WebSocket.

use crate::connection::{ConnectionHandler, StdioConnection, WebSocketConnection};
use crate::error::{IFlowError, Result};
use crate::logger::MessageLogger;
use crate::message::types::Message;
use crate::config::options::IFlowOptions;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

/// Main client for bidirectional communication with iFlow
///
/// This client handles the full lifecycle of communication with iFlow,
/// including process management, connection handling, and message passing.
pub struct IFlowClient {
    options: IFlowOptions,
    message_receiver: Arc<Mutex<mpsc::UnboundedReceiver<Message>>>,
    message_sender: mpsc::UnboundedSender<Message>,
    connected: Arc<Mutex<bool>>,
    connection: Option<Box<dyn ConnectionHandler>>,
    #[allow(dead_code)]
    logger: Option<MessageLogger>,
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
        tracing::debug!("Connecting to iFlow via stdio");

        let mut connection = StdioConnection::new(self.message_sender.clone());
        connection.initialize(&self.options).await?;

        self.connection = Some(Box::new(connection));
        *self.connected.lock().await = true;
        tracing::debug!("Connected to iFlow via stdio");

        Ok(())
    }

    /// Connect to iFlow via WebSocket
    async fn connect_websocket(&mut self) -> Result<()> {
        tracing::debug!("Connecting to iFlow via WebSocket");

        let mut connection = WebSocketConnection::new(
            self.message_sender.clone(),
            self.options.timeout,
        );
        connection.initialize(&self.options).await?;

        self.connection = Some(Box::new(connection));
        *self.connected.lock().await = true;
        tracing::debug!("Connected to iFlow via WebSocket");

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

        // Create a session if needed
        let session_id = if let Some(conn) = self.connection.as_mut() {
            if conn.session_id().is_none() {
                conn.create_session(&self.options).await?
            } else {
                conn.session_id().unwrap().to_string()
            }
        } else {
            return Err(IFlowError::NotConnected);
        };

        // Send the message
        if let Some(conn) = self.connection.as_mut() {
            conn.send_message(&session_id, text).await?;
        } else {
            return Err(IFlowError::NotConnected);
        }

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
    pub fn messages(&self) -> crate::client::stream::MessageStream {
        crate::client::stream::MessageStream::new(self.message_receiver.clone())
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

        if let Some(mut conn) = self.connection.take() {
            conn.close().await?;
        }

        tracing::debug!("Disconnected from iFlow");
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