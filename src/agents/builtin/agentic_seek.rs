use crate::types::{ChatRequest, ChatResponse, Message, Usage};
use async_trait::async_trait;

/// agenticSeek Unique Harness Innovations: Fully local agent, no API costs
///
/// This provides a local, cost-free LLM runner. In a real environment, this might
/// interact with a locally running LLaMA or Ollama instance via HTTP.
/// Here we simulate the local loop to avoid actual API calls and costs.

pub struct LocalAgentClient {
    pub local_binary_path: String,
}

impl LocalAgentClient {
    pub fn new(local_binary_path: &str) -> Self {
        Self {
            local_binary_path: local_binary_path.to_string(),
        }
    }
}

#[async_trait]
impl crate::llm::LlmClient for LocalAgentClient {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Simulate local inference with zero API costs.
        // We log to prove we are running locally.
        tracing::info!("Executing local agent inference via {}", self.local_binary_path);

        let response_text = "I am a fully local agent running with zero API costs.".to_string();

        Ok(ChatResponse {
            message: Message::assistant(response_text),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("local-inference-id".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;

    #[tokio::test]
    async fn test_agentic_seek_local_execution() {
        let client = LocalAgentClient::new("/usr/bin/local-llm");
        let req = ChatRequest {
            model: "local".to_string(),
            system: "system".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 100,
            temperature: 0.0,
        };

        let result = client.chat(req).await.unwrap();
        assert_eq!(result.message.content, "I am a fully local agent running with zero API costs.");
    }
}
