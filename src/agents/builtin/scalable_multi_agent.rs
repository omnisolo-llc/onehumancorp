use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::agent::{Agent, AgentRunConfig};

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
/// This module implements the abstraction for scaling an agent from a single local instance
/// to a large-scale cloud deployment.

#[derive(Debug, Clone)]
pub enum DeploymentTier {
    LocalCli,
    Cluster(usize), // Number of nodes
    Cloud(usize),   // Number of concurrent agents requested
}

#[derive(Debug, Clone)]
pub struct AgentDeploymentConfig {
    pub tier: DeploymentTier,
    pub auto_scale: bool,
    pub max_agents: usize,
}

impl Default for AgentDeploymentConfig {
    fn default() -> Self {
        Self {
            tier: DeploymentTier::LocalCli,
            auto_scale: false,
            max_agents: 1,
        }
    }
}

pub struct CloudAgentManager {
    config: AgentDeploymentConfig,
    active_agents: Mutex<HashMap<String, Arc<Agent>>>,
}

impl CloudAgentManager {
    pub fn new(config: AgentDeploymentConfig) -> Self {
        Self {
            config,
            active_agents: Mutex::new(HashMap::new()),
        }
    }

    pub async fn deploy_agent(&self, agent_id: String, agent: Arc<Agent>) -> Result<(), String> {
        let mut active = self.active_agents.lock().await;

        let limit = match self.config.tier {
            DeploymentTier::LocalCli => 1,
            DeploymentTier::Cluster(n) => n * 10,
            DeploymentTier::Cloud(n) => n,
        };

        let effective_limit = if self.config.auto_scale {
            self.config.max_agents.max(limit)
        } else {
            limit
        };

        if active.len() >= effective_limit {
            return Err(format!(
                "Deployment limit reached. Max allowed: {}",
                effective_limit
            ));
        }

        active.insert(agent_id.clone(), agent);
        info!("Successfully deployed agent {} to {:?}", agent_id, self.config.tier);
        Ok(())
    }

    pub async fn get_agent(&self, agent_id: &str) -> Option<Arc<Agent>> {
        let active = self.active_agents.lock().await;
        active.get(agent_id).cloned()
    }

    pub async fn scale_up(&mut self, additional_capacity: usize) -> Result<(), String> {
        if !self.config.auto_scale {
            return Err("Auto-scaling is disabled for this deployment config.".to_string());
        }

        match &mut self.config.tier {
            DeploymentTier::LocalCli => {
                warn!("Cannot scale up LocalCli tier directly. Converting to Cloud tier.");
                self.config.tier = DeploymentTier::Cloud(1 + additional_capacity);
            }
            DeploymentTier::Cluster(nodes) => {
                *nodes += additional_capacity;
                info!("Scaled cluster to {} nodes.", nodes);
            }
            DeploymentTier::Cloud(capacity) => {
                *capacity += additional_capacity;
                info!("Scaled cloud capacity to {}.", capacity);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};

    struct DummyLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for DummyLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("dummy"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_local_cli_limit() {
        let config = AgentDeploymentConfig {
            tier: DeploymentTier::LocalCli,
            auto_scale: false,
            max_agents: 1,
        };
        let manager = CloudAgentManager::new(config);

        let agent1 = Arc::new(Agent::new(Arc::new(DummyLlmClient), vec![]));
        let agent2 = Arc::new(Agent::new(Arc::new(DummyLlmClient), vec![]));

        assert!(manager.deploy_agent("agent-1".to_string(), agent1).await.is_ok());
        let err = manager.deploy_agent("agent-2".to_string(), agent2).await;
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), "Deployment limit reached. Max allowed: 1");
    }

    #[tokio::test]
    async fn test_cloud_scale() {
        let config = AgentDeploymentConfig {
            tier: DeploymentTier::Cloud(1000), // Simulating 1000+ agent cloud deployment
            auto_scale: true,
            max_agents: 2000,
        };
        let mut manager = CloudAgentManager::new(config);

        let agent = Arc::new(Agent::new(Arc::new(DummyLlmClient), vec![]));
        assert!(manager.deploy_agent("agent-cloud-1".to_string(), agent).await.is_ok());

        assert!(manager.scale_up(500).await.is_ok());
        if let DeploymentTier::Cloud(capacity) = manager.config.tier {
            assert_eq!(capacity, 1500);
        } else {
            panic!("Tier should be Cloud");
        }
    }
}
