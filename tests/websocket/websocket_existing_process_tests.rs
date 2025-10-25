//! Test for connecting to an existing iFlow process via WebSocket
//!
//! This test verifies that the IFlowClient can successfully connect to
//! an existing iFlow process using WebSocket transport.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::config::websocket::WebSocketConfig;
    use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions};
    use std::process::Command;
    use std::time::Duration;

    #[tokio::test]
    async fn test_websocket_connection_to_existing_process() {
        // Start an iFlow process manually
        let mut iflow_process = Command::new("iflow")
            .arg("--experimental-acp")
            .arg("--port")
            .arg("8092")
            .spawn()
            .expect("Failed to start iFlow process");

        // Give the process a moment to start - poll until the port is listening
        let mut attempts = 0;
        let max_attempts = 30; // 30 attempts * 1 second = 30 seconds total

        while attempts < max_attempts {
            if std::net::TcpStream::connect_timeout(
                &"127.0.0.1:8092".parse().unwrap(),
                std::time::Duration::from_millis(100),
            )
            .is_ok()
            {
                break;
            }

            attempts += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        if attempts >= max_attempts {
            // Clean up the iFlow process
            let _ = iflow_process.kill();
            let _ = iflow_process.wait();
            panic!(
                "iFlow process failed to start WebSocket server on port 8092 after {} seconds",
                max_attempts
            );
        }

        // Configure client options to connect to the existing process
        let options = IFlowOptions::new().with_websocket_config(WebSocketConfig::new(
            "ws://localhost:8092/acp?peer=iflow".to_string(),
        ));

        // Create and connect client
        let mut client = IFlowClient::new(Some(options));

        // Connect to the existing process
        let connect_result = client.connect().await;

        // Clean up the iFlow process
        let _ = iflow_process.kill();
        let _ = iflow_process.wait();

        // Verify connection was successful
        assert!(
            connect_result.is_ok(),
            "Failed to connect to existing iFlow process via WebSocket"
        );
    }
}
