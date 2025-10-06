//! Comprehensive tests for types module
//!
//! These tests improve coverage of the type definitions and message structures.

use iflow_cli_sdk_rust::types::{
    PermissionMode, PlanEntry, PlanPriority, PlanStatus, ProcessConfig, ToolCallStatus,
    UserMessage, UserMessageChunk, WebSocketConfig,
};
use std::path::PathBuf;

#[test]
fn test_permission_mode_variants() {
    // Test all permission mode variants
    let auto = PermissionMode::Auto;
    let manual = PermissionMode::Manual;
    let selective = PermissionMode::Selective;

    // Test Debug formatting
    assert_eq!(format!("{:?}", auto), "Auto");
    assert_eq!(format!("{:?}", manual), "Manual");
    assert_eq!(format!("{:?}", selective), "Selective");

    // Test equality
    assert_eq!(auto, PermissionMode::Auto);
    assert_ne!(auto, manual);

    // Test default
    assert_eq!(PermissionMode::default(), PermissionMode::Auto);
}

#[test]
fn test_permission_mode_serialization() {
    // Test serialization
    let auto = PermissionMode::Auto;
    let json = serde_json::to_string(&auto).unwrap();
    assert_eq!(json, "\"auto\"");

    let manual = PermissionMode::Manual;
    let json = serde_json::to_string(&manual).unwrap();
    assert_eq!(json, "\"manual\"");

    let selective = PermissionMode::Selective;
    let json = serde_json::to_string(&selective).unwrap();
    assert_eq!(json, "\"selective\"");
}

#[test]
fn test_permission_mode_deserialization() {
    // Test deserialization
    let auto: PermissionMode = serde_json::from_str("\"auto\"").unwrap();
    assert_eq!(auto, PermissionMode::Auto);

    let manual: PermissionMode = serde_json::from_str("\"manual\"").unwrap();
    assert_eq!(manual, PermissionMode::Manual);

    let selective: PermissionMode = serde_json::from_str("\"selective\"").unwrap();
    assert_eq!(selective, PermissionMode::Selective);
}

#[test]
fn test_tool_call_status_variants() {
    // Test all tool call status variants
    let pending = ToolCallStatus::Pending;
    let in_progress = ToolCallStatus::InProgress;
    let completed = ToolCallStatus::Completed;
    let failed = ToolCallStatus::Failed;
    let running = ToolCallStatus::Running;
    let finished = ToolCallStatus::Finished;
    let error = ToolCallStatus::Error;

    // Test Debug formatting
    assert_eq!(format!("{:?}", pending), "Pending");
    assert_eq!(format!("{:?}", in_progress), "InProgress");
    assert_eq!(format!("{:?}", completed), "Completed");
    assert_eq!(format!("{:?}", failed), "Failed");
    assert_eq!(format!("{:?}", running), "Running");
    assert_eq!(format!("{:?}", finished), "Finished");
    assert_eq!(format!("{:?}", error), "Error");

    // Test equality
    assert_eq!(pending, ToolCallStatus::Pending);
    assert_ne!(pending, in_progress);
}

#[test]
fn test_tool_call_status_serialization() {
    // Test serialization
    let pending = ToolCallStatus::Pending;
    let json = serde_json::to_string(&pending).unwrap();
    assert_eq!(json, "\"pending\"");

    let in_progress = ToolCallStatus::InProgress;
    let json = serde_json::to_string(&in_progress).unwrap();
    assert_eq!(json, "\"in_progress\"");

    let running = ToolCallStatus::Running;
    let json = serde_json::to_string(&running).unwrap();
    assert_eq!(json, "\"running\"");
}

#[test]
fn test_plan_priority_variants() {
    // Test all plan priority variants
    let high = PlanPriority::High;
    let medium = PlanPriority::Medium;
    let low = PlanPriority::Low;

    // Test Debug formatting
    assert_eq!(format!("{:?}", high), "High");
    assert_eq!(format!("{:?}", medium), "Medium");
    assert_eq!(format!("{:?}", low), "Low");

    // Test equality
    assert_eq!(high, PlanPriority::High);
    assert_ne!(high, medium);

    // Test default
    assert_eq!(PlanPriority::default(), PlanPriority::Medium);
}

#[test]
fn test_plan_priority_serialization() {
    let high = PlanPriority::High;
    let json = serde_json::to_string(&high).unwrap();
    assert_eq!(json, "\"high\"");

    let medium = PlanPriority::Medium;
    let json = serde_json::to_string(&medium).unwrap();
    assert_eq!(json, "\"medium\"");

    let low = PlanPriority::Low;
    let json = serde_json::to_string(&low).unwrap();
    assert_eq!(json, "\"low\"");
}

#[test]
fn test_plan_status_variants() {
    // Test all plan status variants
    let pending = PlanStatus::Pending;
    let in_progress = PlanStatus::InProgress;
    let completed = PlanStatus::Completed;

    // Test Debug formatting
    assert_eq!(format!("{:?}", pending), "Pending");
    assert_eq!(format!("{:?}", in_progress), "InProgress");
    assert_eq!(format!("{:?}", completed), "Completed");

    // Test equality
    assert_eq!(pending, PlanStatus::Pending);
    assert_ne!(pending, in_progress);

    // Test default
    assert_eq!(PlanStatus::default(), PlanStatus::Pending);
}

#[test]
fn test_plan_status_serialization() {
    let pending = PlanStatus::Pending;
    let json = serde_json::to_string(&pending).unwrap();
    assert_eq!(json, "\"pending\"");

    let in_progress = PlanStatus::InProgress;
    let json = serde_json::to_string(&in_progress).unwrap();
    assert_eq!(json, "\"in_progress\"");

    let completed = PlanStatus::Completed;
    let json = serde_json::to_string(&completed).unwrap();
    assert_eq!(json, "\"completed\"");
}

#[test]
fn test_plan_entry_default() {
    let entry = PlanEntry::default();
    assert_eq!(entry.content, "");
    assert_eq!(entry.priority, PlanPriority::Medium);
    assert_eq!(entry.status, PlanStatus::Pending);
}

#[test]
fn test_plan_entry_serialization() {
    let entry = PlanEntry {
        content: "Test task".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };

    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("Test task"));
    assert!(json.contains("high"));
    assert!(json.contains("in_progress"));
}

#[test]
fn test_plan_entry_deserialization() {
    let json = r#"{"content":"Test task","priority":"high","status":"in_progress"}"#;
    let entry: PlanEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.content, "Test task");
    assert_eq!(entry.priority, PlanPriority::High);
    assert_eq!(entry.status, PlanStatus::InProgress);
}

#[test]
fn test_user_message_chunk_text() {
    let chunk = UserMessageChunk::Text {
        content: "Hello, world!".to_string(),
    };

    match chunk {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Hello, world!");
        }
        _ => panic!("Expected Text chunk"),
    }
}

#[test]
fn test_user_message_chunk_path() {
    let chunk = UserMessageChunk::Path {
        path: PathBuf::from("/tmp/test.txt"),
    };

    match chunk {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, PathBuf::from("/tmp/test.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }
}

#[test]
fn test_user_message_chunk_serialization() {
    let text_chunk = UserMessageChunk::Text {
        content: "Hello".to_string(),
    };
    let json = serde_json::to_string(&text_chunk).unwrap();
    assert!(json.contains("Hello"));

    let path_chunk = UserMessageChunk::Path {
        path: PathBuf::from("/tmp/test.txt"),
    };
    let json = serde_json::to_string(&path_chunk).unwrap();
    assert!(json.contains("/tmp/test.txt"));
}

#[test]
fn test_user_message_new_text() {
    let message = UserMessage::new_text("Test message".to_string());
    assert_eq!(message.message_type, "user");
    assert_eq!(message.chunks.len(), 1);

    match &message.chunks[0] {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Test message");
        }
        _ => panic!("Expected Text chunk"),
    }
}

#[test]
fn test_user_message_new_path() {
    let message = UserMessage::new_path(PathBuf::from("/tmp/file.txt"));
    assert_eq!(message.message_type, "user");
    assert_eq!(message.chunks.len(), 1);

    match &message.chunks[0] {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, &PathBuf::from("/tmp/file.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }
}

#[test]
fn test_user_message_new_with_chunks() {
    let chunks = vec![
        UserMessageChunk::Text {
            content: "Part 1".to_string(),
        },
        UserMessageChunk::Path {
            path: PathBuf::from("/tmp/file.txt"),
        },
        UserMessageChunk::Text {
            content: "Part 2".to_string(),
        },
    ];

    let message = UserMessage::new(chunks);
    assert_eq!(message.message_type, "user");
    assert_eq!(message.chunks.len(), 3);
}

#[test]
fn test_process_config_default() {
    let config = ProcessConfig::default();
    assert!(config.auto_start);
    assert!(config.start_port.is_none());
}

#[test]
fn test_process_config_builder() {
    let config = ProcessConfig::new()
        .enable_auto_start()
        .start_port(8080);

    assert!(config.auto_start);
    assert_eq!(config.start_port, Some(8080));
}

#[test]
fn test_process_config_stdio_mode() {
    let config = ProcessConfig::new().start_port(8080).stdio_mode();
    assert!(config.start_port.is_none());
}

#[test]
fn test_process_config_manual_start() {
    let config = ProcessConfig::new().manual_start();
    assert!(!config.auto_start);
}

#[test]
fn test_websocket_config_default() {
    let config = WebSocketConfig::default();
    assert_eq!(
        config.url,
        Some("ws://localhost:8090/acp?peer=iflow".to_string())
    );
    assert_eq!(config.reconnect_attempts, 3);
    assert_eq!(config.reconnect_interval, std::time::Duration::from_secs(5));
}

#[test]
fn test_websocket_config_new() {
    let config = WebSocketConfig::new("ws://localhost:8080".to_string());
    assert_eq!(config.url, Some("ws://localhost:8080".to_string()));
}

#[test]
fn test_websocket_config_auto_start() {
    let config = WebSocketConfig::auto_start();
    assert_eq!(config.url, None);
    assert_eq!(config.reconnect_attempts, 3);
}

#[test]
fn test_websocket_config_with_reconnect_settings() {
    let config = WebSocketConfig::with_reconnect_settings(
        "ws://localhost:8080".to_string(),
        5,
        std::time::Duration::from_secs(2),
    );

    assert_eq!(config.url, Some("ws://localhost:8080".to_string()));
    assert_eq!(config.reconnect_attempts, 5);
    assert_eq!(config.reconnect_interval, std::time::Duration::from_secs(2));
}

#[test]
fn test_websocket_config_auto_start_with_reconnect() {
    let config =
        WebSocketConfig::auto_start_with_reconnect_settings(10, std::time::Duration::from_secs(3));

    assert_eq!(config.url, None);
    assert_eq!(config.reconnect_attempts, 10);
    assert_eq!(config.reconnect_interval, std::time::Duration::from_secs(3));
}

#[test]
fn test_websocket_config_clone() {
    let config1 = WebSocketConfig::new("ws://localhost:8080".to_string());
    let config2 = config1.clone();

    assert_eq!(config1.url, config2.url);
    assert_eq!(config1.reconnect_attempts, config2.reconnect_attempts);
}

#[test]
fn test_websocket_config_debug() {
    let config = WebSocketConfig::auto_start();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("WebSocketConfig"));
}
