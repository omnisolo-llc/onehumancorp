use async_trait::async_trait;
use crate::types::{ChatRequest, ChatResponse};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub mod anthropic;
pub mod openai;
pub mod ollama;
