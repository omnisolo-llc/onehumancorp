use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agent::{Agent, AgentRunConfig};

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus

#[derive(Clone)]
pub struct SwarmAgent {
    pub name: String,
    pub description: String,
    pub agent: Arc<Agent>,
    pub run_config: AgentRunConfig,
}

#[derive(Clone)]
pub enum Topology {
    /// Lead agent delegates to subagents and synthesizes the result
    Hierarchical { lead: SwarmAgent, workers: Vec<SwarmAgent> },
    /// All agents converse in a shared pool
    Mesh { agents: Vec<SwarmAgent>, max_rounds: usize },
    /// Agents vote on the answer; requires consensus
    AdaptiveConsensus { agents: Vec<SwarmAgent>, consensus_threshold: usize, judge: SwarmAgent },
}

pub struct SwarmOrchestrator {
    pub topology: Topology,
    pub transcript: Arc<RwLock<Vec<Message>>>,
}

impl SwarmOrchestrator {
    pub fn new(topology: Topology) -> Self {
        Self {
            topology,
            transcript: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn run(&self, task: &str) -> Result<String, String> {
        self.transcript.write().await.push(Message::user(format!("Task: {}", task)));

        match &self.topology {
            Topology::Hierarchical { lead, workers } => {
                self.run_hierarchical(task, lead, workers).await
            }
            Topology::Mesh { agents, max_rounds } => {
                self.run_mesh(task, agents, *max_rounds).await
            }
            Topology::AdaptiveConsensus { agents, consensus_threshold, judge } => {
                self.run_consensus(task, agents, *consensus_threshold, judge).await
            }
        }
    }

    async fn run_hierarchical(&self, task: &str, lead: &SwarmAgent, workers: &[SwarmAgent]) -> Result<String, String> {
        let mut worker_results = Vec::new();

        tracing::info!("Hierarchical: Lead {} delegating to {} workers", lead.name, workers.len());

        let mut subtask_futures = Vec::new();
        for worker in workers {
            let worker_cfg = worker.run_config.clone();
            let worker_agent = worker.agent.clone();
            let w_name = worker.name.clone();
            let w_desc = worker.description.clone();
            let lead_agent = lead.agent.clone();
            let lead_cfg = lead.run_config.clone();
            let task_clone = task.to_string();

            subtask_futures.push(async move {
                // 1. Lead analyzes task and creates subtasks for each worker
                let delegation_prompt = format!(
                    "Task: {}\n\nYou are delegating a subtask to the worker '{}' (Description: {}).\nWrite the specific instructions for this worker to complete their part of the task.",
                    task_clone, w_name, w_desc
                );

                let mut on_event = |_| {};
                let subtask = lead_agent.run(&lead_cfg, &delegation_prompt, &mut on_event).await.unwrap_or_else(|e| format!("Error generating subtask: {}", e));

                // 2. Worker executes
                let worker_prompt = format!("As a {}, complete this subtask:\n{}", w_name, subtask);
                let mut on_event2 = |_| {};
                let result = worker_agent.run(&worker_cfg, &worker_prompt, &mut on_event2).await.unwrap_or_else(|e| format!("Error executing subtask: {}", e));
                format!("{}: {}", w_name, result)
            });
        }

        let results = futures::future::join_all(subtask_futures).await;
        for res in &results {
            worker_results.push(res.clone());
            self.transcript.write().await.push(Message::assistant(res.clone()));
        }

        // 3. Lead synthesizes
        let synthesis_prompt = format!(
            "Task: {}\n\nWorker Results:\n{}\n\nSynthesize these results into a final cohesive answer.",
            task, worker_results.join("\n\n")
        );

        let mut on_event = |_| {};
        let final_result = lead.agent.run(&lead.run_config, &synthesis_prompt, &mut on_event)
            .await
            .map_err(|e| e.to_string())?;

        self.transcript.write().await.push(Message::assistant(format!("{}: {}", lead.name, final_result)));

        Ok(final_result)
    }

    async fn run_mesh(&self, _task: &str, agents: &[SwarmAgent], max_rounds: usize) -> Result<String, String> {
        tracing::info!("Mesh: {} agents conversing for {} rounds", agents.len(), max_rounds);

        let mut final_answer = String::new();

        for _round in 0..max_rounds {
            for agent in agents {
                let current_transcript = self.transcript.read().await.clone();

                let mut context = format!("You are {}. Current conversation:\n", agent.name);
                for msg in &current_transcript {
                    context.push_str(&format!("{}\n", msg.content));
                }
                context.push_str("\nProvide your next contribution. If the task is resolved, say TERMINATE.");

                let mut on_event = |_| {};
                let result = agent.agent.run(&agent.run_config, &context, &mut on_event)
                    .await
                    .map_err(|e| e.to_string())?;

                let formatted = format!("{}: {}", agent.name, result);
                self.transcript.write().await.push(Message::assistant(formatted.clone()));
                final_answer = result.clone();

                if result.contains("TERMINATE") {
                    return Ok(final_answer);
                }
            }
        }

        Ok(final_answer)
    }

    async fn run_consensus(&self, task: &str, agents: &[SwarmAgent], threshold: usize, judge: &SwarmAgent) -> Result<String, String> {
        tracing::info!("Consensus: {} agents, threshold {}", agents.len(), threshold);

        let mut futures = Vec::new();
        for agent in agents {
            let cfg = agent.run_config.clone();
            let ag = agent.agent.clone();
            let name = agent.name.clone();
            let task_clone = format!("Task: {}. Provide your proposed solution clearly.", task);

            futures.push(async move {
                let mut on_event = |_| {};
                let result = ag.run(&cfg, &task_clone, &mut on_event).await.unwrap_or_else(|e| format!("Error: {}", e));
                (name, result)
            });
        }

        let proposals = futures::future::join_all(futures).await;

        for (name, prop) in &proposals {
             self.transcript.write().await.push(Message::assistant(format!("{}: {}", name, prop)));
        }

        // Judge evaluates consensus
        let mut consensus_groups: Vec<Vec<&str>> = Vec::new();

        for (_name, prop) in &proposals {
            let mut added = false;
            for group in &mut consensus_groups {
                let rep = group[0];
                let judge_prompt = format!(
                    "Are these two proposals semantically identical in their core conclusion?\n\nProposal 1: {}\n\nProposal 2: {}\n\nAnswer with EXACTLY 'YES' or 'NO'.",
                    rep, prop
                );

                let mut on_event = |_| {};
                let eval = judge.agent.run(&judge.run_config, &judge_prompt, &mut on_event).await.unwrap_or_default();

                if eval.trim() == "YES" {
                    group.push(prop);
                    added = true;
                    break;
                }
            }
            if !added {
                consensus_groups.push(vec![prop]);
            }
        }

        for group in consensus_groups {
            if group.len() >= threshold {
                return Ok(format!("Consensus reached based on {} proposals. Final answer: {}", group.len(), group[0]));
            }
        }

        Err("Failed to reach consensus among agents.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};

    struct MockSwarmLlm {
        response: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockSwarmLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.response),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    struct JudgeMockLlm;

    #[async_trait::async_trait]
    impl LlmClient for JudgeMockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("YES"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    fn create_agent(name: &str, response: &str) -> SwarmAgent {
        SwarmAgent {
            name: name.to_string(),
            description: "Mock agent".to_string(),
            agent: Arc::new(Agent::new(Arc::new(MockSwarmLlm { response: response.to_string() }), vec![])),
            run_config: AgentRunConfig::default(),
        }
    }

    fn create_judge_agent() -> SwarmAgent {
        SwarmAgent {
            name: "Judge".to_string(),
            description: "Judge agent".to_string(),
            agent: Arc::new(Agent::new(Arc::new(JudgeMockLlm), vec![])),
            run_config: AgentRunConfig::default(),
        }
    }

    #[tokio::test]
    async fn test_hierarchical_swarm() {
        let lead = create_agent("Lead", "Final synthesized answer");
        let worker1 = create_agent("Worker1", "Worker 1 result");
        let worker2 = create_agent("Worker2", "Worker 2 result");

        let orchestrator = SwarmOrchestrator::new(Topology::Hierarchical {
            lead,
            workers: vec![worker1, worker2],
        });

        let result = orchestrator.run("Solve X").await.unwrap();
        assert_eq!(result, "Final synthesized answer");

        let transcript = orchestrator.transcript.read().await;
        assert!(transcript.iter().any(|m| m.content.contains("Worker1: Worker 1 result")));
        assert!(transcript.iter().any(|m| m.content.contains("Worker2: Worker 2 result")));
    }

    #[tokio::test]
    async fn test_mesh_swarm() {
        let agent1 = create_agent("Agent1", "I think X");
        let agent2 = create_agent("Agent2", "I agree, TERMINATE");

        let orchestrator = SwarmOrchestrator::new(Topology::Mesh {
            agents: vec![agent1, agent2],
            max_rounds: 3,
        });

        let result = orchestrator.run("Discuss X").await.unwrap();
        assert_eq!(result, "I agree, TERMINATE");
    }

    #[tokio::test]
    async fn test_consensus_swarm() {
        let agent1 = create_agent("Agent1", "Proposal A");
        let agent2 = create_agent("Agent2", "Proposal A");
        let judge = create_judge_agent();

        let orchestrator = SwarmOrchestrator::new(Topology::AdaptiveConsensus {
            agents: vec![agent1, agent2],
            consensus_threshold: 2,
            judge,
        });

        let result = orchestrator.run("Decide X").await.unwrap();
        assert!(result.contains("Consensus reached"));
        assert!(result.contains("Proposal A"));
    }
}
