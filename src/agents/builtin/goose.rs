use ohc_builtin_agent_tools::Tool;
use ohc_builtin_agent_tools::mcp_dynamic::load_mcp_server_tools;
use agent_service_proto::ohc::agent::service::McpServerConfig;

/// goose Unique Harness Innovations: 70+ MCP extensions
/// A dynamic registry inspired by Agentic AI's Goose that allows registering and dynamically loading many MCP extensions.
pub struct GooseMcpRegistry {
    servers: Vec<McpServerConfig>,
}

impl GooseMcpRegistry {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
        }
    }

    /// Registers a new MCP extension with its server configuration.
    pub fn register_extension(&mut self, server: McpServerConfig) {
        self.servers.push(server);
    }

    /// Loads all registered MCP extensions and returns them as a Vec<Tool>.
    pub async fn load_all_extensions(&self) -> Vec<Tool> {
        load_mcp_server_tools(&self.servers).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_service_proto::ohc::agent::service::McpTransportType;

    #[tokio::test]
    async fn test_goose_mcp_registry() {
        let mut registry = GooseMcpRegistry::new();
        registry.register_extension(McpServerConfig {
            name: "test_server".to_string(),
            transport: McpTransportType::McpTransportUnspecified as i32,
            command: vec!["".to_string()],
            env: std::collections::HashMap::new(),
            endpoint: "".to_string(),
            allowed_tools: vec![],
        });

        let tools = registry.load_all_extensions().await;
        // Without an actual running server, it should return 0 tools via dynamic discovery
        assert_eq!(tools.len(), 0);
    }
}
