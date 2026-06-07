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
    registry_path: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceRegistry {
    pub agents: HashMap<String, MarketplaceAgent>,
}

impl MarketplaceClient {
    pub fn new(provider: Box<dyn MarketplaceProvider>) -> Self {
        let mut registry_path = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        registry_path.push(".ohc");
        registry_path.push("agents");
        registry_path.push("registry.json");

        Self {
            provider,
            cache: std::sync::RwLock::new(HashMap::new()),
            registry_path,
        }
    }

    // Test helper to allow injecting custom path
    pub fn with_path(provider: Box<dyn MarketplaceProvider>, path: std::path::PathBuf) -> Self {
        Self {
            provider,
            cache: std::sync::RwLock::new(HashMap::new()),
            registry_path: path,
        }
    }

    fn load_registry(&self) -> MarketplaceRegistry {
        if let Ok(contents) = std::fs::read_to_string(&self.registry_path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            MarketplaceRegistry::default()
        }
    }

    fn save_registry(&self, registry: &MarketplaceRegistry) -> Result<(), String> {
        if let Some(parent) = self.registry_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(registry).map_err(|e| e.to_string())?;
        std::fs::write(&self.registry_path, json).map_err(|e| e.to_string())
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

    /// Install an agent from the marketplace
    pub async fn install_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
        let mut registry = self.load_registry();

        let agent = self.fetch_agent(agent_id).await?;

        if let Some(existing) = registry.agents.get(&agent.id) {
            // Version check assumption: We assume version string matches semver or simply replacing is fine if strings differ
            if existing.version == agent.version {
                return Ok(existing.clone()); // Already installed with the exact version
            }

            // Very naive semver comparison for demonstration of versioning logic
            let parsed_existing: Result<semver::Version, _> = existing.version.parse();
            let parsed_new: Result<semver::Version, _> = agent.version.parse();

            if let (Ok(ex), Ok(nw)) = (parsed_existing, parsed_new) {
                if nw <= ex {
                    return Ok(existing.clone()); // Do not downgrade
                }
            }
        }

        registry.agents.insert(agent.id.clone(), agent.clone());
        self.save_registry(&registry)?;

        Ok(agent)
    }


    /// Uninstall an agent
    pub fn uninstall_agent(&self, agent_id: &str) -> Result<(), String> {
        let mut registry = self.load_registry();
        if registry.agents.remove(agent_id).is_some() {
            self.save_registry(&registry)?;
            Ok(())
        } else {
            Err(format!("Agent '{}' is not installed.", agent_id))
        }
    }

    /// List all installed agents
    pub fn list_installed_agents(&self) -> Vec<MarketplaceAgent> {
        let registry = self.load_registry();
        registry.agents.values().cloned().collect()
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


    fn test_client() -> MarketplaceClient {
        let temp_dir = tempfile::tempdir().unwrap();
        MarketplaceClient::with_path(Box::new(super::test_utils::MockMarketplaceProvider), temp_dir.path().join("registry.json"))
    }

    #[tokio::test]
    async fn test_marketplace_search() {
        let client = test_client();
        let results = client.search("data").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Data Analyst");
    }

    #[tokio::test]
    async fn test_marketplace_fetch() {
        let client = test_client();
        let agent = client.fetch_agent("agent-1").await.unwrap();
        assert_eq!(agent.name, "Data Analyst");

        // Test caching (should return immediately)
        let agent2 = client.fetch_agent("agent-1").await.unwrap();
        assert_eq!(agent2.id, agent.id);

        let not_found = client.fetch_agent("unknown").await;
        assert!(not_found.is_err());
    }
}

#[cfg(test)]
mod install_tests {
    use super::*;


    fn test_client() -> MarketplaceClient {
        let temp_dir = tempfile::tempdir().unwrap();
        MarketplaceClient::with_path(Box::new(super::test_utils::MockMarketplaceProvider), temp_dir.path().join("registry.json"))
    }

    #[tokio::test]
    async fn test_marketplace_install_and_uninstall() {
        let client = test_client();

        // Initial state
        assert_eq!(client.list_installed_agents().len(), 0);

        // Install agent
        let agent = client.install_agent("agent-1").await.unwrap();
        assert_eq!(agent.name, "Data Analyst");
        assert_eq!(client.list_installed_agents().len(), 1);

        // Install same agent again (should be no-op/success)
        let agent_dup = client.install_agent("agent-1").await.unwrap();
        assert_eq!(agent_dup.id, agent.id);
        assert_eq!(client.list_installed_agents().len(), 1);

        // Uninstall agent
        assert!(client.uninstall_agent("agent-1").is_ok());
        assert_eq!(client.list_installed_agents().len(), 0);

        // Uninstall non-existent agent
        assert!(client.uninstall_agent("unknown").is_err());
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;


    struct MockMarketplaceProviderVersioned;

    #[async_trait::async_trait]
    impl MarketplaceProvider for MockMarketplaceProviderVersioned {
        async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
            Ok(vec![])
        }

        async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
            if agent_id == "agent-v1" {
                Ok(MarketplaceAgent {
                    id: "agent-1".to_string(), // ID matches!
                    name: "Data Analyst".to_string(),
                    description: "Analyzes CSV files and generates charts.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "1.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-1".to_string(),
                })
            } else if agent_id == "agent-v2" {
                Ok(MarketplaceAgent {
                    id: "agent-1".to_string(), // Same ID, new version
                    name: "Data Analyst".to_string(),
                    description: "Analyzes CSV files and generates charts.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "2.0.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-1".to_string(),
                })
            } else if agent_id == "agent-vold" {
                Ok(MarketplaceAgent {
                    id: "agent-1".to_string(), // Same ID, old version
                    name: "Data Analyst".to_string(),
                    description: "Analyzes CSV files and generates charts.".to_string(),
                    author: "AutoGPT".to_string(),
                    version: "0.9.0".to_string(),
                    endpoint: "https://marketplace.example.com/agents/agent-1".to_string(),
                })
            } else {
                Err("Not found".to_string())
            }
        }
    }

    fn test_client() -> MarketplaceClient {
        let temp_dir = tempfile::tempdir().unwrap();
        MarketplaceClient::with_path(Box::new(MockMarketplaceProviderVersioned), temp_dir.path().join("registry.json"))
    }

    #[tokio::test]
    async fn test_marketplace_version_install() {
        let client = test_client();

        // Install version 1
        let agent1 = client.install_agent("agent-v1").await.unwrap();
        assert_eq!(agent1.version, "1.0.0");

        let installed = client.list_installed_agents();
        assert_eq!(installed.len(), 1);
        assert_eq!(installed[0].version, "1.0.0");

        // Try to install older version (should remain at 1.0.0)
        let agent_old = client.install_agent("agent-vold").await.unwrap();
        // It returns the existing 1.0.0 version since 0.9.0 <= 1.0.0
        assert_eq!(agent_old.version, "1.0.0");

        // Upgrade to version 2
        let agent2 = client.install_agent("agent-v2").await.unwrap();
        assert_eq!(agent2.version, "2.0.0");

        let installed2 = client.list_installed_agents();
        assert_eq!(installed2.len(), 1);
        assert_eq!(installed2[0].version, "2.0.0");
    }
}
