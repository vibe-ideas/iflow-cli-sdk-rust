//! WebSocket configuration for iFlow SDK
//!
//! This module contains the WebSocket configuration for the iFlow SDK.

use std::time::Duration;

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
    pub fn with_reconnect_settings(
        url: String,
        reconnect_attempts: u32,
        reconnect_interval: Duration,
    ) -> Self {
        Self {
            url: Some(url),
            reconnect_attempts,
            reconnect_interval,
        }
    }

    /// Create a new WebSocketConfig for auto-start mode with custom reconnect settings
    pub fn auto_start_with_reconnect_settings(
        reconnect_attempts: u32,
        reconnect_interval: Duration,
    ) -> Self {
        Self {
            url: None,
            reconnect_attempts,
            reconnect_interval,
        }
    }
}