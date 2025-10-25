//! Notification handling for ACP protocol in iFlow SDK
//!
//! This module handles incoming notifications and client method calls from the server.

use crate::error::{IFlowError, Result};
use crate::message::types::{PlanEntry, PlanPriority, PlanStatus};
use crate::protocol::core::ACPProtocol;
use agent_client_protocol::{RequestPermissionOutcome, RequestPermissionResponse};
use serde_json::{Value, json};
use std::time::Duration;
use tokio::time::timeout;

impl ACPProtocol {
    /// Wait for a response to a specific request
    ///
    /// # Arguments
    /// * `request_id` - The ID of the request to wait for
    ///
    /// # Returns
    /// * `Ok(Value)` containing the response
    /// * `Err(IFlowError)` if waiting failed
    pub async fn wait_for_response(&mut self, request_id: u32) -> Result<Value> {
        let timeout_duration = Duration::from_secs_f64(self.timeout_secs);
        let start_time = std::time::Instant::now();

        loop {
            if start_time.elapsed() > timeout_duration {
                return Err(IFlowError::Timeout(format!(
                    "Timeout waiting for response to request {}",
                    request_id
                )));
            }

            // Use a shorter timeout for receiving messages to allow for periodic checks
            let receive_timeout = Duration::from_secs_f64(self.timeout_secs.min(1.0));
            let msg = match timeout(receive_timeout, self.transport.receive()).await {
                Ok(Ok(msg)) => msg,
                Ok(Err(e)) => {
                    tracing::error!("Transport error while waiting for response: {}", e);
                    return Err(e);
                }
                Err(_) => {
                    // Timeout is expected, continue waiting
                    tracing::debug!(
                        "No message received, continuing to wait for response to request {}...",
                        request_id
                    );
                    continue;
                }
            };

            // Skip control messages
            if msg.starts_with("//") {
                tracing::debug!("Control message: {}", msg);
                continue;
            }

            // Try to parse as JSON
            let data: Value = match serde_json::from_str(&msg) {
                Ok(data) => data,
                Err(e) => {
                    tracing::debug!("Failed to parse message as JSON: {}, message: {}", e, msg);
                    continue;
                }
            };

            // Check if this is the response we're waiting for
            if let Some(id) = data.get("id").and_then(|v| v.as_u64())
                && id == request_id as u64 {
                return Ok(data);
            }

            // If not our response, process as a notification
            if let Err(e) = self.handle_notification(data).await {
                tracing::warn!("Failed to handle notification: {}", e);
                // Don't fail the entire wait, just log and continue
            }
        }
    }

    /// Wait for a response to a specific request while handling notifications
    ///
    /// # Arguments
    /// * `request_id` - The ID of the request to wait for
    ///
    /// # Returns
    /// * `Ok(Value)` containing the response
    /// * `Err(IFlowError)` if waiting failed
    pub async fn wait_for_response_with_notifications(&mut self, request_id: u32) -> Result<Value> {
        let timeout_duration = Duration::from_secs_f64(self.timeout_secs);
        let start_time = std::time::Instant::now();

        loop {
            if start_time.elapsed() > timeout_duration {
                return Err(IFlowError::Timeout(format!(
                    "Timeout waiting for response to request {}",
                    request_id
                )));
            }

            // Use a shorter timeout for receiving messages to allow for periodic checks
            let receive_timeout = Duration::from_secs_f64(self.timeout_secs.min(1.0));
            let msg = match timeout(receive_timeout, self.transport.receive()).await {
                Ok(Ok(msg)) => msg,
                Ok(Err(e)) => {
                    tracing::error!("Transport error while waiting for response: {}", e);
                    return Err(e);
                }
                Err(_) => {
                    // Timeout is expected, continue waiting
                    tracing::debug!(
                        "No message received, continuing to wait for response to request {}...",
                        request_id
                    );
                    continue;
                }
            };

            // Skip control messages
            if msg.starts_with("//") {
                tracing::debug!("Control message: {}", msg);
                continue;
            }

            // Try to parse as JSON
            let data: Value = match serde_json::from_str(&msg) {
                Ok(data) => data,
                Err(e) => {
                    tracing::debug!("Failed to parse message as JSON: {}, message: {}", e, msg);
                    continue;
                }
            };

            // Check if this is the response we're waiting for
            if let Some(id) = data.get("id").and_then(|v| v.as_u64()) {
                // Handle permission requests that come with an ID
                if let Some(method) = data.get("method").and_then(|v| v.as_str())
                    && method == "session/request_permission" {
                    tracing::debug!("Handling session/request_permission with ID: {}", id);
                    // Process the permission request immediately
                    if let Err(e) = self.handle_client_method(method, data.clone()).await {
                        tracing::warn!("Failed to handle permission request: {}", e);
                    }
                    // Continue waiting for the main response
                    continue;
                }

                // If this is the response we're waiting for, return it
                if id == request_id as u64 {
                    return Ok(data);
                }
            }

            // If not our response, process as a notification
            if let Err(e) = self.handle_notification(data).await {
                tracing::warn!("Failed to handle notification: {}", e);
                // Don't fail the entire wait, just log and continue
            }
        }
    }

    /// Handle incoming notifications from the server
    ///
    /// # Arguments
    /// * `data` - The JSON data of the notification
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_notification(&mut self, data: Value) -> Result<()> {
        // Handle method calls from server (client interface)
        if let Some(method) = data.get("method").and_then(|v| v.as_str())
            && data.get("result").is_none() && data.get("error").is_none() {
            self.handle_client_method(method, data.clone()).await?;
        }

        Ok(())
    }

    /// Handle client method calls from the server
    ///
    /// # Arguments
    /// * `method` - The method name
    /// * `data` - The JSON data of the method call
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_client_method(&mut self, method: &str, data: Value) -> Result<()> {
        let params = data.get("params").cloned().unwrap_or(Value::Null);
        let request_id = data.get("id").and_then(|v| v.as_u64());

        match method {
            "session/update" => {
                if let Some(update_obj) = params.get("update").and_then(|v| v.as_object())
                    && let Some(session_update) =
                        update_obj.get("sessionUpdate").and_then(|v| v.as_str())
                {
                    self.handle_session_update(session_update, update_obj, request_id)
                        .await?;
                }
            }
            "session/request_permission" => {
                // Handle permission request from CLI
                tracing::debug!("Handling session/request_permission");
                self.handle_permission_request(params, request_id).await?;
            }
            _ => {
                tracing::warn!("Unknown method: {}", method);
                // Send error response for unknown methods
                if let Some(id) = request_id {
                    let error_response = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32601,
                            "message": "Method not found"
                        }
                    });
                    self.transport.send(&error_response).await?;
                }
            }
        }

        Ok(())
    }

    /// Handle permission request from the CLI
    ///
    /// # Arguments
    /// * `params` - The parameters of the permission request
    /// * `request_id` - Optional request ID for responses
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_permission_request(
        &mut self,
        params: Value,
        request_id: Option<u64>,
    ) -> Result<()> {
        // Extract tool call information from params
        let tool_call = params.get("toolCall").unwrap_or(&Value::Null);
        let options = params.get("options").unwrap_or(&Value::Null);
        let _session_id = params.get("sessionId").and_then(|v| v.as_str());

        // Log the tool call information
        let tool_title = tool_call
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let tool_type = tool_call
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        tracing::debug!(
            "Permission request for tool '{}' (type: {})",
            tool_title,
            tool_type
        );

        // Determine response based on permission_mode
        let auto_approve = match self.permission_mode {
            crate::message::types::PermissionMode::Auto => {
                // Auto-approve all tool calls
                true
            }
            crate::message::types::PermissionMode::Manual => {
                // Require manual confirmation for all
                false
            }
            crate::message::types::PermissionMode::Selective => {
                // Auto-approve based on tool type
                // For now, we'll auto-approve read/fetch operations
                tool_type == "read" || tool_type == "fetch" || tool_type == "list"
            }
        };

        let permission_response = if auto_approve {
            // Find the appropriate option from the provided options
            let mut selected_option = "proceed_once".to_string();
            if let Some(options_array) = options.as_array() {
                for option in options_array {
                    if let Some(option_id) = option.get("optionId").and_then(|v| v.as_str()) {
                        if option_id == "proceed_once" {
                            selected_option = option_id.to_string();
                            break;
                        } else if option_id == "proceed_always" {
                            selected_option = option_id.to_string();
                        }
                    }
                }
                // Fallback to first option's optionId if no specific option found
                if selected_option == "proceed_once" && !options_array.is_empty()
                    && let Some(first_option_id) = options_array[0].get("optionId").and_then(|v| v.as_str()) {
                    selected_option = first_option_id.to_string();
                }
            }
            RequestPermissionResponse {
                outcome: RequestPermissionOutcome::Selected {
                    option_id: agent_client_protocol::PermissionOptionId(std::sync::Arc::from(selected_option)),
                },
                meta: None,
            }
        } else {
            RequestPermissionResponse {
                outcome: RequestPermissionOutcome::Cancelled,
                meta: None,
            }
        };

        // Send response if request ID is provided
        if let Some(id) = request_id {
            let response_message = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": permission_response
            });
            self.transport.send(&response_message).await?;
        }

        let outcome_str = match &permission_response.outcome {
            RequestPermissionOutcome::Cancelled => "cancelled",
            RequestPermissionOutcome::Selected { option_id } => &*option_id.0,
        };
        tracing::debug!("Permission request for tool '{}': {}", tool_title, outcome_str);

        Ok(())
    }

    /// Handle session update notifications
    ///
    /// # Arguments
    /// * `update_type` - The type of update
    /// * `update` - The update data
    /// * `request_id` - Optional request ID for responses
    ///
    /// # Returns
    /// * `Ok(())` if handling was successful
    /// * `Err(IFlowError)` if handling failed
    async fn handle_session_update(
        &mut self,
        update_type: &str,
        update: &serde_json::Map<String, Value>,
        request_id: Option<u64>,
    ) -> Result<()> {
        match update_type {
            "agent_message_chunk" => {
                if let Some(content) = update.get("content") {
                    let text = match content {
                        Value::Object(obj) => {
                            if let Some(text_content) = obj.get("text").and_then(|v| v.as_str()) {
                                text_content.to_string()
                            } else {
                                "<unknown>".to_string()
                            }
                        }
                        _ => "<unknown>".to_string(),
                    };

                    let msg = crate::message::types::Message::Assistant { content: text };
                    let _ = self.message_sender.send(msg);
                }
            }
            "user_message_chunk" => {
                if let Some(content) = update.get("content") {
                    let text = match content {
                        Value::Object(obj) => {
                            if let Some(text_content) = obj.get("text").and_then(|v| v.as_str()) {
                                text_content.to_string()
                            } else {
                                "<unknown>".to_string()
                            }
                        }
                        _ => "<unknown>".to_string(),
                    };

                    let msg = crate::message::types::Message::User { content: text };
                    let _ = self.message_sender.send(msg);
                }
            }
            "tool_call" => {
                if let Some(tool_call) = update.get("toolCall") {
                    let id = tool_call
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let name = tool_call
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let status = tool_call
                        .get("status")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let msg = crate::message::types::Message::ToolCall { id, name, status };
                    let _ = self.message_sender.send(msg);
                }
            }
            "plan" => {
                if let Some(entries) = update.get("entries").and_then(|v| v.as_array()) {
                    let entries: Vec<PlanEntry> = entries
                        .iter()
                        .filter_map(|entry| {
                            let content =
                                entry.get("content").and_then(|v| v.as_str())?.to_string();
                            let priority_str = entry
                                .get("priority")
                                .and_then(|v| v.as_str())
                                .unwrap_or("medium");
                            let status_str = entry
                                .get("status")
                                .and_then(|v| v.as_str())
                                .unwrap_or("pending");

                            let priority = match priority_str {
                                "high" => PlanPriority::High,
                                "medium" => PlanPriority::Medium,
                                "low" => PlanPriority::Low,
                                _ => PlanPriority::Medium,
                            };

                            let status = match status_str {
                                "pending" => PlanStatus::Pending,
                                "in_progress" => PlanStatus::InProgress,
                                "completed" => PlanStatus::Completed,
                                _ => PlanStatus::Pending,
                            };

                            Some(PlanEntry {
                                content,
                                priority,
                                status,
                            })
                        })
                        .collect();

                    let msg = crate::message::types::Message::Plan { entries };
                    let _ = self.message_sender.send(msg);
                }
            }
            "tool_call_update" => {
                // For now, we'll just acknowledge the update if there's a request ID
                if let Some(id) = request_id {
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": null
                    });
                    self.transport.send(&response).await?;
                }
            }
            "agent_thought_chunk" | "current_mode_update" | "available_commands_update" => {
                // Ignore these for now
            }
            _ => {
                tracing::debug!("Unhandled session update type: {}", update_type);
            }
        }

        // Send acknowledgment for notifications that require it
        if let Some(id) = request_id {
            match update_type {
                "tool_call_update" | "notifyTaskFinish" => {
                    let response = json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": null
                    });
                    self.transport.send(&response).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}