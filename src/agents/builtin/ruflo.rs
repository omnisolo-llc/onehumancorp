use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;
use futures::future::join_all;

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwarmTopology {
    /// Lead agent delegates to sub-agents and synthesizes the result.
    Hierarchical,
    /// All agents process the task and a consensus (majority voting or synthesis) is reached.
    Mesh,
    /// Chooses between Hierarchical and Mesh based on task complexity (assessed by Lead agent).
    Adaptive,
}

pub struct SwarmAgent {
    pub name: String,
    pub llm: Arc<dyn LlmClient>,
}

impl SwarmAgent {
    pub fn new(name: &str, llm: Arc<dyn LlmClient>) -> Self {
        Self {
            name: name.to_string(),
            llm,
        }
    }

    pub async fn process_task(&self, task: &str, instruction: &str) -> Result<String, String> {
        let req = ChatRequest {
            model: "default".to_string(),
            system: format!("You are {}. {}", self.name, instruction),
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.2,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content),
            Err(e) => Err(format!("{} error: {}", self.name, e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Usage;

    struct MockLlmClient {
        resp: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let role = req.system;
            let output = if role.contains("Lead") {
                format!("Lead Agent Output: {}", self.resp)
            } else {
                format!("Worker Output: {}", self.resp)
            };

            Ok(ChatResponse {
                message: Message::assistant(output),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_hierarchical_swarm() {
        let lead_llm = Arc::new(MockLlmClient { resp: "Synthesis complete.".to_string() });
        let worker_llm = Arc::new(MockLlmClient { resp: "Task executed.".to_string() });

        let lead = SwarmAgent::new("Lead", lead_llm);
        let worker1 = SwarmAgent::new("Worker1", worker_llm.clone());
        let worker2 = SwarmAgent::new("Worker2", worker_llm);

        let coordinator = SwarmCoordinator::new(lead, vec![worker1, worker2], SwarmTopology::Hierarchical);
        let result = coordinator.execute("Do this task").await.unwrap();

        assert!(result.contains("Lead Agent Output: Synthesis complete."));
    }

    #[tokio::test]
    async fn test_mesh_swarm() {
        let lead_llm = Arc::new(MockLlmClient { resp: "Consensus reached.".to_string() });
        let worker_llm = Arc::new(MockLlmClient { resp: "My vote is yes.".to_string() });

        let lead = SwarmAgent::new("Lead", lead_llm);
        let worker1 = SwarmAgent::new("Worker1", worker_llm.clone());
        let worker2 = SwarmAgent::new("Worker2", worker_llm);

        let coordinator = SwarmCoordinator::new(lead, vec![worker1, worker2], SwarmTopology::Mesh);
        let result = coordinator.execute("Do this task").await.unwrap();

        assert!(result.contains("Lead Agent Output: Consensus reached."));
    }

    struct AdaptiveMockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for AdaptiveMockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let output = if req.system.contains("COMPLEX") {
                "COMPLEX".to_string()
            } else {
                "Adaptive output".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(output),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_adaptive_swarm() {
        let llm = Arc::new(AdaptiveMockLlmClient);
        let lead = SwarmAgent::new("Lead", llm.clone());
        let worker1 = SwarmAgent::new("Worker1", llm);

        let coordinator = SwarmCoordinator::new(lead, vec![worker1], SwarmTopology::Adaptive);
        let result = coordinator.execute("Do this complex task").await.unwrap();

        // The mock always returns "COMPLEX" for complexity check, which triggers Mesh topology
        assert!(result.contains("Adaptive output"));
    }
}

pub struct SwarmCoordinator {
    pub lead_agent: SwarmAgent,
    pub workers: Vec<SwarmAgent>,
    pub topology: SwarmTopology,
}

impl SwarmCoordinator {
    pub fn new(lead_agent: SwarmAgent, workers: Vec<SwarmAgent>, topology: SwarmTopology) -> Self {
        Self {
            lead_agent,
            workers,
            topology,
        }
    }

    pub async fn execute(&self, task: &str) -> Result<String, String> {
        let active_topology = if self.topology == SwarmTopology::Adaptive {
            // Adaptive logic: Lead agent evaluates complexity
            let eval_instruction = "Assess the complexity of the following task. Reply ONLY with 'COMPLEX' or 'SIMPLE'.";
            let complexity = self.lead_agent.process_task(task, eval_instruction).await.unwrap_or_else(|_| "SIMPLE".to_string());
            if complexity.contains("COMPLEX") {
                SwarmTopology::Mesh
            } else {
                SwarmTopology::Hierarchical
            }
        } else {
            self.topology.clone()
        };

        match active_topology {
            SwarmTopology::Hierarchical => {
                // Lead agent delegates to workers (simulated here by simply passing the task to each)
                let mut futures = Vec::new();
                for worker in &self.workers {
                    let task_clone = task.to_string();
                    let instruction = "Provide a specialized solution for this task.";
                    futures.push(Box::pin(async move {
                        worker.process_task(&task_clone, instruction).await
                    }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>);
                }

                let results = join_all(futures).await;
                let mut aggregated = String::new();
                for (i, res) in results.into_iter().enumerate() {
                    if let Ok(content) = res {
                        aggregated.push_str(&format!("Worker {} says:\n{}\n", i + 1, content));
                    }
                }

                // Lead agent synthesizes
                let synthesis_instruction = "Synthesize the following worker outputs into a final cohesive response.";
                let final_prompt = format!("Original Task: {}\n\nWorker Outputs:\n{}", task, aggregated);
                self.lead_agent.process_task(&final_prompt, synthesis_instruction).await
            }
            SwarmTopology::Mesh | SwarmTopology::Adaptive => {
                // Mesh: All agents (including lead) evaluate, vote/consensus on the answer
                let mut futures = Vec::new();

                // Include lead agent in mesh
                let task_clone = task.to_string();
                let instruction = "Analyze the task and provide your independent solution.";
                let lead_clone = task.to_string();

                futures.push(Box::pin(async move {
                    self.lead_agent.process_task(&lead_clone, instruction).await
                }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>);

                for worker in &self.workers {
                    let task_clone = task.to_string();
                    futures.push(Box::pin(async move {
                        worker.process_task(&task_clone, "Analyze the task and provide your independent solution.").await
                    }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>);
                }

                let results = join_all(futures).await;
                let mut all_outputs = String::new();
                for (i, res) in results.into_iter().enumerate() {
                    if let Ok(content) = res {
                        all_outputs.push_str(&format!("Agent {} says:\n{}\n", i, content));
                    }
                }

                // Consensus mechanism (Simulated by Lead agent acting as consensus resolver)
                let consensus_instruction = "Review the following independent agent solutions for the task. Find the consensus or majority opinion and provide the final definitive answer.";
                let final_prompt = format!("Original Task: {}\n\nAgent Solutions:\n{}", task, all_outputs);
                self.lead_agent.process_task(&final_prompt, consensus_instruction).await
            }
        }
    }
}
