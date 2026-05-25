use ohc_builtin_agent_core::types::{ChatRequest, Message};
use crate::agent::{Agent, AgentRunConfig, AgentEvent};
use std::sync::Arc;
use futures::future::join_all;
use serde::Deserialize;

/// DeerFlow Unique Harness Innovations: Sub-agent orchestration
/// Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results
pub struct DeerFlowOrchestrator {
    pub lead_agent: Arc<Agent>,
}

#[derive(Deserialize, Debug)]
struct TaskDecomposition {
    subtasks: Vec<String>,
}

impl DeerFlowOrchestrator {
    pub fn new(lead_agent: Arc<Agent>) -> Self {
        Self { lead_agent }
    }

    pub async fn execute<F>(
        &self,
        task: &str,
        cfg: &AgentRunConfig,
        mut on_event: F,
    ) -> Result<String, String>
    where
        F: FnMut(AgentEvent) + Send + Sync + Clone + 'static,
    {
        // 1. Decompose the task
        let decompose_prompt = format!(
            "You are the Lead Agent in a DeerFlow orchestration.\nDecompose the following task into a JSON object containing a 'subtasks' array of string descriptions for parallel execution.\n\nTask: {}",
            task
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You output pure JSON only.".to_string(),
            messages: vec![Message::user(decompose_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.1,
        };

        let resp = self.lead_agent.llm.chat(req).await.map_err(|e| e.to_string())?;
        let json_text = resp.message.content.trim_matches(|c| c == '`' || c == '\n').to_string();

        let json_str = if json_text.starts_with("json\n") {
            json_text[5..].trim()
        } else if json_text.starts_with("json") {
            json_text[4..].trim()
        } else {
            &json_text
        };

        let decomposition: TaskDecomposition = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse task decomposition: {}", e))?;

        // 2. Spawn parallel subagents
        let mut futures = Vec::new();
        for subtask in decomposition.subtasks {
            let agent_clone = Arc::new(Agent::new(self.lead_agent.llm.clone(), self.lead_agent.tools.clone()));
            let mut event_cb = on_event.clone();
            let cfg_clone = cfg.clone();
            let task_clone = subtask.clone();

            let fut = async move {
                match agent_clone.run(&cfg_clone, &task_clone, &mut event_cb).await {
                    Ok(res) => Ok(format!("Subtask '{}' Result:\n{}", task_clone, res)),
                    Err(e) => Err(format!("Subtask '{}' Failed: {}", task_clone, e)),
                }
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;

        let mut synthesized_context = String::new();
        for res in results {
            match res {
                Ok(r) => synthesized_context.push_str(&format!("{}\n\n", r)),
                Err(e) => synthesized_context.push_str(&format!("{}\n\n", e)),
            }
        }

        // 3. Synthesize results
        let synthesis_prompt = format!(
            "You are the Lead Agent in a DeerFlow orchestration.\nSynthesize the following subtask results into a final cohesive response for the original task: '{}'\n\nResults:\n{}",
            task, synthesized_context
        );

        let synth_req = ChatRequest {
            model: "default".to_string(),
            system: "You are an expert synthesizer. Provide the final response directly without meta-commentary.".to_string(),
            messages: vec![Message::user(synthesis_prompt)],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.2,
        };

        let synth_resp = self.lead_agent.llm.chat(synth_req).await.map_err(|e| e.to_string())?;

        Ok(synth_resp.message.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage, ToolCall};
    use crate::llm::LlmClient;
    use tokio::sync::Mutex;

    struct MockDeerFlowLlmClient {
        responses: Mutex<Vec<String>>,
    }


    #[async_trait::async_trait]
    impl LlmClient for MockDeerFlowLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            // Find the last message (usually user input or tool result)
            let mut content = String::new();
            for m in &req.messages {
                content.push_str(&m.content);
            }

            if content.contains("Decompose the following task") {
                return Ok(ChatResponse {
                    message: Message::assistant(r#"{"subtasks": ["sub1", "sub2"]}"#),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id1".to_string()),
                });
            }

            if content.contains("Synthesize the following subtask results") {
                return Ok(ChatResponse {
                    message: Message::assistant("Final synthesized result"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id4".to_string()),
                });
            }

            let mut resps = self.responses.lock().await;
            let response = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Default response".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(response),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id-default".to_string()),
            })
        }
    }
    #[tokio::test]
    async fn test_deerflow_orchestration() {
        let client = Arc::new(MockDeerFlowLlmClient {
            responses: Mutex::new(vec![
                "Result for sub1".to_string(),
                "Result for sub2".to_string(),
            ]),
        });

        let lead_agent = Arc::new(Agent::new(client, vec![]));
        let orchestrator = DeerFlowOrchestrator::new(lead_agent);
        let cfg = AgentRunConfig::default();

        let on_event = |_e: AgentEvent| {};

        let result = orchestrator.execute("Build a complex system", &cfg, on_event).await;

        assert!(result.is_ok(), "DeerFlow execution failed: {:?}", result.err());
        let final_output = result.unwrap();
        assert!(final_output.contains("Final synthesized result"));
    }
}
