use std::sync::Arc;
use futures::future::join_all;
use crate::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use crate::agent::{Agent, AgentRunConfig};

/// DeerFlow Unique Harness Innovations: Sub-agent orchestration:
/// Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.
pub struct DeerFlowOrchestrator {
    pub lead_llm: Arc<dyn LlmClient>,
    pub sub_agent_factory: Box<dyn Fn(String) -> Arc<Agent> + Send + Sync>,
}

impl DeerFlowOrchestrator {
    pub fn new(
        lead_llm: Arc<dyn LlmClient>,
        sub_agent_factory: impl Fn(String) -> Arc<Agent> + Send + Sync + 'static,
    ) -> Self {
        Self {
            lead_llm,
            sub_agent_factory: Box::new(sub_agent_factory),
        }
    }

    pub async fn execute_task(
        &self,
        task: &str,
        config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Step 1: Decompose the task
        let decompose_prompt = format!(
            "You are a lead agent. Decompose the following task into a set of independent sub-tasks that can be executed in parallel.\n\
            Return your output strictly as a JSON array of strings, where each string is a sub-task description.\n\
            Task: {}",
            task
        );

        let decompose_req = ChatRequest {
            model: "default".to_string(),
            system: "You decompose tasks into independent JSON arrays of subtasks. Return nothing else but the JSON array.".to_string(),
            messages: vec![Message::user(decompose_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.1,
        };

        let decompose_resp = self.lead_llm.chat(decompose_req).await?;
        let sub_tasks_json = decompose_resp.message.content.trim();

        // Extract json array if it's wrapped in markdown
        let mut clean_json = sub_tasks_json;
        if clean_json.starts_with("```json") {
            clean_json = clean_json.trim_start_matches("```json");
        } else if clean_json.starts_with("```") {
            clean_json = clean_json.trim_start_matches("```");
        }
        clean_json = clean_json.trim_end_matches("```").trim();

        let sub_tasks: Vec<String> = match serde_json::from_str(clean_json) {
            Ok(tasks) => tasks,
            Err(e) => {
                return Err(format!("Failed to parse sub-tasks from lead agent output: {}. Output was: {}", e, sub_tasks_json).into());
            }
        };

        if sub_tasks.is_empty() {
            return Err("Lead agent decomposed task into 0 sub-tasks.".into());
        }

        // Step 2: Spawn parallel sub-agents
        let mut futures = Vec::new();
        for (i, sub_task) in sub_tasks.iter().enumerate() {
            let sub_agent = (self.sub_agent_factory)(format!("SubAgent-{}", i));
            let sub_task_clone = sub_task.clone();
            let config_clone = config.clone();

            let fut = async move {
                let mut on_event = |_| {};
                sub_agent.run(&config_clone, &sub_task_clone, &mut on_event).await
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;

        // Step 3: Synthesize results
        let mut combined_results = String::new();
        for (i, res) in results.into_iter().enumerate() {
            let sub_task_desc = &sub_tasks[i];
            match res {
                Ok(output) => {
                    combined_results.push_str(&format!("Sub-task '{}' result:\n{}\n\n", sub_task_desc, output));
                }
                Err(e) => {
                    combined_results.push_str(&format!("Sub-task '{}' failed:\n{}\n\n", sub_task_desc, e));
                }
            }
        }

        let synthesize_prompt = format!(
            "You are a lead agent. You previously decomposed a task and delegated it to sub-agents.\n\
            Here is the original task:\n{}\n\n\
            Here are the results from the sub-agents:\n{}\n\n\
            Please synthesize these results into a single, comprehensive final answer.",
            task, combined_results
        );

        let synthesize_req = ChatRequest {
            model: "default".to_string(),
            system: "You are a lead agent that synthesizes results from sub-agents into a final answer.".to_string(),
            messages: vec![Message::user(synthesize_prompt)],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.2,
        };

        let synthesize_resp = self.lead_llm.chat(synthesize_req).await?;
        Ok(synthesize_resp.message.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
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
    async fn test_deerflow_subagent_orchestration() {
        // First response is the decomposition (JSON array).
        // Second response is the synthesis.
        let lead_llm = Arc::new(MockLlm {
            responses: Mutex::new(vec![
                r#"["Sub-task 1", "Sub-task 2"]"#.to_string(),
                "Final synthesized answer based on sub-task results".to_string(),
            ]),
        });

        // Factory returns a dummy agent that just returns a static string simulating sub-agent work
        let factory = |_name: String| {
            let sub_llm = Arc::new(MockLlm {
                responses: Mutex::new(vec!["Sub-agent output".to_string()]),
            });
            Arc::new(Agent::new(sub_llm as Arc<dyn LlmClient>, vec![]))
        };

        let orchestrator = DeerFlowOrchestrator::new(lead_llm, factory);
        let config = AgentRunConfig::default();

        let final_result = orchestrator.execute_task("Do complex thing", &config).await.unwrap();

        assert_eq!(final_result, "Final synthesized answer based on sub-task results");
    }
}
