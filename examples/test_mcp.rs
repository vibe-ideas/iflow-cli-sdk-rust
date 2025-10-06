use iflow_cli_sdk_rust::McpServer;
use std::path::PathBuf;

fn main() {
    let server = McpServer::Stdio {
        name: "test".to_string(),
        command: PathBuf::from("test-cmd"),
        args: vec![],
        env: vec![],
    };
    
    println!("{:?}", server);
}