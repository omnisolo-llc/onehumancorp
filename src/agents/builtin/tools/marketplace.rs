use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// AutoGPT Unique Harness Innovations: Agent Marketplace
/// SOTA Harness Pattern: AutoGPT Agent Marketplace API distribution
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
    async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String>;
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

    async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
        let url = format!("{}/agents", self.registry_url);
        let response = self.http_client.post(&url)
            .json(&agent)
            .send()
            .await
            .map_err(|e| format!("Failed to publish agent: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Marketplace returned status: {}", response.status()));
        }

        let published_agent: MarketplaceAgent = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(published_agent)
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
        if let Ok(cache) = self.cache.read() && let Some(agent) = cache.get(agent_id) {
            return Ok(agent.clone());
        }

        // Fetch using provider
        let agent = self.provider.fetch_agent(agent_id).await?;

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(agent_id.to_string(), agent.clone());
        }
        Ok(agent)
    }

    /// Publish a new agent to the marketplace
    pub async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
        let published_agent = self.provider.publish_agent(agent).await?;
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(published_agent.id.clone(), published_agent.clone());
        }
        Ok(published_agent)
    }
}

pub mod test_utils {
    use super::*;

    pub struct MockMarketplaceProvider;

    #[async_trait::async_trait]
    impl MarketplaceProvider for MockMarketplaceProvider {
        async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
            if query == "error" {
                return Err("Mock error".to_string());
            }
            let mut results = vec![
                MarketplaceAgent {
                    id: "agent-1".to_string(),
                    name: "Data Analyst".to_string(),
                    description: "Analyzes CSV files and generates charts.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-1".to_string(),
                },
                MarketplaceAgent {
                    id: "agent-2".to_string(),
                    name: "Senior Rust Developer".to_string(),
                    description: "Writes highly optimized Rust code.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-2".to_string(),
                },
                MarketplaceAgent {
                    id: "agent-3".to_string(),
                    name: "Technical Writer".to_string(),
                    description: "Writes comprehensive documentation.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-3".to_string(),
                }
            ];

            if !query.is_empty() {
                let q_lower = query.to_lowercase();
                results.retain(|a| a.name.to_lowercase().contains(&q_lower) || a.description.to_lowercase().contains(&q_lower));
            }
            Ok(results)
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
            } else if agent_id == "agent-2" {
                Ok(MarketplaceAgent {
                    id: "agent-2".to_string(),
                    name: "Senior Rust Developer".to_string(),
                    description: "Writes highly optimized Rust code.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-2".to_string(),
                })
            } else if agent_id == "agent-3" {
                Ok(MarketplaceAgent {
                    id: "agent-3".to_string(),
                    name: "Technical Writer".to_string(),
                    description: "Writes comprehensive documentation.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-3".to_string(),
                })
            } else {
                Err("Not found".to_string())
            }
        }

        async fn publish_agent(&self, mut agent: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
            if agent.name == "error" {
                return Err("Mock publish error".to_string());
            }
            if agent.id.is_empty() {
                agent.id = "mock-id-123".to_string();
            }
            Ok(agent)
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

    #[tokio::test]
    async fn test_marketplace_publish() {
        let client = MarketplaceClient::new(Box::new(MockMarketplaceProvider));
        let new_agent = MarketplaceAgent {
            id: "".to_string(),
            name: "New Agent".to_string(),
            description: "A new test agent".to_string(),
            author: "Tester".to_string(),
            version: "1.0".to_string(),
            endpoint: "http://example.com".to_string(),
        };

        let published = client.publish_agent(new_agent).await.unwrap();
        assert_eq!(published.id, "mock-id-123");
        assert_eq!(published.name, "New Agent");

        let error_agent = MarketplaceAgent {
            id: "".to_string(),
            name: "error".to_string(),
            description: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            endpoint: "".to_string(),
        };
        let error_res = client.publish_agent(error_agent).await;
        assert!(error_res.is_err());
    }
}
