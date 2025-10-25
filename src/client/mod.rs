//! Main client implementation for iFlow SDK
//!
//! This module provides the core client functionality for communicating with iFlow
//! using the [Agent Client Protocol (ACP)](https://github.com/agentclientprotocol/agent-client-protocol) over stdio or WebSocket.

pub mod acp_handler;
pub mod handler;
pub mod stream;

pub use handler::IFlowClient;
pub use stream::MessageStream;