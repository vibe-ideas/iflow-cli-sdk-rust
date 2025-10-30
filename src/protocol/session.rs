//! Session management for ACP protocol in iFlow SDK
//!
//! This module handles session creation and management for the ACP protocol.

use crate::error::{IFlowError, Result};
use crate::protocol::core::ACPProtocol;
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;
use tracing::debug;

impl ACPProtocol {
    /// Create a new session
    ///
    /// # Arguments
    /// * `cwd` - Working directory for the session
    /// * `mcp_servers` - MCP servers to connect to
    ///
    /// # Returns
    /// * `Ok(String)` containing the session ID
    /// * `Err(IFlowError)` if session creation failed
    pub async fn create_session(
        &mut self,
        cwd: &str,
        mcp_servers: Vec<serde_json::Value>,
    ) -> Result<String> {
        if !self.initialized {
            return Err(IFlowError::Protocol(
                "Protocol not initialized. Call initialize() first.".to_string(),
            ));
        }

        if !self.authenticated {
            return Err(IFlowError::Protocol(
                "Not authenticated. Call authenticate() first.".to_string(),
            ));
        }

        let request_id = self.next_request_id();
        let params = json!({
            "cwd": cwd,
            "mcpServers": mcp_servers,
        });

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "session/new",
            "params": params,
        });

        self.transport.send(&request).await?;
        debug!(
            "Sent session/new request with cwd: {} and mcpServers: {:?}",
            cwd, mcp_servers
        );

        // Wait for response with timeout
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response(request_id))
            .await
            .map_err(|_| {
                IFlowError::Timeout("Timeout waiting for session creation response".to_string())
            })?
            .map_err(|e| IFlowError::Protocol(format!("Failed to create session: {}", e)))?;

        if let Some(result) = response.get("result") {
            if let Some(session_id) = result.get("sessionId").and_then(|v| v.as_str()) {
                debug!("Created session: {}", session_id);
                Ok(session_id.to_string())
            } else {
                debug!(
                    "Invalid session/new response, using fallback ID: session_{}",
                    request_id
                );
                Ok(format!("session_{}", request_id))
            }
        } else if let Some(error) = response.get("error") {
            Err(IFlowError::Protocol(format!(
                "session/new failed: {:?}",
                error
            )))
        } else {
            Err(IFlowError::Protocol(
                "Invalid session/new response".to_string(),
            ))
        }
    }

    /// Send a prompt to the session and wait for response
    ///
    /// # Arguments
    /// * `session_id` - The session ID from create_session()
    /// * `prompt` - The prompt text to send
    ///
    /// # Returns
    /// * `Ok(u32)` containing the request ID for tracking the message
    /// * `Err(IFlowError)` if sending failed
    pub async fn send_prompt(&mut self, session_id: &str, prompt: &str) -> Result<u32> {
        if !self.initialized {
            return Err(IFlowError::Protocol(
                "Protocol not initialized. Call initialize() first.".to_string(),
            ));
        }

        if !self.authenticated {
            return Err(IFlowError::Protocol(
                "Not authenticated. Call authenticate() first.".to_string(),
            ));
        }

        let request_id = self.next_request_id();
        // Create prompt as a list of content blocks
        let prompt_blocks = vec![json!({
            "type": "text",
            "text": prompt
        })];

        let params = json!({
            "sessionId": session_id,
            "prompt": prompt_blocks,
        });

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "session/prompt",
            "params": params,
        });

        self.transport.send(&request).await?;
        debug!("Sent session/prompt");

        // Wait for response
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response_with_notifications(request_id))
            .await
            .map_err(|_| IFlowError::Timeout("Timeout waiting for prompt response".to_string()))?
            .map_err(|e| IFlowError::Protocol(format!("Failed to send prompt: {}", e)))?;

        // Check for errors in response
        if let Some(error) = response.get("error") {
            return Err(IFlowError::Protocol(format!("Prompt failed: {:?}", error)));
        }

        // Send task finish message to indicate completion
        let msg = crate::message::types::Message::TaskFinish {
            reason: Some("completed".to_string()),
        };
        let _ = self.message_sender.send(msg);

        Ok(request_id)
    }
}