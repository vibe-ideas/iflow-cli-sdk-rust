//! File access configuration for iFlow SDK
//!
//! This module contains the file access configuration for the iFlow SDK.

use std::path::PathBuf;

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