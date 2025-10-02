//! Test for process cleanup on timeout

use iflow_cli_sdk_rust::query_with_timeout;
use std::time::Instant;
use tokio::time::{sleep, Duration};

/// Test that iFlow process is properly cleaned up after a timeout
#[tokio::test]
async fn test_process_cleanup_on_timeout() {
    // Record start time
    let start = Instant::now();
    
    // Run a query with a very short timeout (0.1 seconds)
    // This should timeout before iFlow can respond
    let result = query_with_timeout("This is a test query that should timeout", 0.1).await;
    
    // Record end time
    let duration = start.elapsed();
    
    // Check that the operation completed in a reasonable time
    // Should be close to the timeout value (0.1 seconds) plus some overhead
    assert!(duration.as_secs_f64() < 5.0, "Operation took too long, process may not have been cleaned up properly");
    
    // The query should timeout and return an error
    assert!(result.is_err(), "Query should have timed out");
    
    // Small delay to ensure cleanup has time to complete
    sleep(Duration::from_millis(100)).await;
    
    // Additional check - try to run another query to verify the system is in a clean state
    // This might also timeout, but it shouldn't fail due to process conflicts
    let start2 = Instant::now();
    let result2 = query_with_timeout("Second test query", 0.1).await;
    let duration2 = start2.elapsed();
    
    // This should also complete quickly
    assert!(duration2.as_secs_f64() < 5.0, "Second operation took too long, process may not have been cleaned up properly");
    
    // Both queries should have timed out
    assert!(result2.is_err(), "Second query should have timed out");
}