
use ohc_builtin_agent_core::types::{Message, Role, ToolResult};
use crate::agent::AgentRunConfig;
use std::sync::Arc;
use crate::llm::LlmClient;

pub enum VerificationType {
    ComputationalGuides,
    Visual,
    InferentialSensors,
}

pub struct VerificationManager {
    llm_client: Arc<dyn LlmClient + Send + Sync>,
}

impl VerificationManager {
    pub fn new(llm_client: Arc<dyn LlmClient + Send + Sync>) -> Self {
        Self { llm_client }
    }

    pub async fn run_inferential_sensor(&self, proposed_output: &str, task_context: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "You are an LLM-as-a-judge. Please verify if the proposed output perfectly satisfies the task context.\nTask: {}\nOutput: {}\nRespond strictly with 'APPROVE' or 'REJECT'.",
            task_context, proposed_output
        );

        let mut messages = vec![];
        messages.push(Message {
            role: Role::User,
            content: prompt,
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        });

        let resp = self.llm_client.chat(ohc_builtin_agent_core::types::ChatRequest {
            model: "sensor_model".to_string(),
            system: "".to_string(),
            messages,
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        }).await?;

        Ok(resp.message.content.contains("APPROVE"))
    }

    pub fn run_computational_guide(&self, output: &str) -> Result<(), String> {
        if output.contains("ERROR:") {
            return Err("Computational guide failed: Linter error detected.".to_string());
        }
        Ok(())
    }
}
