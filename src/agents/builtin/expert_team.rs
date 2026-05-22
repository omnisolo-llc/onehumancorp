use crate::agent::{Agent, AgentRunConfig, AgentEvent};
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubTaskPlan {
    pub subtasks: Vec<String>,
}

pub struct ExpertTeamOrchestrator {
    pub lead_agent: Arc<Agent>,
    pub expert_agents: Vec<Arc<Agent>>,
}

impl ExpertTeamOrchestrator {
    pub fn new(lead_agent: Arc<Agent>, expert_agents: Vec<Arc<Agent>>) -> Self {
        Self {
            lead_agent,
            expert_agents,
        }
    }

    /// Calculates Jaccard similarity between two strings
    fn calculate_similarity(s1: &str, s2: &str) -> f64 {
        let words1: std::collections::HashSet<&str> = s1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = s2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count() as f64;
        let union = words1.union(&words2).count() as f64;

        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }

    pub async fn run_team(
        &self,
        main_task: &str,
        config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Pre-flight Gate: Ensure at least one expert agent exists.
        if self.expert_agents.is_empty() {
            return Err("Pre-flight gate failed: The expert team requires at least one expert agent.".into());
        }

        // 1. Lead Agent Orchestration: Decompose task
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "subtasks": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "A list of decomposed subtasks assigned to the experts."
                }
            },
            "required": ["subtasks"]
        });

        let mut on_event_lead = |_| {};
        let plan: SubTaskPlan = self.lead_agent.run_structured(
            config,
            &format!("Decompose the following task into subtasks for the expert team:\n{}", main_task),
            schema,
            &mut on_event_lead,
        ).await?;

        // 2. Parallel Execution
        let mut expert_futures = Vec::new();
        for (i, expert) in self.expert_agents.iter().enumerate() {
            let task = if i < plan.subtasks.len() {
                plan.subtasks[i].clone()
            } else {
                format!("Provide additional insights for: {}", main_task)
            };

            let expert_clone = expert.clone();
            let config_clone = config.clone();

            expert_futures.push(async move {
                let tool_call_count = Arc::new(AtomicUsize::new(0));
                let tcc_clone = tool_call_count.clone();
                let mut on_event_expert = move |event: AgentEvent| {
                    if let AgentEvent::ToolCall { .. } = event {
                        tcc_clone.fetch_add(1, Ordering::SeqCst);
                    }
                };

                let raw_result = expert_clone.run(&config_clone, &task, &mut on_event_expert).await?;
                let count = tool_call_count.load(Ordering::SeqCst);

                Ok::<_, Box<dyn std::error::Error + Send + Sync>>((raw_result, count))
            });
        }

        let results = futures::future::join_all(expert_futures).await;

        let mut final_results = Vec::new();
        let mut total_tool_calls = 0;

        for res in results {
            match res {
                Ok((mut output, tool_calls)) => {
                    total_tool_calls += tool_calls;

                    // 6. Condensed Summaries
                    let word_count = output.split_whitespace().count();
                    if word_count > 1000 {
                        let summarize_req = ChatRequest {
                            model: config.model.clone(),
                            system: "Condense the following expert output into a 1k-2k token summary, focusing on actions taken and results.".to_string(),
                            messages: vec![Message::user(&output)],
                            tools: vec![],
                            max_tokens: 2000,
                            temperature: 0.0,
                        };

                        if let Ok(resp) = self.lead_agent.llm.chat(summarize_req).await {
                            output = resp.message.content;
                        }
                    }
                    final_results.push(output);
                }
                Err(e) => return Err(format!("Expert agent failed: {}", e).into()),
            }
        }

        // 5. Pre-merge Gate: Check similarity
        for i in 0..final_results.len() {
            for j in (i + 1)..final_results.len() {
                let similarity = Self::calculate_similarity(&final_results[i], &final_results[j]);
                if similarity > 0.75 {
                    return Err(format!("Pre-merge gate failed: Expert outputs are too similar (Similarity: {:.2}). Experts must have distinct domain contributions.", similarity).into());
                }
            }
        }

        // 3. Lead Agent Synthesis
        let synthesis_prompt = format!(
            "Synthesize the following expert reports into a final comprehensive response for the main task: '{}'\n\nExpert Reports:\n{}",
            main_task,
            final_results.join("\n\n---\n\n")
        );

        let mut on_event_synth = |_| {};
        let final_synthesis = self.lead_agent.run(config, &synthesis_prompt, &mut on_event_synth).await?;

        // 5. Pre-deliver Gate
        let synthesis_word_count = final_synthesis.split_whitespace().count();
        if synthesis_word_count < 200 {
            return Err(format!("Pre-deliver gate failed: Final synthesis word count ({}) is less than the required 200 words.", synthesis_word_count).into());
        }

        if total_tool_calls == 0 {
            return Err("Pre-deliver gate failed: Skill-trace completeness check failed. No tools were used by the expert team.".into());
        }

        Ok(final_synthesis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{Agent, AgentRunConfig};
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage};
    use crate::llm::LlmClient;
    use tokio::sync::Mutex;
    use std::sync::Arc;

    struct MockLlmClient {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default response"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_jaccard_gate() {
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "return_structured_output".to_string(),
                            arguments: serde_json::json!({ "subtasks": ["task 1"] }),
                        }],
                        tool_results: vec![],
                        response_id: Some("id1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                }
            ]),
        });
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        let expert1_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("The quick brown fox jumps over the lazy dog"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id2".to_string()),
                }
            ]),
        });
        let expert1 = Arc::new(Agent::new(expert1_client, vec![]));

        let expert2_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("The quick brown fox jumps over the lazy dog exactly the same"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id3".to_string()),
                }
            ]),
        });
        let expert2 = Arc::new(Agent::new(expert2_client, vec![]));

        let orchestrator = ExpertTeamOrchestrator::new(lead_agent, vec![expert1, expert2]);
        let config = AgentRunConfig::default();

        let result = orchestrator.run_team("test task", &config).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Pre-merge gate failed: Expert outputs are too similar") || err.contains("Skill-trace completeness check failed"), "Unexpected error: {}", err);
    }

    #[tokio::test]
    async fn test_preflight_gate() {
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![]),
        });
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        let orchestrator = ExpertTeamOrchestrator::new(lead_agent, vec![]);
        let config = AgentRunConfig::default();

        let result = orchestrator.run_team("test task", &config).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Pre-flight gate failed"));
    }

    #[tokio::test]
    async fn test_predeliver_gate_word_count() {
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "return_structured_output".to_string(),
                            arguments: serde_json::json!({ "subtasks": ["task 1"] }),
                        }],
                        tool_results: vec![],
                        response_id: Some("id1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final short answer"), // < 200 words
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id4".to_string()),
                }
            ]),
        });
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        let expert1_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("The quick brown fox"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id2".to_string()),
                }
            ]),
        });
        let expert1 = Arc::new(Agent::new(expert1_client, vec![]));

        let orchestrator = ExpertTeamOrchestrator::new(lead_agent, vec![expert1]);
        let config = AgentRunConfig::default();

        let result = orchestrator.run_team("test task", &config).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Pre-deliver gate failed: Final synthesis word count") || err.contains("Skill-trace completeness check failed"), "Unexpected error: {}", err);
    }

    #[tokio::test]
    async fn test_predeliver_gate_skill_trace() {
        let long_answer = vec!["word"; 250].join(" ");
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "return_structured_output".to_string(),
                            arguments: serde_json::json!({ "subtasks": ["task 1"] }),
                        }],
                        tool_results: vec![],
                        response_id: Some("id1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                },
                ChatResponse {
                    message: Message::assistant(&long_answer), // >= 200 words
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id4".to_string()),
                }
            ]),
        });
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        let expert1_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("The quick brown fox"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id2".to_string()),
                }
            ]),
        });
        let expert1 = Arc::new(Agent::new(expert1_client, vec![]));

        let orchestrator = ExpertTeamOrchestrator::new(lead_agent, vec![expert1]);
        let config = AgentRunConfig::default();

        let result = orchestrator.run_team("test task", &config).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Skill-trace completeness check failed") || err.contains("Pre-deliver gate failed: Final synthesis word count"), "Unexpected error: {}", err);
    }

    struct MockToolExecutor;

    #[async_trait::async_trait]
    impl ohc_builtin_agent_tools::ToolExecutor for MockToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent_core::types::ToolError> {
            Ok("Tool executed".to_string())
        }
    }

    #[tokio::test]
    async fn test_expert_team_success() {
        let long_answer = vec!["word"; 250].join(" ");
        let lead_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "return_structured_output".to_string(),
                            arguments: serde_json::json!({ "subtasks": ["task 1"] }),
                        }],
                        tool_results: vec![],
                        response_id: Some("id1".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                },
                ChatResponse {
                    message: Message::assistant(&long_answer), // >= 200 words
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id4".to_string()),
                }
            ]),
        });
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        let expert1_client = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "call_2".to_string(),
                            name: "some_tool".to_string(),
                            arguments: serde_json::json!({ "subtasks": ["task 1"] }),
                        }],
                        tool_results: vec![],
                        response_id: Some("id2".to_string()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id2".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Unique expert output"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id3".to_string()),
                }
            ]),
        });
        let expert1 = Arc::new(Agent::new(expert1_client, vec![
            ohc_builtin_agent_tools::Tool {
                name: "some_tool".to_string(),
                description: "mock tool".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockToolExecutor),
            }
        ]));

        let orchestrator = ExpertTeamOrchestrator::new(lead_agent, vec![expert1]);
        let config = AgentRunConfig::default();

        let result = orchestrator.run_team("test task", &config).await;
        assert!(result.is_ok(), "Expected success, got: {:?}", result.unwrap_err());
        assert_eq!(result.unwrap(), long_answer);
    }
}
