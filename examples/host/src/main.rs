use std::collections::HashMap;

mod mcp_manger;

use mcp_manger::McpManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mcp_manager = McpManager::new();
    let result = mcp_manager.init();

    if let Err(e) = result {
        println!("Error initializing MCP manager: {}", e);
    }

    Ok(())
}
