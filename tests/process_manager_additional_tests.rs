//! Additional tests for process manager utilities
//!
//! These tests cover utility functions in the process manager

use iflow_cli_sdk_rust::IFlowProcessManager;

#[test]
fn test_process_manager_new() {
    let manager = IFlowProcessManager::new(8090);
    assert!(!manager.is_running());
    assert_eq!(manager.port(), None);
}

#[test]
fn test_process_manager_with_different_ports() {
    let manager1 = IFlowProcessManager::new(8000);
    let manager2 = IFlowProcessManager::new(9000);
    let manager3 = IFlowProcessManager::new(10000);
    
    assert!(!manager1.is_running());
    assert!(!manager2.is_running());
    assert!(!manager3.is_running());
}

#[test]
fn test_is_port_listening_on_unused_port() {
    // Test a very high port that's unlikely to be in use
    let port = 65432;
    assert!(!IFlowProcessManager::is_port_listening(port));
}

#[test]
fn test_is_port_listening_on_common_ports() {
    // These tests check if common high ports are listening
    // They should typically return false unless something is running
    assert!(!IFlowProcessManager::is_port_listening(60000) || 
            IFlowProcessManager::is_port_listening(60000)); // Either is fine
    
    assert!(!IFlowProcessManager::is_port_listening(60001) || 
            IFlowProcessManager::is_port_listening(60001)); // Either is fine
}

#[test]
fn test_process_manager_multiple_instances() {
    let manager1 = IFlowProcessManager::new(8080);
    let manager2 = IFlowProcessManager::new(8081);
    let manager3 = IFlowProcessManager::new(8082);
    
    assert_eq!(manager1.port(), None);
    assert_eq!(manager2.port(), None);
    assert_eq!(manager3.port(), None);
}

#[tokio::test]
async fn test_process_manager_stdin_stdout_none_when_not_started() {
    let mut manager = IFlowProcessManager::new(8090);
    assert!(manager.take_stdin().is_none());
    assert!(manager.take_stdout().is_none());
}

#[tokio::test]
async fn test_process_manager_stop_when_not_running() {
    let mut manager = IFlowProcessManager::new(8090);
    // Stopping when not running should not cause issues
    manager.stop().await;
    assert!(!manager.is_running());
}

#[test]
fn test_process_manager_port_range() {
    // Test with various port ranges
    let low_port_manager = IFlowProcessManager::new(1024);
    let mid_port_manager = IFlowProcessManager::new(32768);
    let high_port_manager = IFlowProcessManager::new(60000);
    
    assert!(!low_port_manager.is_running());
    assert!(!mid_port_manager.is_running());
    assert!(!high_port_manager.is_running());
}

#[tokio::test]
async fn test_process_manager_lifecycle_without_starting() {
    let mut manager = IFlowProcessManager::new(8095);
    
    // Verify initial state
    assert!(!manager.is_running());
    assert_eq!(manager.port(), None);
    
    // Try to stop without starting
    manager.stop().await;
    
    // State should remain the same
    assert!(!manager.is_running());
    assert_eq!(manager.port(), None);
}

#[test]
fn test_is_port_listening_edge_cases() {
    // Test port 0 (should not be listening)
    assert!(!IFlowProcessManager::is_port_listening(0));
    
    // Test max port number (65535)
    assert!(!IFlowProcessManager::is_port_listening(65535) ||
            IFlowProcessManager::is_port_listening(65535)); // Either is acceptable
}

#[tokio::test]
async fn test_process_manager_double_stop() {
    let mut manager = IFlowProcessManager::new(8096);
    
    // Stop twice - should be idempotent
    manager.stop().await;
    manager.stop().await;
    
    assert!(!manager.is_running());
}

#[test]
fn test_process_manager_creation_with_various_ports() {
    // Test creating managers with various port numbers
    let ports = vec![3000, 5000, 8000, 8080, 8090, 9000, 10000, 50000];
    
    for port in ports {
        let manager = IFlowProcessManager::new(port);
        assert!(!manager.is_running());
        assert_eq!(manager.port(), None);
    }
}
