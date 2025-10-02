//! Comprehensive tests for message types in iFlow SDK

use iflow_cli_sdk_rust::types::{
    PlanEntry, PlanPriority, PlanStatus, ToolCallMessage, Icon, UserMessage, UserMessageChunk,
    ErrorMessageDetails, Message
};
use std::path::PathBuf;

/// Tests for PlanMessage functionality
#[tokio::test]
async fn test_plan_message_comprehensive() {
    // Test PlanEntry with all fields
    let entry = PlanEntry {
        content: "Implement user authentication".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };

    assert_eq!(entry.content, "Implement user authentication");
    assert_eq!(entry.priority, PlanPriority::High);
    assert_eq!(entry.status, PlanStatus::InProgress);

    // Test PlanEntry with default values
    let default_entry = PlanEntry::default();
    assert_eq!(default_entry.content, "");
    assert_eq!(default_entry.priority, PlanPriority::Medium);
    assert_eq!(default_entry.status, PlanStatus::Pending);

    // Test PlanPriority variants
    assert_eq!(PlanPriority::High as i32, PlanPriority::High as i32);
    assert_eq!(PlanPriority::Medium as i32, PlanPriority::Medium as i32);
    assert_eq!(PlanPriority::Low as i32, PlanPriority::Low as i32);

    // Test PlanStatus variants
    assert_eq!(PlanStatus::Pending as i32, PlanStatus::Pending as i32);
    assert_eq!(PlanStatus::InProgress as i32, PlanStatus::InProgress as i32);
    assert_eq!(PlanStatus::Completed as i32, PlanStatus::Completed as i32);

    // Test PlanMessage in Message enum
    let plan_entries = vec![
        PlanEntry {
            content: "Task 1".to_string(),
            priority: PlanPriority::High,
            status: PlanStatus::Pending,
        },
        PlanEntry {
            content: "Task 2".to_string(),
            priority: PlanPriority::Medium,
            status: PlanStatus::InProgress,
        }
    ];

    let plan_message = Message::Plan {
        entries: plan_entries.clone(),
    };

    match &plan_message {
        Message::Plan { entries } => {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].content, "Task 1");
            assert_eq!(entries[0].priority, PlanPriority::High);
            assert_eq!(entries[0].status, PlanStatus::Pending);
            assert_eq!(entries[1].content, "Task 2");
            assert_eq!(entries[1].priority, PlanPriority::Medium);
            assert_eq!(entries[1].status, PlanStatus::InProgress);
        }
        _ => panic!("Expected Plan message"),
    }

    // Test is_task_finish method with Plan message (should be false)
    assert!(!plan_message.is_task_finish());

    // Test get_text method with Plan message (should be None)
    assert!(plan_message.get_text().is_none());
}

/// Tests for ToolCallMessage functionality
#[tokio::test]
async fn test_tool_call_message_comprehensive() {
    // Test Icon creation
    let icon = Icon {
        icon_type: "tool".to_string(),
        value: "gear".to_string(),
    };
    assert_eq!(icon.icon_type, "tool");
    assert_eq!(icon.value, "gear");

    // Test ToolCallMessage creation with new method
    let tool_call = ToolCallMessage::new(
        "12345".to_string(),
        "File Reader".to_string(),
        icon.clone(),
        iflow_cli_sdk_rust::types::ToolCallStatus::Running,
    );

    assert_eq!(tool_call.message_type, "tool_call");
    assert_eq!(tool_call.id, "12345");
    assert_eq!(tool_call.label, "File Reader");
    assert_eq!(tool_call.icon.icon_type, "tool");
    assert_eq!(tool_call.icon.value, "gear");
    assert_eq!(tool_call.status, iflow_cli_sdk_rust::types::ToolCallStatus::Running);
    assert!(tool_call.tool_name.is_none());
    assert!(tool_call.content.is_none());
    assert!(tool_call.locations.is_none());
    assert!(tool_call.confirmation.is_none());
    assert!(tool_call.agent_id.is_none());
    assert!(tool_call.agent_info.is_none());

    // Test ToolCallMessage with all optional fields
    let mut tool_call_with_fields = tool_call.clone();
    tool_call_with_fields.tool_name = Some("read_file".to_string());
    tool_call_with_fields.agent_id = Some("agent_1".to_string());

    assert_eq!(tool_call_with_fields.tool_name, Some("read_file".to_string()));
    assert_eq!(tool_call_with_fields.agent_id, Some("agent_1".to_string()));

    // Test ToolCallMessage in Message enum
    let tool_message = Message::ToolCall {
        id: "tool_1".to_string(),
        name: "Test Tool".to_string(),
        status: "completed".to_string(),
    };

    match &tool_message {
        Message::ToolCall { id, name, status } => {
            assert_eq!(id, "tool_1");
            assert_eq!(name, "Test Tool");
            assert_eq!(status, "completed");
        }
        _ => panic!("Expected ToolCall message"),
    }

    // Test is_task_finish method with ToolCall message (should be false)
    assert!(!tool_message.is_task_finish());

    // Test get_text method with ToolCall message (should be None)
    assert!(tool_message.get_text().is_none());

    // Test is_error method with ToolCall message (should be false)
    assert!(!tool_message.is_error());
}

/// Tests for UserMessage functionality
#[tokio::test]
async fn test_user_message_comprehensive() {
    // Test UserMessageChunk Text variant
    let text_chunk = UserMessageChunk::Text {
        content: "Hello, iFlow!".to_string(),
    };

    match &text_chunk {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Hello, iFlow!");
        }
        _ => panic!("Expected Text chunk"),
    }

    // Test UserMessageChunk Path variant
    let path_chunk = UserMessageChunk::Path {
        path: PathBuf::from("/test/file.txt"),
    };

    match &path_chunk {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, &PathBuf::from("/test/file.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }

    // Test UserMessage::new_text method
    let text_message = UserMessage::new_text("Hello, iFlow!".to_string());
    assert_eq!(text_message.message_type, "user");
    assert_eq!(text_message.chunks.len(), 1);
    
    match &text_message.chunks[0] {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Hello, iFlow!");
        }
        _ => panic!("Expected Text chunk"),
    }

    // Test UserMessage::new_path method
    let path_message = UserMessage::new_path(PathBuf::from("/test/file.txt"));
    assert_eq!(path_message.message_type, "user");
    assert_eq!(path_message.chunks.len(), 1);
    
    match &path_message.chunks[0] {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, &PathBuf::from("/test/file.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }

    // Test UserMessage::new method with multiple chunks
    let multi_chunk_message = UserMessage::new(vec![
        UserMessageChunk::Text {
            content: "Check this file: ".to_string(),
        },
        UserMessageChunk::Path {
            path: PathBuf::from("/test/file.txt"),
        },
    ]);

    assert_eq!(multi_chunk_message.message_type, "user");
    assert_eq!(multi_chunk_message.chunks.len(), 2);
    
    match &multi_chunk_message.chunks[0] {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Check this file: ");
        }
        _ => panic!("Expected Text chunk"),
    }
    
    match &multi_chunk_message.chunks[1] {
        UserMessageChunk::Path { path } => {
            assert_eq!(path, &PathBuf::from("/test/file.txt"));
        }
        _ => panic!("Expected Path chunk"),
    }

    // Test UserMessage in Message enum
    let user_message = Message::User {
        content: "Hello, iFlow!".to_string(),
    };

    match &user_message {
        Message::User { content } => {
            assert_eq!(content, "Hello, iFlow!");
        }
        _ => panic!("Expected User message"),
    }

    // Test is_task_finish method with User message (should be false)
    assert!(!user_message.is_task_finish());

    // Test get_text method with User message (should return Some)
    assert_eq!(user_message.get_text(), Some("Hello, iFlow!"));

    // Test is_error method with User message (should be false)
    assert!(!user_message.is_error());
}

/// Tests for ErrorMessage functionality
#[tokio::test]
async fn test_error_message_comprehensive() {
    // Test ErrorMessageDetails::new method
    let error_details = ErrorMessageDetails::new(404, "Not Found".to_string());
    assert_eq!(error_details.code, 404);
    assert_eq!(error_details.message, "Not Found");
    assert!(error_details.details.is_none());

    // Test ErrorMessageDetails::with_details method
    let mut details_map = std::collections::HashMap::new();
    details_map.insert("path".to_string(), serde_json::Value::String("/test".to_string()));
    details_map.insert("method".to_string(), serde_json::Value::String("GET".to_string()));

    let error_with_details = ErrorMessageDetails::with_details(
        500,
        "Internal Server Error".to_string(),
        details_map.clone(),
    );

    assert_eq!(error_with_details.code, 500);
    assert_eq!(error_with_details.message, "Internal Server Error");
    assert!(error_with_details.details.is_some());
    
    if let Some(details) = &error_with_details.details {
        assert_eq!(details.len(), 2);
        assert_eq!(details.get("path"), Some(&serde_json::Value::String("/test".to_string())));
        assert_eq!(details.get("method"), Some(&serde_json::Value::String("GET".to_string())));
    }

    // Test Message::error method
    let error_message = Message::error(400, "Bad Request".to_string());
    
    match &error_message {
        Message::Error { code, message, details } => {
            assert_eq!(*code, 400);
            assert_eq!(message, "Bad Request");
            assert!(details.is_none());
        }
        _ => panic!("Expected Error message"),
    }

    // Test Message::error_with_details method
    let error_message_with_details = Message::error_with_details(
        500,
        "Internal Server Error".to_string(),
        details_map.clone(),
    );

    match &error_message_with_details {
        Message::Error { code, message, details } => {
            assert_eq!(*code, 500);
            assert_eq!(message, "Internal Server Error");
            assert!(details.is_some());
            
            if let Some(details) = details {
                assert_eq!(details.len(), 2);
                assert_eq!(details.get("path"), Some(&serde_json::Value::String("/test".to_string())));
                assert_eq!(details.get("method"), Some(&serde_json::Value::String("GET".to_string())));
            }
        }
        _ => panic!("Expected Error message"),
    }

    // Test is_task_finish method with Error message (should be false)
    assert!(!error_message.is_task_finish());

    // Test get_text method with Error message (should be None)
    assert!(error_message.get_text().is_none());

    // Test is_error method with Error message (should be true)
    assert!(error_message.is_error());

    // Test is_error method with non-error message (should be false)
    let user_message = Message::User {
        content: "Hello".to_string(),
    };
    assert!(!user_message.is_error());
}