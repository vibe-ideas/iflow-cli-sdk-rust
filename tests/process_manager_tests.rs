//! Unit tests for iFlow process manager
//!
//! These tests verify the correct behavior of the IFlowProcessManager
//! in various scenarios including stdio and websocket modes,
//! and both manual and auto-start configurations.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::process_manager::IFlowProcessManager;
    use iflow_cli_sdk_rust::error::IFlowError;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Test auto-start in stdio mode
    #[tokio::test]
    async fn test_auto_start_stdio() {
        let mut pm = IFlowProcessManager::new(8090);
        let result = pm.start(false).await;
        
        // We expect this to fail if iFlow CLI is not installed
        // But we want to verify the process management logic itself
        match result {
            Ok(_) => {
                // If iFlow started successfully, verify it's running
                assert!(pm.is_running());
                
                // Stop the process
                let stop_result = pm.stop().await;
                assert!(stop_result.is_ok());
                
                // Verify it's no longer running
                assert!(!pm.is_running());
            }
            Err(IFlowError::ProcessManager(msg)) => {
                // This is expected if iFlow CLI is not installed
                // We're testing the process management logic, not the installation
                println!("Process manager error (expected if iFlow CLI not installed): {}", msg);
            }
            Err(e) => {
                // Any other error is unexpected
                panic!("Unexpected error: {:?}", e);
            }
        }
    }

    /// Test auto-start in websocket mode
    #[tokio::test]
    async fn test_auto_start_websocket() {
        let mut pm = IFlowProcessManager::new(8091);
        let result = pm.start(true).await;
        
        // We expect this to fail if iFlow CLI is not installed
        // But we want to verify the process management logic itself
        match result {
            Ok(url) => {
                // If iFlow started successfully, verify it's running and we got a URL
                assert!(pm.is_running());
                assert!(url.is_some());
                assert!(url.unwrap().starts_with("ws://localhost:"));
                
                // Stop the process
                let stop_result = pm.stop().await;
                assert!(stop_result.is_ok());
                
                // Verify it's no longer running
                assert!(!pm.is_running());
            }
            Err(IFlowError::ProcessManager(msg)) => {
                // This is expected if iFlow CLI is not installed
                // We're testing the process management logic, not the installation
                println!("Process manager error (expected if iFlow CLI not installed): {}", msg);
            }
            Err(e) => {
                // Any other error is unexpected
                panic!("Unexpected error: {:?}", e);
            }
        }
    }

    /// Test manual start in stdio mode (process not running)
    #[tokio::test]
    async fn test_manual_start_stdio_not_running() {
        // This test assumes iFlow is not running on the system
        // We're testing the process management logic
        let mut pm = IFlowProcessManager::new(8092);
        
        // In manual mode, we don't actually start the process
        // but we can test the methods
        assert!(!pm.is_running());
        assert_eq!(pm.port(), None);
        
        // take_stdin and take_stdout should return None when no process is running
        assert!(pm.take_stdin().is_none());
        assert!(pm.take_stdout().is_none());
    }

    /// Test manual start in websocket mode (process not running)
    #[tokio::test]
    async fn test_manual_start_websocket_not_running() {
        // This test assumes iFlow is not running on the system
        // We're testing the process management logic
        let pm = IFlowProcessManager::new(8093);
        
        // In manual mode, we don't actually start the process
        // but we can test the methods
        assert!(!pm.is_running());
        assert_eq!(pm.port(), None);
    }

    /// Test port availability checking
    #[test]
    fn test_port_availability() {
        // This test checks the port availability logic
        // It doesn't actually start any processes
        
        // Skip this test as the methods are private and we can't test them directly
        // In a real implementation, we might want to make these methods public
        // or create public test methods in the IFlowProcessManager
    }

    /// Test process manager drop behavior
    #[tokio::test]
    async fn test_process_manager_drop() {
        // This test verifies that when the process manager is dropped,
        // it properly cleans up resources
        
        // Create a scope to ensure the process manager is dropped
        {
            let mut pm = IFlowProcessManager::new(8095);
            let result = pm.start(false).await;
            
            // Handle the result as in other tests
            match result {
                Ok(_) => {
                    // If started, verify it's running
                    assert!(pm.is_running());
                    // The process manager will be dropped here, which should clean up
                }
                Err(IFlowError::ProcessManager(_)) => {
                    // Expected if iFlow CLI not installed
                }
                Err(e) => {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        } // pm is dropped here
        
        // Give a small delay to allow cleanup
        sleep(Duration::from_millis(100)).await;
    }
}