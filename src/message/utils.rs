//! Utilities for processing and converting messages between different formats

use crate::message::types::{Message, PlanEntry, PlanPriority, PlanStatus};
use agent_client_protocol::{ContentBlock, SessionUpdate};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

/// Convert content block to text
///
/// # Arguments
/// * `content` - The content block to convert
///
/// # Returns
/// The text representation of the content block
pub fn convert_content_block_to_text(content: ContentBlock) -> String {
    match content {
        ContentBlock::Text(text_content) => text_content.text,
        ContentBlock::Image(_) => "<image>".into(),
        ContentBlock::Audio(_) => "<audio>".into(),
        ContentBlock::ResourceLink(resource_link) => resource_link.uri,
        ContentBlock::Resource(_) => "<resource>".into(),
    }
}

/// Create a message from a content block with proper type classification
///
/// # Arguments
/// * `content` - The content block to convert
/// * `is_user` - Whether the content is from a user
///
/// # Returns
/// A Message variant based on the content block
pub fn create_message_from_content(content: ContentBlock, is_user: bool) -> Message {
    let text = convert_content_block_to_text(content);
    if is_user {
        Message::User { content: text }
    } else {
        Message::Assistant { content: text }
    }
}

/// Convert agent-client-protocol PlanEntry to our PlanEntry
///
/// # Arguments
/// * `entry` - The agent-client-protocol PlanEntry to convert
///
/// # Returns
/// Our PlanEntry representation
pub fn convert_plan_entry(entry: agent_client_protocol::PlanEntry) -> PlanEntry {
    PlanEntry {
        content: entry.content,
        priority: match entry.priority {
            agent_client_protocol::PlanEntryPriority::High => PlanPriority::High,
            agent_client_protocol::PlanEntryPriority::Medium => PlanPriority::Medium,
            agent_client_protocol::PlanEntryPriority::Low => PlanPriority::Low,
        },
        status: match entry.status {
            agent_client_protocol::PlanEntryStatus::Pending => PlanStatus::Pending,
            agent_client_protocol::PlanEntryStatus::InProgress => PlanStatus::InProgress,
            agent_client_protocol::PlanEntryStatus::Completed => PlanStatus::Completed,
        },
    }
}

/// Process session updates from the agent-client-protocol
///
/// # Arguments
/// * `update` - The session update to process
/// * `message_sender` - The sender to use for sending messages
///
/// # Returns
/// Result indicating success or failure
pub fn process_session_update(
    update: SessionUpdate,
    message_sender: &UnboundedSender<Message>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = match update {
        SessionUpdate::AgentMessageChunk { content } => {
            create_message_from_content(content, false)
        }
        SessionUpdate::UserMessageChunk { content } => {
            create_message_from_content(content, true)
        }
        SessionUpdate::ToolCall(tool_call) => Message::ToolCall {
            id: tool_call.id.0.to_string(),
            name: tool_call.title.clone(),
            status: format!("{:?}", tool_call.status),
        },
        SessionUpdate::Plan(plan) => {
            let entries = plan
                .entries
                .into_iter()
                .map(convert_plan_entry)
                .collect();
            Message::Plan { entries }
        }
        SessionUpdate::AgentThoughtChunk { .. }
        | SessionUpdate::ToolCallUpdate(_)
        | SessionUpdate::CurrentModeUpdate { .. }
        | SessionUpdate::AvailableCommandsUpdate { .. } => {
            // These are ignored for now, but we could log them
            return Ok(());
        }
    };

    // Send the message
    let _ = message_sender.send(msg.clone());

    Ok(())
}

/// Builder for creating error messages with a fluent API
pub struct ErrorMessageBuilder {
    code: i32,
    message: String,
    details: Option<HashMap<String, Value>>,
}

impl ErrorMessageBuilder {
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            details: None,
        }
    }

    pub fn with_details(mut self, details: HashMap<String, Value>) -> Self {
        self.details = Some(details);
        self
    }

    pub fn build(self) -> Message {
        Message::Error {
            code: self.code,
            message: self.message,
            details: self.details,
        }
    }
}

/// Utility for parsing and validating incoming messages
pub struct MessageParser;

impl MessageParser {
    /// Parse a JSON string into a Message
    pub fn parse_json_message(json_str: &str) -> Result<Message, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Validate that a message has the required fields
    pub fn validate_message(message: &Message) -> Result<(), String> {
        match message {
            Message::User { content } | Message::Assistant { content } => {
                if content.is_empty() {
                    Err("Message content cannot be empty".to_string())
                } else {
                    Ok(())
                }
            }
            Message::ToolCall { id, name, status } => {
                if id.is_empty() || name.is_empty() || status.is_empty() {
                    Err("ToolCall message missing required fields".to_string())
                } else {
                    Ok(())
                }
            }
            Message::Plan { entries } => {
                // Validate plan entries
                for entry in entries {
                    if entry.content.is_empty() {
                        return Err("Plan entry content cannot be empty".to_string());
                    }
                }
                Ok(())
            }
            Message::TaskFinish { reason: _ } => Ok(()),
            Message::Error {
                code: _,
                message,
                details: _,
            } => {
                if message.is_empty() {
                    Err("Error message cannot be empty".to_string())
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_client_protocol::TextContent;

    #[test]
    fn test_convert_content_block_to_text() {
        let text_content = ContentBlock::Text(TextContent {
            text: "Hello, world!".to_string(),
            annotations: None,
            meta: None,
        });
        assert_eq!(
            convert_content_block_to_text(text_content),
            "Hello, world!"
        );

        let image_content = ContentBlock::Image(agent_client_protocol::ImageContent {
            mime_type: "image/png".to_string(),
            data: "base64data".to_string(),
            annotations: None,
            uri: None,
            meta: None,
        });
        assert_eq!(convert_content_block_to_text(image_content), "<image>");
    }

    #[tokio::test]
    async fn test_create_message_from_content() {
        let text_content = ContentBlock::Text(TextContent {
            text: "Test message".to_string(),
            annotations: None,
            meta: None,
        });

        let user_message = create_message_from_content(text_content.clone(), true);
        match user_message {
            Message::User { content } => assert_eq!(content, "Test message"),
            _ => panic!("Expected User message"),
        }

        let assistant_message = create_message_from_content(text_content, false);
        match assistant_message {
            Message::Assistant { content } => assert_eq!(content, "Test message"),
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_error_message_builder() {
        let error_msg = ErrorMessageBuilder::new(404, "Not found".to_string())
            .with_details(HashMap::new())
            .build();

        match error_msg {
            Message::Error {
                code,
                message,
                details,
            } => {
                assert_eq!(code, 404);
                assert_eq!(message, "Not found");
                assert!(details.is_some());
            }
            _ => panic!("Expected Error message"),
        }
    }
}