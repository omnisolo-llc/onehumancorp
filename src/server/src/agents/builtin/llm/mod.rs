use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub mod anthropic;
pub mod openai;
pub mod ollama;
pub mod gemini;
