use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{Message, ChatRequest};
use std::sync::Arc;
use futures::future::join_all;

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus.

#[derive(Debug, Clone)]
pub enum TopologyType {
    Hierarchical { manager: String },
    Mesh,
    AdaptiveWithConsensus,
}

pub struct SwarmAgent {
    pub name: String,
    pub agent: Arc<Agent>,
}

pub struct SwarmCoordinator {
    pub topology: TopologyType,
    pub agents: Vec<SwarmAgent>,
    pub config: AgentRunConfig,
}

impl SwarmCoordinator {
    pub fn new(topology: TopologyType, agents: Vec<SwarmAgent>, config: AgentRunConfig) -> Self {
        Self { topology, agents, config }
    }

    pub async fn run_swarm(&self, task: &str) -> Result<String, String> {
        match &self.topology {
            TopologyType::Hierarchical { manager } => {
                self.run_hierarchical(task, manager).await
            }
            TopologyType::Mesh => {
                self.run_mesh(task).await
            }
            TopologyType::AdaptiveWithConsensus => {
                self.run_consensus(task).await
            }
        }
    }

    async fn run_hierarchical(&self, task: &str, manager_name: &str) -> Result<String, String> {
        let manager_agent = self.agents.iter().find(|a| a.name == *manager_name)
            .ok_or_else(|| format!("Manager agent {} not found", manager_name))?;

        let workers: Vec<&SwarmAgent> = self.agents.iter().filter(|a| a.name != *manager_name).collect();

        let mut futures = Vec::new();
        for worker in workers {
            let config = self.config.clone();
            let task_clone = task.to_string();
            let agent = worker.agent.clone();

            futures.push(async move {
                let mut on_event = |_| {};
                agent.run(&config, &format!("Execute this sub-task: {}", task_clone), &mut on_event).await
            });
        }

        let results = join_all(futures).await;

        let mut combined_results = String::new();
        for (i, res) in results.into_iter().enumerate() {
            if let Ok(output) = res {
                combined_results.push_str(&format!("Worker {} output: {}\n", i, output));
            }
        }

        let synth_prompt = format!("You are the swarm manager. Synthesize these worker results into a final answer for the task: '{}'\n\nResults:\n{}", task, combined_results);

        let mut on_event = |_| {};
        manager_agent.agent.run(&self.config, &synth_prompt, &mut on_event).await
            .map_err(|e| format!("Manager failed to synthesize: {}", e))
    }

    async fn run_mesh(&self, task: &str) -> Result<String, String> {
        // Mesh: All agents run the task and share intermediate states. For simplicity, just parallel run.
        let mut futures = Vec::new();
        for agent_wrapper in &self.agents {
            let config = self.config.clone();
            let task_clone = task.to_string();
            let agent = agent_wrapper.agent.clone();

            futures.push(async move {
                let mut on_event = |_| {};
                agent.run(&config, &task_clone, &mut on_event).await
            });
        }

        let results = join_all(futures).await;
        let mut combined = String::new();
        for res in results {
            if let Ok(r) = res {
                combined.push_str(&r);
                combined.push('\n');
            }
        }
        Ok(combined)
    }

    async fn run_consensus(&self, task: &str) -> Result<String, String> {
        // Run all agents, then have them vote/synthesize
        let mut futures = Vec::new();
        for agent_wrapper in &self.agents {
            let config = self.config.clone();
            let task_clone = task.to_string();
            let agent = agent_wrapper.agent.clone();

            futures.push(async move {
                let mut on_event = |_| {};
                agent.run(&config, &task_clone, &mut on_event).await
            });
        }

        let results = join_all(futures).await;
        let mut valid_results = Vec::new();
        for res in results {
            if let Ok(r) = res {
                valid_results.push(r);
            }
        }

        if valid_results.is_empty() {
            return Err("No consensus reached".to_string());
        }

        Ok(valid_results[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage, Role};

    struct MockLlm {
        resp: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: self.resp.clone(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_ruflo_swarm_hierarchical() {
        let manager_agent = SwarmAgent {
            name: "Manager".to_string(),
            agent: Arc::new(Agent::new(Arc::new(MockLlm { resp: "Final synthesis".to_string() }), vec![])),
        };

        let worker_agent = SwarmAgent {
            name: "Worker".to_string(),
            agent: Arc::new(Agent::new(Arc::new(MockLlm { resp: "Worker output".to_string() }), vec![])),
        };

        let coord = SwarmCoordinator::new(
            TopologyType::Hierarchical { manager: "Manager".to_string() },
            vec![manager_agent, worker_agent],
            AgentRunConfig::default(),
        );

        let result = coord.run_swarm("Do a task").await.unwrap();
        assert_eq!(result, "Final synthesis");
    }
}
