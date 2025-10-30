//! Additional exception path tests for the IFlowClient
//!
//! These tests focus on testing error conditions and edge cases
//! in the IFlowClient implementation, particularly for message sending
//! and interruption functionality.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::{
        client::IFlowClient,
        error::IFlowError,
        types::{IFlowOptions, ProcessConfig, WebSocketConfig},
    };
    use std::path::Path;

    /// Test sending message with empty text
    #[tokio::test]
    async fn test_send_message_with_empty_text() {
        let mut client = IFlowClient::new(None);
        let result = client.send_message("", None).await;

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

    /// Test sending message with very long text
    #[tokio::test]
    async fn test_send_message_with_very_long_text() {
        let mut client = IFlowClient::new(None);
        let long_text = "a".repeat(10000); // 10K characters
        let result = client.send_message(&long_text, None).await;

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

    /// Test sending message with files (not implemented)
    #[tokio::test]
    async fn test_send_message_with_files() {
        let mut client = IFlowClient::new(None);
        let files = vec![Path::new("test.txt")];
        let result = client
            .send_message(
                "test message",
                Some(files.iter().map(|p| p.as_ref()).collect()),
            )
            .await;

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

    /// Test interrupt functionality without connecting
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

    /// Test receiving message stream without connecting
    #[tokio::test]
    async fn test_receive_message_stream_without_connecting() {
        let client = IFlowClient::new(None);
        let stream = client.messages();

        // We can't easily test the stream without connecting, but at least
        // we verify the method exists and doesn't panic
        assert!(true);

        // Prevent unused variable warning
        let _ = stream;
    }

    /// Test client creation with various option combinations
    #[test]
    fn test_client_creation_with_various_options() {
        // Test with stdio mode
        let options1 = IFlowOptions::new()
            .with_auto_start(true)
            .with_process_config(ProcessConfig::new().enable_auto_start().stdio_mode());
        let _client1 = IFlowClient::new(Some(options1));

        // Test with websocket mode
        let options2 = IFlowOptions::new()
            .with_auto_start(true)
            .with_websocket_config(WebSocketConfig::new(
                "ws://localhost:8090/acp?peer=iflow".to_string(),
            ));
        let _client2 = IFlowClient::new(Some(options2));

        // Test with manual start
        let options3 = IFlowOptions::new()
            .with_auto_start(false)
            .with_process_config(ProcessConfig::new().manual_start());
        let _client3 = IFlowClient::new(Some(options3));

        // All should be created successfully
        assert!(true);
    }

    /// Test client creation with extreme timeout values
    #[test]
    fn test_client_creation_with_extreme_timeouts() {
        // Test with very small timeout
        let options1 = IFlowOptions::new().with_timeout(0.001);
        let _client1 = IFlowClient::new(Some(options1));

        // Test with very large timeout
        let options2 = IFlowOptions::new().with_timeout(86400.0); // 24 hours
        let _client2 = IFlowClient::new(Some(options2));

        // Test with zero timeout
        let options3 = IFlowOptions::new().with_timeout(0.0);
        let _client3 = IFlowClient::new(Some(options3));

        // All should be created successfully
        assert!(true);
    }

    /// Test double disconnect is idempotent
    #[tokio::test]
    async fn test_double_disconnect_idempotent() {
        let mut client = IFlowClient::new(None);

        // First disconnect
        let result1 = client.disconnect().await;
        assert!(result1.is_ok());

        // Second disconnect
        let result2 = client.disconnect().await;
        assert!(result2.is_ok());

        // Third disconnect
        let result3 = client.disconnect().await;
        assert!(result3.is_ok());
    }

    /// Test client drop behavior with connection attempts
    #[tokio::test]
    async fn test_client_drop_with_connection_attempts() {
        // Use LocalSet for spawn_local compatibility
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                // Create client in a scope to ensure it gets dropped
                {
                    let mut client = IFlowClient::new(None);

                    // Try multiple connection attempts (all may fail, but that's OK)
                    let _ = client.connect().await;
                    let _ = client.connect().await;
                    let _ = client.disconnect().await;

                    // Client will be dropped here
                }

                // If we get here without panicking, the drop worked correctly
                assert!(true);
            })
            .await;
    }

    /// Test client behavior with chained operations
    #[tokio::test]
    async fn test_client_chained_operations() {
        let mut client = IFlowClient::new(None);

        // Chain multiple operations without connecting
        let result1 = client.send_message("test1", None).await;
        let result2 = client.interrupt().await;
        let result3 = client.disconnect().await;
        let result4 = client.disconnect().await; // Double disconnect

        // First two should fail with NotConnected
        match result1 {
            Err(IFlowError::NotConnected) => assert!(true),
            _ => panic!("Expected NotConnected error for send_message"),
        }

        match result2 {
            Err(IFlowError::NotConnected) => assert!(true),
            _ => panic!("Expected NotConnected error for interrupt"),
        }

        // Disconnects should succeed
        assert!(result3.is_ok());
        assert!(result4.is_ok());
    }
}
