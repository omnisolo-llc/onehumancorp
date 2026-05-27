use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::future::join_all;
use crate::agent::{Agent, AgentRunConfig};

/// SOTA Harness Patterns (2025-2026): Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
///
/// This module implements the "Scalable multi-agent" pattern from the Master Catalog.
/// It introduces a `CloudDeploymentManager` capable of orchestrating thousands of agents concurrently
/// using `tokio` asynchronous tasks, simulating horizontal scaling in a cloud environment.

pub struct CloudDeploymentManager {
    agents: Arc<Mutex<HashMap<String, Arc<Agent>>>>,
}

impl CloudDeploymentManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a large number of agents into the cloud deployment.
    pub async fn register_agents(&self, agent_batch: Vec<(String, Arc<Agent>)>) {
        let mut map = self.agents.lock().await;
        for (id, agent) in agent_batch {
            map.insert(id, agent);
        }
    }

    /// Broadcasts a message or task to a specific subset of agents concurrently.
    /// This simulates "fan-out" in a scalable cloud deployment.
    pub async fn broadcast_task(&self, target_ids: Vec<String>, task: &str, cfg: &AgentRunConfig) -> HashMap<String, Result<String, String>> {
        let mut futures = Vec::new();

        let agents_lock = self.agents.lock().await;
        for id in target_ids {
            if let Some(agent) = agents_lock.get(&id) {
                let agent_clone = agent.clone();
                let task_clone = task.to_string();
                let cfg_clone = cfg.clone();
                let id_clone = id.clone();

                let fut = async move {
                    let mut on_event = |_| {};
                    let res = agent_clone.run(&cfg_clone, &task_clone, &mut on_event).await;
                    match res {
                        Ok(output) => (id_clone, Ok(output)),
                        Err(e) => (id_clone, Err(e.to_string())),
                    }
                };
                futures.push(fut);
            }
        }
        drop(agents_lock);

        let results = join_all(futures).await;
        let mut output_map = HashMap::new();
        for (id, res) in results {
            output_map.insert(id, res);
        }

        output_map
    }

    /// Total number of active deployments.
    pub async fn active_deployments(&self) -> usize {
        let map = self.agents.lock().await;
        map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use crate::llm::LlmClient;
    use std::time::Instant;

    struct CloudMockLlm;

    #[async_trait::async_trait]
    impl LlmClient for CloudMockLlm {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            // Simulate minimal cloud latency
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            let last_user_msg = req.messages.last().map(|m| m.content.clone()).unwrap_or_default();
            Ok(ChatResponse {
                message: Message::assistant(format!("Cloud Agent Received: {}", last_user_msg)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some(String::from("uuid")),
            })
        }
    }

    #[tokio::test]
    async fn test_scalable_multi_agent_deployment_1000_plus() {
        let manager = CloudDeploymentManager::new();
        let llm = Arc::new(CloudMockLlm);

        // Simulate 1000+ agent cloud deployments
        let num_agents = 1050;
        let mut batch = Vec::new();
        let mut target_ids = Vec::new();

        for i in 0..num_agents {
            let id = format!("cloud-agent-{}", i);
            let agent = Arc::new(Agent::new(llm.clone(), vec![]));
            batch.push((id.clone(), agent));
            target_ids.push(id);
        }

        manager.register_agents(batch).await;
        assert_eq!(manager.active_deployments().await, num_agents);

        let cfg = AgentRunConfig::default();
        let start_time = Instant::now();

        // Broadcast a task to all 1000+ agents concurrently
        let results = manager.broadcast_task(target_ids.clone(), "Analyze metric 42", &cfg).await;

        let elapsed = start_time.elapsed();

        assert_eq!(results.len(), num_agents);

        // Ensure that tokio concurrency successfully handled the fan-out
        // If it was sequential, it would take 1050 * 5ms = 5.25 seconds.
        // In a true async scalable setup, it should be much less.
        assert!(elapsed.as_secs_f32() < 2.0, "Execution took too long, suggesting lack of true concurrency: {:?}", elapsed);

        // Verify the output of a random agent
        let sample_output = results.get("cloud-agent-500").unwrap().as_ref().unwrap();
        assert!(sample_output.contains("Cloud Agent Received: Analyze metric 42"));
    }
}
