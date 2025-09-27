//! Type definitions for iFlow SDK
//!
//! This module contains all the type definitions used throughout the iFlow SDK,
//! including configuration options and message types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// Import logger configuration
use super::logger::LoggerConfig;

// Re-export the types we need from agent-client-protocol
pub use agent_client_protocol::{
    ContentBlock, EnvVariable, Error, ImageContent, McpServer, Plan, SessionId, StopReason,
    TextContent, ToolCall, ToolCallUpdate,
};

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Configuration options for iFlow SDK
///
/// This struct contains all the configuration options for the iFlow SDK,
/// including connection settings, security options, and logging configuration.
#[derive(Debug, Clone)]
pub struct IFlowOptions {
    /// Current working directory
    pub cwd: PathBuf,
    /// MCP servers to connect to
    pub mcp_servers: Vec<McpServer>,
    /// Request timeout in seconds
    pub timeout: f64,
    /// Log level
    pub log_level: String,
    /// Additional metadata to include in requests
    pub metadata: HashMap<String, serde_json::Value>,
    /// Whether to allow file access
    pub file_access: bool,
    /// Allowed directories for file access
    pub file_allowed_dirs: Option<Vec<PathBuf>>,
    /// Whether file access is read-only
    pub file_read_only: bool,
    /// Maximum file size for file access
    pub file_max_size: u64,
    /// Whether to automatically start the iFlow process
    pub auto_start_process: bool,
    /// Port to start the iFlow process on (deprecated)
    pub process_start_port: u16,
    /// Authentication method ID
    pub auth_method_id: Option<String>,
    /// Logger configuration
    pub log_config: LoggerConfig,
}

impl Default for IFlowOptions {
    fn default() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            mcp_servers: Vec::new(),
            timeout: 30.0,
            log_level: "INFO".to_string(),
            metadata: HashMap::new(),
            file_access: false,
            file_allowed_dirs: None,
            file_read_only: false,
            file_max_size: 10 * 1024 * 1024, // 10MB
            auto_start_process: true,
            process_start_port: 8090,
            auth_method_id: None,
            log_config: LoggerConfig::default(),
        }
    }
}

impl IFlowOptions {
    /// Create a new IFlowOptions instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create options for sandbox mode (deprecated)
    ///
    /// This method is deprecated as sandbox mode is no longer supported with stdio transport.
    pub fn for_sandbox(_sandbox_url: &str) -> Self {
        // Sandbox mode is no longer supported with stdio transport
        Self::default()
    }

    /// Set the request timeout
    ///
    /// # Arguments
    /// * `timeout` - Timeout in seconds
    pub fn with_timeout(mut self, timeout: f64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the current working directory
    ///
    /// # Arguments
    /// * `cwd` - The current working directory
    pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }

    /// Enable or disable file access
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable file access
    pub fn with_file_access(mut self, enabled: bool) -> Self {
        self.file_access = enabled;
        self
    }

    /// Enable or disable automatic process start
    ///
    /// # Arguments
    /// * `enabled` - Whether to automatically start the iFlow process
    pub fn with_auto_start_process(mut self, enabled: bool) -> Self {
        self.auto_start_process = enabled;
        self
    }

    /// Enable or disable logging
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable logging
    pub fn with_logging(mut self, enabled: bool) -> Self {
        self.log_config.enabled = enabled;
        self
    }

    /// Set log file path
    ///
    /// # Arguments
    /// * `path` - The path to the log file
    pub fn with_log_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.log_config.log_file = path.into();
        self
    }

    /// Set maximum log file size (bytes)
    ///
    /// # Arguments
    /// * `size` - Maximum log file size in bytes
    pub fn with_max_log_size(mut self, size: u64) -> Self {
        self.log_config.max_file_size = size;
        self
    }
}

/// Message types for communication with iFlow
///
/// These are the various message types that can be exchanged with iFlow
/// during a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    /// User message
    #[serde(rename = "user")]
    User { content: String },

    /// Assistant message
    #[serde(rename = "assistant")]
    Assistant { content: String },

    /// Tool call message
    #[serde(rename = "tool_call")]
    ToolCall {
        id: String,
        name: String,
        status: String,
    },

    /// Plan message
    #[serde(rename = "plan")]
    Plan { entries: Vec<String> },

    /// Task finish message
    #[serde(rename = "task_finish")]
    TaskFinish { reason: Option<String> },

    /// Error message
    #[serde(rename = "error")]
    Error { code: i32, message: String },
}

impl Message {
    /// Check if this is a task finish message
    pub fn is_task_finish(&self) -> bool {
        matches!(self, Message::TaskFinish { .. })
    }

    /// Check if this is an error message
    pub fn is_error(&self) -> bool {
        matches!(self, Message::Error { .. })
    }

    /// Get the text content of the message if it has any
    ///
    /// # Returns
    /// `Some(&str)` containing the text content if the message has text content,
    /// `None` otherwise
    pub fn get_text(&self) -> Option<&str> {
        match self {
            Message::User { content } => Some(content),
            Message::Assistant { content } => Some(content),
            _ => None,
        }
    }
}
