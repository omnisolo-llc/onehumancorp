use ohc_builtin_agent_core::types::{ChatRequest, Message};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;
use futures::future::join_all;

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus
/// SONA neural patterns: Self-learning trajectory patterns

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SonaPattern {
    pub task_signature: String,
    pub successful_trajectory: String,
}

#[async_trait::async_trait]
pub trait SonaMemory: Send + Sync {
    async fn store_pattern(&self, pattern: SonaPattern) -> Result<(), String>;
    async fn recall_pattern(&self, task: &str) -> Result<Option<SonaPattern>, String>;
}

pub struct SonaMemoryStore {
    patterns: std::sync::Arc<tokio::sync::Mutex<Vec<SonaPattern>>>,
}

impl Default for SonaMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SonaMemoryStore {
    pub fn new() -> Self {
        Self {
            patterns: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl SonaMemory for SonaMemoryStore {
    async fn store_pattern(&self, pattern: SonaPattern) -> Result<(), String> {
        let mut patterns = self.patterns.lock().await;
        patterns.push(pattern);
        Ok(())
    }

    async fn recall_pattern(&self, task: &str) -> Result<Option<SonaPattern>, String> {
        let patterns = self.patterns.lock().await;
        // Simple heuristic for demonstration: if task contains the signature keyword
        for pattern in patterns.iter() {
            if task.contains(&pattern.task_signature) {
                return Ok(Some(pattern.clone()));
            }
        }
        Ok(None)
    }
}

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

    #[tokio::test]
    async fn test_sona_neural_patterns() {
        let llm = Arc::new(MockLlmClient { resp: "Execution complete.".to_string() });
        let lead = SwarmAgent::new("Lead", llm.clone());
        let worker = SwarmAgent::new("Worker", llm);
        let memory = std::sync::Arc::new(SonaMemoryStore::new());

        let coordinator = SwarmCoordinator::new(lead, vec![worker], SwarmTopology::Hierarchical)
            .with_sona_memory(memory.clone());

        // Initial task execution
        let task1 = "Unique task 1";
        let _ = coordinator.execute(task1).await.unwrap();

        // Check if pattern was stored
        let pattern_opt = memory.recall_pattern(task1).await.unwrap();
        assert!(pattern_opt.is_some());
        let pattern = pattern_opt.unwrap();
        assert_eq!(pattern.task_signature, task1);
        assert!(pattern.successful_trajectory.contains("Lead Agent Output: Execution complete."));

        // Second execution should pick up the hint
        // Wait, the MockLlmClient ignores the actual prompt and just returns self.resp
        // We'll use a special mock to verify the hint was injected.
        struct HintMockLlmClient;
        #[async_trait::async_trait]
        impl LlmClient for HintMockLlmClient {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let msg_content = &req.messages[0].content;
                let output = if msg_content.contains("SONA Trajectory Hint") {
                    "Hint recognized".to_string()
                } else {
                    "No hint".to_string()
                };
                Ok(ChatResponse {
                    message: Message::assistant(output),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id".to_string()),
                })
            }
        }

        let hint_llm = Arc::new(HintMockLlmClient);
        let lead2 = SwarmAgent::new("Lead", hint_llm.clone());
        let worker2 = SwarmAgent::new("Worker", hint_llm);
        let coordinator2 = SwarmCoordinator::new(lead2, vec![worker2], SwarmTopology::Hierarchical)
            .with_sona_memory(memory.clone());

        // Execute task 1 again
        let result2 = coordinator2.execute(task1).await.unwrap();
        // Because of how Hierarchical topology works, the lead agent synthesizes.
        // It will pass the task text (which includes the hint) to the lead agent's synthesis.
        assert_eq!(result2, "Hint recognized");
    }
}

pub struct SwarmCoordinator {
    pub lead_agent: SwarmAgent,
    pub workers: Vec<SwarmAgent>,
    pub topology: SwarmTopology,
    pub sona_memory: Option<std::sync::Arc<dyn SonaMemory>>,
}

impl SwarmCoordinator {
    pub fn new(lead_agent: SwarmAgent, workers: Vec<SwarmAgent>, topology: SwarmTopology) -> Self {
        Self {
            lead_agent,
            workers,
            topology,
            sona_memory: None,
        }
    }

    pub fn with_sona_memory(mut self, memory: std::sync::Arc<dyn SonaMemory>) -> Self {
        self.sona_memory = Some(memory);
        self
    }

    pub async fn execute(&self, task: &str) -> Result<String, String> {
        let original_task = task.to_string();
        let mut actual_task = original_task.clone();

        if let Some(memory) = &self.sona_memory {
            if let Ok(Some(pattern)) = memory.recall_pattern(task).await {
                actual_task = format!(
                    "[SONA Trajectory Hint: A similar past task followed this successful trajectory:\n{}\n]\n\nCurrent Task: {}",
                    pattern.successful_trajectory, task
                );
            }
        }

        let task = &actual_task;

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

        let result = match active_topology {
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
                let _task_clone = task.to_string();
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
        };

        if let Ok(res_str) = &result {
            if let Some(memory) = &self.sona_memory {
                let extract_instruction = "Extract a concise SONA trajectory pattern from the execution outcome. What were the key steps taken to solve this task? Return ONLY the trajectory steps.";
                let trajectory_prompt = format!("Task: {}\nResult: {}\n", original_task, res_str);
                if let Ok(trajectory) = self.lead_agent.process_task(&trajectory_prompt, extract_instruction).await {
                    let pattern = SonaPattern {
                        task_signature: original_task.clone(), // In a real system, LLM extracts the signature
                        successful_trajectory: trajectory,
                    };
                    let _ = memory.store_pattern(pattern).await;
                }
            }
        }

        result
    }
}
