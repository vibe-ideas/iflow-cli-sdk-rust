//! Connection management for iFlow SDK
//!
//! This module provides abstractions for different connection types
//! (stdio, WebSocket) and handles the connection lifecycle.

pub mod handler;
pub mod stdio;
pub mod websocket;

pub use handler::{ConnectionHandler, ConnectionError};
pub use crate::error::Result;
pub use stdio::StdioConnection;
pub use websocket::WebSocketConnection;