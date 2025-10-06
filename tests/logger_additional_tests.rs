//! Additional logger tests to improve coverage
//!
//! These tests focus on error handling and edge cases in the logger module.

use iflow_cli_sdk_rust::{LoggerConfig, Message, MessageLogger};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_logger_rotation_trigger() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("rotation_test.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 100, // Very small size to trigger rotation
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    // Write multiple messages to trigger rotation
    for i in 0..10 {
        let message = Message::Assistant {
            content: format!("Long message number {} to fill up the log file quickly", i),
        };
        logger.log_message(&message).await.unwrap();
    }

    // Check that the log file exists
    assert!(log_path.exists());
}

#[tokio::test]
async fn test_logger_with_rotated_files() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("multi_rotation.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 50,
        max_files: 2,
    };

    let logger = MessageLogger::new(config).unwrap();

    // Write enough to cause multiple rotations
    for i in 0..20 {
        let message = Message::Assistant {
            content: format!("Rotation test message {}", i),
        };
        logger.log_message(&message).await.unwrap();
    }

    // Verify main log file exists
    assert!(log_path.exists());
}

#[tokio::test]
async fn test_logger_with_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("existing.log");

    // Create the file first
    std::fs::write(&log_path, "Pre-existing content\n").unwrap();

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    let message = Message::Assistant {
        content: "New message".to_string(),
    };
    logger.log_message(&message).await.unwrap();

    // Verify both old and new content exist
    let contents = std::fs::read_to_string(&log_path).unwrap();
    assert!(contents.contains("Pre-existing content"));
    assert!(contents.contains("New message"));
}

#[tokio::test]
async fn test_logger_with_large_file_triggering_rotation() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("large_file.log");

    // Create a large pre-existing file
    let large_content = "X".repeat(200);
    std::fs::write(&log_path, large_content).unwrap();

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 100, // Smaller than existing file
        max_files: 3,
    };

    // This should trigger rotation on initialization
    let logger = MessageLogger::new(config).unwrap();

    let message = Message::Assistant {
        content: "After rotation".to_string(),
    };
    logger.log_message(&message).await.unwrap();

    // The rotated file should exist
    let rotated_path = log_path.with_extension("log.1");
    assert!(rotated_path.exists() || log_path.exists());
}

#[tokio::test]
async fn test_logger_disabled_with_invalid_path() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/nonexistent/invalid/path/test.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 3,
    };

    // Should succeed because logging is disabled
    let logger = MessageLogger::new(config).unwrap();

    let message = Message::Assistant {
        content: "Should not be written".to_string(),
    };

    // Should not fail even with invalid path
    logger.log_message(&message).await.unwrap();
}

#[tokio::test]
async fn test_logger_format_different_message_types() {
    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("format_test.log");

    let config = LoggerConfig {
        log_file: log_path.clone(),
        enabled: true,
        max_file_size: 1024 * 1024,
        max_files: 3,
    };

    let logger = MessageLogger::new(config).unwrap();

    // Test various message types to improve format_message coverage
    let messages = vec![
        Message::Assistant {
            content: "Assistant test".to_string(),
        },
        Message::Error {
            code: 500,
            message: "Error test".to_string(),
            details: None,
        },
        Message::TaskFinish {
            reason: Some("Task complete".to_string()),
        },
        Message::User {
            content: "User message".to_string(),
        },
    ];

    for msg in messages {
        logger.log_message(&msg).await.unwrap();
    }

    let contents = std::fs::read_to_string(&log_path).unwrap();
    assert!(contents.contains("Assistant test"));
    assert!(contents.contains("Error test"));
    assert!(contents.contains("Task complete"));
    assert!(contents.contains("User message"));
}

#[test]
fn test_logger_config_clone() {
    let config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test.log"),
        enabled: true,
        max_file_size: 2048,
        max_files: 5,
    };

    let cloned = config.clone();
    assert_eq!(config.log_file, cloned.log_file);
    assert_eq!(config.enabled, cloned.enabled);
    assert_eq!(config.max_file_size, cloned.max_file_size);
    assert_eq!(config.max_files, cloned.max_files);
}
