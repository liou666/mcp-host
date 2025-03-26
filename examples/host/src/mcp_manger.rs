use mcp_client::{
    client::{ClientCapabilities, ClientInfo, McpClient, McpClientTrait},
    transport::{SseTransport, StdioTransport, Transport},
    McpService,
};

use std::{collections::HashMap, sync::Arc};

#[derive(serde::Deserialize, Debug)]
struct McpServer {
    name: String,
    description: Option<String>,
    base_url: Option<String>,
    command: Option<String>,
    args: Option<Vec<String>>,
    env: Option<HashMap<String, String>>,
    is_active: bool,
}

#[derive(serde::Deserialize, Debug)]
struct McpConfig {
    mcpServers: HashMap<String, McpServer>,
}

pub struct McpManager {
    pub servers: Vec<McpServer>,
}

impl McpManager {
    pub fn new() -> Self {
        Self { servers: vec![] }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string("mcp.json")?;

        let config: McpConfig = serde_json::from_str(&content)?;
        self.servers = config.mcpServers.into_values().collect();
        println!("config: {:?}", self.servers);

        Ok(())
    }

    pub fn add_server(&mut self, server: McpServer) {
        self.servers.push(server);
    }
}
