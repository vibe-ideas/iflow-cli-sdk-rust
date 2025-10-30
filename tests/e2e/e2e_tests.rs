//! E2E tests for iFlow CLI SDK examples

use std::fs;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::{Arc, Mutex};
use serial_test::serial;

/// Test the basic_client example
#[test]
#[serial]
fn test_basic_client() {
    run_example_test("basic_client");
}

/// Test the explore_api example
#[test]
#[serial]
fn test_explore_api() {
    run_example_test("explore_api");
}

/// Test the logging_example example
#[test]
#[serial]
fn test_logging_example() {
    // Clean up previous log files
    let _ = fs::remove_file("logs/iflow_messages.log");
    let _ = fs::remove_dir_all("logs");

    run_example_test("logging_example");

    // Check if log file was created
    assert!(
        fs::metadata("logs/iflow_messages.log").is_ok(),
        "Log file should be created"
    );
}

/// Test the permission_modes example
#[test]
#[serial]
fn test_permission_modes() {
    run_example_test("permission_modes");
}

/// Test the query example
#[test]
#[serial]
fn test_query() {
    run_example_test("query");
}

/// Test the query_with_config example
#[test]
#[serial]
fn test_query_with_config() {
    run_example_test("query_with_config");
}

/// Test the test_response example
#[test]
#[serial]
fn test_test_response() {
    run_example_test("test_response");
}

/// Test the mcp_example example
#[test]
#[serial]
fn test_mcp_example() {
    run_example_test("mcp_example");
}

/// Test the websocket_mcp example
#[test]
#[serial]
fn test_websocket_mcp() {
    run_example_test("websocket_mcp");
}

/// Test the websocket_client example
#[test]
#[serial]
fn test_websocket_client() {
    run_example_test("websocket_client");
}

/// Helper function to run an example and check its execution
fn run_example_test(example_name: &str) {
    println!("Running example: {}", example_name);
    
    let mut child = Command::new("cargo")
        .args(&["run", "--example", example_name])
        .stdout(Stdio::piped())  // Pipe stdout to read and print in real-time
        .stderr(Stdio::piped())  // Pipe stderr to read and print in real-time
        .spawn()
        .expect("Failed to execute example");

    // Shared buffers to capture output for error checking
    let stderr_buffer = Arc::new(Mutex::new(String::new()));
    
    // Clone references for the threads
    let stderr_buffer_clone = Arc::clone(&stderr_buffer);

    // Get the stdout and stderr handles
    let stdout = child.stdout.take().expect("Failed to get stdout handle");
    let stderr = child.stderr.take().expect("Failed to get stderr handle");

    // Spawn threads to read and print stdout and stderr in real-time
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(err) => eprintln!("Error reading stdout: {}", err),
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        let mut buffer = stderr_buffer_clone.lock().unwrap();
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    eprintln!("{}", line);
                    buffer.push_str(&line);
                    buffer.push('\n');
                },
                Err(err) => {
                    eprintln!("Error reading stderr: {}", err);
                    buffer.push_str(&format!("Error reading stderr: {}\n", err));
                }
            }
        }
    });

    // Wait for the process to complete
    let status = child.wait().expect("Failed to wait for child process");
    
    // Wait for the output threads to finish
    stdout_thread.join().expect("Failed to join stdout thread");
    stderr_thread.join().expect("Failed to join stderr thread");

    // Get the captured stderr content
    let stderr_content = stderr_buffer.lock().unwrap().clone();

    // Check if the process finished successfully (exit code 0)
    // Note: Some examples might fail due to missing iFlow CLI, which is expected in E2E tests
    // We're primarily checking that the examples don't panic or crash
    if !status.success() {
        // Allow "Connection failed" messages as they're expected when iFlow CLI is not installed
        if !stderr_content.contains("Connection failed") {
            panic!(
                "Example {} failed with exit code: {:?}",
                example_name,
                status.code()
            );
        } else {
            println!("Example {} failed as expected due to missing iFlow CLI", example_name);
        }
    } else {
        println!("Example {} completed successfully", example_name);
    }
}