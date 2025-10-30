//! Connection handler trait for iFlow SDK
//!
//! This module defines the abstract interface for connection handling
//! that all connection implementations must adhere to.

use crate::error::Result;
use crate::config::options::IFlowOptions;

/// Error types for connection handling
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    /// Connection is not initialized
    #[error("Connection not initialized")]
    NotInitialized,

    /// Connection is not authenticated
    #[error("Connection not authenticated")]
    NotAuthenticated,

    /// No session available
    #[error("No session available")]
    NoSession,

    /// Underlying connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// Abstract connection handler trait
///
/// This trait defines the interface that all connection implementations
/// must implement to be used with the iFlow client.
#[async_trait::async_trait(?Send)]
pub trait ConnectionHandler: Send {
    /// Initialize the connection
    ///
    /// # Arguments
    /// * `options` - Configuration options for the connection
    ///
    /// # Returns
    /// * `Ok(())` if initialization was successful
    /// * `Err(ConnectionError)` if initialization failed
    async fn initialize(&mut self, options: &IFlowOptions) -> Result<()>;

    /// Create a new session
    ///
    /// # Arguments
    /// * `options` - Configuration options for the session
    ///
    /// # Returns
    /// * `Ok(String)` containing the session ID
    /// * `Err(ConnectionError)` if session creation failed
    async fn create_session(&mut self, options: &IFlowOptions) -> Result<String>;

    /// Send a message to the session
    ///
    /// # Arguments
    /// * `session_id` - The session ID from create_session()
    /// * `text` - The text message to send
    ///
    /// # Returns
    /// * `Ok(())` if the message was sent successfully
    /// * `Err(ConnectionError)` if sending failed
    async fn send_message(&mut self, session_id: &str, text: &str) -> Result<()>;

    /// Close the connection
    ///
    /// # Returns
    /// * `Ok(())` if the connection was closed successfully
    /// * `Err(ConnectionError)` if closing failed
    async fn close(&mut self) -> Result<()>;

    /// Check if the connection is initialized
    ///
    /// # Returns
    /// True if initialized, False otherwise
    fn is_initialized(&self) -> bool;

    /// Check if the connection is authenticated
    ///
    /// # Returns
    /// True if authenticated, False otherwise
    fn is_authenticated(&self) -> bool;

    /// Get the session ID if available
    ///
    /// # Returns
    /// * `Some(&str)` containing the session ID
    /// * `None` if no session exists
    fn session_id(&self) -> Option<&str>;
}