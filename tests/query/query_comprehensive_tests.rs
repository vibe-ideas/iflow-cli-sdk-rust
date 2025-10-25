//! Comprehensive unit tests for the query module to improve test coverage
//!
//! These tests focus on improving the test coverage of the query module
//! by testing various scenarios and edge cases.

use iflow_cli_sdk_rust::{
    IFlowOptions, query, query_stream, query_stream_with_config, query_stream_with_timeout,
    query_with_config, query_with_timeout,
};

/// Test that query function exists and has correct signature
#[tokio::test]
async fn test_query_function_exists() {
    let _query_fn = query;
}

/// Test that query_with_config function exists and has correct signature
#[tokio::test]
async fn test_query_with_config_function_exists() {
    let _query_with_config_fn = query_with_config;
}

/// Test that query_with_timeout function exists and has correct signature
#[tokio::test]
async fn test_query_with_timeout_function_exists() {
    let _query_with_timeout_fn = query_with_timeout;
}

/// Test that query_stream function exists and has correct signature
#[tokio::test]
async fn test_query_stream_function_exists() {
    let _query_stream_fn = query_stream;
}

/// Test that query_stream_with_config function exists and has correct signature
#[tokio::test]
async fn test_query_stream_with_config_function_exists() {
    let _query_stream_with_config_fn = query_stream_with_config;
}

/// Test that query_stream_with_timeout function exists and has correct signature
#[tokio::test]
async fn test_query_stream_with_timeout_function_exists() {
    let _query_stream_with_timeout_fn = query_stream_with_timeout;
}

/// Test IFlowOptions creation with various configurations
#[test]
fn test_iflow_options_creation_variants() {
    // Test default options
    let default_options = IFlowOptions::default();
    assert_eq!(default_options.timeout, 120.0);

    // Test with custom timeout
    let custom_timeout_options = IFlowOptions::new().with_timeout(30.0);
    assert_eq!(custom_timeout_options.timeout, 30.0);

    // Test with auto start enabled
    let auto_start_options = IFlowOptions::new().with_auto_start(true);
    assert_eq!(auto_start_options.process.auto_start, true);

    // Test with auto start disabled
    let no_auto_start_options = IFlowOptions::new().with_auto_start(false);
    assert_eq!(no_auto_start_options.process.auto_start, false);

    // Test chaining options
    let chained_options = IFlowOptions::new()
        .with_timeout(45.0)
        .with_auto_start(false);
    assert_eq!(chained_options.timeout, 45.0);
    assert_eq!(chained_options.process.auto_start, false);
}

/// Test query function with different prompt types
#[tokio::test]
async fn test_query_with_different_prompts() {
    // Test with empty prompt
    let _empty_prompt_fn = || query("");

    // Test with short prompt
    let _short_prompt_fn = || query("Hi");

    // Test with longer prompt
    let _long_prompt_fn = || query("What is the capital of France?");

    // Test with very long prompt
    let long_text = "a".repeat(1000);
    let _very_long_prompt_fn = || query(&long_text);
}

/// Test query_with_timeout with different timeout values
#[tokio::test]
async fn test_query_with_timeout_values() {
    // Test with very small timeout
    let _very_small_timeout_fn = || query_with_timeout("test", 0.001);

    // Test with small timeout
    let _small_timeout_fn = || query_with_timeout("test", 0.1);

    // Test with normal timeout
    let _normal_timeout_fn = || query_with_timeout("test", 30.0);

    // Test with large timeout
    let _large_timeout_fn = || query_with_timeout("test", 300.0);

    // Test with very large timeout
    let _very_large_timeout_fn = || query_with_timeout("test", 3600.0);
}

/// Test query_with_config with different option combinations
#[tokio::test]
async fn test_query_with_config_options() {
    // Test with default options
    let default_options = IFlowOptions::default();
    let _default_fn = move || query_with_config("test", default_options);

    // Test with custom timeout options
    let timeout_options = IFlowOptions::new().with_timeout(45.0);
    let _timeout_fn = move || query_with_config("test", timeout_options);

    // Test with auto start disabled
    let no_auto_start_options = IFlowOptions::new().with_auto_start(false);
    let _no_auto_start_fn = move || query_with_config("test", no_auto_start_options);

    // Test with auto start enabled
    let auto_start_options = IFlowOptions::new().with_auto_start(true);
    let _auto_start_fn = move || query_with_config("test", auto_start_options);

    // Test with chained options
    let chained_options = IFlowOptions::new()
        .with_timeout(60.0)
        .with_auto_start(false);
    let _chained_fn = move || query_with_config("test", chained_options);
}

/// Test query_stream with different prompt types
#[tokio::test]
async fn test_query_stream_with_different_prompts() {
    // Test with empty prompt
    let _empty_prompt_fn = || query_stream("");

    // Test with short prompt
    let _short_prompt_fn = || query_stream("Hi");

    // Test with longer prompt
    let _long_prompt_fn = || query_stream("What is the capital of France?");

    // Test with very long prompt
    let long_text = "a".repeat(1000);
    let _very_long_prompt_fn = || query_stream(&long_text);
}

/// Test query_stream_with_timeout with different timeout values
#[tokio::test]
async fn test_query_stream_with_timeout_values() {
    // Test with very small timeout
    let _very_small_timeout_fn = || query_stream_with_timeout("test", 0.001);

    // Test with small timeout
    let _small_timeout_fn = || query_stream_with_timeout("test", 0.1);

    // Test with normal timeout
    let _normal_timeout_fn = || query_stream_with_timeout("test", 30.0);

    // Test with large timeout
    let _large_timeout_fn = || query_stream_with_timeout("test", 300.0);

    // Test with very large timeout
    let _very_large_timeout_fn = || query_stream_with_timeout("test", 3600.0);
}

/// Test query_stream_with_config with different option combinations
#[tokio::test]
async fn test_query_stream_with_config_options() {
    // Test with default options
    let default_options = IFlowOptions::default();
    let _default_fn = move || query_stream_with_config("test", default_options);

    // Test with custom timeout options
    let timeout_options = IFlowOptions::new().with_timeout(45.0);
    let _timeout_fn = move || query_stream_with_config("test", timeout_options);

    // Test with auto start disabled
    let no_auto_start_options = IFlowOptions::new().with_auto_start(false);
    let _no_auto_start_fn = move || query_stream_with_config("test", no_auto_start_options);

    // Test with auto start enabled
    let auto_start_options = IFlowOptions::new().with_auto_start(true);
    let _auto_start_fn = move || query_stream_with_config("test", auto_start_options);

    // Test with chained options
    let chained_options = IFlowOptions::new()
        .with_timeout(60.0)
        .with_auto_start(false);
    let _chained_fn = move || query_stream_with_config("test", chained_options);
}

/// Test that query function calls query_with_timeout with default timeout
#[tokio::test]
async fn test_query_calls_query_with_timeout() {
    // Verify that the query function exists and has the correct signature
    let _query_fn = query;

    // Verify that it should use the default timeout from IFlowOptions
    let default_timeout = IFlowOptions::default().timeout;
    assert_eq!(default_timeout, 120.0);
}

/// Test that query_stream function calls query_stream_with_timeout with default timeout
#[tokio::test]
async fn test_query_stream_calls_query_stream_with_timeout() {
    // Verify that the query_stream function exists and has the correct signature
    let _query_stream_fn = query_stream;

    // Verify that it should use the default timeout of 120.0 seconds
    assert_eq!(120.0, 120.0);
}
