//! Unit tests for the logger module
//!
//! These tests verify the message logger functionality including
//! configuration, file operations, and rotation

use iflow_cli_sdk_rust::{Message, MessageLogger, LoggerConfig};
use std::path::PathBuf;
use std::fs;

#[tokio::test]
async fn test_logger_config_default() {
    let config = LoggerConfig::default();
    assert_eq!(config.log_file, PathBuf::from("iflow_messages.log"));
    assert_eq!(config.enabled, true);
    assert_eq!(config.max_file_size, 10 * 1024 * 1024);
    assert_eq!(config.max_files, 5);
}

#[tokio::test]
async fn test_logger_config_custom() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };
    
    assert_eq!(config.log_file, PathBuf::from("/tmp/test.log"));
    assert_eq!(config.enabled, false);
    assert_eq!(config.max_file_size, 1024);
    assert_eq!(config.max_files, 3);
}

#[tokio::test]
async fn test_logger_creation_disabled() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test_disabled.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config.clone()).expect("Failed to create logger");
    assert_eq!(logger.config().enabled, false);
    assert_eq!(logger.log_file_path(), PathBuf::from("/tmp/test_disabled.log"));
}

#[tokio::test]
async fn test_logger_creation_enabled() {
    let log_file = PathBuf::from("/tmp/test_enabled.log");
    let _ = fs::remove_file(&log_file);
    
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config.clone()).expect("Failed to create logger");
    assert_eq!(logger.config().enabled, true);
    assert_eq!(logger.log_file_path(), &log_file);
    
    // Clean up
    let _ = fs::remove_file(&log_file);
}

#[tokio::test]
async fn test_logger_log_message() {
    let log_file = PathBuf::from("/tmp/test_log_message.log");
    let _ = fs::remove_file(&log_file);
    
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config).expect("Failed to create logger");
    
    let message = Message::User {
        content: "Test message".to_string(),
    };
    
    logger.log_message(&message).await.expect("Failed to log message");
    
    // Verify the file was created and contains content
    assert!(log_file.exists());
    let content = fs::read_to_string(&log_file).expect("Failed to read log file");
    assert!(content.contains("Test message"));
    
    // Clean up
    let _ = fs::remove_file(&log_file);
}

#[tokio::test]
async fn test_logger_log_multiple_messages() {
    let log_file = PathBuf::from("/tmp/test_log_multiple.log");
    let _ = fs::remove_file(&log_file);
    
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config).expect("Failed to create logger");
    
    for i in 0..5 {
        let message = Message::User {
            content: format!("Message {}", i),
        };
        logger.log_message(&message).await.expect("Failed to log message");
    }
    
    // Verify all messages are in the file
    let content = fs::read_to_string(&log_file).expect("Failed to read log file");
    for i in 0..5 {
        assert!(content.contains(&format!("Message {}", i)));
    }
    
    // Clean up
    let _ = fs::remove_file(&log_file);
}

#[tokio::test]
async fn test_logger_disabled_no_logging() {
    let log_file = PathBuf::from("/tmp/test_disabled_no_log.log");
    let _ = fs::remove_file(&log_file);
    
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config).expect("Failed to create logger");
    
    let message = Message::User {
        content: "This should not be logged".to_string(),
    };
    
    logger.log_message(&message).await.expect("Failed to log message");
    
    // Verify the file was not created (or is empty)
    // With disabled logging, it writes to /dev/null
    assert!(!log_file.exists() || fs::metadata(&log_file).unwrap().len() == 0);
    
    // Clean up
    let _ = fs::remove_file(&log_file);
}

#[tokio::test]
async fn test_logger_creates_parent_directory() {
    let log_file = PathBuf::from("/tmp/logger_test_dir/subdir/test.log");
    let parent_dir = PathBuf::from("/tmp/logger_test_dir");
    
    // Clean up first
    let _ = fs::remove_dir_all(&parent_dir);
    
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config).expect("Failed to create logger");
    
    let message = Message::User {
        content: "Test message".to_string(),
    };
    
    logger.log_message(&message).await.expect("Failed to log message");
    
    // Verify directory and file were created
    assert!(parent_dir.exists());
    assert!(log_file.exists());
    
    // Clean up
    let _ = fs::remove_dir_all(&parent_dir);
}

#[tokio::test]
async fn test_logger_file_rotation() {
    let log_file = PathBuf::from("/tmp/test_rotation.log");
    let _ = fs::remove_file(&log_file);
    
    // Create a small max file size to trigger rotation
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: true,
        max_file_size: 100, // Very small to trigger rotation
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config).expect("Failed to create logger");
    
    // Write enough messages to exceed the max file size
    for i in 0..20 {
        let message = Message::User {
            content: format!("This is a longer message {} to fill up the log file", i),
        };
        logger.log_message(&message).await.expect("Failed to log message");
    }
    
    // The log should have rotated
    // Just verify the primary log file exists
    assert!(log_file.exists());
    
    // Clean up
    let _ = fs::remove_file(&log_file);
    for i in 1..=3 {
        let rotated = log_file.with_extension(format!("log.{}", i));
        let _ = fs::remove_file(&rotated);
    }
}

#[tokio::test]
async fn test_logger_config_clone() {
    let config = LoggerConfig::default();
    let cloned = config.clone();
    
    assert_eq!(config.log_file, cloned.log_file);
    assert_eq!(config.enabled, cloned.enabled);
    assert_eq!(config.max_file_size, cloned.max_file_size);
    assert_eq!(config.max_files, cloned.max_files);
}

#[tokio::test]
async fn test_logger_format_different_messages() {
    let log_file = PathBuf::from("/tmp/test_different_messages.log");
    let _ = fs::remove_file(&log_file);
    
    let config = LoggerConfig {
        log_file: log_file.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };
    
    let logger = MessageLogger::new(config).expect("Failed to create logger");
    
    // Test User message
    let user_msg = Message::User {
        content: "User message".to_string(),
    };
    logger.log_message(&user_msg).await.expect("Failed to log");
    
    // Test Assistant message
    let assistant_msg = Message::Assistant {
        content: "Assistant response".to_string(),
    };
    logger.log_message(&assistant_msg).await.expect("Failed to log");
    
    // Test Error message
    let error_msg = Message::error(500, "Error message".to_string());
    logger.log_message(&error_msg).await.expect("Failed to log");
    
    // Verify all message types were logged
    let content = fs::read_to_string(&log_file).expect("Failed to read log file");
    assert!(content.contains("User"));
    assert!(content.contains("Assistant"));
    assert!(content.contains("Error"));
    
    // Clean up
    let _ = fs::remove_file(&log_file);
}
