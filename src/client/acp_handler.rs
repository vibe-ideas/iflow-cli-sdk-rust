//! ACP Client Handler for iFlow SDK
//!
//! This module provides the implementation of the agent_client_protocol::Client trait
//! for handling ACP messages in the stdio connection.

use agent_client_protocol::{
    Client, ContentBlock, SessionNotification, SessionUpdate, RequestPermissionOutcome, RequestPermissionRequest,
    RequestPermissionResponse, WriteTextFileRequest, WriteTextFileResponse, ReadTextFileRequest, ReadTextFileResponse,
    CreateTerminalRequest, CreateTerminalResponse, TerminalOutputRequest, TerminalOutputResponse,
    ReleaseTerminalRequest, ReleaseTerminalResponse, WaitForTerminalExitRequest, WaitForTerminalExitResponse,
    KillTerminalCommandRequest, KillTerminalCommandResponse, Error
};
use anyhow::Result as AnyhowResult;
use tokio::sync::mpsc::UnboundedSender;
use crate::logger::MessageLogger;
use crate::message::types::{Message, PlanEntry, PlanPriority, PlanStatus};

/// Handler for ACP messages when using stdio connection
pub struct IFlowClientHandler {
    message_sender: UnboundedSender<Message>,
    logger: Option<MessageLogger>,
}

impl IFlowClientHandler {
    /// Create a new IFlowClientHandler
    pub fn new(message_sender: UnboundedSender<Message>, logger: Option<MessageLogger>) -> Self {
        Self {
            message_sender,
            logger,
        }
    }
}

#[async_trait::async_trait(?Send)]
impl Client for IFlowClientHandler {
    async fn request_permission(
        &self,
        _args: RequestPermissionRequest,
    ) -> AnyhowResult<RequestPermissionResponse, Error> {
        // For now, cancel all permissions
        Ok(RequestPermissionResponse {
            outcome: RequestPermissionOutcome::Cancelled,
            meta: None,
        })
    }

    async fn write_text_file(
        &self,
        _args: WriteTextFileRequest,
    ) -> AnyhowResult<WriteTextFileResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn read_text_file(
        &self,
        _args: ReadTextFileRequest,
    ) -> AnyhowResult<ReadTextFileResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn create_terminal(
        &self,
        _args: CreateTerminalRequest,
    ) -> AnyhowResult<CreateTerminalResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn terminal_output(
        &self,
        _args: TerminalOutputRequest,
    ) -> AnyhowResult<TerminalOutputResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn release_terminal(
        &self,
        _args: ReleaseTerminalRequest,
    ) -> AnyhowResult<ReleaseTerminalResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn wait_for_terminal_exit(
        &self,
        _args: WaitForTerminalExitRequest,
    ) -> AnyhowResult<WaitForTerminalExitResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn kill_terminal_command(
        &self,
        _args: KillTerminalCommandRequest,
    ) -> AnyhowResult<KillTerminalCommandResponse, Error> {
        Err(Error::method_not_found())
    }

    async fn session_notification(
        &self,
        args: SessionNotification,
    ) -> AnyhowResult<(), Error> {
        match args.update {
            SessionUpdate::AgentMessageChunk(content) => {
                let text = match content.content {
                    ContentBlock::Text(text_content) => text_content.text,
                    ContentBlock::Image(_) => "<image>".into(),
                    ContentBlock::Audio(_) => "<audio>".into(),
                    ContentBlock::ResourceLink(resource_link) => resource_link.uri,
                    ContentBlock::Resource(_) => "<resource>".into(),
                };
                let msg = Message::Assistant { content: text };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::UserMessageChunk(content) => {
                let text = match content.content {
                    ContentBlock::Text(text_content) => text_content.text,
                    ContentBlock::Image(_) => "<image>".into(),
                    ContentBlock::Audio(_) => "<audio>".into(),
                    ContentBlock::ResourceLink(resource_link) => resource_link.uri,
                    ContentBlock::Resource(_) => "<resource>".into(),
                };
                let msg = Message::User { content: text };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::ToolCall(tool_call) => {
                let msg = Message::ToolCall {
                    id: tool_call.id.0.to_string(),
                    name: tool_call.title.clone(),
                    status: format!("{:?}", tool_call.status),
                };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::Plan(plan) => {
                let entries = plan
                    .entries
                    .into_iter()
                    .map(|entry| {
                        // Convert agent-client-protocol PlanEntry to our PlanEntry
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
                    })
                    .collect();

                let msg = Message::Plan { entries };
                let _ = self.message_sender.send(msg.clone());

                // Log the message if logger is available
                if let Some(logger) = &self.logger {
                    let _ = logger.log_message(&msg).await;
                }
            }
            SessionUpdate::AgentThoughtChunk { .. }
            | SessionUpdate::ToolCallUpdate(_)
            | SessionUpdate::CurrentModeUpdate { .. }
            | SessionUpdate::AvailableCommandsUpdate { .. } => {
                // Ignore these for now
            }
        }

        Ok(())
    }
}