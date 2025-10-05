//! Unit tests for WebSocketConfig port handling logic
//!
//! These tests verify the correct behavior of the WebSocketConfig
//! in auto-start mode with different port configurations.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::types::{IFlowOptions, ProcessConfig, WebSocketConfig};
    use std::time::Duration;

    /// Test WebSocketConfig auto-start with default settings
    #[test]
    fn test_websocket_config_auto_start_default() {
        let config = WebSocketConfig::auto_start();

        // Verify URL is None for auto-start mode
        assert_eq!(config.url, None);

        // Verify default reconnect settings
        assert_eq!(config.reconnect_attempts, 3);
        assert_eq!(config.reconnect_interval, Duration::from_secs(5));
    }

    /// Test WebSocketConfig auto-start with custom reconnect settings
    #[test]
    fn test_websocket_config_auto_start_custom_reconnect() {
        let config =
            WebSocketConfig::auto_start_with_reconnect_settings(5, Duration::from_secs(10));

        // Verify URL is None for auto-start mode
        assert_eq!(config.url, None);

        // Verify custom reconnect settings
        assert_eq!(config.reconnect_attempts, 5);
        assert_eq!(config.reconnect_interval, Duration::from_secs(10));
    }

    /// Test WebSocketConfig with specific URL
    #[test]
    fn test_websocket_config_with_url() {
        let url = "ws://localhost:8090/acp?peer=iflow".to_string();
        let config = WebSocketConfig::new(url.clone());

        // Verify URL is set
        assert_eq!(config.url, Some(url));

        // Verify default reconnect settings
        assert_eq!(config.reconnect_attempts, 3);
        assert_eq!(config.reconnect_interval, Duration::from_secs(5));
    }

    /// Test WebSocketConfig with custom reconnect settings and URL
    #[test]
    fn test_websocket_config_with_url_and_custom_reconnect() {
        let url = "ws://localhost:8090/acp?peer=iflow".to_string();
        let config =
            WebSocketConfig::with_reconnect_settings(url.clone(), 7, Duration::from_secs(15));

        // Verify URL is set
        assert_eq!(config.url, Some(url));

        // Verify custom reconnect settings
        assert_eq!(config.reconnect_attempts, 7);
        assert_eq!(config.reconnect_interval, Duration::from_secs(15));
    }

    /// Test WebSocketConfig default implementation
    #[test]
    fn test_websocket_config_default() {
        let config = WebSocketConfig::default();

        // Verify default URL is set
        assert_eq!(
            config.url,
            Some("ws://localhost:8090/acp?peer=iflow".to_string())
        );

        // Verify default reconnect settings
        assert_eq!(config.reconnect_attempts, 3);
        assert_eq!(config.reconnect_interval, Duration::from_secs(5));
    }

    /// Test IFlowOptions with WebSocketConfig auto-start
    #[test]
    fn test_iflow_options_with_websocket_auto_start() {
        let options = IFlowOptions::new().with_websocket_config(WebSocketConfig::auto_start());

        // Verify WebSocket config is set
        assert!(options.websocket.is_some());

        let websocket_config = options.websocket.unwrap();
        assert_eq!(websocket_config.url, None);
        assert_eq!(websocket_config.reconnect_attempts, 3);
        assert_eq!(websocket_config.reconnect_interval, Duration::from_secs(5));
    }

    /// Test IFlowOptions with WebSocketConfig auto-start and custom port
    #[test]
    fn test_iflow_options_with_websocket_auto_start_and_custom_port() {
        let options = IFlowOptions::new()
            .with_websocket_config(WebSocketConfig::auto_start())
            .with_process_config(ProcessConfig::new().start_port(9000));

        // Verify WebSocket config is set
        assert!(options.websocket.is_some());
        let websocket_config = options.websocket.unwrap();
        assert_eq!(websocket_config.url, None);
        assert_eq!(websocket_config.reconnect_attempts, 3);
        assert_eq!(websocket_config.reconnect_interval, Duration::from_secs(5));

        // Verify process config is set with custom port
        assert_eq!(options.process.start_port, Some(9000));
    }

    /// Test IFlowOptions with WebSocketConfig and specific URL
    #[test]
    fn test_iflow_options_with_websocket_url() {
        let url = "ws://localhost:8090/acp?peer=iflow".to_string();
        let options = IFlowOptions::new().with_websocket_config(WebSocketConfig::new(url.clone()));

        // Verify WebSocket config is set
        assert!(options.websocket.is_some());

        let websocket_config = options.websocket.unwrap();
        assert_eq!(websocket_config.url, Some(url));
        assert_eq!(websocket_config.reconnect_attempts, 3);
        assert_eq!(websocket_config.reconnect_interval, Duration::from_secs(5));
    }
}
