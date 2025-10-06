//! Unit tests for the error module
//!
//! These tests verify the error types and their Display implementations

use iflow_cli_sdk_rust::IFlowError;
use std::io;

#[test]
fn test_connection_error() {
    let err = IFlowError::Connection("Failed to connect".to_string());
    assert_eq!(format!("{}", err), "Connection error: Failed to connect");
}

#[test]
fn test_protocol_error() {
    let err = IFlowError::Protocol("Invalid protocol version".to_string());
    assert_eq!(format!("{}", err), "Protocol error: Invalid protocol version");
}

#[test]
fn test_authentication_error() {
    let err = IFlowError::Authentication("Auth failed".to_string());
    assert_eq!(format!("{}", err), "Authentication error: Auth failed");
}

#[test]
fn test_timeout_error() {
    let err = IFlowError::Timeout("Operation timed out".to_string());
    assert_eq!(format!("{}", err), "Timeout error: Operation timed out");
}

#[test]
fn test_tool_call_error() {
    let err = IFlowError::ToolCall("Tool execution failed".to_string());
    assert_eq!(format!("{}", err), "Tool call error: Tool execution failed");
}

#[test]
fn test_validation_error() {
    let err = IFlowError::Validation("Invalid input".to_string());
    assert_eq!(format!("{}", err), "Validation error: Invalid input");
}

#[test]
fn test_transport_error() {
    let err = IFlowError::Transport("Transport layer failed".to_string());
    assert_eq!(format!("{}", err), "Transport error: Transport layer failed");
}

#[test]
fn test_websocket_error() {
    let err = IFlowError::WebSocket("WebSocket connection failed".to_string());
    assert_eq!(format!("{}", err), "WebSocket error: WebSocket connection failed");
}

#[test]
fn test_process_manager_error() {
    let err = IFlowError::ProcessManager("Failed to start process".to_string());
    assert_eq!(format!("{}", err), "Process manager error: Failed to start process");
}

#[test]
fn test_not_connected_error() {
    let err = IFlowError::NotConnected;
    assert_eq!(format!("{}", err), "Not connected");
}

#[test]
fn test_session_not_found_error() {
    let err = IFlowError::SessionNotFound;
    assert_eq!(format!("{}", err), "Session not found");
}

#[test]
fn test_invalid_message_error() {
    let err = IFlowError::InvalidMessage("Malformed JSON".to_string());
    assert_eq!(format!("{}", err), "Invalid message format: Malformed JSON");
}

#[test]
fn test_unknown_error() {
    let err = IFlowError::Unknown("Something went wrong".to_string());
    assert_eq!(format!("{}", err), "Unknown error: Something went wrong");
}

#[test]
fn test_json_parse_error() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let err = IFlowError::JsonParse(json_err);
    assert!(format!("{}", err).contains("JSON parsing error"));
}

#[test]
fn test_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let err = IFlowError::Io(io_err);
    assert_eq!(format!("{}", err), "IO error: File not found");
}

#[test]
fn test_error_debug_format() {
    let err = IFlowError::Connection("test".to_string());
    assert!(format!("{:?}", err).contains("Connection"));
}

#[test]
fn test_error_from_json_error() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
    let err: IFlowError = json_err.into();
    assert!(matches!(err, IFlowError::JsonParse(_)));
}

#[test]
fn test_error_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
    let err: IFlowError = io_err.into();
    assert!(matches!(err, IFlowError::Io(_)));
}
