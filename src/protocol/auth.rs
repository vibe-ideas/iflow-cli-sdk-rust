//! Authentication implementation for ACP protocol in iFlow SDK
//!
//! This module handles authentication for the ACP protocol.

use crate::error::{IFlowError, Result};
use crate::protocol::core::ACPProtocol;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use tracing::debug;

impl ACPProtocol {
    /// Perform authentication if required
    ///
    /// This method should be called if initialize() indicates
    /// that authentication is needed (isAuthenticated = False).
    ///
    /// # Arguments
    /// * `method_id` - Authentication method ID
    /// * `method_info` - Optional authentication info
    ///
    /// # Returns
    /// * `Ok(())` if authentication was successful
    /// * `Err(IFlowError)` if authentication failed
    pub async fn authenticate(
        &mut self,
        method_id: &str,
        method_info: Option<HashMap<String, String>>,
    ) -> Result<()> {
        if self.authenticated {
            debug!("Already authenticated");
            return Ok(());
        }

        let request_id = self.next_request_id();
        let mut params = json!({
            "methodId": method_id,
        });

        if let Some(info) = method_info {
            params["methodInfo"] = json!(info);
        }

        let request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "authenticate",
            "params": params,
        });

        self.transport.send(&request).await?;
        debug!("Sent authenticate request with method: {}", method_id);

        // Wait for authentication response with timeout
        let response_timeout = Duration::from_secs_f64(self.timeout_secs);
        let response = timeout(response_timeout, self.wait_for_response(request_id))
            .await
            .map_err(|_| {
                IFlowError::Timeout("Timeout waiting for authentication response".to_string())
            })?
            .map_err(|e| IFlowError::Protocol(format!("Failed to authenticate: {}", e)))?;

        if let Some(result) = response.get("result") {
            if let Some(response_method) = result.get("methodId").and_then(|v| v.as_str()) {
                if response_method == method_id {
                    self.authenticated = true;
                    debug!("Authentication successful with method: {}", response_method);
                } else {
                    tracing::warn!(
                        "Unexpected methodId in response: {} (expected {})",
                        response_method,
                        method_id
                    );
                    // Still mark as authenticated if we got a response
                    self.authenticated = true;
                }
            } else {
                self.authenticated = true;
            }
        } else if let Some(error) = response.get("error") {
            return Err(IFlowError::Authentication(format!(
                "Authentication failed: {:?}",
                error
            )));
        } else {
            return Err(IFlowError::Protocol(
                "Invalid authenticate response".to_string(),
            ));
        }

        Ok(())
    }
}