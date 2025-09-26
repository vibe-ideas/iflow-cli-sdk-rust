//! Error types for iFlow SDK

use thiserror::Error;

/// Main error type for iFlow SDK
#[derive(Error, Debug)]
pub enum IFlowError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Tool call error: {0}")]
    ToolCall(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Process manager error: {0}")]
    ProcessManager(String),
    
    #[error("Not connected")]
    NotConnected,
    
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for iFlow SDK
pub type Result<T> = std::result::Result<T, IFlowError>;