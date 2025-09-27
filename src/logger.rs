//! Logger module for recording iflow messages

use crate::Message;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Log file path
    pub log_file: PathBuf,
    /// Whether to enable logging
    pub enabled: bool,
    /// Maximum log file size (bytes), will rotate when exceeded
    pub max_file_size: u64,
    /// Number of log files to retain
    pub max_files: u32,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_file: PathBuf::from("iflow_messages.log"),
            enabled: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
        }
    }
}

/// Message logger
#[derive(Clone)]
pub struct MessageLogger {
    config: LoggerConfig,
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl MessageLogger {
    /// Create a new logger
    pub fn new(config: LoggerConfig) -> Result<Self, io::Error> {
        if !config.enabled {
            return Ok(Self {
                config,
                writer: Arc::new(Mutex::new(BufWriter::new(File::create("/dev/null")?))),
            });
        }

        // Check if log file rotation is needed
        if let Some(parent) = config.log_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Check file size
        if config.log_file.exists() {
            let metadata = std::fs::metadata(&config.log_file)?;
            if metadata.len() >= config.max_file_size {
                Self::rotate_log_file(&config)?;
            }
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.log_file)?;

        Ok(Self {
            config,
            writer: Arc::new(Mutex::new(BufWriter::new(file))),
        })
    }

    /// Rotate log files
    fn rotate_log_file(config: &LoggerConfig) -> Result<(), io::Error> {
        if !config.log_file.exists() {
            return Ok(());
        }

        // Delete the oldest log file
        for i in (0..config.max_files).rev() {
            let old_path = if i == 0 {
                config.log_file.clone()
            } else {
                config.log_file.with_extension(format!("log.{}", i))
            };

            let new_path = if i + 1 >= config.max_files {
                // Delete files that exceed the retention count
                if old_path.exists() {
                    std::fs::remove_file(&old_path)?;
                }
                continue;
            } else {
                config.log_file.with_extension(format!("log.{}", i + 1))
            };

            if old_path.exists() {
                std::fs::rename(&old_path, &new_path)?;
            }
        }

        Ok(())
    }

    /// Log a message
    pub async fn log_message(&self, message: &Message) -> Result<(), io::Error> {
        if !self.config.enabled {
            return Ok(());
        }

        let log_entry = self.format_message(message);
        let mut writer = self.writer.lock().await;

        writeln!(writer, "{}", log_entry)?;
        writer.flush()?;

        // Check file size
        if writer.get_ref().metadata()?.len() >= self.config.max_file_size {
            Self::rotate_log_file(&self.config)?;

            // Reopen the file
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.config.log_file)?;
            *writer = BufWriter::new(file);
        }

        Ok(())
    }

    /// Log raw message using Debug format without any processing
    fn format_message(&self, message: &Message) -> String {
        // Output raw message structure using Debug format
        format!("{:?}", message)
    }

    /// Get current log file path
    pub fn log_file_path(&self) -> &Path {
        &self.config.log_file
    }

    /// Get configuration
    pub fn config(&self) -> &LoggerConfig {
        &self.config
    }
}
