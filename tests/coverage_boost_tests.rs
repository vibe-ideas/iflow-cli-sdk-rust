//! Additional unit tests to improve overall coverage
//!
//! These tests target specific uncovered code paths.

use iflow_cli_sdk_rust::{IFlowOptions, Message};
use std::path::PathBuf;

#[tokio::test]
async fn test_message_enum_variants() {
    // Test all Message enum variants for Debug formatting
    let assistant = Message::Assistant {
        content: "test content".to_string(),
    };
    let debug_str = format!("{:?}", assistant);
    assert!(debug_str.contains("Assistant"));
    assert!(debug_str.contains("test content"));

    let error_msg = Message::Error {
        code: 500,
        message: "error message".to_string(),
        details: Some(std::collections::HashMap::new()),
    };
    let debug_str = format!("{:?}", error_msg);
    assert!(debug_str.contains("Error"));

    let task_finish = Message::TaskFinish {
        reason: Some("done".to_string()),
    };
    let debug_str = format!("{:?}", task_finish);
    assert!(debug_str.contains("TaskFinish"));
}

#[tokio::test]
async fn test_iflow_options_builder_patterns() {
    // Test various builder pattern combinations
    let options = IFlowOptions::new()
        .with_timeout(90.0)
        .with_cwd(PathBuf::from("/tmp"))
        .with_auto_start(true);

    assert_eq!(options.timeout, 90.0);
    assert_eq!(options.cwd, PathBuf::from("/tmp"));
    assert!(options.process.auto_start);
}

#[tokio::test]
async fn test_iflow_options_auth_method() {
    let options = IFlowOptions::new().with_auth_method_id("test_auth".to_string());

    assert_eq!(options.auth_method_id, Some("test_auth".to_string()));
}

#[tokio::test]
async fn test_iflow_options_permission_mode() {
    use iflow_cli_sdk_rust::types::PermissionMode;

    let options = IFlowOptions::new().with_permission_mode(PermissionMode::Manual);

    assert_eq!(options.permission_mode, PermissionMode::Manual);
}

#[tokio::test]
async fn test_iflow_options_process_config() {
    use iflow_cli_sdk_rust::types::ProcessConfig;

    let process_config = ProcessConfig::new().enable_auto_start().start_port(9000);

    let options = IFlowOptions::new().with_process_config(process_config);

    assert!(options.process.auto_start);
    assert_eq!(options.process.start_port, Some(9000));
}

#[tokio::test]
async fn test_iflow_options_websocket_config() {
    use iflow_cli_sdk_rust::types::WebSocketConfig;

    let ws_config = WebSocketConfig::new("ws://localhost:8080".to_string());

    let options = IFlowOptions::new().with_websocket_config(ws_config);

    assert!(options.websocket.is_some());
    assert_eq!(
        options.websocket.unwrap().url,
        Some("ws://localhost:8080".to_string())
    );
}

#[test]
fn test_iflow_options_debug() {
    let options = IFlowOptions::default();
    let debug_str = format!("{:?}", options);
    assert!(debug_str.contains("IFlowOptions"));
}

#[test]
fn test_iflow_options_clone() {
    let options1 = IFlowOptions::new().with_timeout(120.0);
    let options2 = options1.clone();

    assert_eq!(options1.timeout, options2.timeout);
}

#[test]
fn test_constants() {
    // Test exported constants
    use iflow_cli_sdk_rust::{PROTOCOL_VERSION, VERSION};

    assert_eq!(PROTOCOL_VERSION, 1);
    assert!(!VERSION.is_empty());
}

#[test]
fn test_file_access_config() {
    use iflow_cli_sdk_rust::types::FileAccessConfig;

    let config = FileAccessConfig::default();
    // Just test it's created successfully
    let _ = format!("{:?}", config);
}

#[test]
fn test_logging_config() {
    use iflow_cli_sdk_rust::types::LoggingConfig;
    use iflow_cli_sdk_rust::LoggerConfig;

    let config = LoggingConfig::default();
    assert!(!config.enabled);
    assert_eq!(config.level, "INFO");

    let custom_config = LoggingConfig {
        enabled: true,
        level: "DEBUG".to_string(),
        logger_config: LoggerConfig::default(),
    };
    assert!(custom_config.enabled);
}

#[test]
fn test_plan_message_struct() {
    use iflow_cli_sdk_rust::types::{PlanEntry, PlanPriority, PlanStatus};

    let entry1 = PlanEntry {
        content: "Task 1".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };

    let entry2 = PlanEntry {
        content: "Task 2".to_string(),
        priority: PlanPriority::Low,
        status: PlanStatus::Pending,
    };

    let entries = vec![entry1, entry2];

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].priority, PlanPriority::High);
}
