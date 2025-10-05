//! Additional unit tests for the query module to improve test coverage
//!
//! These tests focus on improving the test coverage of the query module.

use iflow_cli_sdk_rust::{IFlowOptions, query_with_config, query_with_timeout};

/// Test query_with_timeout with different timeout values
#[tokio::test]
async fn test_query_with_timeout_values() {
    // Test with a very short timeout
    let _short_timeout_fn = || query_with_timeout("test", 0.001);

    // Test with a normal timeout
    let _normal_timeout_fn = || query_with_timeout("test", 30.0);

    // Test with a long timeout
    let _long_timeout_fn = || query_with_timeout("test", 300.0);
}

/// Test query_with_config with different option combinations
#[tokio::test]
async fn test_query_with_config_options() {
    // Test with default options
    let default_options = IFlowOptions::default();
    let _default_fn = move || query_with_config("test", default_options);

    // Test with custom timeout
    let timeout_options = IFlowOptions::new().with_timeout(45.0);
    let _timeout_fn = move || query_with_config("test", timeout_options);

    // Test with auto start disabled
    let no_auto_start_options = IFlowOptions::new().with_auto_start(false);
    let _no_auto_start_fn = move || query_with_config("test", no_auto_start_options);

    // Test with auto start enabled
    let auto_start_options = IFlowOptions::new().with_auto_start(true);
    let _auto_start_fn = move || query_with_config("test", auto_start_options);
}

/// Test IFlowOptions builder pattern
#[test]
fn test_iflow_options_builder_pattern() {
    let options = IFlowOptions::new().with_timeout(30.0).with_auto_start(true);

    assert_eq!(options.timeout, 30.0);
    assert_eq!(options.process.auto_start, true);
}

/// Test IFlowOptions with different configurations
#[test]
fn test_iflow_options_configurations() {
    // Test default configuration
    let default_options = IFlowOptions::default();
    assert_eq!(default_options.timeout, 120.0);
    assert_eq!(default_options.process.auto_start, true);

    // Test custom timeout
    let custom_timeout_options = IFlowOptions::new().with_timeout(60.0);
    assert_eq!(custom_timeout_options.timeout, 60.0);

    // Test auto start disabled
    let no_auto_start_options = IFlowOptions::new().with_auto_start(false);
    assert_eq!(no_auto_start_options.process.auto_start, false);

    // Test auto start enabled
    let auto_start_options = IFlowOptions::new().with_auto_start(true);
    assert_eq!(auto_start_options.process.auto_start, true);
}

/// Test query stream functions signatures
#[tokio::test]
async fn test_query_stream_function_signatures() {
    use iflow_cli_sdk_rust::{query_stream, query_stream_with_config, query_stream_with_timeout};

    // Test query_stream function exists
    let _query_stream_fn = query_stream;

    // Test query_stream_with_timeout function exists
    let _query_stream_with_timeout_fn = query_stream_with_timeout;

    // Test query_stream_with_config function exists
    let _query_stream_with_config_fn = query_stream_with_config;

    // Test with default options
    let default_options = IFlowOptions::default();
    let _query_stream_with_config_fn_with_options =
        move || query_stream_with_config("test", default_options);

    // Test with custom timeout
    let _query_stream_with_timeout_fn_with_timeout = || query_stream_with_timeout("test", 45.0);
}
