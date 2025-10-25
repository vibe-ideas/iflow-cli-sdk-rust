//! Configuration types for iFlow SDK
//!
//! This module contains all the configuration types used throughout the iFlow SDK,
//! including connection settings, security options, and logging configuration.

pub mod options;
pub mod websocket;
pub mod file_access;
pub mod process;
pub mod logging;

pub use options::{IFlowOptions, ErrorMessageDetails};
pub use websocket::WebSocketConfig;
pub use file_access::FileAccessConfig;
pub use process::ProcessConfig;
pub use logging::LoggingConfig;