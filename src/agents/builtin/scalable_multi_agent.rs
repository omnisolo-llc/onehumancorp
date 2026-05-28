use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
pub struct AgentCloudDeployment {
    pub agents: Mutex<HashMap<String, Arc<crate::agent::Agent>>>,
}

impl AgentCloudDeployment {
    pub fn new() -> Self {
        Self {
            agents: Mutex::new(HashMap::new()),
        }
    }

    pub async fn deploy_agent(&self, id: String, agent: Arc<crate::agent::Agent>) {
        let mut map = self.agents.lock().await;
        map.insert(id, agent);
    }

    pub async fn get_agent(&self, id: &str) -> Option<Arc<crate::agent::Agent>> {
        let map = self.agents.lock().await;
        map.get(id).cloned()
    }

    pub async fn remove_agent(&self, id: &str) -> Option<Arc<crate::agent::Agent>> {
        let mut map = self.agents.lock().await;
        map.remove(id)
    }

    pub async fn scale_to(&self, base_id: &str, count: usize, agent_template: Arc<crate::agent::Agent>) {
        let mut map = self.agents.lock().await;
        for i in 0..count {
            let id = format!("{}-{}", base_id, i);
            map.insert(id, agent_template.clone());
        }
    }

    pub async fn active_agent_count(&self) -> usize {
        let map = self.agents.lock().await;
        map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Agent;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};

    struct DummyLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for DummyLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("dummy"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_agent_cloud_deployment() {
        let deployment = AgentCloudDeployment::new();
        let agent = Arc::new(Agent::new(Arc::new(DummyLlmClient), vec![]));

        // Test single deploy
        deployment.deploy_agent("agent-1".to_string(), agent.clone()).await;
        assert_eq!(deployment.active_agent_count().await, 1);

        let retrieved = deployment.get_agent("agent-1").await;
        assert!(retrieved.is_some());

        // Test scaling to 1000+ agents
        deployment.scale_to("cloud-worker", 1500, agent.clone()).await;
        assert_eq!(deployment.active_agent_count().await, 1501);

        let worker_500 = deployment.get_agent("cloud-worker-500").await;
        assert!(worker_500.is_some());

        // Test removal
        deployment.remove_agent("agent-1").await;
        assert_eq!(deployment.active_agent_count().await, 1500);
    }
}
