//! Process management configuration for iFlow SDK
//!
//! This module contains the process management configuration for the iFlow SDK.


/// Configuration for process management
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    /// Whether to automatically start the iFlow process
    pub auto_start: bool,
    /// Port to start the iFlow process on (only used in auto-start WebSocket mode)
    pub start_port: Option<u16>,
    /// Whether to start the iFlow process in debug mode
    pub debug: bool,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            auto_start: true,
            start_port: None, // No port needed for stdio mode
            debug: false,
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

    /// Set whether to start the iFlow process in debug mode
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
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

    /// Enable debug mode
    pub fn enable_debug(self) -> Self {
        self.debug(true)
    }

    /// Configure for stdio mode (no port needed)
    pub fn stdio_mode(mut self) -> Self {
        self.start_port = None;
        self
    }
}