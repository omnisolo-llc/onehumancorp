use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// AutoGPT Unique Harness Innovations: Agent Marketplace
/// Pre-built agent distribution.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub role: String,
    pub system_prompt: String,
}

pub struct Marketplace {
    registry: RwLock<HashMap<String, AgentDefinition>>,
}

impl Marketplace {
    pub fn new() -> Self {
        let mut registry = HashMap::new();

        let coder = AgentDefinition {
            id: "coder-1".to_string(),
            name: "Senior Rust Developer".to_string(),
            description: "An expert in Rust capable of building concurrent and safe systems.".to_string(),
            role: "Developer".to_string(),
            system_prompt: "You are a senior Rust developer. Write idiomatic, safe, and concurrent Rust code.".to_string(),
        };
        registry.insert(coder.id.clone(), coder);

        let writer = AgentDefinition {
            id: "writer-1".to_string(),
            name: "Technical Writer".to_string(),
            description: "Produces high-quality technical documentation.".to_string(),
            role: "Writer".to_string(),
            system_prompt: "You are a technical writer. Create clear, concise, and accurate documentation.".to_string(),
        };
        registry.insert(writer.id.clone(), writer);

        Self { registry: RwLock::new(registry) }
    }

    /// Add or update an agent in the marketplace.
    pub fn publish_agent(&self, agent: AgentDefinition) -> Result<(), String> {
        let mut reg = self.registry.write().map_err(|_| "Failed to acquire write lock".to_string())?;
        reg.insert(agent.id.clone(), agent);
        Ok(())
    }

    /// Remove an agent from the marketplace.
    pub fn unpublish_agent(&self, id: &str) -> Result<(), String> {
        let mut reg = self.registry.write().map_err(|_| "Failed to acquire write lock".to_string())?;
        if reg.remove(id).is_some() {
            Ok(())
        } else {
            Err(format!("Agent '{}' not found.", id))
        }
    }

    /// List all available agents in the marketplace.
    pub fn list_agents(&self) -> Result<Vec<AgentDefinition>, String> {
        let reg = self.registry.read().map_err(|_| "Failed to acquire read lock".to_string())?;
        Ok(reg.values().cloned().collect())
    }

    /// Download/fetch a specific agent by id.
    pub fn download_agent(&self, id: &str) -> Result<AgentDefinition, String> {
        let reg = self.registry.read().map_err(|_| "Failed to acquire read lock".to_string())?;
        reg.get(id).cloned().ok_or_else(|| format!("Agent '{}' not found in the marketplace.", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_agents() {
        let marketplace = Marketplace::new();
        let agents = marketplace.list_agents().unwrap();
        assert_eq!(agents.len(), 2);

        let has_coder = agents.iter().any(|a| a.name == "Senior Rust Developer");
        let has_writer = agents.iter().any(|a| a.name == "Technical Writer");

        assert!(has_coder);
        assert!(has_writer);
    }

    #[test]
    fn test_download_agent_success() {
        let marketplace = Marketplace::new();
        let result = marketplace.download_agent("coder-1");
        assert!(result.is_ok());
        let agent = result.unwrap();
        assert_eq!(agent.name, "Senior Rust Developer");
        assert_eq!(agent.role, "Developer");
    }

    #[test]
    fn test_download_agent_not_found() {
        let marketplace = Marketplace::new();
        let result = marketplace.download_agent("non-existent-1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Agent 'non-existent-1' not found in the marketplace.");
    }

    #[test]
    fn test_publish_unpublish_agent() {
        let marketplace = Marketplace::new();
        let new_agent = AgentDefinition {
            id: "designer-1".to_string(),
            name: "UI Designer".to_string(),
            description: "Creates beautiful UI designs.".to_string(),
            role: "Designer".to_string(),
            system_prompt: "You are a UI designer.".to_string(),
        };

        // Publish
        assert!(marketplace.publish_agent(new_agent.clone()).is_ok());

        // Verify it was added
        let fetched = marketplace.download_agent("designer-1").unwrap();
        assert_eq!(fetched.name, "UI Designer");

        let agents = marketplace.list_agents().unwrap();
        assert_eq!(agents.len(), 3);

        // Unpublish
        assert!(marketplace.unpublish_agent("designer-1").is_ok());

        // Verify it was removed
        assert!(marketplace.download_agent("designer-1").is_err());

        let agents_after = marketplace.list_agents().unwrap();
        assert_eq!(agents_after.len(), 2);
    }
}

// Implement LocalMarketplaceProvider in the builtin crate (which has access to both tools and builtin structs)

use ohc_builtin_agent_tools::marketplace::{MarketplaceProvider, MarketplaceAgent};
use async_trait::async_trait;
use std::sync::Arc;

/// A Local Marketplace Provider that directly uses the Marketplace registry.
pub struct LocalMarketplaceProvider {
    marketplace: Arc<Marketplace>,
}

impl LocalMarketplaceProvider {
    pub fn new(marketplace: Arc<Marketplace>) -> Self {
        Self { marketplace }
    }
}

#[async_trait]
impl MarketplaceProvider for LocalMarketplaceProvider {
    async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
        let agents = self.marketplace.list_agents()?;

        let query_lower = query.to_lowercase();

        let filtered = agents.into_iter()
            .filter(|a| {
                query_lower.is_empty()
                || a.name.to_lowercase().contains(&query_lower)
                || a.description.to_lowercase().contains(&query_lower)
            })
            .map(|a| MarketplaceAgent {
                id: a.id.clone(),
                name: a.name.clone(),
                description: a.description.clone(),
                author: "LocalMarketplace".to_string(),
                version: "1.0.0".to_string(),
                endpoint: format!("local://{}", a.id),
            })
            .collect();

        Ok(filtered)
    }

    async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
        let agent = self.marketplace.download_agent(agent_id)?;

        Ok(MarketplaceAgent {
            id: agent.id.clone(),
            name: agent.name.clone(),
            description: agent.description.clone(),
            author: "LocalMarketplace".to_string(),
            version: "1.0.0".to_string(),
            endpoint: format!("local://{}", agent.id),
        })
    }
}

#[cfg(test)]
mod provider_tests {
    use super::*;

    #[tokio::test]
    async fn test_local_marketplace_provider() {
        let local_marketplace = Arc::new(Marketplace::new());
        let provider = LocalMarketplaceProvider::new(local_marketplace);

        let agents = provider.search("rust").await.unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "Senior Rust Developer");

        let agent = provider.fetch_agent("coder-1").await.unwrap();
        assert_eq!(agent.name, "Senior Rust Developer");
    }
}
