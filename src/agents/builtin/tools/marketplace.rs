use serde::{Deserialize, Serialize};

/// Master Catalog C.21. AutoGPT Unique Harness Innovations: Agent Marketplace
/// SOTA Harness Pattern: AutoGPT Agent Marketplace API distribution
/// Pre-built agent distribution.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceAgent {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub endpoint: String, // Where to fetch the agent payload/definition
}

impl MarketplaceAgent {
    pub fn validate_publication(&self) -> Result<(), String> {
        if [&self.name, &self.author, &self.version, &self.endpoint]
            .iter()
            .any(|field| field.trim().is_empty())
        {
            return Err("A complete marketplace descriptor is required".into());
        }
        let endpoint =
            reqwest::Url::parse(&self.endpoint).map_err(|_| "Invalid agent definition endpoint")?;
        if !matches!(endpoint.scheme(), "http" | "https")
            || endpoint.host_str().is_none()
            || !endpoint.username().is_empty()
            || endpoint.password().is_some()
        {
            return Err("Invalid agent definition endpoint".into());
        }
        Ok(())
    }
    fn validate_receipt(&self) -> Result<(), String> {
        self.validate_publication()?;
        if self.id.trim().is_empty() {
            return Err("Marketplace receipt has no agent ID".into());
        }
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait MarketplaceProvider: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String>;
    async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String>;
    async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String>;
}

struct UnavailableMarketplaceProvider;
#[async_trait::async_trait]
impl MarketplaceProvider for UnavailableMarketplaceProvider {
    async fn search(&self, _: &str) -> Result<Vec<MarketplaceAgent>, String> {
        Err("Marketplace registry is unavailable or not configured".into())
    }
    async fn fetch_agent(&self, _: &str) -> Result<MarketplaceAgent, String> {
        Err("Marketplace registry is unavailable or not configured".into())
    }
    async fn publish_agent(&self, _: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
        Err("Marketplace registry is unavailable or not configured".into())
    }
}

/// Configuration selects the real registry or an explicit unavailable provider.
/// Test catalogs are supplied only through deliberate dependency injection.
pub fn configured_provider(raw: &str) -> Box<dyn MarketplaceProvider> {
    let valid = reqwest::Url::parse(raw).ok().filter(|url| {
        matches!(url.scheme(), "http" | "https")
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
            && url.host_str().is_some_and(|host| {
                !["example.com", "example.org", "example.net"]
                    .iter()
                    .any(|reserved| host == *reserved || host.ends_with(&format!(".{reserved}")))
            })
    });
    match valid {
        Some(url) => Box::new(HttpMarketplaceProvider::new(
            url.as_str().trim_end_matches('/'),
        )),
        None => Box::new(UnavailableMarketplaceProvider),
    }
}

pub struct HttpMarketplaceProvider {
    pub registry_url: String,
    pub http_client: reqwest::Client,
}

impl HttpMarketplaceProvider {
    pub fn new(registry_url: &str) -> Self {
        Self {
            registry_url: registry_url.to_string(),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(2))
                .connect_timeout(std::time::Duration::from_secs(2))
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("static marketplace HTTP client configuration is valid"),
        }
    }
}

#[async_trait::async_trait]
impl MarketplaceProvider for HttpMarketplaceProvider {
    async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
        let url = format!("{}/search", self.registry_url);
        let response = self
            .http_client
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await
            .map_err(|e| format!("Failed to search marketplace: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Marketplace returned status: {}",
                response.status()
            ));
        }

        let agents: Vec<MarketplaceAgent> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(agents)
    }

    async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
        let mut url = reqwest::Url::parse(&format!("{}/agents/", self.registry_url))
            .map_err(|_| "Invalid marketplace URL")?;
        url.path_segments_mut()
            .map_err(|_| "Invalid marketplace URL")?
            .pop_if_empty()
            .push(agent_id);
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch agent: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Marketplace returned status: {}",
                response.status()
            ));
        }

        let agent: MarketplaceAgent = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(agent)
    }

    async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
        let url = format!("{}/agents", self.registry_url);
        let response = self
            .http_client
            .post(&url)
            .json(&agent)
            .send()
            .await
            .map_err(|e| format!("Failed to publish agent: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Marketplace returned status: {}",
                response.status()
            ));
        }

        let published_agent: MarketplaceAgent = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(published_agent)
    }
}

pub struct MarketplaceClient {
    pub provider: Box<dyn MarketplaceProvider>,
}

impl MarketplaceClient {
    pub fn new(provider: Box<dyn MarketplaceProvider>) -> Self {
        Self { provider }
    }

    /// Read the authoritative registry on every request, including after publication.
    pub async fn search(&self, query: &str) -> Result<Vec<MarketplaceAgent>, String> {
        let results = self.provider.search(query).await?;
        for agent in &results {
            agent.validate_receipt()?;
        }
        Ok(results)
    }

    pub async fn fetch_agent(&self, agent_id: &str) -> Result<MarketplaceAgent, String> {
        if agent_id.trim().is_empty() {
            return Err("An agent ID is required".into());
        }
        let agent = self.provider.fetch_agent(agent_id).await?;
        agent.validate_receipt()?;
        if agent.id != agent_id {
            return Err("Marketplace returned a different agent".into());
        }
        Ok(agent)
    }

    pub async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
        agent.validate_publication()?;
        let published = self.provider.publish_agent(agent.clone()).await
            .map_err(|error| format!("Publication could not be confirmed; reconcile with the registry before retrying: {error}"))?;
        published.validate_receipt().map_err(|error| format!("Publication could not be confirmed; reconcile with the registry before retrying: {error}"))?;
        if (!agent.id.is_empty() && agent.id != published.id)
            || agent.name != published.name
            || agent.description != published.description
            || agent.author != published.author
            || agent.version != published.version
            || agent.endpoint != published.endpoint
        {
            return Err("Publication acknowledgement differs from the reviewed descriptor; reconcile with the registry before retrying".into());
        }
        Ok(published)
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
                },
            ];

            if !query.is_empty() {
                let q_lower = query.to_lowercase();
                results.retain(|a| {
                    a.name.to_lowercase().contains(&q_lower)
                        || a.description.to_lowercase().contains(&q_lower)
                });
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

        async fn publish_agent(
            &self,
            mut agent: MarketplaceAgent,
        ) -> Result<MarketplaceAgent, String> {
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
    use super::test_utils::MockMarketplaceProvider;
    use super::*;

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

#[cfg(test)]
mod authority_tests {
    use super::*;
    use std::sync::{
        Arc, RwLock,
        atomic::{AtomicUsize, Ordering},
    };

    fn descriptor() -> MarketplaceAgent {
        MarketplaceAgent {
            id: "saved-agent".into(),
            name: "Reviewed name".into(),
            description: "Reviewed description".into(),
            author: "Owner".into(),
            version: "1.0.0".into(),
            endpoint: "https://registry.example.test/definitions/saved-agent".into(),
        }
    }
    struct AuthoritativeProvider {
        current: Arc<RwLock<Option<MarketplaceAgent>>>,
        writes: Arc<AtomicUsize>,
        publication_reply: Option<MarketplaceAgent>,
    }
    #[async_trait::async_trait]
    impl MarketplaceProvider for AuthoritativeProvider {
        async fn search(&self, _: &str) -> Result<Vec<MarketplaceAgent>, String> {
            Ok(self.current.read().unwrap().clone().into_iter().collect())
        }
        async fn fetch_agent(&self, _: &str) -> Result<MarketplaceAgent, String> {
            self.current
                .read()
                .unwrap()
                .clone()
                .ok_or("Agent removed".into())
        }
        async fn publish_agent(&self, agent: MarketplaceAgent) -> Result<MarketplaceAgent, String> {
            self.writes.fetch_add(1, Ordering::SeqCst);
            let reply = self.publication_reply.clone().unwrap_or(agent);
            *self.current.write().unwrap() = Some(reply.clone());
            Ok(reply)
        }
    }
    fn client(
        current: &Arc<RwLock<Option<MarketplaceAgent>>>,
        writes: &Arc<AtomicUsize>,
        publication_reply: Option<MarketplaceAgent>,
    ) -> MarketplaceClient {
        MarketplaceClient::new(Box::new(AuthoritativeProvider {
            current: current.clone(),
            writes: writes.clone(),
            publication_reply,
        }))
    }
    #[tokio::test]
    async fn provider_deletion_stays_authoritative_before_and_after_client_restart() {
        let current = Arc::new(RwLock::new(None));
        let writes = Arc::new(AtomicUsize::new(0));
        let existing = client(&current, &writes, None);
        existing.publish_agent(descriptor()).await.unwrap();
        assert_eq!(
            existing.fetch_agent("saved-agent").await.unwrap().name,
            "Reviewed name"
        );
        *current.write().unwrap() = None;
        assert!(
            existing.search("").await.unwrap().is_empty(),
            "cached publication must not reappear after provider deletion"
        );
        assert!(existing.fetch_agent("saved-agent").await.is_err());
        let restarted = client(&current, &writes, None);
        assert!(restarted.search("").await.unwrap().is_empty());
        assert!(restarted.fetch_agent("saved-agent").await.is_err());
        assert_eq!(writes.load(Ordering::SeqCst), 1);
    }
    #[tokio::test]
    async fn publication_rejects_incomplete_descriptor_before_provider_effects() {
        let current = Arc::new(RwLock::new(None));
        let writes = Arc::new(AtomicUsize::new(0));
        let existing = client(&current, &writes, None);
        let mut input = descriptor();
        input.endpoint.clear();
        assert!(existing.publish_agent(input).await.is_err());
        assert_eq!(writes.load(Ordering::SeqCst), 0);
        assert!(current.read().unwrap().is_none());
    }
    #[tokio::test]
    async fn publication_cannot_acknowledge_changed_or_missing_descriptor_fields() {
        for field in ["id", "name", "description", "author", "version", "endpoint"] {
            let current = Arc::new(RwLock::new(None));
            let writes = Arc::new(AtomicUsize::new(0));
            let mut reply = descriptor();
            match field {
                "id" => reply.id.clear(),
                "name" => reply.name = "Unreviewed".into(),
                "description" => reply.description = "Unreviewed".into(),
                "author" => reply.author = "Unreviewed".into(),
                "version" => reply.version = "2.0.0".into(),
                _ => reply.endpoint = "https://other.example.test/definition".into(),
            }
            assert!(
                client(&current, &writes, Some(reply))
                    .publish_agent(descriptor())
                    .await
                    .is_err(),
                "changed {field} must not be acknowledged"
            );
            assert_eq!(writes.load(Ordering::SeqCst), 1);
        }
    }
    #[tokio::test]
    async fn provider_fetch_cannot_return_a_different_agent_for_the_requested_id() {
        let current = Arc::new(RwLock::new(Some(descriptor())));
        let writes = Arc::new(AtomicUsize::new(0));
        assert!(
            client(&current, &writes, None)
                .fetch_agent("another-agent")
                .await
                .is_err()
        );
    }
}

#[cfg(test)]
mod http_contract_tests {
    use super::*;
    use axum::{
        Json, Router,
        extract::{Path, State},
        routing::{get, post},
    };
    use std::sync::{
        Arc, RwLock,
        atomic::{AtomicUsize, Ordering},
    };
    #[derive(Clone)]
    struct Registry {
        saved: Arc<RwLock<Option<MarketplaceAgent>>>,
        writes: Arc<AtomicUsize>,
        contradictory: bool,
    }
    async fn search(State(state): State<Registry>) -> Json<Vec<MarketplaceAgent>> {
        Json(state.saved.read().unwrap().clone().into_iter().collect())
    }
    async fn fetch(
        State(state): State<Registry>,
        Path(id): Path<String>,
    ) -> axum::response::Response {
        use axum::response::IntoResponse;
        match state
            .saved
            .read()
            .unwrap()
            .clone()
            .filter(|agent| agent.id == id)
        {
            Some(agent) => Json(agent).into_response(),
            None => axum::http::StatusCode::NOT_FOUND.into_response(),
        }
    }
    async fn publish(
        State(state): State<Registry>,
        Json(mut agent): Json<MarketplaceAgent>,
    ) -> Json<serde_json::Value> {
        state.writes.fetch_add(1, Ordering::SeqCst);
        if agent.id.is_empty() {
            agent.id = "assigned-by-registry".into();
        }
        *state.saved.write().unwrap() = Some(agent.clone());
        let mut value = serde_json::to_value(agent).unwrap();
        if state.contradictory {
            value["success"] = serde_json::json!(false);
        }
        Json(value)
    }
    async fn registry(contradictory: bool) -> (String, Registry, tokio::task::JoinHandle<()>) {
        let state = Registry {
            saved: Arc::new(RwLock::new(None)),
            writes: Arc::new(AtomicUsize::new(0)),
            contradictory,
        };
        let router = Router::new()
            .route("/search", get(search))
            .route("/agents", post(publish))
            .route("/agents/{id}", get(fetch))
            .with_state(state.clone());
        let socket = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", socket.local_addr().unwrap());
        let task = tokio::spawn(async move {
            axum::serve(socket, router).await.unwrap();
        });
        (url, state, task)
    }
    fn input() -> MarketplaceAgent {
        MarketplaceAgent {
            id: String::new(),
            name: "Registry-backed agent".into(),
            description: "Full descriptor".into(),
            author: "Owner".into(),
            version: "2.0".into(),
            endpoint: "https://registry.example.test/definition".into(),
        }
    }
    #[tokio::test]
    async fn real_http_receipt_and_reads_survive_client_restart_and_respect_provider_deletion() {
        let (url, state, task) = registry(false).await;
        let client = MarketplaceClient::new(configured_provider(&url));
        let saved = client.publish_agent(input()).await.unwrap();
        assert_eq!(saved.id, "assigned-by-registry");
        assert_eq!(
            client.fetch_agent(&saved.id).await.unwrap().name,
            saved.name
        );
        let restarted = MarketplaceClient::new(Box::new(HttpMarketplaceProvider::new(&url)));
        assert_eq!(restarted.search("").await.unwrap().len(), 1);
        assert_eq!(
            restarted.fetch_agent(&saved.id).await.unwrap().endpoint,
            saved.endpoint
        );
        *state.saved.write().unwrap() = None;
        assert!(client.search("").await.unwrap().is_empty());
        assert!(client.fetch_agent(&saved.id).await.is_err());
        assert!(restarted.search("").await.unwrap().is_empty());
        assert_eq!(state.writes.load(Ordering::SeqCst), 1);
        task.abort();
    }
    #[tokio::test]
    async fn a_complete_descriptor_with_explicit_failure_is_never_a_publication_acknowledgement() {
        let (url, state, task) = registry(true).await;
        let client = MarketplaceClient::new(Box::new(HttpMarketplaceProvider::new(&url)));
        assert!(client.publish_agent(input()).await.is_err());
        assert_eq!(state.writes.load(Ordering::SeqCst), 1);
        task.abort();
    }
    #[tokio::test]
    async fn unknown_publication_body_never_retries_the_post() {
        let writes = Arc::new(AtomicUsize::new(0));
        let observed = writes.clone();
        let router = Router::new().route(
            "/agents",
            post(move || {
                let observed = observed.clone();
                async move {
                    observed.fetch_add(1, Ordering::SeqCst);
                    "not a receipt"
                }
            }),
        );
        let socket = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", socket.local_addr().unwrap());
        let task = tokio::spawn(async move {
            axum::serve(socket, router).await.unwrap();
        });
        let client = MarketplaceClient::new(Box::new(HttpMarketplaceProvider::new(&url)));
        assert!(client.publish_agent(input()).await.is_err());
        assert_eq!(writes.load(Ordering::SeqCst), 1);
        task.abort();
    }
}
