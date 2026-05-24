use ohc_builtin_agent_core::types::{ChatRequest, Message};
use crate::llm::LlmClient;
use crate::agent::{Agent, AgentRunConfig};
use crate::autogen::ChatAgent;
use std::sync::Arc;
use futures::future::join_all;

/// DeerFlow Unique Harness Innovations: Sub-agent orchestration.
/// Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SubTask {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TaskDecompositionOutput {
    pub sub_tasks: Vec<SubTask>,
}

pub struct DeerFlowOrchestrator {
    pub lead_llm: Arc<dyn LlmClient>,
    pub sub_agents: Vec<ChatAgent>,
}

impl DeerFlowOrchestrator {
    pub fn new(lead_llm: Arc<dyn LlmClient>, sub_agents: Vec<ChatAgent>) -> Self {
        Self {
            lead_llm,
            sub_agents,
        }
    }

    /// Step 1: Decompose the task
    async fn decompose_task(&self, task: &str) -> Result<Vec<SubTask>, String> {
        let system_prompt = "You are a Lead Orchestrator Agent. Your job is to decompose the user's complex task into smaller, independent sub-tasks that can be executed in parallel. You must return your output strictly as a JSON object matching this schema: {\"sub_tasks\": [{\"id\": \"task_1\", \"description\": \"detailed description\"}]}";

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.1,
        };

        let resp = self.lead_llm.chat(req).await.map_err(|e| format!("Decomposition LLM failed: {}", e))?;
        let output_text = resp.message.content.trim();

        if output_text.is_empty() {
            return Err("LLM returned an empty response during decomposition.".to_string());
        }

        // Very basic json extraction for the DeerFlow sub-agent orchestration
        let start_idx = output_text.find('{').unwrap_or(0);
        let end_idx = output_text.rfind('}').unwrap_or(output_text.len().saturating_sub(1));

        if start_idx > end_idx {
            return Err(format!("Invalid JSON structure returned: closing brace appears before opening brace or missing. Output: {}", output_text));
        }

        let json_str = &output_text[start_idx..=end_idx];

        let parsed: TaskDecompositionOutput = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse decomposition output: {}\nRaw output: {}", e, output_text))?;

        Ok(parsed.sub_tasks)
    }

    /// Step 2: Spawn parallel sub-agents
    async fn run_parallel_sub_agents(&self, sub_tasks: Vec<SubTask>) -> Result<Vec<(String, String)>, String> {
        let mut futures = Vec::new();

        // Assign sub-tasks to agents in a round-robin fashion, or just spawn an agent per task if we treat sub_agents as a pool.
        // For simplicity, we just use the first agent as a template, or round-robin if multiple.
        if self.sub_agents.is_empty() {
            return Err("No sub-agents available to execute tasks.".to_string());
        }

        for (i, sub_task) in sub_tasks.into_iter().enumerate() {
            let agent_cfg = &self.sub_agents[i % self.sub_agents.len()];
            let agent = agent_cfg.agent.clone();
            let mut run_cfg = agent_cfg.run_config.clone();

            // Set the system prompt for the sub-agent
            run_cfg.server_system_message = format!("You are a sub-agent. Your assigned task is: {}", sub_task.description);
            let description = sub_task.description.clone();

            let fut = async move {
                let mut on_event = |_| {};
                let prompt = format!("Execute the following sub-task: {}", description);
                match agent.run(&run_cfg, &prompt, &mut on_event).await {
                    Ok(res) => Ok((sub_task.id, res)),
                    Err(e) => Err(format!("Sub-agent failed on task {}: {}", sub_task.id, e)),
                }
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;

        let mut final_results = Vec::new();
        for res in results {
            final_results.push(res?);
        }

        Ok(final_results)
    }

    /// Step 3: Synthesize results
    async fn synthesize_results(&self, original_task: &str, results: Vec<(String, String)>) -> Result<String, String> {
        let mut results_text = String::new();
        for (task_id, result) in results {
            results_text.push_str(&format!("--- Result for Sub-task [{}] ---\n{}\n\n", task_id, result));
        }

        let system_prompt = "You are a Lead Orchestrator Agent. Your sub-agents have completed their parallel sub-tasks. Your job is to synthesize their individual results into a single, cohesive final answer that addresses the user's original task.";
        let user_prompt = format!("Original Task: {}\n\nSub-agent Results:\n{}", original_task, results_text);

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.2,
        };

        let resp = self.lead_llm.chat(req).await.map_err(|e| format!("Synthesis LLM failed: {}", e))?;
        Ok(resp.message.content)
    }

    /// The full DeerFlow Sub-agent Orchestration pipeline
    pub async fn run_orchestration(&self, task: &str) -> Result<String, String> {
        // 1. Decompose
        let sub_tasks = self.decompose_task(task).await?;
        if sub_tasks.is_empty() {
            return Err("Task decomposition yielded no sub-tasks.".to_string());
        }

        // 2. Parallel Execution
        let parallel_results = self.run_parallel_sub_agents(sub_tasks).await?;

        // 3. Synthesis
        self.synthesize_results(task, parallel_results).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};

    struct MockDeerFlowLlmClient {
        // We'll return decomposition json, then synthesis.
        responses: tokio::sync::Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockDeerFlowLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                // If it's a subagent run
                "subagent success".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_deerflow_orchestration() {
        let decomposition_json = r#"{
            "sub_tasks": [
                {"id": "t1", "description": "Do part 1"},
                {"id": "t2", "description": "Do part 2"}
            ]
        }"#;

        let synthesis_text = "Final synthesized answer based on parts 1 and 2.";

        let lead_llm = Arc::new(MockDeerFlowLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                decomposition_json.to_string(),
                synthesis_text.to_string(),
            ]),
        });

        let sub_llm = Arc::new(MockDeerFlowLlmClient {
            responses: tokio::sync::Mutex::new(vec![]),
        });

        let sub_agent = ChatAgent {
            name: "Worker".to_string(),
            description: "worker".to_string(),
            agent: Arc::new(Agent::new(sub_llm.clone(), vec![])),
            run_config: AgentRunConfig::default(),
        };

        let orchestrator = DeerFlowOrchestrator::new(lead_llm, vec![sub_agent]);

        let result = orchestrator.run_orchestration("Complex task").await.unwrap();
        assert_eq!(result, synthesis_text);
    }
}
