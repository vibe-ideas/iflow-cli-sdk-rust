//! Main configuration options for iFlow SDK
//!
//! This module contains the main configuration options for the iFlow SDK,
//! including connection settings, security options, and logging configuration.

use crate::config::{FileAccessConfig, ProcessConfig, LoggingConfig, WebSocketConfig};
use crate::message::types::PermissionMode;
use agent_client_protocol::McpServer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
    /// Additional metadata to include in requests
    pub metadata: HashMap<String, serde_json::Value>,
    /// File access configuration
    pub file_access: FileAccessConfig,
    /// Process management configuration
    pub process: ProcessConfig,
    /// Authentication method ID
    pub auth_method_id: Option<String>,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// WebSocket configuration (if None, use stdio)
    pub websocket: Option<WebSocketConfig>,
    /// Permission mode for tool calls
    pub permission_mode: PermissionMode,
}

impl Default for IFlowOptions {
    fn default() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            mcp_servers: Vec::new(),
            timeout: 120.0,
            metadata: HashMap::new(),
            file_access: FileAccessConfig::default(),
            process: ProcessConfig::default(),
            auth_method_id: None,
            logging: LoggingConfig::default(),
            websocket: None,
            permission_mode: PermissionMode::Auto,
        }
    }
}

impl IFlowOptions {
    /// Create a new IFlowOptions instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current working directory
    ///
    /// # Arguments
    /// * `cwd` - The current working directory
    pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }

    /// Set the request timeout
    ///
    /// # Arguments
    /// * `timeout` - Timeout in seconds
    pub fn with_timeout(mut self, timeout: f64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set MCP servers to connect to
    ///
    /// # Arguments
    /// * `servers` - The MCP servers to connect to
    pub fn with_mcp_servers(mut self, servers: Vec<McpServer>) -> Self {
        self.mcp_servers = servers;
        self
    }

    /// Set additional metadata to include in requests
    ///
    /// # Arguments
    /// * `metadata` - The metadata to include
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Set file access configuration
    ///
    /// # Arguments
    /// * `config` - The file access configuration
    pub fn with_file_access_config(mut self, config: FileAccessConfig) -> Self {
        self.file_access = config;
        self
    }

    /// Set process management configuration
    ///
    /// # Arguments
    /// * `config` - The process management configuration
    pub fn with_process_config(mut self, config: ProcessConfig) -> Self {
        self.process = config;
        self
    }

    /// Set auto start process
    ///
    /// # Arguments
    /// * `auto_start` - Whether to automatically start the iFlow process
    pub fn with_auto_start(mut self, auto_start: bool) -> Self {
        self.process.auto_start = auto_start;
        self
    }

    /// Set authentication method ID
    ///
    /// # Arguments
    /// * `method_id` - The authentication method ID
    pub fn with_auth_method_id(mut self, method_id: String) -> Self {
        self.auth_method_id = Some(method_id);
        self
    }

    /// Set logging configuration
    ///
    /// # Arguments
    /// * `config` - The logging configuration
    pub fn with_logging_config(mut self, config: LoggingConfig) -> Self {
        self.logging = config;
        self
    }

    /// Set WebSocket configuration
    ///
    /// # Arguments
    /// * `config` - The WebSocket configuration
    pub fn with_websocket_config(mut self, config: WebSocketConfig) -> Self {
        self.websocket = Some(config);
        self
    }

    /// Set permission mode for tool calls
    ///
    /// # Arguments
    /// * `mode` - The permission mode to use
    pub fn with_permission_mode(mut self, mode: PermissionMode) -> Self {
        self.permission_mode = mode;
        self
    }
}

/// Error message details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessageDetails {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl ErrorMessageDetails {
    /// Create a new error message details
    ///
    /// # Arguments
    /// * `code` - The error code
    /// * `message` - The error message
    ///
    /// # Returns
    /// A new ErrorMessageDetails instance
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            details: None,
        }
    }

    /// Create a new error message details with details
    ///
    /// # Arguments
    /// * `code` - The error code
    /// * `message` - The error message
    /// * `details` - Additional error details
    ///
    /// # Returns
    /// A new ErrorMessageDetails instance
    pub fn with_details(
        code: i32,
        message: String,
        details: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            code,
            message,
            details: Some(details),
        }
    }
}