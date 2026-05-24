use crate::agent::{Agent, AgentRunConfig};
use std::sync::Arc;
use std::collections::HashMap;

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus

#[derive(Debug, Clone)]
pub enum Topology {
    /// One leader, multiple workers. Leader delegates.
    Hierarchical {
        leader: String,
        workers: Vec<String>,
    },
    /// All agents can talk to all agents.
    Mesh {
        agents: Vec<String>,
    },
    /// Agents vote on the result.
    AdaptiveConsensus {
        agents: Vec<String>,
        consensus_threshold: usize,
    },
}

pub struct RufloAgent {
    pub name: String,
    pub agent: Arc<Agent>,
    pub run_config: AgentRunConfig,
}

pub struct RufloSwarmManager {
    pub topology: Topology,
    pub agents: HashMap<String, RufloAgent>,
}

impl RufloSwarmManager {
    pub fn new(topology: Topology, agents_list: Vec<RufloAgent>) -> Self {
        let mut agents = HashMap::new();
        for a in agents_list {
            agents.insert(a.name.clone(), a);
        }
        Self { topology, agents }
    }

    pub async fn execute(&self, task: &str) -> Result<String, String> {
        match &self.topology {
            Topology::Hierarchical { leader, workers } => {
                self.execute_hierarchical(leader, workers, task).await
            }
            Topology::Mesh { agents } => {
                self.execute_mesh(agents, task).await
            }
            Topology::AdaptiveConsensus { agents, consensus_threshold } => {
                self.execute_adaptive_consensus(agents, *consensus_threshold, task).await
            }
        }
    }

    async fn execute_hierarchical(&self, leader_name: &str, workers: &[String], task: &str) -> Result<String, String> {
        let leader = self.agents.get(leader_name).ok_or("Leader not found")?;

        // 1. Leader processes task and creates subtasks (simulated)
        let mut on_event = |_e| {};
        let initial_leader_prompt = format!("You are the leader. Delegate this task to workers: {}. Task: {}", workers.join(", "), task);

        let delegation_plan = leader.agent.run(&leader.run_config, &initial_leader_prompt, &mut on_event)
            .await.map_err(|e| e.to_string())?;

        // 2. Workers execute based on delegation
        let mut worker_results = Vec::new();
        for w_name in workers {
            if let Some(worker) = self.agents.get(w_name) {
                let worker_prompt = format!("Task from leader: {}. Plan: {}", task, delegation_plan);
                let res = worker.agent.run(&worker.run_config, &worker_prompt, &mut on_event)
                    .await.unwrap_or_else(|e| format!("Worker error: {}", e));
                worker_results.push(format!("{}: {}", w_name, res));
            }
        }

        // 3. Leader synthesizes
        let synthesis_prompt = format!("Synthesize these worker results into a final output: {}", worker_results.join("\n"));
        let final_result = leader.agent.run(&leader.run_config, &synthesis_prompt, &mut on_event)
            .await.map_err(|e| e.to_string())?;

        Ok(final_result)
    }

    async fn execute_mesh(&self, agent_names: &[String], task: &str) -> Result<String, String> {
        // Mesh: All agents execute the task and share their results with the next agent in a round-robin style
        let mut shared_context = task.to_string();
        let mut final_result = String::new();

        for name in agent_names {
            if let Some(agent) = self.agents.get(name) {
                let mut on_event = |_e| {};
                let prompt = format!("Current context: {}. Please contribute to the task.", shared_context);
                let res = agent.agent.run(&agent.run_config, &prompt, &mut on_event)
                    .await.unwrap_or_else(|e| format!("Error: {}", e));

                shared_context.push_str(&format!("\n[{} contributed]: {}", name, res));
                final_result = res;
            }
        }

        Ok(final_result)
    }

    async fn execute_adaptive_consensus(&self, agent_names: &[String], threshold: usize, task: &str) -> Result<String, String> {
        let mut results = HashMap::new();
        let mut on_event = |_e| {};

        for name in agent_names {
            if let Some(agent) = self.agents.get(name) {
                let prompt = format!("Task: {}. Provide a concise answer.", task);
                let res = agent.agent.run(&agent.run_config, &prompt, &mut on_event)
                    .await.unwrap_or_else(|e| format!("Error: {}", e));

                // Simulated exact string matching for consensus
                let count = results.entry(res.clone()).or_insert(0);
                *count += 1;

                if *count >= threshold {
                    return Ok(format!("Consensus reached (threshold {}): {}", threshold, res));
                }
            }
        }

        // If no consensus reached, just return all
        let mut fallback = String::from("No consensus reached. Results:");
        for (res, count) in results {
            fallback.push_str(&format!("\n- {} ({} votes)", res, count));
        }

        Ok(fallback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::llm::LlmClient;

    struct MockLlmClient {
        response: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.response),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    fn create_mock_ruflo_agent(name: &str, resp: &str) -> RufloAgent {
        let client = Arc::new(MockLlmClient { response: resp.to_string() });
        let agent = Arc::new(Agent::new(client, vec![]));
        RufloAgent {
            name: name.to_string(),
            agent,
            run_config: AgentRunConfig::default(),
        }
    }

    #[tokio::test]
    async fn test_hierarchical_topology() {
        let leader = create_mock_ruflo_agent("Leader", "Leader synthesis");
        let worker1 = create_mock_ruflo_agent("Worker1", "Worker 1 data");
        let worker2 = create_mock_ruflo_agent("Worker2", "Worker 2 data");

        let topology = Topology::Hierarchical {
            leader: "Leader".to_string(),
            workers: vec!["Worker1".to_string(), "Worker2".to_string()],
        };

        let manager = RufloSwarmManager::new(topology, vec![leader, worker1, worker2]);
        let res = manager.execute("Solve a problem").await.unwrap();
        assert_eq!(res, "Leader synthesis");
    }

    #[tokio::test]
    async fn test_mesh_topology() {
        let agent1 = create_mock_ruflo_agent("A1", "Result from A1");
        let agent2 = create_mock_ruflo_agent("A2", "Final mesh result");

        let topology = Topology::Mesh {
            agents: vec!["A1".to_string(), "A2".to_string()],
        };

        let manager = RufloSwarmManager::new(topology, vec![agent1, agent2]);
        let res = manager.execute("Process data").await.unwrap();
        assert_eq!(res, "Final mesh result");
    }

    #[tokio::test]
    async fn test_adaptive_consensus() {
        let agent1 = create_mock_ruflo_agent("Voter1", "42");
        let agent2 = create_mock_ruflo_agent("Voter2", "42");
        let agent3 = create_mock_ruflo_agent("Voter3", "43");

        let topology = Topology::AdaptiveConsensus {
            agents: vec!["Voter1".to_string(), "Voter2".to_string(), "Voter3".to_string()],
            consensus_threshold: 2,
        };

        let manager = RufloSwarmManager::new(topology, vec![agent1, agent2, agent3]);
        let res = manager.execute("What is 6x7?").await.unwrap();
        assert!(res.contains("Consensus reached"));
        assert!(res.contains("42"));
    }
}
