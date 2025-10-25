//! Logging configuration for iFlow SDK
//!
//! This module contains the logging configuration for the iFlow SDK.

use crate::logger::LoggerConfig;

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