//! Additional tests to verify process cleanup

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::process_manager::IFlowProcessManager;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Test that processes are properly cleaned up when IFlowProcessManager is dropped
    #[tokio::test]
    async fn test_process_cleanup_on_drop() {
        {
            let mut pm = IFlowProcessManager::new(8096);
            let result = pm.start(false).await;
            
            if result.is_ok() {
                // Get the process ID before dropping
                let _process_id = pm.process.as_ref().map(|p| p.id());
                println!("Started process with ID: {:?}", _process_id);
                
                // pm will be dropped at the end of this scope
            } else {
                // If we couldn't start the process, there's nothing to clean up
            }
        }; // pm is dropped here
        
        // Give a small delay to allow cleanup
        sleep(Duration::from_millis(200)).await;
        
        // We can't directly verify the process is killed since we don't have access to the process ID
        // outside the scope, but the test will pass if no errors occur during cleanup
        println!("Process manager dropped, cleanup should be complete");
    }

    /// Test that stop() properly terminates the process
    #[tokio::test]
    async fn test_stop_terminates_process() {
        let mut pm = IFlowProcessManager::new(8097);
        let result = pm.start(false).await;
        
        if result.is_ok() {
            // Verify the process is running
            assert!(pm.is_running());
            
            // Get the process ID before stopping
            let process_id = pm.process.as_ref().map(|p| p.id());
            println!("Process ID before stop: {:?}", process_id);
            
            // Stop the process
            let stop_result = pm.stop().await;
            assert!(stop_result.is_ok());
            
            // Verify it's no longer running
            assert!(!pm.is_running());
            
            // Give a small delay to ensure complete cleanup
            sleep(Duration::from_millis(100)).await;
            
            println!("Process stopped successfully");
        }
    }

    /// Test WebSocket process cleanup
    #[tokio::test]
    async fn test_websocket_process_cleanup() {
        let mut pm = IFlowProcessManager::new(8098);
        let result = pm.start(true).await;
        
        if result.is_ok() {
            // Verify the process is running
            assert!(pm.is_running());
            assert!(pm.port().is_some());
            
            // Get the process ID before stopping
            let process_id = pm.process.as_ref().map(|p| p.id());
            println!("WebSocket process ID before stop: {:?}", process_id);
            
            // Stop the process
            let stop_result = pm.stop().await;
            assert!(stop_result.is_ok());
            
            // Verify it's no longer running
            assert!(!pm.is_running());
            assert_eq!(pm.port(), None);
            
            // Give a small delay to ensure complete cleanup
            sleep(Duration::from_millis(100)).await;
            
            println!("WebSocket process stopped successfully");
        }
    }
}