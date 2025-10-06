//! Additional unit tests for the types module to improve coverage
//!
//! These tests cover type creation, serialization, and edge cases

use iflow_cli_sdk_rust::{
    PermissionMode, ToolCallStatus, PlanPriority, PlanStatus, PlanEntry,
    UserMessageChunk, UserMessage, ProcessConfig, WebSocketConfig, IFlowOptions,
};
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn test_permission_mode_variants() {
    let auto = PermissionMode::Auto;
    let manual = PermissionMode::Manual;
    let selective = PermissionMode::Selective;
    
    assert_eq!(auto, PermissionMode::Auto);
    assert_eq!(manual, PermissionMode::Manual);
    assert_eq!(selective, PermissionMode::Selective);
}

#[test]
fn test_permission_mode_default() {
    let default_mode: PermissionMode = Default::default();
    assert_eq!(default_mode, PermissionMode::Auto);
}

#[test]
fn test_permission_mode_serialization() {
    let auto = PermissionMode::Auto;
    let json = serde_json::to_string(&auto).unwrap();
    assert_eq!(json, r#""auto""#);
    
    let manual = PermissionMode::Manual;
    let json = serde_json::to_string(&manual).unwrap();
    assert_eq!(json, r#""manual""#);
    
    let selective = PermissionMode::Selective;
    let json = serde_json::to_string(&selective).unwrap();
    assert_eq!(json, r#""selective""#);
}

#[test]
fn test_permission_mode_deserialization() {
    let auto: PermissionMode = serde_json::from_str(r#""auto""#).unwrap();
    assert_eq!(auto, PermissionMode::Auto);
    
    let manual: PermissionMode = serde_json::from_str(r#""manual""#).unwrap();
    assert_eq!(manual, PermissionMode::Manual);
    
    let selective: PermissionMode = serde_json::from_str(r#""selective""#).unwrap();
    assert_eq!(selective, PermissionMode::Selective);
}

#[test]
fn test_tool_call_status_variants() {
    assert!(matches!(ToolCallStatus::Pending, ToolCallStatus::Pending));
    assert!(matches!(ToolCallStatus::InProgress, ToolCallStatus::InProgress));
    assert!(matches!(ToolCallStatus::Completed, ToolCallStatus::Completed));
    assert!(matches!(ToolCallStatus::Failed, ToolCallStatus::Failed));
    assert!(matches!(ToolCallStatus::Running, ToolCallStatus::Running));
    assert!(matches!(ToolCallStatus::Finished, ToolCallStatus::Finished));
    assert!(matches!(ToolCallStatus::Error, ToolCallStatus::Error));
}

#[test]
fn test_tool_call_status_serialization() {
    let pending = ToolCallStatus::Pending;
    let json = serde_json::to_string(&pending).unwrap();
    assert_eq!(json, r#""pending""#);
    
    let in_progress = ToolCallStatus::InProgress;
    let json = serde_json::to_string(&in_progress).unwrap();
    assert_eq!(json, r#""in_progress""#);
    
    let completed = ToolCallStatus::Completed;
    let json = serde_json::to_string(&completed).unwrap();
    assert_eq!(json, r#""completed""#);
}

#[test]
fn test_plan_priority_variants() {
    assert_eq!(PlanPriority::High, PlanPriority::High);
    assert_eq!(PlanPriority::Medium, PlanPriority::Medium);
    assert_eq!(PlanPriority::Low, PlanPriority::Low);
}

#[test]
fn test_plan_priority_default() {
    let default: PlanPriority = Default::default();
    assert_eq!(default, PlanPriority::Medium);
}

#[test]
fn test_plan_status_variants() {
    assert_eq!(PlanStatus::Pending, PlanStatus::Pending);
    assert_eq!(PlanStatus::InProgress, PlanStatus::InProgress);
    assert_eq!(PlanStatus::Completed, PlanStatus::Completed);
}

#[test]
fn test_plan_status_default() {
    let default: PlanStatus = Default::default();
    assert_eq!(default, PlanStatus::Pending);
}

#[test]
fn test_plan_entry_creation() {
    let entry = PlanEntry {
        content: "Test task".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };
    
    assert_eq!(entry.content, "Test task");
    assert_eq!(entry.priority, PlanPriority::High);
    assert_eq!(entry.status, PlanStatus::InProgress);
}

#[test]
fn test_plan_entry_default() {
    let entry: PlanEntry = Default::default();
    assert_eq!(entry.content, "");
    assert_eq!(entry.priority, PlanPriority::Medium);
    assert_eq!(entry.status, PlanStatus::Pending);
}

#[test]
fn test_user_message_chunk_text() {
    let chunk = UserMessageChunk::Text {
        content: "Hello".to_string(),
    };
    
    match chunk {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Hello");
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
fn test_user_message_with_text() {
    let msg = UserMessage::new_text("Test message".to_string());
    assert_eq!(msg.message_type, "user");
    assert_eq!(msg.chunks.len(), 1);
    
    match &msg.chunks[0] {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Test message");
        }
        _ => panic!("Expected Text chunk"),
    }
}

#[test]
fn test_user_message_with_path() {
    let msg = UserMessage::new_path(PathBuf::from("/tmp/file.txt"));
    assert_eq!(msg.message_type, "user");
    assert_eq!(msg.chunks.len(), 1);
    
    match &msg.chunks[0] {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, &PathBuf::from("/tmp/file.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }
}

#[test]
fn test_user_message_with_multiple_chunks() {
    let chunks = vec![
        UserMessageChunk::Text { content: "Message".to_string() },
        UserMessageChunk::Path { path: PathBuf::from("/tmp/file.txt") },
    ];
    let msg = UserMessage::new(chunks);
    
    assert_eq!(msg.message_type, "user");
    assert_eq!(msg.chunks.len(), 2);
    
    // First chunk should be text
    match &msg.chunks[0] {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Message");
        }
        _ => panic!("Expected Text chunk"),
    }
    
    // Second chunk should be path
    match &msg.chunks[1] {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, &PathBuf::from("/tmp/file.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }
}

#[test]
fn test_process_config_defaults() {
    let config = ProcessConfig::new();
    assert_eq!(config.auto_start, true);
    assert_eq!(config.start_port, None);
}

#[test]
fn test_process_config_enable_auto_start() {
    let config = ProcessConfig::new().enable_auto_start();
    assert_eq!(config.auto_start, true);
}

#[test]
fn test_process_config_disable_auto_start() {
    let config = ProcessConfig::new().manual_start();
    assert_eq!(config.auto_start, false);
}

#[test]
fn test_process_config_manual_start() {
    let config = ProcessConfig::new().manual_start();
    assert_eq!(config.auto_start, false);
}

#[test]
fn test_process_config_start_port() {
    let config = ProcessConfig::new().start_port(9000);
    assert_eq!(config.start_port, Some(9000));
}

#[test]
fn test_process_config_stdio_mode() {
    let config = ProcessConfig::new().start_port(9000).stdio_mode();
    assert_eq!(config.start_port, None);
}

#[test]
fn test_process_config_chaining() {
    let config = ProcessConfig::new()
        .enable_auto_start()
        .start_port(9000);
    
    assert_eq!(config.auto_start, true);
    assert_eq!(config.start_port, Some(9000));
}

#[test]
fn test_websocket_config_default() {
    let config = WebSocketConfig::default();
    assert!(config.url.is_some());
    assert_eq!(config.reconnect_attempts, 3);
    assert_eq!(config.reconnect_interval, Duration::from_secs(5));
}

#[test]
fn test_websocket_config_new() {
    let url = "ws://localhost:8090/acp?peer=iflow".to_string();
    let config = WebSocketConfig::new(url.clone());
    
    assert_eq!(config.url, Some(url));
    assert_eq!(config.reconnect_attempts, 3);
    assert_eq!(config.reconnect_interval, Duration::from_secs(5));
}

#[test]
fn test_websocket_config_auto_start() {
    let config = WebSocketConfig::auto_start();
    assert!(config.url.is_none());
    assert_eq!(config.reconnect_attempts, 3);
    assert_eq!(config.reconnect_interval, Duration::from_secs(5));
}

#[test]
fn test_websocket_config_auto_start_with_reconnect() {
    let config = WebSocketConfig::auto_start_with_reconnect_settings(5, Duration::from_secs(10));
    assert!(config.url.is_none());
    assert_eq!(config.reconnect_attempts, 5);
    assert_eq!(config.reconnect_interval, Duration::from_secs(10));
}

#[test]
fn test_websocket_config_with_reconnect_settings() {
    let url = "ws://localhost:8090/acp?peer=iflow".to_string();
    let config = WebSocketConfig::with_reconnect_settings(url.clone(), 7, Duration::from_secs(15));
    
    assert_eq!(config.url, Some(url));
    assert_eq!(config.reconnect_attempts, 7);
    assert_eq!(config.reconnect_interval, Duration::from_secs(15));
}

#[test]
fn test_iflow_options_defaults() {
    let options = IFlowOptions::default();
    assert_eq!(options.timeout, 120.0);
    assert_eq!(options.process.auto_start, true);
}

#[test]
fn test_iflow_options_new() {
    let options = IFlowOptions::new();
    assert_eq!(options.timeout, 120.0);
}

#[test]
fn test_iflow_options_with_timeout() {
    let options = IFlowOptions::new().with_timeout(60.0);
    assert_eq!(options.timeout, 60.0);
}

#[test]
fn test_iflow_options_with_auto_start() {
    let options = IFlowOptions::new().with_auto_start(false);
    assert_eq!(options.process.auto_start, false);
}

#[test]
fn test_iflow_options_with_process_config() {
    let process_config = ProcessConfig::new()
        .manual_start()
        .start_port(9000);
    
    let options = IFlowOptions::new().with_process_config(process_config);
    assert_eq!(options.process.auto_start, false);
    assert_eq!(options.process.start_port, Some(9000));
}

#[test]
fn test_iflow_options_with_websocket_config() {
    let ws_config = WebSocketConfig::auto_start();
    let options = IFlowOptions::new().with_websocket_config(ws_config);
    
    assert!(options.websocket.is_some());
    if let Some(ws) = options.websocket {
        assert!(ws.url.is_none());
    }
}

#[test]
fn test_iflow_options_chaining() {
    let options = IFlowOptions::new()
        .with_timeout(90.0)
        .with_process_config(ProcessConfig::new().manual_start().start_port(9000));
    
    assert_eq!(options.timeout, 90.0);
    assert_eq!(options.process.auto_start, false);
    assert_eq!(options.process.start_port, Some(9000));
}

#[test]
fn test_permission_mode_clone() {
    let original = PermissionMode::Auto;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_tool_call_status_clone() {
    let original = ToolCallStatus::InProgress;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_plan_priority_clone() {
    let original = PlanPriority::High;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_plan_status_clone() {
    let original = PlanStatus::Completed;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_plan_entry_clone() {
    let original = PlanEntry {
        content: "Test".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::Pending,
    };
    let cloned = original.clone();
    assert_eq!(original.content, cloned.content);
    assert_eq!(original.priority, cloned.priority);
    assert_eq!(original.status, cloned.status);
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
    assert!(json.contains("test.txt"));
}

#[test]
fn test_process_config_clone() {
    let original = ProcessConfig::new().start_port(9000);
    let cloned = original.clone();
    assert_eq!(original.start_port, cloned.start_port);
}

#[test]
fn test_websocket_config_clone() {
    let original = WebSocketConfig::auto_start();
    let cloned = original.clone();
    assert_eq!(original.reconnect_attempts, cloned.reconnect_attempts);
}

#[test]
fn test_iflow_options_clone() {
    let original = IFlowOptions::new().with_timeout(90.0);
    let cloned = original.clone();
    assert_eq!(original.timeout, cloned.timeout);
}
