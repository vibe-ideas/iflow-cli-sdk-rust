//! Comprehensive tests for logger module
//!
//! These tests improve coverage of the message logging functionality.

use iflow_cli_sdk_rust::{LoggerConfig, Message, MessageLogger};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_logger_config_default() {
    let config = LoggerConfig::default();
    assert_eq!(config.log_file, PathBuf::from("iflow_messages.log"));
    assert!(config.enabled);
    assert_eq!(config.max_file_size, 10 * 1024 * 1024);
    assert_eq!(config.max_files, 5);
}

#[tokio::test]
async fn test_logger_config_custom() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/custom.log"),
        enabled: false,
        max_file_size: 5 * 1024 * 1024,
        max_files: 3,
    };
    assert_eq!(config.log_file, PathBuf::from("/tmp/custom.log"));
    assert!(!config.enabled);
    assert_eq!(config.max_file_size, 5 * 1024 * 1024);
    assert_eq!(config.max_files, 3);
}

#[tokio::test]
async fn test_logger_creation_enabled() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("test.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();
    assert_eq!(logger.log_file_path(), log_path);
}

#[tokio::test]
async fn test_logger_creation_disabled() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/nonexistent/path/test.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };

    // Should succeed even with invalid path when disabled
    let logger = MessageLogger::new(config);
    assert!(logger.is_ok());
}

#[tokio::test]
async fn test_logger_log_message() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("messages.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    let message = Message::Assistant {
        content: "Test message".to_string(),
    };

    logger.log_message(&message).await.unwrap();

    // Verify log file was created and contains data
    assert!(log_path.exists());
    let contents = std::fs::read_to_string(&log_path).unwrap();
    assert!(contents.contains("Test message"));
}

#[tokio::test]
async fn test_logger_disabled_no_write() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/disabled_test.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    let message = Message::Assistant {
        content: "Should not be logged".to_string(),
    };

    // Should succeed but not write anything
    let result = logger.log_message(&message).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_logger_get_config() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("config_test.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 2048,
        max_files: 4,
    };

    let logger = MessageLogger::new(config.clone()).unwrap();
    let retrieved_config = logger.config();

    assert_eq!(retrieved_config.log_file, log_path);
    assert_eq!(retrieved_config.enabled, true);
    assert_eq!(retrieved_config.max_file_size, 2048);
    assert_eq!(retrieved_config.max_files, 4);
}

#[tokio::test]
async fn test_logger_clone() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("clone_test.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger1 = MessageLogger::new(config).unwrap();
    let logger2 = logger1.clone();

    // Both loggers should have the same path
    assert_eq!(logger1.log_file_path(), logger2.log_file_path());
}

#[tokio::test]
async fn test_logger_creates_parent_directory() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("nested").join("dir").join("test.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    let message = Message::Assistant {
        content: "Nested directory test".to_string(),
    };

    logger.log_message(&message).await.unwrap();

    // Verify nested directories and file were created
    assert!(log_path.exists());
    assert!(log_path.parent().unwrap().exists());
}

#[tokio::test]
async fn test_logger_multiple_message_types() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("multi_messages.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    // Test different message types
    let messages = vec![
        Message::Assistant {
            content: "Assistant message".to_string(),
        },
        Message::Error {
            code: 404,
            message: "Error message".to_string(),
            details: None,
        },
        Message::TaskFinish {
            reason: Some("Completed".to_string()),
        },
    ];

    for msg in messages {
        logger.log_message(&msg).await.unwrap();
    }

    let contents = std::fs::read_to_string(&log_path).unwrap();
    assert!(contents.contains("Assistant message"));
    assert!(contents.contains("Error message"));
    assert!(contents.contains("Completed"));
}

#[tokio::test]
async fn test_logger_config_debug() {
    let config = LoggerConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("LoggerConfig"));
    assert!(debug_str.contains("enabled"));
}
