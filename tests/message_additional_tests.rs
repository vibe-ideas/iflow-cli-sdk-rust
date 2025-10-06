//! Additional comprehensive unit tests for Message types
//!
//! These tests cover all Message variants and their methods

use iflow_cli_sdk_rust::{Message, PlanEntry, PlanPriority, PlanStatus};
use std::collections::HashMap;

#[test]
fn test_message_user_variant() {
    let msg = Message::User {
        content: "Hello".to_string(),
    };
    
    match msg {
        Message::User { content } => {
            assert_eq!(content, "Hello");
        }
        _ => panic!("Expected User message"),
    }
}

#[test]
fn test_message_assistant_variant() {
    let msg = Message::Assistant {
        content: "Response".to_string(),
    };
    
    match msg {
        Message::Assistant { content } => {
            assert_eq!(content, "Response");
        }
        _ => panic!("Expected Assistant message"),
    }
}

#[test]
fn test_message_tool_call_variant() {
    let msg = Message::ToolCall {
        id: "tool1".to_string(),
        name: "execute".to_string(),
        status: "running".to_string(),
    };
    
    match msg {
        Message::ToolCall { id, name, status } => {
            assert_eq!(id, "tool1");
            assert_eq!(name, "execute");
            assert_eq!(status, "running");
        }
        _ => panic!("Expected ToolCall message"),
    }
}

#[test]
fn test_message_plan_variant() {
    let entries = vec![
        PlanEntry {
            content: "Task 1".to_string(),
            priority: PlanPriority::High,
            status: PlanStatus::Pending,
        },
    ];
    
    let msg = Message::Plan {
        entries: entries.clone(),
    };
    
    match msg {
        Message::Plan { entries } => {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].content, "Task 1");
        }
        _ => panic!("Expected Plan message"),
    }
}

#[test]
fn test_message_task_finish_variant() {
    let msg = Message::TaskFinish {
        reason: Some("completed".to_string()),
    };
    
    match msg {
        Message::TaskFinish { reason } => {
            assert_eq!(reason, Some("completed".to_string()));
        }
        _ => panic!("Expected TaskFinish message"),
    }
}

#[test]
fn test_message_task_finish_no_reason() {
    let msg = Message::TaskFinish { reason: None };
    
    match msg {
        Message::TaskFinish { reason } => {
            assert_eq!(reason, None);
        }
        _ => panic!("Expected TaskFinish message"),
    }
}

#[test]
fn test_message_error_variant() {
    let msg = Message::Error {
        code: 500,
        message: "Internal error".to_string(),
        details: None,
    };
    
    match msg {
        Message::Error { code, message, details } => {
            assert_eq!(code, 500);
            assert_eq!(message, "Internal error");
            assert!(details.is_none());
        }
        _ => panic!("Expected Error message"),
    }
}

#[test]
fn test_message_is_task_finish() {
    let finish_msg = Message::TaskFinish { reason: None };
    assert!(finish_msg.is_task_finish());
    
    let user_msg = Message::User {
        content: "test".to_string(),
    };
    assert!(!user_msg.is_task_finish());
}

#[test]
fn test_message_is_error() {
    let error_msg = Message::error(404, "Not found".to_string());
    assert!(error_msg.is_error());
    
    let user_msg = Message::User {
        content: "test".to_string(),
    };
    assert!(!user_msg.is_error());
}

#[test]
fn test_message_get_text() {
    let user_msg = Message::User {
        content: "User text".to_string(),
    };
    assert_eq!(user_msg.get_text(), Some("User text"));
    
    let assistant_msg = Message::Assistant {
        content: "Assistant text".to_string(),
    };
    assert_eq!(assistant_msg.get_text(), Some("Assistant text"));
    
    let tool_msg = Message::ToolCall {
        id: "1".to_string(),
        name: "tool".to_string(),
        status: "running".to_string(),
    };
    assert_eq!(tool_msg.get_text(), None);
}

#[test]
fn test_message_error_constructor() {
    let msg = Message::error(400, "Bad request".to_string());
    
    match msg {
        Message::Error { code, message, details } => {
            assert_eq!(code, 400);
            assert_eq!(message, "Bad request");
            assert!(details.is_none());
        }
        _ => panic!("Expected Error message"),
    }
}

#[test]
fn test_message_error_with_details() {
    let mut details_map = HashMap::new();
    details_map.insert(
        "field".to_string(),
        serde_json::Value::String("invalid".to_string()),
    );
    
    let msg = Message::error_with_details(422, "Validation error".to_string(), details_map.clone());
    
    match msg {
        Message::Error { code, message, details } => {
            assert_eq!(code, 422);
            assert_eq!(message, "Validation error");
            assert!(details.is_some());
            
            if let Some(d) = details {
                assert_eq!(d.get("field"), Some(&serde_json::Value::String("invalid".to_string())));
            }
        }
        _ => panic!("Expected Error message"),
    }
}

#[test]
fn test_message_serialization() {
    let user_msg = Message::User {
        content: "test".to_string(),
    };
    let json = serde_json::to_string(&user_msg).unwrap();
    assert!(json.contains("user"));
    assert!(json.contains("test"));
}

#[test]
fn test_message_deserialization() {
    let json = r#"{"type":"user","content":"test"}"#;
    let msg: Message = serde_json::from_str(json).unwrap();
    
    match msg {
        Message::User { content } => {
            assert_eq!(content, "test");
        }
        _ => panic!("Expected User message"),
    }
}

#[test]
fn test_message_clone() {
    let original = Message::User {
        content: "test".to_string(),
    };
    let cloned = original.clone();
    
    assert_eq!(original.get_text(), cloned.get_text());
}

#[test]
fn test_message_debug_format() {
    let msg = Message::User {
        content: "test".to_string(),
    };
    let debug_str = format!("{:?}", msg);
    assert!(debug_str.contains("User"));
}

#[test]
fn test_message_plan_empty() {
    let msg = Message::Plan {
        entries: Vec::new(),
    };
    
    match msg {
        Message::Plan { entries } => {
            assert_eq!(entries.len(), 0);
        }
        _ => panic!("Expected Plan message"),
    }
}

#[test]
fn test_message_plan_multiple_entries() {
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
    
    match msg {
        Message::Plan { entries } => {
            assert_eq!(entries.len(), 3);
            assert_eq!(entries[0].priority, PlanPriority::High);
            assert_eq!(entries[1].status, PlanStatus::InProgress);
            assert_eq!(entries[2].content, "Task 3");
        }
        _ => panic!("Expected Plan message"),
    }
}

#[test]
fn test_all_message_variants_not_task_finish() {
    let messages = vec![
        Message::User { content: "test".to_string() },
        Message::Assistant { content: "test".to_string() },
        Message::ToolCall {
            id: "1".to_string(),
            name: "tool".to_string(),
            status: "running".to_string(),
        },
        Message::Plan { entries: Vec::new() },
        Message::error(500, "error".to_string()),
    ];
    
    for msg in messages {
        assert!(!msg.is_task_finish());
    }
}

#[test]
fn test_all_message_variants_not_error() {
    let messages = vec![
        Message::User { content: "test".to_string() },
        Message::Assistant { content: "test".to_string() },
        Message::ToolCall {
            id: "1".to_string(),
            name: "tool".to_string(),
            status: "running".to_string(),
        },
        Message::Plan { entries: Vec::new() },
        Message::TaskFinish { reason: None },
    ];
    
    for msg in messages {
        assert!(!msg.is_error());
    }
}
