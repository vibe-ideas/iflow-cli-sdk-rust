//! Additional tests for UserMessage and related types
//!
//! These tests improve coverage for complex message types

use iflow_cli_sdk_rust::types::{
    UserMessage, UserMessageChunk, PlanEntry, PlanPriority, PlanStatus,
    Icon, ToolCallConfirmation, ToolCallContent, ToolCallLocation, AgentInfo,
};
use std::path::PathBuf;

#[test]
fn test_user_message_new_text() {
    let message = UserMessage::new_text("Hello, iFlow!".to_string());
    assert_eq!(message.message_type, "user");
    assert_eq!(message.chunks.len(), 1);
    
    match &message.chunks[0] {
        UserMessageChunk::Text { content } => {
            assert_eq!(content, "Hello, iFlow!");
        }
        _ => panic!("Expected Text chunk"),
    }
}

#[test]
fn test_user_message_new_path() {
    let path = PathBuf::from("/tmp/test.txt");
    let message = UserMessage::new_path(path.clone());
    assert_eq!(message.message_type, "user");
    assert_eq!(message.chunks.len(), 1);
    
    match &message.chunks[0] {
        UserMessageChunk::Path { path: p } => {
            assert_eq!(p, &path);
        }
        _ => panic!("Expected Path chunk"),
    }
}

#[test]
fn test_user_message_new_multiple_chunks() {
    let chunks = vec![
        UserMessageChunk::Text {
            content: "Test".to_string(),
        },
        UserMessageChunk::Path {
            path: PathBuf::from("/test"),
        },
    ];
    
    let message = UserMessage::new(chunks.clone());
    assert_eq!(message.message_type, "user");
    assert_eq!(message.chunks.len(), 2);
}

#[test]
fn test_user_message_chunk_text_debug() {
    let chunk = UserMessageChunk::Text {
        content: "test".to_string(),
    };
    let debug_str = format!("{:?}", chunk);
    assert!(debug_str.contains("Text"));
}

#[test]
fn test_user_message_chunk_path_debug() {
    let chunk = UserMessageChunk::Path {
        path: PathBuf::from("/test"),
    };
    let debug_str = format!("{:?}", chunk);
    assert!(debug_str.contains("Path"));
}

#[test]
fn test_user_message_chunk_clone() {
    let chunk1 = UserMessageChunk::Text {
        content: "test".to_string(),
    };
    let chunk2 = chunk1.clone();
    match (&chunk1, &chunk2) {
        (
            UserMessageChunk::Text { content: c1 },
            UserMessageChunk::Text { content: c2 },
        ) => {
            assert_eq!(c1, c2);
        }
        _ => panic!("Expected Text chunks"),
    }
}

#[test]
fn test_user_message_debug() {
    let message = UserMessage::new_text("test".to_string());
    let debug_str = format!("{:?}", message);
    assert!(debug_str.contains("UserMessage"));
}

#[test]
fn test_user_message_clone() {
    let message1 = UserMessage::new_text("test".to_string());
    let message2 = message1.clone();
    assert_eq!(message1.message_type, message2.message_type);
}

#[test]
fn test_user_message_serialization() {
    let message = UserMessage::new_text("test".to_string());
    let json = serde_json::to_string(&message).unwrap();
    assert!(json.contains("user"));
    assert!(json.contains("test"));
}

#[test]
fn test_plan_entry_default() {
    let entry = PlanEntry::default();
    assert_eq!(entry.content, "");
    assert_eq!(entry.priority, PlanPriority::Medium);
    assert_eq!(entry.status, PlanStatus::Pending);
}

#[test]
fn test_plan_entry_creation() {
    let entry = PlanEntry {
        content: "Task 1".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };
    assert_eq!(entry.content, "Task 1");
    assert_eq!(entry.priority, PlanPriority::High);
    assert_eq!(entry.status, PlanStatus::InProgress);
}

#[test]
fn test_plan_entry_clone() {
    let entry1 = PlanEntry {
        content: "Task".to_string(),
        priority: PlanPriority::Low,
        status: PlanStatus::Completed,
    };
    let entry2 = entry1.clone();
    assert_eq!(entry1.content, entry2.content);
}

#[test]
fn test_plan_entry_debug() {
    let entry = PlanEntry::default();
    let debug_str = format!("{:?}", entry);
    assert!(debug_str.contains("PlanEntry"));
}

#[test]
fn test_icon_creation() {
    let icon = Icon {
        icon_type: "emoji".to_string(),
        value: "🚀".to_string(),
    };
    assert_eq!(icon.icon_type, "emoji");
    assert_eq!(icon.value, "🚀");
}

#[test]
fn test_icon_debug() {
    let icon = Icon {
        icon_type: "test".to_string(),
        value: "value".to_string(),
    };
    let debug_str = format!("{:?}", icon);
    assert!(debug_str.contains("Icon"));
}

#[test]
fn test_icon_clone() {
    let icon1 = Icon {
        icon_type: "test".to_string(),
        value: "value".to_string(),
    };
    let icon2 = icon1.clone();
    assert_eq!(icon1.icon_type, icon2.icon_type);
}

#[test]
fn test_tool_call_confirmation_creation() {
    let confirmation = ToolCallConfirmation {
        confirmation_type: "execute".to_string(),
        description: Some("Run command".to_string()),
        command: Some("ls".to_string()),
        root_command: Some("sudo".to_string()),
        server_name: None,
        tool_name: None,
        tool_display_name: None,
        urls: None,
    };
    assert_eq!(confirmation.confirmation_type, "execute");
    assert_eq!(confirmation.command, Some("ls".to_string()));
}

#[test]
fn test_tool_call_confirmation_debug() {
    let confirmation = ToolCallConfirmation {
        confirmation_type: "test".to_string(),
        description: None,
        command: None,
        root_command: None,
        server_name: None,
        tool_name: None,
        tool_display_name: None,
        urls: None,
    };
    let debug_str = format!("{:?}", confirmation);
    assert!(debug_str.contains("ToolCallConfirmation"));
}

#[test]
fn test_tool_call_content_creation() {
    let content = ToolCallContent {
        content_type: "markdown".to_string(),
        markdown: Some("# Title".to_string()),
        path: None,
        old_text: None,
        new_text: None,
    };
    assert_eq!(content.content_type, "markdown");
    assert_eq!(content.markdown, Some("# Title".to_string()));
}

#[test]
fn test_tool_call_content_debug() {
    let content = ToolCallContent {
        content_type: "test".to_string(),
        markdown: None,
        path: None,
        old_text: None,
        new_text: None,
    };
    let debug_str = format!("{:?}", content);
    assert!(debug_str.contains("ToolCallContent"));
}

#[test]
fn test_tool_call_location_creation() {
    let location = ToolCallLocation {
        path: "/test/file.rs".to_string(),
        line_start: Some(10),
        line_end: Some(20),
    };
    assert_eq!(location.path, "/test/file.rs");
    assert_eq!(location.line_start, Some(10));
    assert_eq!(location.line_end, Some(20));
}

#[test]
fn test_tool_call_location_debug() {
    let location = ToolCallLocation {
        path: "/test".to_string(),
        line_start: None,
        line_end: None,
    };
    let debug_str = format!("{:?}", location);
    assert!(debug_str.contains("ToolCallLocation"));
}

#[test]
fn test_agent_info_creation() {
    let info = AgentInfo {
        agent_id: "agent_123".to_string(),
        agent_index: Some(1),
        task_id: Some("task_456".to_string()),
        timestamp: Some(1234567890),
    };
    assert_eq!(info.agent_id, "agent_123");
    assert_eq!(info.agent_index, Some(1));
    assert_eq!(info.task_id, Some("task_456".to_string()));
    assert_eq!(info.timestamp, Some(1234567890));
}

#[test]
fn test_agent_info_debug() {
    let info = AgentInfo {
        agent_id: "test".to_string(),
        agent_index: None,
        task_id: None,
        timestamp: None,
    };
    let debug_str = format!("{:?}", info);
    assert!(debug_str.contains("AgentInfo"));
}

#[test]
fn test_tool_call_confirmation_with_urls() {
    let confirmation = ToolCallConfirmation {
        confirmation_type: "fetch".to_string(),
        description: Some("Fetch URLs".to_string()),
        command: None,
        root_command: None,
        server_name: None,
        tool_name: None,
        tool_display_name: None,
        urls: Some(vec![
            "https://example.com".to_string(),
            "https://test.com".to_string(),
        ]),
    };
    assert_eq!(confirmation.urls.as_ref().unwrap().len(), 2);
}

#[test]
fn test_tool_call_content_diff_type() {
    let content = ToolCallContent {
        content_type: "diff".to_string(),
        markdown: None,
        path: Some("/file.rs".to_string()),
        old_text: Some("old".to_string()),
        new_text: Some("new".to_string()),
    };
    assert_eq!(content.content_type, "diff");
    assert_eq!(content.path, Some("/file.rs".to_string()));
}
