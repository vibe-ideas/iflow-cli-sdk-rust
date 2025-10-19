//! iFlow CLI SDK for Rust
//!
//! A powerful SDK for interacting with iFlow using the [Agent Client Protocol (ACP)](https://github.com/agentclientprotocol/agent-client-protocol).
//! Built on top of the official agent-client-protocol crate.
//!
//! # Examples
//!
//! ## Basic usage with automatic process management
//! ```no_run
//! use iflow_cli_sdk_rust::IFlowClient;
//! use futures::stream::StreamExt;
//! use std::io::Write;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = IFlowClient::new(None);
//!     client.connect().await?;
//!     
//!     client.send_message("Hello, iFlow!", None).await?;
//!     
//!     // Listen for messages
//!     let mut message_stream = client.messages();
//!     while let Some(message) = message_stream.next().await {
//!         match message {
//!             iflow_cli_sdk_rust::Message::Assistant { content } => {
//!                 print!("{}", content);
//!                 std::io::stdout().flush()?;
//!             }
//!             iflow_cli_sdk_rust::Message::TaskFinish { .. } => {
//!                 break;
//!             }
//!             _ => {
//!                 // Handle other message types
//!             }
//!         }
//!     }
//!     
//!     client.disconnect().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Simple query
//! ```no_run
//! use iflow_cli_sdk_rust::query;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let response = query("What is 2 + 2?").await?;
//!     println!("{}", response);
//!     Ok(())
//! }
//! ```

pub mod acp_protocol;
pub mod client;
pub mod error;
pub mod logger;
pub mod process_manager;
pub mod query;
pub mod types;
pub mod websocket_transport;

// Re-export main types
pub use client::IFlowClient;
pub use error::{IFlowError, Result};
pub use logger::{LoggerConfig, MessageLogger};
pub use process_manager::IFlowProcessManager;
pub use query::{
    query, query_stream, query_stream_with_config, query_stream_with_timeout, query_with_config,
    query_with_timeout,
};
pub use types::{IFlowOptions, Message};

// Re-export types from agent-client-protocol that we actually use
pub use agent_client_protocol::{EnvVariable, McpServer, SessionId, StopReason};

// Version info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PROTOCOL_VERSION: u32 = 1;
