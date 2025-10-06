//! Comprehensive integration-style tests for various error scenarios
//!
//! These tests exercise error handling across multiple modules

use iflow_cli_sdk_rust::{IFlowError, IFlowClient, IFlowOptions};

#[tokio::test]
async fn test_client_new_with_none_options() {
    let client = IFlowClient::new(None);
    // Should create successfully with default options
    drop(client);
}

#[tokio::test]
async fn test_client_new_with_some_options() {
    let options = IFlowOptions::new().with_timeout(60.0);
    let client = IFlowClient::new(Some(options));
    drop(client);
}

#[tokio::test]
async fn test_client_new_with_various_options() {
    let options1 = IFlowOptions::new().with_timeout(30.0).with_auto_start(false);
    let client1 = IFlowClient::new(Some(options1));
    
    let options2 = IFlowOptions::new().with_timeout(120.0).with_auto_start(true);
    let client2 = IFlowClient::new(Some(options2));
    
    drop(client1);
    drop(client2);
}

#[test]
fn test_error_display_messages() {
    let errors = vec![
        IFlowError::Connection("test".to_string()),
        IFlowError::Protocol("test".to_string()),
        IFlowError::Authentication("test".to_string()),
        IFlowError::Timeout("test".to_string()),
        IFlowError::ToolCall("test".to_string()),
        IFlowError::Validation("test".to_string()),
        IFlowError::Transport("test".to_string()),
        IFlowError::WebSocket("test".to_string()),
        IFlowError::ProcessManager("test".to_string()),
        IFlowError::NotConnected,
        IFlowError::SessionNotFound,
        IFlowError::InvalidMessage("test".to_string()),
        IFlowError::Unknown("test".to_string()),
    ];
    
    for error in errors {
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_error_from_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
    let flow_err: IFlowError = io_err.into();
    assert!(matches!(flow_err, IFlowError::Io(_)));
}

#[test]
fn test_error_from_json_error() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
    let flow_err: IFlowError = json_err.into();
    assert!(matches!(flow_err, IFlowError::JsonParse(_)));
}

#[tokio::test]
async fn test_client_messages_stream_without_connection() {
    let client = IFlowClient::new(None);
    let mut stream = client.messages();
    
    use futures::stream::StreamExt;
    // Try to get a message from stream (should be empty or error)
    let _ = stream.next().await;
}

#[tokio::test]
async fn test_client_receive_message_without_connection() {
    let client = IFlowClient::new(None);
    let result = client.receive_message().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_client_interrupt_without_connection() {
    let client = IFlowClient::new(None);
    let result = client.interrupt().await;
    assert!(result.is_err());
}

#[test]
fn test_result_type_alias() {
    fn returns_result() -> iflow_cli_sdk_rust::Result<i32> {
        Ok(42)
    }
    
    let result = returns_result();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_type_alias_with_error() {
    fn returns_error() -> iflow_cli_sdk_rust::Result<i32> {
        Err(IFlowError::Unknown("test".to_string()))
    }
    
    let result = returns_error();
    assert!(result.is_err());
}

#[test]
fn test_error_source_trait() {
    use std::error::Error;
    
    let err = IFlowError::Connection("test".to_string());
    let _source = err.source(); // Should not panic
}

#[test]
fn test_error_debug_all_variants() {
    let errors = vec![
        IFlowError::Connection("c".to_string()),
        IFlowError::Protocol("p".to_string()),
        IFlowError::Authentication("a".to_string()),
        IFlowError::Timeout("t".to_string()),
        IFlowError::ToolCall("tc".to_string()),
        IFlowError::Validation("v".to_string()),
        IFlowError::Transport("tr".to_string()),
        IFlowError::WebSocket("w".to_string()),
        IFlowError::ProcessManager("pm".to_string()),
        IFlowError::NotConnected,
        IFlowError::SessionNotFound,
        IFlowError::InvalidMessage("i".to_string()),
        IFlowError::Unknown("u".to_string()),
    ];
    
    for error in errors {
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
    }
}

#[tokio::test]
async fn test_multiple_client_instances() {
    let client1 = IFlowClient::new(None);
    let client2 = IFlowClient::new(None);
    let client3 = IFlowClient::new(None);
    
    drop(client1);
    drop(client2);
    drop(client3);
}

#[test]
fn test_error_variants_equality() {
    let err1 = IFlowError::NotConnected;
    let err2 = IFlowError::NotConnected;
    
    // Both should display the same message
    assert_eq!(format!("{}", err1), format!("{}", err2));
}
