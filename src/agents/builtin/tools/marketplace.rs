use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// AutoGPT Unique Harness Innovations: Agent Marketplace
/// Pre-built agent distribution.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceAgent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub endpoint: String, // Where to fetch the agent payload/definition
}

#[async_trait::async_trait]
pub trait MarketplaceProvider: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String>;
    async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String>;
}

pub struct HttpMarketplaceProvider {
    pub registry_url: String,
    pub http_client: reqwest::Client,
}

impl HttpMarketplaceProvider {
    pub fn new(registry_url: &str) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl MarketplaceProvider for HttpMarketplaceProvider {
    async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
        let url = format!("{}/search", self.registry_url);
        let response = self.http_client.get(&url)
            .query(&[("q", query)])
            .send()
            .await
            .map_err(|e| format!("Failed to search marketplace: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Marketplace returned status: {}", response.status()));
        }

        let agents: Vec<MarketplaceAgent> = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(agents)
    }

    async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
        let url = format!("{}/agents/{}", self.registry_url, agent_id);
        let response = self.http_client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch agent: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Marketplace returned status: {}", response.status()));
        }

        let agent: MarketplaceAgent = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(agent)
    }
}

pub struct MarketplaceClient {
    pub provider: Box<dyn MarketplaceProvider>,
    // Caches fetched agent definitions
    cache: std::sync::RwLock<HashMap<String, MarketplaceAgent>>,
}

impl MarketplaceClient {
    pub fn new(provider: Box<dyn MarketplaceProvider>) -> Self {
        Self {
            provider,
            cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Search for agents in the marketplace
    pub async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
        self.provider.search(query).await
    }

    /// Fetch a specific agent's definition
    pub async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
        // Check cache first
        if let Ok(cache) = self.cache.read() {
            if let Some(agent) = cache.get(agent_id) {
                return Ok(agent.clone());
            }
        }

        // Fetch using provider
        let agent = self.provider.fetch_agent(agent_id).await?;

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(agent_id.to_string(), agent.clone());
        }
        Ok(agent)
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub struct MockMarketplaceProvider;

    #[async_trait::async_trait]
    impl MarketplaceProvider for MockMarketplaceProvider {
        async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
            if query == "error" {
                return Err("Mock error".to_string());
            }
            Ok(vec![MarketplaceAgent {
                id: "agent-1".to_string(),
                name: "Data Analyst".to_string(),
                description: "Analyzes CSV files and generates charts.".to_string(),
                author: "AutoGPT".to_string(),
                version: "1.0.0".to_string(),
                endpoint: "https://marketplace.example.com/agents/agent-1".to_string(),
            }])
        }

        async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
            if agent_id == "agent-1" {
                Ok(MarketplaceAgent {
                    id: "agent-1".to_string(),
                    name: "Data Analyst".to_string(),
                    description: "Analyzes CSV files and generates charts.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-1".to_string(),
                })
            } else {
                Err("Not found".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_utils::MockMarketplaceProvider;

    #[tokio::test]
    async fn test_marketplace_search() {
        let client = MarketplaceClient::new(Box::new(MockMarketplaceProvider));
        let results = client.search("data").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Data Analyst");
    }

    #[tokio::test]
    async fn test_marketplace_fetch() {
        let client = MarketplaceClient::new(Box::new(MockMarketplaceProvider));
        let agent = client.fetch_agent("agent-1").await.unwrap();
        assert_eq!(agent.name, "Data Analyst");

        // Test caching (should return immediately)
        let agent2 = client.fetch_agent("agent-1").await.unwrap();
        assert_eq!(agent2.id, agent.id);

        let not_found = client.fetch_agent("unknown").await;
        assert!(not_found.is_err());
    }
}
