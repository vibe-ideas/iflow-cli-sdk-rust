//! Additional unit tests for WebSocketTransport
//!
//! These tests cover basic initialization and state management

use iflow_cli_sdk_rust::websocket_transport::WebSocketTransport;

#[test]
fn test_websocket_transport_new() {
    let transport = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    assert!(!transport.is_connected());
}

#[test]
fn test_websocket_transport_new_with_different_urls() {
    let transport1 = WebSocketTransport::new("ws://localhost:8080/acp".to_string(), 60.0);
    let transport2 = WebSocketTransport::new("ws://example.com:9000/ws".to_string(), 45.0);
    let transport3 = WebSocketTransport::new("wss://secure.example.com/socket".to_string(), 120.0);
    
    assert!(!transport1.is_connected());
    assert!(!transport2.is_connected());
    assert!(!transport3.is_connected());
}

#[test]
fn test_websocket_transport_various_timeouts() {
    let short_timeout = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 1.0);
    let medium_timeout = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    let long_timeout = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 300.0);
    
    assert!(!short_timeout.is_connected());
    assert!(!medium_timeout.is_connected());
    assert!(!long_timeout.is_connected());
}

#[tokio::test]
async fn test_websocket_transport_send_when_not_connected() {
    let mut transport = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    let message = serde_json::json!({"type": "test"});
    
    let result = transport.send(&message).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_websocket_transport_receive_when_not_connected() {
    let mut transport = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    
    let result = transport.receive().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_websocket_transport_disconnect_when_not_connected() {
    let mut transport = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    
    // Closing when not connected should not error
    let result = transport.close().await;
    assert!(result.is_ok());
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_websocket_transport_connect_to_invalid_url() {
    let mut transport = WebSocketTransport::new("not a valid url".to_string(), 1.0);
    
    let result = transport.connect().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_websocket_transport_connect_to_unreachable_host() {
    // Using a timeout that's very short to fail quickly
    let mut transport = WebSocketTransport::new("ws://192.0.2.1:9999/acp".to_string(), 0.1);
    
    let result = transport.connect().await;
    assert!(result.is_err());
}

#[test]
fn test_websocket_transport_url_variations() {
    let urls = vec![
        "ws://localhost:8000",
        "ws://127.0.0.1:8080/path",
        "wss://example.com",
        "wss://example.com:443/secure",
        "ws://[::1]:8090/ipv6",
    ];
    
    for url in urls {
        let transport = WebSocketTransport::new(url.to_string(), 30.0);
        assert!(!transport.is_connected());
    }
}

#[tokio::test]
async fn test_websocket_transport_multiple_disconnect_calls() {
    let mut transport = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    
    // Multiple close calls should be safe
    let result1 = transport.close().await;
    let result2 = transport.close().await;
    let result3 = transport.close().await;
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
    assert!(!transport.is_connected());
}

#[test]
fn test_websocket_transport_url_accessor() {
    let url = "ws://localhost:8090/acp?peer=iflow".to_string();
    let transport = WebSocketTransport::new(url.clone(), 30.0);
    assert_eq!(transport.url(), &url);
}

#[tokio::test]
async fn test_websocket_transport_send_raw_when_not_connected() {
    let mut transport = WebSocketTransport::new("ws://localhost:8090/acp".to_string(), 30.0);
    
    let result = transport.send_raw("test message").await;
    assert!(result.is_err());
}
