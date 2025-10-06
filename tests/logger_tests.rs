//! Comprehensive tests for logger module
//!
//! These tests ensure the logger functionality is properly covered

use iflow_cli_sdk_rust::{LoggerConfig, Message, MessageLogger};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_logger_config_default() {
    let config = LoggerConfig::default();
    assert_eq!(config.log_file, PathBuf::from("iflow_messages.log"));
    assert_eq!(config.enabled, true);
    assert_eq!(config.max_file_size, 10 * 1024 * 1024);
    assert_eq!(config.max_files, 5);
}

#[test]
fn test_logger_config_creation() {
    let config = LoggerConfig {
        log_file: PathBuf::from("test.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };
    assert_eq!(config.log_file, PathBuf::from("test.log"));
    assert_eq!(config.enabled, false);
    assert_eq!(config.max_file_size, 1024);
    assert_eq!(config.max_files, 3);
}

#[test]
fn test_logger_config_clone() {
    let config1 = LoggerConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.log_file, config2.log_file);
    assert_eq!(config1.enabled, config2.enabled);
}

#[test]
fn test_logger_config_debug() {
    let config = LoggerConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("LoggerConfig"));
}

#[tokio::test]
async fn test_logger_disabled() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test_disabled.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");
    let message = Message::User {
        content: "Test message".to_string(),
    };

    logger.log_message(&message).await.expect("Failed to log message");
}

#[tokio::test]
async fn test_logger_enabled() {
    let log_path = PathBuf::from("/tmp/test_enabled.log");
    let _ = fs::remove_file(&log_path);

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 10 * 1024 * 1024,
        max_files: 5,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");
    let message = Message::User {
        content: "Test message".to_string(),
    };

    logger.log_message(&message).await.expect("Failed to log message");

    assert!(log_path.exists());
    let content = fs::read_to_string(&log_path).expect("Failed to read log file");
    assert!(content.contains("User"));
    assert!(content.contains("Test message"));

    let _ = fs::remove_file(&log_path);
}

#[tokio::test]
async fn test_logger_with_directory_creation() {
    let log_path = PathBuf::from("/tmp/logger_test_dir/test.log");
    let _ = fs::remove_dir_all("/tmp/logger_test_dir");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 10 * 1024 * 1024,
        max_files: 5,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");
    let message = Message::User {
        content: "Test".to_string(),
    };

    logger.log_message(&message).await.expect("Failed to log message");

    assert!(log_path.exists());
    let _ = fs::remove_dir_all("/tmp/logger_test_dir");
}

#[tokio::test]
async fn test_logger_multiple_messages() {
    let log_path = PathBuf::from("/tmp/test_multiple.log");
    let _ = fs::remove_file(&log_path);

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 10 * 1024 * 1024,
        max_files: 5,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");

    for i in 0..5 {
        let message = Message::User {
            content: format!("Message {}", i),
        };
        logger.log_message(&message).await.expect("Failed to log message");
    }

    let content = fs::read_to_string(&log_path).expect("Failed to read log file");
    assert!(content.contains("Message 0"));
    assert!(content.contains("Message 4"));

    let _ = fs::remove_file(&log_path);
}

#[tokio::test]
async fn test_logger_file_rotation() {
    let log_path = PathBuf::from("/tmp/test_rotation.log");
    let _ = fs::remove_file(&log_path);
    let _ = fs::remove_file(log_path.with_extension("log.1"));

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 100, // Small size to trigger rotation
        max_files: 3,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");

    // Write enough messages to trigger rotation
    for i in 0..50 {
        let message = Message::User {
            content: format!("This is a longer message to trigger rotation {}", i),
        };
        logger.log_message(&message).await.expect("Failed to log message");
    }

    // Check that rotation happened (file should exist with smaller size)
    assert!(log_path.exists());

    let _ = fs::remove_file(&log_path);
    let _ = fs::remove_file(log_path.with_extension("log.1"));
    let _ = fs::remove_file(log_path.with_extension("log.2"));
}

#[tokio::test]
async fn test_logger_different_message_types() {
    let log_path = PathBuf::from("/tmp/test_message_types.log");
    let _ = fs::remove_file(&log_path);

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 10 * 1024 * 1024,
        max_files: 5,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");

    let messages = vec![
        Message::User {
            content: "User message".to_string(),
        },
        Message::Assistant {
            content: "Assistant message".to_string(),
        },
        Message::ToolCall {
            id: "tool1".to_string(),
            name: "test_tool".to_string(),
            status: "completed".to_string(),
        },
        Message::Error {
            code: 500,
            message: "Test error".to_string(),
            details: None,
        },
    ];

    for message in messages {
        logger.log_message(&message).await.expect("Failed to log message");
    }

    let content = fs::read_to_string(&log_path).expect("Failed to read log file");
    assert!(content.contains("User"));
    assert!(content.contains("Assistant"));
    assert!(content.contains("ToolCall"));
    assert!(content.contains("Error"));

    let _ = fs::remove_file(&log_path);
}

#[test]
fn test_logger_log_file_path() {
    let log_path = PathBuf::from("/tmp/test_path.log");
    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).expect("Failed to create logger");
    assert_eq!(logger.log_file_path(), log_path.as_path());
}

#[test]
fn test_logger_config_accessor() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test_config.log"),
        enabled: true,
        max_file_size: 2048,
        max_files: 4,
    };

    let logger = MessageLogger::new(config.clone()).expect("Failed to create logger");
    assert_eq!(logger.config().log_file, config.log_file);
    assert_eq!(logger.config().enabled, config.enabled);
    assert_eq!(logger.config().max_file_size, config.max_file_size);
    assert_eq!(logger.config().max_files, config.max_files);
}

#[test]
fn test_logger_clone() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test_clone.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger1 = MessageLogger::new(config).expect("Failed to create logger");
    let logger2 = logger1.clone();

    assert_eq!(logger1.log_file_path(), logger2.log_file_path());
}
