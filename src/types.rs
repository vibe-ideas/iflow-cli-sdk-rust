//! Type definitions for iFlow SDK

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
#[derive(Debug, Clone)]
pub struct IFlowOptions {
    pub cwd: PathBuf,
    pub mcp_servers: Vec<McpServer>,
    pub timeout: f64,
    pub log_level: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub file_access: bool,
    pub file_allowed_dirs: Option<Vec<PathBuf>>,
    pub file_read_only: bool,
    pub file_max_size: u64,
    pub auto_start_process: bool,
    pub process_start_port: u16,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_sandbox(_sandbox_url: &str) -> Self {
        // Sandbox mode is no longer supported with stdio transport
        Self::default()
    }

    pub fn with_timeout(mut self, timeout: f64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }

    pub fn with_file_access(mut self, enabled: bool) -> Self {
        self.file_access = enabled;
        self
    }

    pub fn with_auto_start_process(mut self, enabled: bool) -> Self {
        self.auto_start_process = enabled;
        self
    }

    /// Enable or disable logging
    pub fn with_logging(mut self, enabled: bool) -> Self {
        self.log_config.enabled = enabled;
        self
    }

    /// Set log file path
    pub fn with_log_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.log_config.log_file = path.into();
        self
    }

    /// Set maximum log file size (bytes)
    pub fn with_max_log_size(mut self, size: u64) -> Self {
        self.log_config.max_file_size = size;
        self
    }
}

/// Message types for communication with iFlow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "user")]
    User { content: String },

    #[serde(rename = "assistant")]
    Assistant { content: String },

    #[serde(rename = "tool_call")]
    ToolCall {
        id: String,
        name: String,
        status: String,
    },

    #[serde(rename = "plan")]
    Plan { entries: Vec<String> },

    #[serde(rename = "task_finish")]
    TaskFinish { reason: Option<String> },

    #[serde(rename = "error")]
    Error { code: i32, message: String },
}

impl Message {
    pub fn is_task_finish(&self) -> bool {
        matches!(self, Message::TaskFinish { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Message::Error { .. })
    }

    pub fn get_text(&self) -> Option<&str> {
        match self {
            Message::User { content } => Some(content),
            Message::Assistant { content } => Some(content),
            _ => None,
        }
    }
}
