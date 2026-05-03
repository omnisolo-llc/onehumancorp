use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

#[path = "../../../src/agents/builtin/llm/anthropic.rs"]
pub mod anthropic;
#[path = "../../../src/agents/builtin/llm/openai.rs"]
pub mod openai;
#[path = "../../../src/agents/builtin/llm/ollama.rs"]
pub mod ollama;
#[path = "../../../src/agents/builtin/llm/gemini.rs"]
pub mod gemini;
