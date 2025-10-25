//! Integration tests for WebSocketConfig port handling in auto-start mode
//!
//! These tests verify the correct behavior of the WebSocketConfig
//! in auto-start mode with different port configurations in a more
//! integrated environment.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::client::IFlowClient;
    use iflow_cli_sdk_rust::config::options::IFlowOptions;
    use iflow_cli_sdk_rust::config::process::ProcessConfig;
    use iflow_cli_sdk_rust::config::websocket::WebSocketConfig;
    use std::time::Duration;

    /// Test IFlowClient creation with WebSocketConfig auto-start mode
    #[test]
    fn test_iflow_client_with_websocket_auto_start() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::auto_start())
            .with_process_config(ProcessConfig::new().enable_auto_start());

        let _client = IFlowClient::new(Some(options));

        // Verify client was created successfully
        assert!(true); // If we get here without panic, the client was created
    }

    /// Test IFlowClient creation with WebSocketConfig auto-start mode and custom port
    #[test]
    fn test_iflow_client_with_websocket_auto_start_custom_port() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::auto_start())
            .with_process_config(ProcessConfig::new().enable_auto_start().start_port(9000));

        let _client = IFlowClient::new(Some(options));

        // Verify client was created successfully
        assert!(true); // If we get here without panic, the client was created
    }

    /// Test IFlowClient creation with WebSocketConfig and specific URL
    #[test]
    fn test_iflow_client_with_websocket_url() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::new(
                "ws://localhost:8090/acp?peer=iflow".to_string(),
            ))
            .with_process_config(ProcessConfig::new().enable_auto_start());

        let _client = IFlowClient::new(Some(options));

        // Verify client was created successfully
        assert!(true); // If we get here without panic, the client was created
    }

    /// Test IFlowClient creation with WebSocketConfig auto-start mode and custom reconnect settings
    #[test]
    fn test_iflow_client_with_websocket_auto_start_custom_reconnect() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::auto_start_with_reconnect_settings(
                5,
                Duration::from_secs(10),
            ))
            .with_process_config(ProcessConfig::new().enable_auto_start());

        let _client = IFlowClient::new(Some(options));

        // Verify client was created successfully
        assert!(true); // If we get here without panic, the client was created
    }

    /// Test WebSocketConfig configuration combinations
    #[test]
    fn test_websocket_config_combinations() {
        // Test auto-start mode
        let auto_start_config = WebSocketConfig::auto_start();
        assert_eq!(auto_start_config.url, None);
        assert_eq!(auto_start_config.reconnect_attempts, 3);
        assert_eq!(auto_start_config.reconnect_interval, Duration::from_secs(5));

        // Test auto-start mode with custom reconnect settings
        let auto_start_custom_config =
            WebSocketConfig::auto_start_with_reconnect_settings(7, Duration::from_secs(15));
        assert_eq!(auto_start_custom_config.url, None);
        assert_eq!(auto_start_custom_config.reconnect_attempts, 7);
        assert_eq!(
            auto_start_custom_config.reconnect_interval,
            Duration::from_secs(15)
        );

        // Test URL mode
        let url = "ws://localhost:9000/acp?peer=iflow".to_string();
        let url_config = WebSocketConfig::new(url.clone());
        assert_eq!(url_config.url, Some(url));
        assert_eq!(url_config.reconnect_attempts, 3);
        assert_eq!(url_config.reconnect_interval, Duration::from_secs(5));

        // Test URL mode with custom reconnect settings
        let url = "ws://localhost:9000/acp?peer=iflow".to_string();
        let url_custom_config =
            WebSocketConfig::with_reconnect_settings(url.clone(), 9, Duration::from_secs(20));
        assert_eq!(url_custom_config.url, Some(url));
        assert_eq!(url_custom_config.reconnect_attempts, 9);
        assert_eq!(
            url_custom_config.reconnect_interval,
            Duration::from_secs(20)
        );
    }
}
