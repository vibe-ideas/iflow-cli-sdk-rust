//! WebSocket transport implementation for iFlow SDK
//!
//! This module provides the low-level WebSocket communication layer.
//! It handles connection management, message sending/receiving, and
//! basic error handling.

use crate::error::{IFlowError, Result};
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::time::Duration;
use tokio_tungstenite::{WebSocketStream, connect_async, tungstenite::protocol::Message};
use tracing::debug;
use url::Url;

/// WebSocket transport for iFlow communication
///
/// This class provides a low-level WebSocket interface for communicating
/// with iFlow. It handles connection management, message serialization,
/// and error recovery.
pub struct WebSocketTransport {
    /// WebSocket URL to connect to
    url: String,
    /// Active WebSocket connection (if connected)
    websocket: Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    /// Whether the transport is currently connected
    connected: bool,
    /// Connection timeout in seconds
    timeout: f64,
}

impl WebSocketTransport {
    /// Initialize WebSocket transport
    ///
    /// # Arguments
    /// * `url` - WebSocket URL (e.g., ws://localhost:8090/acp?peer=iflow)
    /// * `timeout` - Connection timeout in seconds
    pub fn new(url: String, timeout: f64) -> Self {
        Self {
            url,
            websocket: None,
            connected: false,
            timeout,
        }
    }

    /// Establish WebSocket connection
    ///
    /// # Returns
    /// * `Ok(())` if the connection was successful
    /// * `Err(IFlowError)` if the connection failed
    pub async fn connect(&mut self) -> Result<()> {
        if self.connected {
            tracing::warn!("Already connected to {}", self.url);
            return Ok(());
        }

        debug!("Connecting to {}", self.url);

        // Parse URL to validate it
        let url = Url::parse(&self.url)
            .map_err(|e| IFlowError::Connection(format!("Invalid URL: {}", e)))?;

        // Attempt to connect with timeout
        let (ws_stream, _) =
            tokio::time::timeout(Duration::from_secs_f64(self.timeout), connect_async(url))
                .await
                .map_err(|_| IFlowError::Timeout("Connection timeout".to_string()))?
                .map_err(|e| {
                    IFlowError::Connection(format!("WebSocket connection failed: {}", e))
                })?;

        self.websocket = Some(ws_stream);
        self.connected = true;
        debug!("Connected to {}", self.url);

        Ok(())
    }

    /// Send a message through WebSocket
    ///
    /// # Arguments
    /// * `message` - Message to send (string or JSON Value)
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(IFlowError)` if there was an error
    pub async fn send(&mut self, message: &Value) -> Result<()> {
        if !self.connected {
            return Err(IFlowError::NotConnected);
        }

        let ws_stream = self.websocket.as_mut().ok_or(IFlowError::NotConnected)?;

        // Serialize message to JSON string
        let data = serde_json::to_string(message).map_err(|e| IFlowError::JsonParse(e))?;

        // Send the message
        ws_stream
            .send(Message::Text(data.clone()))
            .await
            .map_err(|e| IFlowError::Transport(format!("Failed to send message: {}", e)))?;

        tracing::debug!(
            "Sent message: {}", data
        );

        Ok(())
    }

    /// Send a raw string message through WebSocket
    ///
    /// # Arguments
    /// * `message` - Raw message string to send
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(IFlowError)` if there was an error
    pub async fn send_raw(&mut self, message: &str) -> Result<()> {
        if !self.connected {
            return Err(IFlowError::NotConnected);
        }

        let ws_stream = self.websocket.as_mut().ok_or(IFlowError::NotConnected)?;

        // Send the message
        ws_stream
            .send(Message::Text(message.to_string()))
            .await
            .map_err(|e| IFlowError::Transport(format!("Failed to send message: {}", e)))?;

        tracing::debug!(
            "Sent raw message: {}",
            message.to_string()
        );

        Ok(())
    }

    /// Receive messages from WebSocket
    ///
    /// This method receives a single message from the WebSocket connection.
    ///
    /// # Returns
    /// * `Ok(String)` containing the received message
    /// * `Err(IFlowError)` if there was an error
    pub async fn receive(&mut self) -> Result<String> {
        if !self.connected {
            return Err(IFlowError::NotConnected);
        }

        let ws_stream = self.websocket.as_mut().ok_or(IFlowError::NotConnected)?;

        // Receive the next message with proper error handling
        loop {
            let msg = match ws_stream.next().await {
                Some(Ok(msg)) => msg,
                Some(Err(e)) => {
                    tracing::error!("WebSocket error: {}", e);
                    self.connected = false;
                    return Err(IFlowError::Transport(format!(
                        "Failed to receive message: {}",
                        e
                    )));
                }
                None => {
                    tracing::debug!("WebSocket connection closed");
                    self.connected = false;
                    return Err(IFlowError::Connection("Connection closed".to_string()));
                }
            };

            match msg {
                Message::Text(text) => {
                    // Clean up the text - remove any non-printable characters at the beginning
                    let cleaned_text = text.trim_start_matches(|c: char| {
                        !c.is_ascii() || c.is_control() && c != '\n' && c != '\r' && c != '\t'
                    });
                    tracing::debug!(
                        "Received message: {}", cleaned_text.to_string()
                    );
                    return Ok(cleaned_text.to_string());
                }
                Message::Binary(data) => {
                    // Convert binary to string if possible
                    match String::from_utf8(data) {
                        Ok(text) => return Ok(text),
                        Err(_) => {
                            tracing::debug!("Received binary message, ignoring");
                            continue;
                        }
                    }
                }
                Message::Ping(data) => {
                    // Respond to ping with pong
                    tracing::debug!("Received ping, sending pong");
                    if let Err(e) = ws_stream.send(Message::Pong(data)).await {
                        tracing::error!("Failed to send pong: {}", e);
                        self.connected = false;
                        return Err(IFlowError::Transport(format!("Failed to send pong: {}", e)));
                    }
                    continue;
                }
                Message::Pong(_) => {
                    tracing::debug!("Received pong");
                    continue;
                }
                Message::Close(close_frame) => {
                    tracing::debug!("Received close frame: {:?}", close_frame);
                    self.connected = false;
                    return Err(IFlowError::Connection(
                        "Connection closed by server".to_string(),
                    ));
                }
                Message::Frame(_) => {
                    tracing::debug!("Received raw frame, ignoring");
                    continue;
                }
            }
        }
    }

    /// Close WebSocket connection gracefully
    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut ws_stream) = self.websocket.take() {
            ws_stream
                .close(None)
                .await
                .map_err(|e| IFlowError::Transport(format!("Error closing WebSocket: {}", e)))?;
            debug!("WebSocket connection closed");
        }
        self.connected = false;
        Ok(())
    }

    /// Check if the WebSocket is connected
    ///
    /// # Returns
    /// True if connected, False otherwise
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get the WebSocket URL
    ///
    /// # Returns
    /// The WebSocket URL
    pub fn url(&self) -> &str {
        &self.url
    }
}
