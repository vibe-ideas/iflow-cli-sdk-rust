//! Additional unit tests for process_manager module
//!
//! These tests verify process manager methods and edge cases

use iflow_cli_sdk_rust::IFlowProcessManager;

#[test]
fn test_process_manager_new() {
    let pm = IFlowProcessManager::new(8090);
    assert!(!pm.is_running());
    assert_eq!(pm.port(), None);
}

#[test]
fn test_process_manager_port_listening() {
    // Test with a port that's definitely not listening
    assert!(!IFlowProcessManager::is_port_listening(65534));
}

#[test]
fn test_process_manager_port_listening_multiple() {
    // Test multiple ports
    for port in [9999, 10000, 10001] {
        let _result = IFlowProcessManager::is_port_listening(port);
        // Just ensure it doesn't panic
    }
}

#[test]
fn test_process_manager_new_with_different_ports() {
    let pm1 = IFlowProcessManager::new(8090);
    let pm2 = IFlowProcessManager::new(9000);
    
    assert!(!pm1.is_running());
    assert!(!pm2.is_running());
    assert_eq!(pm1.port(), None);
    assert_eq!(pm2.port(), None);
}

#[test]
fn test_process_manager_initial_state() {
    let pm = IFlowProcessManager::new(8095);
    
    // Process should not be running initially
    assert!(!pm.is_running());
    
    // Port should be None initially
    assert_eq!(pm.port(), None);
}

#[tokio::test]
async fn test_process_manager_multiple_instances() {
    // Create multiple process managers
    let pm1 = IFlowProcessManager::new(8090);
    let pm2 = IFlowProcessManager::new(8091);
    let pm3 = IFlowProcessManager::new(8092);
    
    // All should be in initial state
    assert!(!pm1.is_running());
    assert!(!pm2.is_running());
    assert!(!pm3.is_running());
}

#[test]
fn test_process_manager_port_check_edge_cases() {
    // Test with minimum port
    let _result1 = IFlowProcessManager::is_port_listening(1024);
    
    // Test with high port number
    let _result2 = IFlowProcessManager::is_port_listening(65535);
    
    // Test with standard port
    let _result3 = IFlowProcessManager::is_port_listening(8080);
}

#[tokio::test]
async fn test_process_manager_lifecycle() {
    let pm = IFlowProcessManager::new(8093);
    
    // Initially not running
    assert!(!pm.is_running());
    
    // Port is None initially
    assert_eq!(pm.port(), None);
}
