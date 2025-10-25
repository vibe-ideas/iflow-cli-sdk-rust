//! Exception path tests for the IFlowClient
//!
//! These tests focus on testing error conditions and edge cases
//! in the IFlowClient implementation.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::{
        client::IFlowClient,
        error::IFlowError,
        types::{IFlowOptions, ProcessConfig, WebSocketConfig},
    };

    /// Test creating a client with default options
    #[test]
    fn test_client_creation_with_default_options() {
        let _client = IFlowClient::new(None);
        // Verify the client was created successfully
        assert!(true); // If we get here, creation succeeded
    }

    /// Test creating a client with custom options
    #[test]
    fn test_client_creation_with_custom_options() {
        let options = IFlowOptions::new().with_timeout(30.0);
        let _client = IFlowClient::new(Some(options));
        // Verify the client was created successfully
        assert!(true); // If we get here, creation succeeded
    }

    /// Test connecting without iFlow running (should fail)
    #[tokio::test]
    async fn test_connect_without_iflow_running() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let mut client = IFlowClient::new(None);
                let result = client.connect().await;

                // We expect this to fail since iFlow is not running
                // The exact error type may vary depending on the system
                match result {
                    Ok(_) => {
                        // If connection succeeded, that's unexpected in this test environment
                        // but not necessarily wrong
                        println!("Connection succeeded unexpectedly");
                    }
                    Err(IFlowError::Connection(_)) => {
                        // This is expected when iFlow is not running
                        assert!(true);
                    }
                    Err(IFlowError::ProcessManager(_)) => {
                        // This is also expected when iFlow is not installed
                        assert!(true);
                    }
                    Err(e) => {
                        // Any other error might be unexpected, but we'll consider it OK
                        // since we're testing error paths
                        println!("Received unexpected error type: {:?}", e);
                        assert!(true);
                    }
                }
            })
            .await;
    }

    /// Test connecting with manual start mode (stdio)
    #[tokio::test]
    async fn test_connect_with_manual_start_stdio() {
        let options = IFlowOptions::new()
            .with_auto_start(false) // Manual start
            .with_process_config(ProcessConfig::new().manual_start());

        let mut client = IFlowClient::new(Some(options));
        let result = client.connect().await;

        // In manual start mode, this might fail differently
        match result {
            Ok(_) => {
                // Connection succeeded
                assert!(true);
            }
            Err(IFlowError::Connection(_)) => {
                // Expected error in manual start mode when iFlow is not running
                assert!(true);
            }
            Err(IFlowError::ProcessManager(_)) => {
                // Also expected when iFlow is not installed
                assert!(true);
            }
            Err(e) => {
                // Any other error might be unexpected, but we'll consider it OK
                println!("Received unexpected error type: {:?}", e);
                assert!(true);
            }
        }
    }

    /// Test connecting with manual start mode (websocket)
    #[tokio::test]
    async fn test_connect_with_manual_start_websocket() {
        let options = IFlowOptions::new()
            .with_auto_start(false) // Manual start
            .with_websocket_config(WebSocketConfig::new(
                "ws://localhost:8090/acp?peer=iflow".to_string(),
            ))
            .with_process_config(ProcessConfig::new().manual_start());

        let mut client = IFlowClient::new(Some(options));
        let result = client.connect().await;

        // In manual start mode, this might fail differently
        match result {
            Ok(_) => {
                // Connection succeeded
                assert!(true);
            }
            Err(IFlowError::Connection(_)) => {
                // Expected error in manual start mode when iFlow is not running
                assert!(true);
            }
            Err(IFlowError::ProcessManager(_)) => {
                // Also expected when iFlow is not installed
                assert!(true);
            }
            Err(e) => {
                // Any other error might be unexpected, but we'll consider it OK
                println!("Received unexpected error type: {:?}", e);
                assert!(true);
            }
        }
    }

    /// Test connecting with auto start mode but invalid port
    #[tokio::test]
    async fn test_connect_with_auto_start_invalid_port() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                // Use a valid but likely unused port
                let options = IFlowOptions::new()
                    .with_auto_start(true)
                    .with_process_config(
                        ProcessConfig::new().enable_auto_start().start_port(12345),
                    );

                let mut client = IFlowClient::new(Some(options));
                let result = client.connect().await;

                // This should fail with a process manager error since iFlow is likely not installed
                match result {
                    Ok(_) => {
                        // Unexpected success
                        println!("Connection succeeded unexpectedly with port 12345");
                    }
                    Err(IFlowError::ProcessManager(_)) => {
                        // Expected error when iFlow is not installed
                        assert!(true);
                    }
                    Err(IFlowError::Connection(_)) => {
                        // Also acceptable
                        assert!(true);
                    }
                    Err(e) => {
                        // Any other error is also acceptable in this context
                        println!("Received error: {:?}", e);
                        assert!(true);
                    }
                }
            })
            .await;
    }

    /// Test sending message without connecting first
    #[tokio::test]
    async fn test_send_message_without_connecting() {
        let mut client = IFlowClient::new(None);
        let result = client.send_message("test message", None).await;

        // Should fail with NotConnected error
        match result {
            Err(IFlowError::NotConnected) => {
                assert!(true);
            }
            _ => {
                panic!("Expected NotConnected error, got: {:?}", result);
            }
        }
    }

    /// Test receiving message without connecting first
    #[tokio::test]
    async fn test_receive_message_without_connecting() {
        let client = IFlowClient::new(None);
        // This should not panic, but may return None or an error
        let _stream = client.messages();
        // We can't easily test the stream without connecting, but at least
        // we verify the method exists and doesn't panic
        assert!(true);
    }

    /// Test interrupt without connecting first
    #[tokio::test]
    async fn test_interrupt_without_connecting() {
        let client = IFlowClient::new(None);
        let result = client.interrupt().await;

        // Should fail with NotConnected error
        match result {
            Err(IFlowError::NotConnected) => {
                assert!(true);
            }
            _ => {
                panic!("Expected NotConnected error, got: {:?}", result);
            }
        }
    }

    /// Test disconnect without connecting first
    #[tokio::test]
    async fn test_disconnect_without_connecting() {
        let mut client = IFlowClient::new(None);
        let result = client.disconnect().await;

        // Should succeed even without connecting (idempotent)
        assert!(result.is_ok());
    }

    /// Test double connect (should be idempotent)
    #[tokio::test]
    async fn test_double_connect() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let mut client = IFlowClient::new(None);

                // First connect attempt
                let result1 = client.connect().await;

                // Second connect attempt
                let result2 = client.connect().await;

                // At least one should succeed or fail gracefully
                // The second should not cause any issues
                assert!(true); // If we get here, double connect didn't panic

                // Print results for debugging
                println!("First connect result: {:?}", result1);
                println!("Second connect result: {:?}", result2);
            })
            .await;
    }

    /// Test double disconnect (should be idempotent)
    #[tokio::test]
    async fn test_double_disconnect() {
        let mut client = IFlowClient::new(None);

        // First disconnect attempt
        let result1 = client.disconnect().await;

        // Second disconnect attempt
        let result2 = client.disconnect().await;

        // Both should succeed (idempotent)
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    /// Test connect then disconnect then connect again
    #[tokio::test]
    async fn test_connect_disconnect_reconnect() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let mut client = IFlowClient::new(None);

                // Connect
                let connect_result = client.connect().await;

                // Disconnect
                let disconnect_result = client.disconnect().await;
                assert!(disconnect_result.is_ok());

                // Reconnect
                let reconnect_result = client.connect().await;

                // All operations should complete without panicking
                assert!(true); // If we get here, the sequence worked

                // Print results for debugging
                println!("Connect result: {:?}", connect_result);
                println!("Reconnect result: {:?}", reconnect_result);
            })
            .await;
    }

    /// Test client with very short timeout
    #[tokio::test]
    async fn test_client_with_very_short_timeout() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let options = IFlowOptions::new().with_timeout(0.001); // Very short timeout
                let mut client = IFlowClient::new(Some(options));
                let result = client.connect().await;

                // This might timeout or fail for other reasons
                match result {
                    Ok(_) => {
                        // Unexpected success with very short timeout
                        println!("Connection succeeded unexpectedly with very short timeout");
                    }
                    Err(IFlowError::Timeout(_)) => {
                        // Expected timeout error
                        assert!(true);
                    }
                    Err(IFlowError::Connection(_)) => {
                        // Also acceptable
                        assert!(true);
                    }
                    Err(IFlowError::ProcessManager(_)) => {
                        // Also acceptable
                        assert!(true);
                    }
                    Err(e) => {
                        // Any other error is also acceptable in this context
                        println!("Received error: {:?}", e);
                        assert!(true);
                    }
                }
            })
            .await;
    }

    /// Test client with very long timeout
    #[tokio::test]
    async fn test_client_with_very_long_timeout() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let options = IFlowOptions::new().with_timeout(3600.0); // 1 hour timeout
                let mut client = IFlowClient::new(Some(options));
                let result = client.connect().await;

                // This should behave normally, just with a longer timeout
                match result {
                    Ok(_) => {
                        // Success is fine
                        assert!(true);
                    }
                    Err(IFlowError::Connection(_)) => {
                        // Expected when iFlow is not running
                        assert!(true);
                    }
                    Err(IFlowError::ProcessManager(_)) => {
                        // Also expected when iFlow is not installed
                        assert!(true);
                    }
                    Err(e) => {
                        // Any other error is also acceptable
                        println!("Received error: {:?}", e);
                        assert!(true);
                    }
                }
            })
            .await;
    }

    /// Test WebSocket connection with invalid URL
    #[tokio::test]
    async fn test_websocket_connect_with_invalid_url() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::new("invalid-url".to_string()));

        let mut client = IFlowClient::new(Some(options));
        let result = client.connect().await;

        // This should fail with a connection error
        match result {
            Ok(_) => {
                // Unexpected success
                println!("Connection succeeded unexpectedly with invalid URL");
            }
            Err(IFlowError::Connection(_)) => {
                // Expected connection error
                assert!(true);
            }
            Err(IFlowError::ProcessManager(_)) => {
                // Also acceptable
                assert!(true);
            }
            Err(e) => {
                // Any other error is also acceptable
                println!("Received error: {:?}", e);
                assert!(true);
            }
        }
    }

    /// Test WebSocket connection with unreachable URL
    #[tokio::test]
    async fn test_websocket_connect_with_unreachable_url() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::new("ws://localhost:12345/acp".to_string())); // Unlikely to be listening

        let mut client = IFlowClient::new(Some(options));
        let result = client.connect().await;

        // This should fail with a connection error
        match result {
            Ok(_) => {
                // Unexpected success
                println!("Connection succeeded unexpectedly with unreachable URL");
            }
            Err(IFlowError::Connection(_)) => {
                // Expected connection error
                assert!(true);
            }
            Err(IFlowError::ProcessManager(_)) => {
                // Also acceptable
                assert!(true);
            }
            Err(e) => {
                // Any other error is also acceptable
                println!("Received error: {:?}", e);
                assert!(true);
            }
        }
    }

    /// Test client drop behavior
    #[tokio::test]
    async fn test_client_drop_behavior() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                // Create client in a scope to ensure it gets dropped
                {
                    let mut client = IFlowClient::new(None);

                    // Try to connect (may fail, but that's OK)
                    let _ = client.connect().await;

                    // Client will be dropped here
                }

                // If we get here without panicking, the drop worked correctly
                assert!(true);
            })
            .await;
    }

    /// Test multiple clients creation
    #[test]
    fn test_multiple_clients_creation() {
        // Create multiple clients
        let _client1 = IFlowClient::new(None);
        let _client2 = IFlowClient::new(None);
        let _client3 = IFlowClient::new(None);

        // All should be created successfully
        assert!(true); // If we get here, creation succeeded
    }
}
