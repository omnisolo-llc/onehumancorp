use crate::agent::{Agent, AgentRunConfig};
use crate::types::{ChatRequest, Message, ToolError};
use futures::future::join_all;
use std::sync::Arc;

/// DeerFlow Unique Harness Innovations: Built-in observability
/// LangSmith and Langfuse integration placeholder to fulfill the telemetry/observability pattern.
pub trait ObservabilityBackend: Send + Sync {
    fn record_trace(&self, agent_id: &str, task: &str, result: &str, duration_ms: u64);
}

pub struct LangSmithMock;
impl ObservabilityBackend for LangSmithMock {
    fn record_trace(&self, agent_id: &str, task: &str, _result: &str, duration_ms: u64) {
        tracing::info!("[LangSmith] trace recorded for agent {}: task length {}, duration {}ms", agent_id, task.len(), duration_ms);
    }
}

pub struct LangfuseMock;
impl ObservabilityBackend for LangfuseMock {
    fn record_trace(&self, agent_id: &str, _task: &str, result: &str, duration_ms: u64) {
        tracing::info!("[Langfuse] trace recorded for agent {}: result length {}, duration {}ms", agent_id, result.len(), duration_ms);
    }
}

/// DeerFlow Unique Harness Innovations: Sub-agent orchestration.
/// Lead agent decomposes tasks, spawns parallel sub-agents, synthesizes results.
pub struct DeerFlowOrchestrator {
    pub lead_agent: Arc<Agent>,
    pub config: AgentRunConfig,
    pub observability: Vec<Arc<dyn ObservabilityBackend>>,
}

impl DeerFlowOrchestrator {
    pub fn new(lead_agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self { lead_agent, config, observability: vec![] }
    }

    pub fn with_observability(mut self, backend: Arc<dyn ObservabilityBackend>) -> Self {
        self.observability.push(backend);
        self
    }

    pub async fn orchestrate(&self, task: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = std::time::Instant::now();

        // Step 1: Decompose task
        let decompose_prompt = format!(
            "You are the lead agent. Decompose the following task into independent sub-tasks that can be executed in parallel. \n\
            Return a JSON array of strings, where each string is a sub-task description.\n\nTask: {}",
            task
        );

        let mut on_event = |_| {};
        let subtasks_json = self.lead_agent.run(&self.config, &decompose_prompt, &mut on_event).await?;

        let subtasks: Vec<String> = serde_json::from_str(&subtasks_json).unwrap_or_else(|_| {
            // Fallback if not valid JSON
            vec![task.to_string()]
        });

        if subtasks.is_empty() {
            return Ok("No subtasks generated.".to_string());
        }

        // Step 2: Spawn parallel sub-agents
        let mut futures = Vec::new();
        for subtask in subtasks {
            let agent_clone = self.lead_agent.clone();
            let config_clone = self.config.clone();

            futures.push(tokio::spawn(async move {
                let mut local_on_event = |_| {};
                let result = agent_clone.run(&config_clone, &subtask, &mut local_on_event).await.unwrap_or_else(|e| format!("Error: {}", e));

                // Enforce condensed summary (1k-2k tokens)
                let max_length = 2000;
                let condensed = if result.chars().count() > max_length {
                    format!("{}... [Condensed]", result.chars().take(max_length).collect::<String>())
                } else {
                    result
                };

                format!("Subtask: {}\nResult: {}", subtask, condensed)
            }));
        }

        let results = join_all(futures).await;
        let mut combined_results = String::new();
        for res in results {
            if let Ok(content) = res {
                combined_results.push_str(&content);
                combined_results.push_str("\n\n---\n\n");
            }
        }

        // Step 3: Synthesize results
        let synthesize_prompt = format!(
            "You are the lead agent. You decomposed the task: '{}' into subtasks. \n\
            The subagents have completed their work. Here are their results:\n\n{}\n\n\
            Synthesize these results into a final, cohesive response to the original task.",
            task, combined_results
        );

        let final_result = self.lead_agent.run(&self.config, &synthesize_prompt, &mut on_event).await?;

        let duration_ms = start_time.elapsed().as_millis() as u64;
        for backend in &self.observability {
            backend.record_trace(&self.config.agent_id, task, &final_result, duration_ms);
        }

        Ok(final_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockDeerFlowLlm {
        call_count: Mutex<usize>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockDeerFlowLlm {
        async fn chat(&self, req: crate::types::ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut count = self.call_count.lock().await;
            *count += 1;

            let response_text = if *count == 1 {
                r#"["Subtask 1", "Subtask 2"]"#.to_string()
            } else if req.messages.last().unwrap().content.contains("Synthesize") {
                "Final synthesized result".to_string()
            } else {
                "Subagent executed subtask".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(response_text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }

        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![])
        }
    }

    struct MockBackend {
        pub recorded: Arc<std::sync::Mutex<bool>>,
    }

    impl ObservabilityBackend for MockBackend {
        fn record_trace(&self, _agent_id: &str, _task: &str, _result: &str, _duration_ms: u64) {
            let mut lock = self.recorded.lock().unwrap();
            *lock = true;
        }
    }

    #[tokio::test]
    async fn test_deerflow_orchestration_with_observability() {
        let llm = Arc::new(MockDeerFlowLlm { call_count: Mutex::new(0) });
        let agent = Arc::new(Agent::new(llm, vec![]));
        let config = AgentRunConfig::default();

        let recorded_flag = Arc::new(std::sync::Mutex::new(false));
        let backend = Arc::new(MockBackend { recorded: recorded_flag.clone() });

        let orchestrator = DeerFlowOrchestrator::new(agent, config)
            .with_observability(backend);

        let result = orchestrator.orchestrate("Do a complex task").await.unwrap();

        assert_eq!(result, "Final synthesized result");

        let was_recorded = *recorded_flag.lock().unwrap();
        assert!(was_recorded, "Observability backend should have been called");
    }
}
