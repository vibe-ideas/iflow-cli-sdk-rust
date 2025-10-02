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

/// Permission mode for tool calls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionMode {
    /// Automatically approve all tool calls
    #[serde(rename = "auto")]
    Auto,
    /// Require manual confirmation for all tool calls
    #[serde(rename = "manual")]
    Manual,
    /// Auto-approve certain types of tool calls
    #[serde(rename = "selective")]
    Selective,
}

impl Default for PermissionMode {
    fn default() -> Self {
        PermissionMode::Auto
    }
}

/// Tool call status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCallStatus {
    /// The tool call is pending
    #[serde(rename = "pending")]
    Pending,
    /// The tool call is in progress
    #[serde(rename = "in_progress")]
    InProgress,
    /// The tool call has completed successfully
    #[serde(rename = "completed")]
    Completed,
    /// The tool call has failed
    #[serde(rename = "failed")]
    Failed,
    /// Legacy alias for InProgress
    #[serde(rename = "running")]
    Running,
    /// Legacy alias for Completed
    #[serde(rename = "finished")]
    Finished,
    /// Legacy alias for Failed
    #[serde(rename = "error")]
    Error,
}

/// Plan entry priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlanPriority {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

impl Default for PlanPriority {
    fn default() -> Self {
        PlanPriority::Medium
    }
}

/// Plan entry status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlanStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl Default for PlanStatus {
    fn default() -> Self {
        PlanStatus::Pending
    }
}

/// Plan entry for task planning
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanEntry {
    /// The content of the plan entry
    pub content: String,
    /// The priority of the plan entry
    #[serde(default)]
    pub priority: PlanPriority,
    /// The status of the plan entry
    #[serde(default)]
    pub status: PlanStatus,
}

/// User message chunk
///
/// A chunk of a user message, which can be either text or a file path.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserMessageChunk {
    /// Text content
    Text {
        #[serde(rename = "text")]
        content: String,
    },
    /// File path content
    Path {
        #[serde(rename = "path")]
        path: PathBuf,
    },
}

/// User message
///
/// A user message consisting of one or more chunks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    /// The type of the message (always "user")
    #[serde(rename = "type")]
    pub message_type: String,
    /// The chunks of the message
    pub chunks: Vec<UserMessageChunk>,
}

impl UserMessage {
    /// Create a new user message with text content
    ///
    /// # Arguments
    /// * `text` - The text content of the message
    ///
    /// # Returns
    /// A new UserMessage instance
    pub fn new_text(text: String) -> Self {
        Self {
            message_type: "user".to_string(),
            chunks: vec![UserMessageChunk::Text { content: text }],
        }
    }

    /// Create a new user message with a file path
    ///
    /// # Arguments
    /// * `path` - The path to the file
    ///
    /// # Returns
    /// A new UserMessage instance
    pub fn new_path(path: PathBuf) -> Self {
        Self {
            message_type: "user".to_string(),
            chunks: vec![UserMessageChunk::Path { path }],
        }
    }

    /// Create a new user message with multiple chunks
    ///
    /// # Arguments
    /// * `chunks` - The chunks of the message
    ///
    /// # Returns
    /// A new UserMessage instance
    pub fn new(chunks: Vec<UserMessageChunk>) -> Self {
        Self {
            message_type: "user".to_string(),
            chunks,
        }
    }
}

/// Icon for tool calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    /// The type of the icon
    #[serde(rename = "type")]
    pub icon_type: String,
    /// The value of the icon
    pub value: String,
}

/// Tool call confirmation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallConfirmation {
    /// The type of the confirmation
    #[serde(rename = "type")]
    pub confirmation_type: String,
    /// Optional description
    pub description: Option<String>,
    /// Command for execute type
    pub command: Option<String>,
    /// Root command for execute type
    #[serde(rename = "rootCommand")]
    pub root_command: Option<String>,
    /// Server name for mcp type
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,
    /// Tool name for mcp type
    #[serde(rename = "toolName")]
    pub tool_name: Option<String>,
    /// Tool display name for mcp type
    #[serde(rename = "toolDisplayName")]
    pub tool_display_name: Option<String>,
    /// URLs for fetch type
    pub urls: Option<Vec<String>>,
}

/// Tool call content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallContent {
    /// The type of the content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Markdown content for markdown type
    pub markdown: Option<String>,
    /// Path for diff type
    pub path: Option<String>,
    /// Old text for diff type
    #[serde(rename = "oldText")]
    pub old_text: Option<String>,
    /// New text for diff type
    #[serde(rename = "newText")]
    pub new_text: Option<String>,
}

/// File location for a tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallLocation {
    /// The path of the file
    pub path: String,
    /// The start line (optional)
    #[serde(rename = "lineStart")]
    pub line_start: Option<u32>,
    /// The end line (optional)
    #[serde(rename = "lineEnd")]
    pub line_end: Option<u32>,
}

/// Agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Raw agentId from iFlow ACP
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// Agent index within task
    #[serde(rename = "agentIndex")]
    pub agent_index: Option<u32>,
    /// Task/call ID from agentId
    #[serde(rename = "taskId")]
    pub task_id: Option<String>,
    /// Creation/event timestamp
    pub timestamp: Option<u64>,
}

/// Tool call message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallMessage {
    /// The type of the message (always "tool_call")
    #[serde(rename = "type")]
    pub message_type: String,
    /// The ID of the tool call
    pub id: String,
    /// The label of the tool call
    pub label: String,
    /// The icon of the tool call
    pub icon: Icon,
    /// The status of the tool call
    pub status: ToolCallStatus,
    /// The name of the tool (optional)
    #[serde(rename = "toolName")]
    pub tool_name: Option<String>,
    /// The content of the tool call (optional)
    pub content: Option<ToolCallContent>,
    /// The locations of the tool call (optional)
    pub locations: Option<Vec<ToolCallLocation>>,
    /// The confirmation details of the tool call (optional)
    pub confirmation: Option<ToolCallConfirmation>,
    /// The agent ID (optional)
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
    /// The agent information (optional)
    #[serde(rename = "agentInfo")]
    pub agent_info: Option<AgentInfo>,
}

impl ToolCallMessage {
    /// Create a new tool call message
    ///
    /// # Arguments
    /// * `id` - The ID of the tool call
    /// * `label` - The label of the tool call
    /// * `icon` - The icon of the tool call
    /// * `status` - The status of the tool call
    ///
    /// # Returns
    /// A new ToolCallMessage instance
    pub fn new(id: String, label: String, icon: Icon, status: ToolCallStatus) -> Self {
        Self {
            message_type: "tool_call".to_string(),
            id,
            label,
            icon,
            status,
            tool_name: None,
            content: None,
            locations: None,
            confirmation: None,
            agent_id: None,
            agent_info: None,
        }
    }
}

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
    /// WebSocket URL for WebSocket connection (if None, use stdio)
    pub websocket_url: Option<String>,
    /// Permission mode for tool calls
    pub permission_mode: PermissionMode,
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
            websocket_url: None,
            permission_mode: PermissionMode::Auto,
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

    /// Set WebSocket URL for WebSocket connection
    ///
    /// # Arguments
    /// * `url` - The WebSocket URL to connect to
    pub fn with_websocket_url<S: Into<String>>(mut self, url: S) -> Self {
        self.websocket_url = Some(url.into());
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
    pub fn with_details(code: i32, message: String, details: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self {
            code,
            message,
            details: Some(details),
        }
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
    Plan { entries: Vec<PlanEntry> },

    /// Task finish message
    #[serde(rename = "task_finish")]
    TaskFinish { reason: Option<String> },

    /// Error message
    #[serde(rename = "error")]
    Error { 
        code: i32, 
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<std::collections::HashMap<String, serde_json::Value>>,
    },
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
    
    /// Create a new error message
    ///
    /// # Arguments
    /// * `code` - The error code
    /// * `message` - The error message
    ///
    /// # Returns
    /// A new Message::Error variant
    pub fn error(code: i32, message: String) -> Self {
        Message::Error {
            code,
            message,
            details: None,
        }
    }
    
    /// Create a new error message with details
    ///
    /// # Arguments
    /// * `code` - The error code
    /// * `message` - The error message
    /// * `details` - Additional error details
    ///
    /// # Returns
    /// A new Message::Error variant
    pub fn error_with_details(code: i32, message: String, details: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Message::Error {
            code,
            message,
            details: Some(details),
        }
    }
}