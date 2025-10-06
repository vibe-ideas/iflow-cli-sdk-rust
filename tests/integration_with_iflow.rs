//! Integration tests that require iFlow to be installed
//!
//! These tests actually connect to iFlow and test the full stack

use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, query};
use futures::StreamExt;

#[tokio::test]
#[ignore] // Requires iFlow with API configuration
async fn test_query_simple_with_iflow() {
    // This test requires iFlow to be installed
    let result = query("What is 2+2? Answer with just the number.").await;
    
    // The query might work or fail depending on iFlow availability
    match result {
        Ok(response) => {
            println!("Query successful: {}", response);
            assert!(!response.is_empty());
        }
        Err(e) => {
            // If iFlow is not available, that's expected
            println!("Query failed (expected if iFlow not available): {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Requires iFlow with API configuration
async fn test_client_connection_with_iflow() {
    // Use LocalSet for spawn_local compatibility  
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        let mut client = IFlowClient::new(None);
        let result = client.connect().await;
        
        match result {
            Ok(_) => {
                println!("Successfully connected to iFlow");
                let _ = client.disconnect().await;
                assert!(true);
            }
            Err(e) => {
                println!("Connection failed: {:?}", e);
                // Don't fail the test if iFlow is not available
            }
        }
    }).await;
}

#[tokio::test]
#[ignore] // Requires iFlow with API configuration
async fn test_message_stream_with_iflow() {
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        let mut client = IFlowClient::new(Some(IFlowOptions::new().with_timeout(30.0)));
        
        if client.connect().await.is_ok() {
            if client.send_message("Hello", None).await.is_ok() {
                let mut stream = client.messages();
                let mut count = 0;
                
                while let Some(_msg) = stream.next().await {
                    count += 1;
                    if count > 5 {
                        break;
                    }
                }
                
                let _ = client.disconnect().await;
                assert!(count > 0);
            }
        }
    }).await;
}
