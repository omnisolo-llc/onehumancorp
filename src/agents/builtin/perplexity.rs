use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
use std::sync::Arc;

/// Perplexity Archetype Implementation
/// Simulates a Perplexity-style harness where the agent heavily relies on search
/// and citations to answer queries.

#[async_trait::async_trait]
pub trait PerplexityLlmClient: Send + Sync {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct PerplexityAgent {
    pub llm: Arc<dyn PerplexityLlmClient>,
    pub model: String,
}

impl PerplexityAgent {
    pub fn new(llm: Arc<dyn PerplexityLlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    pub async fn execute_query(&self, query: &str) -> Result<String, String> {
        let system_prompt = "You are a Perplexity-style search agent. Your primary function is to provide accurate, comprehensive, and up-to-date answers to user queries by performing web searches and synthesizing the results. You must cite your sources.";

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(query)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.1, // Low temperature for factual responses
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content),
            Err(e) => Err(format!("Perplexity LLM Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    struct MockPerplexityLlm {
        response: String,
    }

    #[async_trait::async_trait]
    impl PerplexityLlmClient for MockPerplexityLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.response),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_perplexity_flow() {
        let llm = Arc::new(MockPerplexityLlm {
            response: "According to source [1], the sky is blue. [1] https://example.com".to_string(),
        });
        let agent = PerplexityAgent::new(llm, "test-model".to_string());

        let result = agent.execute_query("Why is the sky blue?").await.unwrap();
        assert!(result.contains("According to source [1]"));
        assert!(result.contains("[1] https://example.com"));
    }
}
