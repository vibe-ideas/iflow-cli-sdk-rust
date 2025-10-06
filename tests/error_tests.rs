//! Comprehensive tests for error module
//!
//! These tests improve coverage of the error types and conversions.

use iflow_cli_sdk_rust::IFlowError;

#[test]
fn test_error_variants() {
    // Test Connection error
    let err = IFlowError::Connection("test connection error".to_string());
    assert_eq!(err.to_string(), "Connection error: test connection error");

    // Test Protocol error
    let err = IFlowError::Protocol("test protocol error".to_string());
    assert_eq!(err.to_string(), "Protocol error: test protocol error");

    // Test Authentication error
    let err = IFlowError::Authentication("test auth error".to_string());
    assert_eq!(err.to_string(), "Authentication error: test auth error");

    // Test Timeout error
    let err = IFlowError::Timeout("test timeout error".to_string());
    assert_eq!(err.to_string(), "Timeout error: test timeout error");

    // Test ToolCall error
    let err = IFlowError::ToolCall("test tool call error".to_string());
    assert_eq!(err.to_string(), "Tool call error: test tool call error");

    // Test Validation error
    let err = IFlowError::Validation("test validation error".to_string());
    assert_eq!(
        err.to_string(),
        "Validation error: test validation error"
    );

    // Test Transport error
    let err = IFlowError::Transport("test transport error".to_string());
    assert_eq!(err.to_string(), "Transport error: test transport error");

    // Test WebSocket error (deprecated)
    let err = IFlowError::WebSocket("test websocket error".to_string());
    assert_eq!(err.to_string(), "WebSocket error: test websocket error");

    // Test ProcessManager error
    let err = IFlowError::ProcessManager("test process manager error".to_string());
    assert_eq!(
        err.to_string(),
        "Process manager error: test process manager error"
    );

    // Test NotConnected error
    let err = IFlowError::NotConnected;
    assert_eq!(err.to_string(), "Not connected");

    // Test SessionNotFound error
    let err = IFlowError::SessionNotFound;
    assert_eq!(err.to_string(), "Session not found");

    // Test InvalidMessage error
    let err = IFlowError::InvalidMessage("test invalid message".to_string());
    assert_eq!(
        err.to_string(),
        "Invalid message format: test invalid message"
    );

    // Test Unknown error
    let err = IFlowError::Unknown("test unknown error".to_string());
    assert_eq!(err.to_string(), "Unknown error: test unknown error");
}

#[test]
fn test_json_parse_error_conversion() {
    // Test JsonParse error conversion from serde_json::Error
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json");
    assert!(json_err.is_err());

    let iflow_err: IFlowError = json_err.unwrap_err().into();
    assert!(iflow_err.to_string().contains("JSON parsing error"));
}

#[test]
fn test_io_error_conversion() {
    // Test IO error conversion from std::io::Error
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let iflow_err: IFlowError = io_err.into();
    assert!(iflow_err.to_string().contains("IO error"));
}

#[test]
fn test_error_debug_format() {
    // Test Debug format for errors
    let err = IFlowError::Connection("debug test".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("Connection"));
    assert!(debug_str.contains("debug test"));
}

#[test]
fn test_result_type_alias() {
    // Test Result type alias usage
    fn example_function() -> iflow_cli_sdk_rust::Result<String> {
        Ok("success".to_string())
    }

    let result = example_function();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_error_propagation() {
    // Test error propagation with the ? operator
    fn inner_function() -> iflow_cli_sdk_rust::Result<()> {
        Err(IFlowError::Validation("test error".to_string()))
    }

    fn outer_function() -> iflow_cli_sdk_rust::Result<String> {
        inner_function()?;
        Ok("success".to_string())
    }

    let result = outer_function();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "Validation error: test error");
    }
}
