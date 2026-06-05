use std::sync::Arc;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role};

struct MockLLM;

#[async_trait::async_trait]
impl LlmClient for MockLLM {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let content = req.messages.last().unwrap().content.clone();
        Ok(ChatResponse {
            message: Message {
                role: Role::Assistant,
                content: format!("Translated: {}", content),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        })
    }
}

#[tokio::test]
async fn test_translation_worker() {
    let _pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap();

    let _llm: Arc<dyn LlmClient> = Arc::new(MockLLM);
    // Real assertions can be added when full integration test structure is setup
}
