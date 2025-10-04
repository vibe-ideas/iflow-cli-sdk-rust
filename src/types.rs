//! Type definitions for iFlow SDK
//!
//! This module contains all the type definitions used throughout the iFlow SDK,
//! including configuration options and message types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

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

/// Configuration for WebSocket connection
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// WebSocket URL to connect to (None means auto-generate in auto-start mode)
    pub url: Option<String>,
    /// Number of reconnect attempts
    pub reconnect_attempts: u32,
    /// Interval between reconnect attempts
    pub reconnect_interval: Duration,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            url: Some("ws://localhost:8090/acp?peer=iflow".to_string()),
            reconnect_attempts: 3,
            reconnect_interval: Duration::from_secs(5),
        }
    }
}

impl WebSocketConfig {
    /// Create a new WebSocketConfig with the specified URL and default reconnect settings
    pub fn new(url: String) -> Self {
        Self {
            url: Some(url),
            ..Default::default()
        }
    }
    
    /// Create a new WebSocketConfig for auto-start mode (URL will be auto-generated)
    pub fn auto_start() -> Self {
        Self {
            url: None,
            ..Default::default()
        }
    }
    
    /// Create a new WebSocketConfig with custom reconnect settings
    pub fn with_reconnect_settings(url: String, reconnect_attempts: u32, reconnect_interval: Duration) -> Self {
        Self {
            url: Some(url),
            reconnect_attempts,
            reconnect_interval,
        }
    }
    
    /// Create a new WebSocketConfig for auto-start mode with custom reconnect settings
    pub fn auto_start_with_reconnect_settings(reconnect_attempts: u32, reconnect_interval: Duration) -> Self {
        Self {
            url: None,
            reconnect_attempts,
            reconnect_interval,
        }
    }
}

/// Configuration for file access
#[derive(Debug, Clone)]
pub struct FileAccessConfig {
    /// Whether file access is enabled
    pub enabled: bool,
    /// Allowed directories for file access
    pub allowed_dirs: Option<Vec<PathBuf>>,
    /// Whether file access is read-only
    pub read_only: bool,
    /// Maximum file size for file access
    pub max_size: u64,
}

impl Default for FileAccessConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_dirs: None,
            read_only: false,
            max_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Configuration for process management
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    /// Whether to automatically start the iFlow process
    pub auto_start: bool,
    /// Port to start the iFlow process on (only used in auto-start WebSocket mode)
    pub start_port: Option<u16>,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            auto_start: true,
            start_port: None, // No port needed for stdio mode
        }
    }
}

impl ProcessConfig {
    /// Create a new ProcessConfig with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set whether to automatically start the iFlow process
    pub fn auto_start(mut self, auto_start: bool) -> Self {
        self.auto_start = auto_start;
        self
    }
    
    /// Set the port to start the iFlow process on (only used in WebSocket mode)
    pub fn start_port(mut self, port: u16) -> Self {
        self.start_port = Some(port);
        self
    }
    
    /// Disable process auto-start
    pub fn manual_start(self) -> Self {
        self.auto_start(false)
    }
    
    /// Enable process auto-start
    pub fn enable_auto_start(self) -> Self {
        self.auto_start(true)
    }
    
    /// Configure for stdio mode (no port needed)
    pub fn stdio_mode(mut self) -> Self {
        self.start_port = None;
        self
    }
}

/// Configuration for logging
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Whether logging is enabled
    pub enabled: bool,
    /// Log level
    pub level: String,
    /// Logger configuration
    pub logger_config: LoggerConfig,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            level: "INFO".to_string(),
            logger_config: LoggerConfig::default(),
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
            timeout: 30.0,
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