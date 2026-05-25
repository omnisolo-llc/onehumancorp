use crate::agent::{Agent, AgentRunConfig};
use futures::future::join_all;
use std::sync::Arc;

/// DeerFlow Unique Harness Innovations: Sub-agent orchestration
/// Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.
pub struct DeerFlowOrchestrator {
    pub lead_agent: Arc<Agent>,
    pub subagent_factory: Arc<dyn Fn() -> Arc<Agent> + Send + Sync>,
}

impl DeerFlowOrchestrator {
    pub fn new(lead_agent: Arc<Agent>, subagent_factory: Arc<dyn Fn() -> Arc<Agent> + Send + Sync>) -> Self {
        Self {
            lead_agent,
            subagent_factory,
        }
    }

    pub async fn execute_task(&self, task: &str, config: &AgentRunConfig) -> Result<String, String> {
        // 1. Decompose Task
        let decompose_prompt = format!(
            "Decompose the following task into a list of independent sub-tasks that can be executed in parallel. Return ONLY a JSON array of strings, where each string is a sub-task.\n\nTask: {}",
            task
        );

        let mut on_event = |_| {};
        let decomposed_str = self.lead_agent.run(config, &decompose_prompt, &mut on_event).await
            .map_err(|e| format!("Decomposition failed: {}", e))?;

        let clean_json = decomposed_str.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

        // Extract JSON array
        let sub_tasks: Vec<String> = match serde_json::from_str(clean_json) {
            Ok(tasks) => tasks,
            Err(_) => {
                // Fallback if not strict JSON
                vec![task.to_string()]
            }
        };

        if sub_tasks.is_empty() {
            return Err("No sub-tasks generated.".to_string());
        }

        // 2. Spawn Parallel Sub-agents
        let mut futures = Vec::new();
        for (i, sub_task) in sub_tasks.iter().enumerate() {
            let subagent = (self.subagent_factory)();
            let sub_config = config.clone();
            let task_clone = format!("{}\n\nIMPORTANT RULE: You must return a condensed summary of your work (1k-2k tokens max), NOT the full context loop.", sub_task);

            let fut = async move {
                let mut local_event = |_| {};
                let result = subagent.run(&sub_config, &task_clone, &mut local_event).await;
                (i, result)
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;

        let mut synthesized_input = String::new();
        for (i, res) in results {
            match res {
                Ok(output) => {
                    synthesized_input.push_str(&format!("Sub-task {} Result:\n{}\n\n", i, output));
                }
                Err(e) => {
                    synthesized_input.push_str(&format!("Sub-task {} Failed:\n{}\n\n", i, e));
                }
            }
        }

        // 3. Synthesize Results
        let synthesis_prompt = format!(
            "You are the lead agent. Synthesize the following results from parallel sub-agents into a final, coherent response for the original task.\n\nOriginal Task: {}\n\nSub-agent Results:\n{}",
            task, synthesized_input
        );

        let final_result = self.lead_agent.run(config, &synthesis_prompt, &mut on_event).await
            .map_err(|e| format!("Synthesis failed: {}", e))?;

        Ok(final_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::llm::LlmClient;

    struct MockDeerFlowLlm {
        responses: tokio::sync::Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockDeerFlowLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
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
        let lead_llm = Arc::new(MockDeerFlowLlm {
            responses: tokio::sync::Mutex::new(vec![
                r#"```json
["Sub-task 1", "Sub-task 2"]
```"#.to_string(), // Decomposition with markdown
                "Final Synthesized Output".to_string() // Synthesis
            ]),
        });
        let lead_agent = Arc::new(Agent::new(lead_llm, vec![]));

        let subagent_factory = Arc::new(|| {
            let sub_llm = Arc::new(MockDeerFlowLlm {
                responses: tokio::sync::Mutex::new(vec![
                    "Subagent result".to_string()
                ]),
            });
            Arc::new(Agent::new(sub_llm, vec![]))
        });

        let orchestrator = DeerFlowOrchestrator::new(lead_agent, subagent_factory);
        let config = AgentRunConfig::default();

        let result = orchestrator.execute_task("Do complex thing", &config).await.unwrap();
        assert_eq!(result, "Final Synthesized Output");
    }
}
