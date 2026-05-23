use std::sync::Arc;
use futures::future::join_all;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use crate::llm::LlmClient;

/// DeerFlow Unique Harness Innovations: Sub-agent orchestration.
/// Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.
pub struct DeerFlowOrchestrator {
    pub lead_llm: Arc<dyn LlmClient>,
    pub sub_agent_llm: Arc<dyn LlmClient>,
}

impl DeerFlowOrchestrator {
    pub fn new(lead_llm: Arc<dyn LlmClient>, sub_agent_llm: Arc<dyn LlmClient>) -> Self {
        Self { lead_llm, sub_agent_llm }
    }

    /// Decomposes a large task into a list of parallel sub-tasks.
    pub async fn decompose_task(&self, task: &str) -> Result<Vec<String>, String> {
        let system_prompt = "You are a Lead Agent. Break down the given task into 3 parallel, independent sub-tasks. Return ONLY a JSON array of strings.";

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.2,
        };

        match self.lead_llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim();
                // Attempt to parse as JSON array
                match serde_json::from_str::<Vec<String>>(text) {
                    Ok(tasks) => Ok(tasks),
                    Err(_) => {
                        // Fallback parsing if LLM didn't return perfect JSON
                        let tasks: Vec<String> = text.lines()
                            .filter(|l| !l.trim().is_empty() && !l.starts_with("```"))
                            .map(|l| l.trim_start_matches("- ").trim().to_string())
                            .collect();
                        if tasks.is_empty() {
                            Err("Failed to decompose task".to_string())
                        } else {
                            Ok(tasks)
                        }
                    }
                }
            }
            Err(e) => Err(format!("Lead agent error: {}", e)),
        }
    }

    /// Spawns parallel sub-agents for each sub-task and collects condensed summaries.
    pub async fn execute_parallel_sub_tasks(&self, sub_tasks: Vec<String>) -> Result<Vec<String>, String> {
        let mut futures = Vec::new();

        for (i, task) in sub_tasks.into_iter().enumerate() {
            let llm = self.sub_agent_llm.clone();
            let sub_task_prompt = format!("You are a sub-agent. Execute the following task and return a detailed response: {}", task);

            let fut = async move {
                let req = ChatRequest {
                    model: format!("sub-agent-{}", i),
                    system: "You are a specialized sub-agent.".to_string(),
                    messages: vec![Message::user(sub_task_prompt)],
                    tools: vec![],
                    max_tokens: 4000,
                    temperature: 0.4,
                };

                let raw_output = match llm.chat(req).await {
                    Ok(resp) => resp.message.content,
                    Err(e) => format!("Error executing sub-task: {}", e),
                };

                // Enforce "condensed summaries" rule: wrap subagent output such that they only return 1k-2k tokens.
                // We simulate this by truncating the text.
                let max_length = 2000;
                if raw_output.len() > max_length {
                    let truncated: String = raw_output.chars().take(max_length).collect();
                    format!("{}... [Condensed]", truncated)
                } else {
                    raw_output
                }
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;
        Ok(results)
    }

    /// Synthesizes the final result from the condensed summaries.
    pub async fn synthesize_results(&self, original_task: &str, summaries: Vec<String>) -> Result<String, String> {
        let system_prompt = "You are a Lead Agent. Synthesize the following sub-task summaries into a cohesive final answer to the original task.";
        let combined_summaries = summaries.join("\n\n---\n\n");
        let prompt = format!("Original Task: {}\n\nSub-task Summaries:\n{}", original_task, combined_summaries);

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.2,
        };

        match self.lead_llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content),
            Err(e) => Err(format!("Synthesis error: {}", e)),
        }
    }

    /// Full orchestration flow: decompose, parallel execute, synthesize.
    pub async fn run_orchestration(&self, task: &str) -> Result<String, String> {
        let sub_tasks = self.decompose_task(task).await?;
        let summaries = self.execute_parallel_sub_tasks(sub_tasks).await?;
        self.synthesize_results(task, summaries).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockLlmClient {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Default response".to_string()
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
        // Setup mock responses
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                r#"["Sub-task 1", "Sub-task 2", "Sub-task 3"]"#.to_string(), // decompose_task response
                "Final synthesized result".to_string(), // synthesize_results response
            ]),
        });

        let sub_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                "Sub-task 1 executed".to_string(),
                "Sub-task 2 executed".to_string(),
                "Sub-task 3 executed".to_string(),
            ]),
        });

        let orchestrator = DeerFlowOrchestrator::new(lead_client, sub_client);

        // 1. Test Decomposition
        let sub_tasks = orchestrator.decompose_task("Do complex task").await.unwrap();
        assert_eq!(sub_tasks.len(), 3);
        assert_eq!(sub_tasks[0], "Sub-task 1");

        // 2. Test Parallel Execution
        // Note: we can't easily guarantee order from the mock if we do parallel requests to the same mock instance in tests without careful locking,
        // but `join_all` processes them sequentially in the vector, meaning the futures are polled.
        // Wait, `futures::future::join_all` polls them, but the order of completion might vary.
        // Since `sub_client` has a simple lock, it will hand out responses in the order the locks are acquired.
        let summaries = orchestrator.execute_parallel_sub_tasks(sub_tasks).await.unwrap();
        assert_eq!(summaries.len(), 3);
        // We just check that the responses are there
        assert!(summaries.iter().any(|s| s == "Sub-task 1 executed"));
        assert!(summaries.iter().any(|s| s == "Sub-task 2 executed"));
        assert!(summaries.iter().any(|s| s == "Sub-task 3 executed"));

        // 3. Test Synthesis
        let final_result = orchestrator.synthesize_results("Do complex task", summaries).await.unwrap();
        assert_eq!(final_result, "Final synthesized result");
    }

    #[tokio::test]
    async fn test_deerflow_condensation_rule() {
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![]),
        });

        // Response larger than 2000 chars
        let long_response = "A".repeat(2500);
        let sub_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![long_response]),
        });

        let orchestrator = DeerFlowOrchestrator::new(lead_client, sub_client);

        let sub_tasks = vec!["Task 1".to_string()];
        let summaries = orchestrator.execute_parallel_sub_tasks(sub_tasks).await.unwrap();

        assert_eq!(summaries.len(), 1);
        let summary = &summaries[0];

        assert!(summary.ends_with("... [Condensed]"));
        assert_eq!(summary.len(), 2000 + "... [Condensed]".len());
    }
}
