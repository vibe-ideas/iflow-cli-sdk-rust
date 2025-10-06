//! Additional comprehensive tests for Message types
//!
//! These tests ensure all Message variants are thoroughly covered

use iflow_cli_sdk_rust::types::{Message, PlanEntry, PlanPriority, PlanStatus};
use std::collections::HashMap;

#[test]
fn test_message_user_variant() {
    let msg = Message::User {
        content: "Hello".to_string(),
    };
    
    assert!(!msg.is_task_finish());
    assert!(!msg.is_error());
    assert_eq!(msg.get_text(), Some("Hello"));
}

#[test]
fn test_message_assistant_variant() {
    let msg = Message::Assistant {
        content: "Response".to_string(),
    };
    
    assert!(!msg.is_task_finish());
    assert!(!msg.is_error());
    assert_eq!(msg.get_text(), Some("Response"));
}

#[test]
fn test_message_tool_call_variant() {
    let msg = Message::ToolCall {
        id: "1".to_string(),
        name: "test_tool".to_string(),
        status: "running".to_string(),
    };
    
    assert!(!msg.is_task_finish());
    assert!(!msg.is_error());
    assert_eq!(msg.get_text(), None);
}

#[test]
fn test_message_plan_variant() {
    let entry = PlanEntry {
        content: "Task 1".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::Pending,
    };
    
    let msg = Message::Plan {
        entries: vec![entry],
    };
    
    assert!(!msg.is_task_finish());
    assert!(!msg.is_error());
    assert_eq!(msg.get_text(), None);
}

#[test]
fn test_message_task_finish_variant() {
    let msg = Message::TaskFinish {
        reason: Some("completed".to_string()),
    };
    
    assert!(msg.is_task_finish());
    assert!(!msg.is_error());
    assert_eq!(msg.get_text(), None);
}

#[test]
fn test_message_task_finish_without_reason() {
    let msg = Message::TaskFinish { reason: None };
    
    assert!(msg.is_task_finish());
    assert!(!msg.is_error());
}

#[test]
fn test_message_error_variant() {
    let msg = Message::Error {
        code: 500,
        message: "Internal error".to_string(),
        details: None,
    };
    
    assert!(!msg.is_task_finish());
    assert!(msg.is_error());
    assert_eq!(msg.get_text(), None);
}

#[test]
fn test_message_error_with_details() {
    let mut details = HashMap::new();
    details.insert(
        "key".to_string(),
        serde_json::Value::String("value".to_string()),
    );
    
    let msg = Message::Error {
        code: 400,
        message: "Bad request".to_string(),
        details: Some(details.clone()),
    };
    
    assert!(msg.is_error());
}

#[test]
fn test_message_error_constructor() {
    let msg = Message::error(404, "Not found".to_string());
    
    match msg {
        Message::Error {
            code,
            message,
            details,
        } => {
            assert_eq!(code, 404);
            assert_eq!(message, "Not found");
            assert!(details.is_none());
        }
        _ => panic!("Expected Error message"),
    }
}

#[test]
fn test_message_error_with_details_constructor() {
    let mut details = HashMap::new();
    details.insert(
        "path".to_string(),
        serde_json::Value::String("/api".to_string()),
    );
    
    let msg = Message::error_with_details(422, "Validation failed".to_string(), details.clone());
    
    match msg {
        Message::Error {
            code,
            message,
            details: msg_details,
        } => {
            assert_eq!(code, 422);
            assert_eq!(message, "Validation failed");
            assert!(msg_details.is_some());
        }
        _ => panic!("Expected Error message"),
    }
}

#[test]
fn test_message_serialization_user() {
    let msg = Message::User {
        content: "test".to_string(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("user"));
    assert!(json.contains("test"));
}

#[test]
fn test_message_serialization_assistant() {
    let msg = Message::Assistant {
        content: "response".to_string(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("assistant"));
    assert!(json.contains("response"));
}

#[test]
fn test_message_serialization_tool_call() {
    let msg = Message::ToolCall {
        id: "tool1".to_string(),
        name: "tool".to_string(),
        status: "done".to_string(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("tool_call"));
}

#[test]
fn test_message_serialization_plan() {
    let msg = Message::Plan {
        entries: vec![PlanEntry::default()],
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("plan"));
}

#[test]
fn test_message_serialization_task_finish() {
    let msg = Message::TaskFinish {
        reason: Some("done".to_string()),
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("task_finish"));
}

#[test]
fn test_message_serialization_error() {
    let msg = Message::Error {
        code: 500,
        message: "error".to_string(),
        details: None,
    };
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("error"));
    assert!(json.contains("500"));
}

#[test]
fn test_message_deserialization_user() {
    let json = r#"{"type":"user","content":"test"}"#;
    let msg: Message = serde_json::from_str(json).unwrap();
    
    match msg {
        Message::User { content } => assert_eq!(content, "test"),
        _ => panic!("Expected User message"),
    }
}

#[test]
fn test_message_deserialization_assistant() {
    let json = r#"{"type":"assistant","content":"response"}"#;
    let msg: Message = serde_json::from_str(json).unwrap();
    
    match msg {
        Message::Assistant { content } => assert_eq!(content, "response"),
        _ => panic!("Expected Assistant message"),
    }
}

#[test]
fn test_message_clone() {
    let msg1 = Message::User {
        content: "test".to_string(),
    };
    let msg2 = msg1.clone();
    
    assert_eq!(msg1.get_text(), msg2.get_text());
}

#[test]
fn test_message_debug() {
    let msg = Message::User {
        content: "test".to_string(),
    };
    let debug_str = format!("{:?}", msg);
    assert!(debug_str.contains("User"));
}

#[test]
fn test_plan_entry_serialization() {
    let entry = PlanEntry {
        content: "Task".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };
    
    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("Task"));
    assert!(json.contains("high"));
    assert!(json.contains("in_progress"));
}

#[test]
fn test_plan_entry_deserialization() {
    let json = r#"{"content":"Task","priority":"low","status":"completed"}"#;
    let entry: PlanEntry = serde_json::from_str(json).unwrap();
    
    assert_eq!(entry.content, "Task");
    assert_eq!(entry.priority, PlanPriority::Low);
    assert_eq!(entry.status, PlanStatus::Completed);
}

#[test]
fn test_message_plan_with_multiple_entries() {
    let entries = vec![
        PlanEntry {
            content: "Task 1".to_string(),
            priority: PlanPriority::High,
            status: PlanStatus::Pending,
        },
        PlanEntry {
            content: "Task 2".to_string(),
            priority: PlanPriority::Medium,
            status: PlanStatus::InProgress,
        },
        PlanEntry {
            content: "Task 3".to_string(),
            priority: PlanPriority::Low,
            status: PlanStatus::Completed,
        },
    ];
    
    let msg = Message::Plan {
        entries: entries.clone(),
    };
    
    assert!(!msg.is_task_finish());
    assert!(!msg.is_error());
}
