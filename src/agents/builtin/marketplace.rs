use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AutoGPT Unique Harness Innovations: Agent Marketplace
/// Pre-built agent distribution.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentDefinition {
    pub name: String,
    pub description: String,
    pub role: String,
    pub system_prompt: String,
}

pub struct Marketplace {
    registry: HashMap<String, AgentDefinition>,
}

impl Marketplace {
    pub fn new() -> Self {
        let mut registry = HashMap::new();

        let coder = AgentDefinition {
            name: "Senior Rust Developer".to_string(),
            description: "An expert in Rust capable of building concurrent and safe systems.".to_string(),
            role: "Developer".to_string(),
            system_prompt: "You are a senior Rust developer. Write idiomatic, safe, and concurrent Rust code.".to_string(),
        };
        registry.insert(coder.name.clone(), coder);

        let writer = AgentDefinition {
            name: "Technical Writer".to_string(),
            description: "Produces high-quality technical documentation.".to_string(),
            role: "Writer".to_string(),
            system_prompt: "You are a technical writer. Create clear, concise, and accurate documentation.".to_string(),
        };
        registry.insert(writer.name.clone(), writer);

        Self { registry }
    }

    /// List all available agents in the marketplace.
    pub fn list_agents(&self) -> Vec<AgentDefinition> {
        self.registry.values().cloned().collect()
    }

    /// Download/fetch a specific agent by name.
    pub fn download_agent(&self, name: &str) -> Result<AgentDefinition, String> {
        self.registry.get(name).cloned().ok_or_else(|| format!("Agent '{}' not found in the marketplace.", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_agents() {
        let marketplace = Marketplace::new();
        let agents = marketplace.list_agents();
        assert_eq!(agents.len(), 2);

        let has_coder = agents.iter().any(|a| a.name == "Senior Rust Developer");
        let has_writer = agents.iter().any(|a| a.name == "Technical Writer");

        assert!(has_coder);
        assert!(has_writer);
    }

    #[test]
    fn test_download_agent_success() {
        let marketplace = Marketplace::new();
        let result = marketplace.download_agent("Senior Rust Developer");
        assert!(result.is_ok());
        let agent = result.unwrap();
        assert_eq!(agent.name, "Senior Rust Developer");
        assert_eq!(agent.role, "Developer");
    }

    #[test]
    fn test_download_agent_not_found() {
        let marketplace = Marketplace::new();
        let result = marketplace.download_agent("Non-existent Agent");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Agent 'Non-existent Agent' not found in the marketplace.");
    }
}
