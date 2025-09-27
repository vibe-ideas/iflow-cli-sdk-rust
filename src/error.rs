//! Error types for iFlow SDK
//!
//! This module defines all the error types that can occur when using the iFlow SDK.

use thiserror::Error;

/// Main error type for iFlow SDK
///
/// This enum encompasses all possible errors that can occur when using the iFlow SDK.
#[derive(Error, Debug)]
pub enum IFlowError {
    /// Connection related errors
    #[error("Connection error: {0}")]
    Connection(String),

    /// Protocol related errors
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Authentication related errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Timeout related errors
    #[error("Timeout error: {0}")]
    Timeout(String),

    /// Tool call related errors
    #[error("Tool call error: {0}")]
    ToolCall(String),

    /// Validation related errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Transport related errors
    #[error("Transport error: {0}")]
    Transport(String),

    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// WebSocket related errors (deprecated)
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// IO related errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Process manager related errors
    #[error("Process manager error: {0}")]
    ProcessManager(String),

    /// Not connected error
    #[error("Not connected")]
    NotConnected,

    /// Session not found error
    #[error("Session not found")]
    SessionNotFound,

    /// Invalid message format error
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for iFlow SDK
///
/// This is a convenience alias for `std::result::Result<T, IFlowError>`.
pub type Result<T> = std::result::Result<T, IFlowError>;
