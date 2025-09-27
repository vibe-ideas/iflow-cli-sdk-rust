//! Logger tests

use iflow_cli_sdk_rust::{LoggerConfig, Message, MessageLogger};

#[tokio::test]
async fn test_logger_creation() {
    let config = LoggerConfig {
        log_file: std::path::PathBuf::from("test_log.log"),
        enabled: true,
        max_file_size: 1024, // 1KB
        max_files: 2,
    };

    let logger = MessageLogger::new(config).unwrap();
    assert!(logger.config().enabled);

    // Clean up test files
    let _ = std::fs::remove_file("test_log.log");
}

#[tokio::test]
async fn test_log_raw_message() {
    let config = LoggerConfig {
        log_file: std::path::PathBuf::from("test_raw.log"),
        enabled: true,
        max_file_size: 1024,
        max_files: 2,
    };

    let logger = MessageLogger::new(config).unwrap();

    // Test user message
    let user_msg = Message::User {
        content: "Hello, world!".to_string(),
    };

    logger.log_message(&user_msg).await.unwrap();

    // Test assistant message
    let assistant_msg = Message::Assistant {
        content: "Hello from assistant!".to_string(),
    };

    logger.log_message(&assistant_msg).await.unwrap();

    // Verify file content (Debug format)
    let content = std::fs::read_to_string("test_raw.log").unwrap();
    assert!(content.contains("User { content: \"Hello, world!\" }"));
    assert!(content.contains("Assistant { content: \"Hello from assistant!\" }"));

    // Clean up test files
    let _ = std::fs::remove_file("test_raw.log");
}

#[tokio::test]
async fn test_logger_disabled() {
    let config = LoggerConfig {
        log_file: std::path::PathBuf::from("should_not_exist.log"),
        enabled: false,
        max_file_size: 1024,
        max_files: 2,
    };

    let logger = MessageLogger::new(config).unwrap();

    let message = Message::User {
        content: "This should not be logged".to_string(),
    };

    // Should not error even when disabled
    logger.log_message(&message).await.unwrap();

    // Ensure the file was not created
    assert!(!std::path::Path::new("should_not_exist.log").exists());
}
