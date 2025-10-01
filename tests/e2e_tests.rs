//! E2E tests for iFlow CLI SDK examples

use std::process::Command;
use std::fs;

/// Test the basic_client example
#[test]
fn test_basic_client() {
    run_example_test("basic_client");
}

/// Test the explore_api example
#[test]
fn test_explore_api() {
    run_example_test("explore_api");
}

/// Test the logging_example example
#[test]
fn test_logging_example() {
    // Clean up previous log files
    let _ = fs::remove_file("logs/iflow_messages.log");
    let _ = fs::remove_dir_all("logs");
    
    run_example_test("logging_example");
    
    // Check if log file was created
    assert!(fs::metadata("logs/iflow_messages.log").is_ok(), "Log file should be created");
}

/// Test the permission_modes example
#[test]
fn test_permission_modes() {
    run_example_test("permission_modes");
}

/// Test the query example
#[test]
fn test_query() {
    run_example_test("query");
}

/// Test the test_both_modes example
#[test]
fn test_test_both_modes() {
    run_example_test("test_both_modes");
}

/// Test the test_realtime example
#[test]
fn test_test_realtime() {
    run_example_test("test_realtime");
}

/// Test the test_response example
#[test]
fn test_test_response() {
    run_example_test("test_response");
}

/// Test the test_stream example
#[test]
fn test_test_stream() {
    run_example_test("test_stream");
}

/// Test the websocket_client example
#[test]
fn test_websocket_client() {
    run_example_test("websocket_client");
}

/// Helper function to run an example and check its execution
fn run_example_test(example_name: &str) {
    let output = Command::new("cargo")
        .args(&["run", "--example", example_name])
        .output()
        .expect("Failed to execute example");

    // Print stdout and stderr for debugging
    println!("Example: {}", example_name);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Check if the process finished successfully (exit code 0)
    // Note: Some examples might fail due to missing iFlow CLI, which is expected in E2E tests
    // We're primarily checking that the examples don't panic or crash
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Allow "Connection failed" messages as they're expected when iFlow CLI is not installed
        if !stderr.contains("Connection failed") {
            panic!("Example {} failed with exit code: {:?}", example_name, output.status.code());
        }
    }
}