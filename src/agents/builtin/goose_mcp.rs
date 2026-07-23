/// Master Catalog C.20. goose (Agentic AI): Rust + TypeScript UI, 70+ MCP extensions
/// Represents the integration layer for the Goose Agent framework.
/// This module implements the goose archetype for MCP extensions.

use crate::agent::AgentRunConfig;

pub struct GooseMcpProvider {
    pub endpoint: String,
}

impl GooseMcpProvider {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub fn build_goose_config(&self) -> AgentRunConfig {
        let mut config = AgentRunConfig::default();
        config.enable_visual_verification = true; // Goose uses UI heavily
        config.max_iterations = 40;
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goose_mcp_config() {
        let provider = GooseMcpProvider::new("http://localhost:8000");
        let config = provider.build_goose_config();

        assert_eq!(provider.endpoint, "http://localhost:8000");
        assert!(config.enable_visual_verification);
        assert_eq!(config.max_iterations, 40);
    }
}
