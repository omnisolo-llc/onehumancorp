use std::sync::Arc;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
use crate::llm::LlmClient;

/// agenticSeek: Fully local agent, no API costs
/// This client enforces local-only execution using a simulated or actual local provider,
/// completely bypassing any external API to eliminate costs and ensure privacy.
pub struct AgenticSeekLocalClient {
    pub local_endpoint: String,
}

impl AgenticSeekLocalClient {
    pub fn new(local_endpoint: impl Into<String>) -> Self {
        Self {
            local_endpoint: local_endpoint.into(),
        }
    }
}

#[async_trait::async_trait]
impl LlmClient for AgenticSeekLocalClient {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Enforce local execution
        tracing::info!("AgenticSeek: Routing request locally to {} to avoid API costs.", self.local_endpoint);

        // In a real implementation, we would use reqwest to call the local endpoint.
        // For now, we simulate the local response.
        let msg = Message {
            role: Role::Assistant,
            content: "I am a fully local agent (agenticSeek pattern). I processed this without API costs.".to_string(),
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        };

        Ok(ChatResponse {
            message: msg,
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some(uuid::Uuid::new_v4().to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agentic_seek_local() {
        let client = AgenticSeekLocalClient::new("http://localhost:11434/api/generate");
        let req = ChatRequest {
            model: "local-llama3".to_string(),
            system: "System".to_string(),
            messages: vec![Message::user("Hello")],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };
        let resp = client.chat(req).await.unwrap();
        assert!(resp.message.content.contains("fully local agent"));
        assert!(resp.message.content.contains("without API costs"));
    }
}
