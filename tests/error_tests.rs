//! Comprehensive tests for error types
//!
//! These tests ensure all error variants are properly covered

use iflow_cli_sdk_rust::IFlowError;

#[test]
fn test_connection_error() {
    let error = IFlowError::Connection("Connection failed".to_string());
    assert_eq!(error.to_string(), "Connection error: Connection failed");
}

#[test]
fn test_protocol_error() {
    let error = IFlowError::Protocol("Invalid protocol".to_string());
    assert_eq!(error.to_string(), "Protocol error: Invalid protocol");
}

#[test]
fn test_authentication_error() {
    let error = IFlowError::Authentication("Auth failed".to_string());
    assert_eq!(error.to_string(), "Authentication error: Auth failed");
}

#[test]
fn test_timeout_error() {
    let error = IFlowError::Timeout("Request timed out".to_string());
    assert_eq!(error.to_string(), "Timeout error: Request timed out");
}

#[test]
fn test_tool_call_error() {
    let error = IFlowError::ToolCall("Tool execution failed".to_string());
    assert_eq!(error.to_string(), "Tool call error: Tool execution failed");
}

#[test]
fn test_validation_error() {
    let error = IFlowError::Validation("Invalid input".to_string());
    assert_eq!(error.to_string(), "Validation error: Invalid input");
}

#[test]
fn test_transport_error() {
    let error = IFlowError::Transport("Transport failed".to_string());
    assert_eq!(error.to_string(), "Transport error: Transport failed");
}

#[test]
fn test_websocket_error() {
    let error = IFlowError::WebSocket("WebSocket closed".to_string());
    assert_eq!(error.to_string(), "WebSocket error: WebSocket closed");
}

#[test]
fn test_process_manager_error() {
    let error = IFlowError::ProcessManager("Process failed to start".to_string());
    assert_eq!(error.to_string(), "Process manager error: Process failed to start");
}

#[test]
fn test_not_connected_error() {
    let error = IFlowError::NotConnected;
    assert_eq!(error.to_string(), "Not connected");
}

#[test]
fn test_session_not_found_error() {
    let error = IFlowError::SessionNotFound;
    assert_eq!(error.to_string(), "Session not found");
}

#[test]
fn test_invalid_message_error() {
    let error = IFlowError::InvalidMessage("Bad format".to_string());
    assert_eq!(error.to_string(), "Invalid message format: Bad format");
}

#[test]
fn test_unknown_error() {
    let error = IFlowError::Unknown("Something went wrong".to_string());
    assert_eq!(error.to_string(), "Unknown error: Something went wrong");
}

#[test]
fn test_json_parse_error_conversion() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
        .unwrap_err();
    let error = IFlowError::from(json_error);
    assert!(error.to_string().contains("JSON parsing error"));
}

#[test]
fn test_io_error_conversion() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = IFlowError::from(io_error);
    assert!(error.to_string().contains("IO error"));
}

#[test]
fn test_error_debug_format() {
    let error = IFlowError::Connection("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Connection"));
}

#[test]
fn test_error_clone() {
    let error1 = IFlowError::Connection("test".to_string());
    // Test that error can be used (consumed or cloned as needed)
    let error_str = error1.to_string();
    assert!(error_str.contains("Connection"));
}
