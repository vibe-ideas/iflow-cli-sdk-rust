//! Unit tests for the query module
//!
//! These tests cover the various query functions in the query module,
//! including query, query_with_config, query_with_timeout, query_stream,
//! query_stream_with_config, and query_stream_with_timeout.

use iflow_cli_sdk_rust::{
    IFlowOptions, query, query_stream, query_stream_with_config, query_stream_with_timeout,
    query_with_config, query_with_timeout,
};

/// Test the basic query function signature
#[tokio::test]
async fn test_query_function_signature() {
    // This test verifies that the function exists and has the right signature
    // In a real test environment, we would mock the IFlowClient or use a test instance
    let _function_exists = query;
}

/// Test the query_with_config function signature
#[tokio::test]
async fn test_query_with_config_function_signature() {
    // This test verifies that the function exists and has the right signature
    let _function_exists = query_with_config;

    // Test with default options
    let options = IFlowOptions::default();
    let _function_with_options = move || query_with_config("test", options);
}

/// Test the query_with_timeout function signature
#[tokio::test]
async fn test_query_with_timeout_function_signature() {
    // This test verifies that the function exists and has the right signature
    let _function_exists = query_with_timeout;

    // Test with a short timeout
    let _function_with_timeout = || query_with_timeout("test", 1.0);
}

/// Test the query_stream function signature
#[tokio::test]
async fn test_query_stream_function_signature() {
    // This test verifies that the function exists and has the right signature
    let _function_exists = query_stream;
}

/// Test the query_stream_with_config function signature
#[tokio::test]
async fn test_query_stream_with_config_function_signature() {
    // This test verifies that the function exists and has the right signature
    let _function_exists = query_stream_with_config;

    // Test with default options
    let options = IFlowOptions::default();
    let _function_with_options = move || query_stream_with_config("test", options);
}

/// Test the query_stream_with_timeout function signature
#[tokio::test]
async fn test_query_stream_with_timeout_function_signature() {
    // This test verifies that the function exists and has the right signature
    let _function_exists = query_stream_with_timeout;

    // Test with a short timeout
    let _function_with_timeout = || query_stream_with_timeout("test", 1.0);
}

/// Test IFlowOptions creation for query functions
#[test]
fn test_iflow_options_creation() {
    let options = IFlowOptions::new();
    assert_eq!(options.timeout, 120.0); // Default timeout

    let options_with_timeout = IFlowOptions::new().with_timeout(60.0);
    assert_eq!(options_with_timeout.timeout, 60.0);

    // Test that we can chain options
    let chained_options = IFlowOptions::new().with_timeout(30.0).with_auto_start(true);
    assert_eq!(chained_options.timeout, 30.0);
    assert_eq!(chained_options.process.auto_start, true);
}
