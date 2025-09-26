//! Basic tests for iFlow SDK

use iflow_cli_sdk_rust::{IFlowOptions, IFlowClient};

#[test]
fn test_iflow_options_default() {
    let options = IFlowOptions::default();
    assert_eq!(options.timeout, 30.0);
    assert!(options.auto_start_process);
}

#[test]
fn test_iflow_options_builder() {
    let options = IFlowOptions::new()
        .with_timeout(60.0)
        .with_file_access(true);
    
    assert_eq!(options.timeout, 60.0);
    assert!(options.file_access);
}

#[test]
fn test_iflow_options_sandbox() {
    let options = IFlowOptions::for_sandbox("wss://sandbox.example.com/acp");
    
    assert!(options.auto_start_process); // Should inherit from default
}

#[tokio::test]
async fn test_client_creation() {
    let _client = IFlowClient::new(None);
    // Client should be created successfully
    // We can't test connection without a running iFlow instance
}

#[tokio::test]
async fn test_client_with_options() {
    let options = IFlowOptions::new()
        .with_timeout(45.0)
        .with_file_access(true);
    
    let _client = IFlowClient::new(Some(options));
    // Client should be created successfully
}