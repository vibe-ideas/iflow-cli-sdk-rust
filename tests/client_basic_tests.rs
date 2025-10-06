//! Tests for client creation and basic operations
//!
//! These tests focus on client initialization and configuration.

use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions};
use std::path::PathBuf;

#[tokio::test]
async fn test_client_creation_with_default_stdio() {
    let client = IFlowClient::new(None);
    // Just verify client can be created
    drop(client);
}

#[tokio::test]
async fn test_client_creation_with_custom_options() {
    let options = IFlowOptions::new()
        .with_timeout(60.0)
        .with_cwd(PathBuf::from("/tmp"));

    let client = IFlowClient::new(Some(options));
    drop(client);
}

#[tokio::test]
async fn test_client_creation_with_auto_start() {
    let options = IFlowOptions::new().with_auto_start(true);

    let client = IFlowClient::new(Some(options));
    drop(client);
}

#[tokio::test]
async fn test_client_creation_with_manual_start() {
    let options = IFlowOptions::new().with_auto_start(false);

    let client = IFlowClient::new(Some(options));
    drop(client);
}

#[tokio::test]
async fn test_client_creation_with_websocket() {
    use iflow_cli_sdk_rust::types::WebSocketConfig;

    let ws_config = WebSocketConfig::new("ws://localhost:8080/acp?peer=iflow".to_string());
    let options = IFlowOptions::new().with_websocket_config(ws_config);

    let client = IFlowClient::new(Some(options));
    drop(client);
}

#[tokio::test]
async fn test_client_creation_with_logging() {
    use iflow_cli_sdk_rust::types::LoggingConfig;
    use iflow_cli_sdk_rust::LoggerConfig;

    let logging_config = LoggingConfig {
        enabled: true,
        level: "DEBUG".to_string(),
        logger_config: LoggerConfig::default(),
    };

    let options = IFlowOptions::new().with_logging_config(logging_config);

    let client = IFlowClient::new(Some(options));
    drop(client);
}

#[tokio::test]
async fn test_client_messages_stream_creation() {
    let client = IFlowClient::new(None);
    let _stream = client.messages();
    // Stream can be created even when not connected
}

#[tokio::test]
async fn test_multiple_message_streams() {
    let client = IFlowClient::new(None);
    let _stream1 = client.messages();
    let _stream2 = client.messages();
    // Multiple streams can be created
}

#[tokio::test]
async fn test_client_with_different_permission_modes() {
    use iflow_cli_sdk_rust::types::PermissionMode;

    for mode in [
        PermissionMode::Auto,
        PermissionMode::Manual,
        PermissionMode::Selective,
    ] {
        let options = IFlowOptions::new().with_permission_mode(mode);
        let client = IFlowClient::new(Some(options));
        drop(client);
    }
}

#[tokio::test]
async fn test_client_with_various_timeouts() {
    for timeout in [1.0, 30.0, 60.0, 120.0, 300.0] {
        let options = IFlowOptions::new().with_timeout(timeout);
        let client = IFlowClient::new(Some(options));
        drop(client);
    }
}

#[tokio::test]
async fn test_client_with_process_config_variants() {
    use iflow_cli_sdk_rust::types::ProcessConfig;

    // Test manual start
    let config1 = ProcessConfig::new().manual_start();
    let options1 = IFlowOptions::new().with_process_config(config1);
    let client1 = IFlowClient::new(Some(options1));
    drop(client1);

    // Test auto start with port
    let config2 = ProcessConfig::new().enable_auto_start().start_port(9000);
    let options2 = IFlowOptions::new().with_process_config(config2);
    let client2 = IFlowClient::new(Some(options2));
    drop(client2);

    // Test stdio mode
    let config3 = ProcessConfig::new().stdio_mode();
    let options3 = IFlowOptions::new().with_process_config(config3);
    let client3 = IFlowClient::new(Some(options3));
    drop(client3);
}
