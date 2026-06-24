use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};

/// AutoGPT Unique Harness Innovations: Agent Marketplace
/// Pre-built agent distribution.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentDefinition {
    pub name: String,
    pub description: String,
    pub role: String,
    pub system_prompt: String,
}

static GLOBAL_REGISTRY: OnceLock<Arc<Mutex<HashMap<String, AgentDefinition>>>> = OnceLock::new();

fn get_registry() -> Arc<Mutex<HashMap<String, AgentDefinition>>> {
    GLOBAL_REGISTRY.get_or_init(|| {
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

        Arc::new(Mutex::new(registry))
    }).clone()
}

pub struct Marketplace;

impl Default for Marketplace {
    fn default() -> Self {
        Self::new()
    }
}

impl Marketplace {
    pub fn new() -> Self {
        Self
    }

    /// List all available agents in the marketplace.
    pub fn list_agents(&self) -> Vec<AgentDefinition> {
        let registry = get_registry();
        let guard = registry.lock().unwrap();
        guard.values().cloned().collect()
    }

    /// Download/fetch a specific agent by name.
    pub fn download_agent(&self, name: &str) -> Result<AgentDefinition, String> {
        let registry = get_registry();
        let guard = registry.lock().unwrap();
        guard
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Agent '{}' not found in the marketplace.", name))
    }

    /// Publish a new agent to the marketplace.
    pub fn publish_agent(&self, agent: AgentDefinition) -> Result<(), String> {
        let registry = get_registry();
        let mut guard = registry.lock().unwrap();
        if guard.contains_key(&agent.name) {
            return Err(format!(
                "Agent '{}' already exists in the marketplace.",
                agent.name
            ));
        }
        guard.insert(agent.name.clone(), agent);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_agents() {
        let marketplace = Marketplace::new();
        let agents = marketplace.list_agents();
        assert!(agents.len() >= 2); // Might be more if other tests run concurrently

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
        assert_eq!(
            result.unwrap_err(),
            "Agent 'Non-existent Agent' not found in the marketplace."
        );
    }

    #[test]
    fn test_publish_agent_success() {
        let marketplace = Marketplace::new();
        let new_agent = AgentDefinition {
            name: "Test Agent".to_string(),
            description: "Test description".to_string(),
            role: "Tester".to_string(),
            system_prompt: "Test prompt".to_string(),
        };

        let result = marketplace.publish_agent(new_agent.clone());
        assert!(result.is_ok());

        let fetched = marketplace.download_agent("Test Agent");
        assert!(fetched.is_ok());
        assert_eq!(fetched.unwrap(), new_agent);
    }
}
