use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ohc_builtin_agent::perplexity::PerplexityAgent;

// We use the exact mock from tests, to make the agent testable in e2e isolated environments
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use std::sync::Mutex;

struct E2EPerplexityLlm {
    responses: Mutex<Vec<String>>,
}

#[async_trait::async_trait]
impl ohc_builtin_agent::perplexity::PerplexityLlmClient for E2EPerplexityLlm {
    async fn chat(
        &self,
        _req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut resps = self.responses.lock().unwrap();
        let content = if !resps.is_empty() {
            resps.remove(0)
        } else {
            "According to source [1], the sky is blue. [1] https://example.com".to_string()
        };

        Ok(ChatResponse {
            message: Message::assistant(&content),
            stop_reason: "stop".to_string(),
            response_id: Some("mock-id".to_string()),
            usage: ohc_builtin_agent_core::types::Usage { input_tokens: 0, output_tokens: 0, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
        })
    }
}

#[derive(Deserialize)]
pub struct PerplexityQueryReq {
    pub query: String,
}

#[derive(Serialize)]
pub struct PerplexityQueryResp {
    pub answer: String,
}

async fn handle_perplexity_query(
    Json(req): Json<PerplexityQueryReq>,
) -> Result<Json<PerplexityQueryResp>, String> {
    let llm = Arc::new(E2EPerplexityLlm {
        responses: Mutex::new(vec!["According to source [1], the sky is blue. [1] https://example.com".to_string()]),
    });

    let agent = PerplexityAgent::new(llm, "default".to_string());

    match agent.execute_query(&req.query).await {
        Ok(answer) => Ok(Json(PerplexityQueryResp { answer })),
        Err(e) => Err(e),
    }
}

pub fn router<S: Send + Sync + Clone + 'static>() -> Router<S> {
    Router::new().route("/query", post(handle_perplexity_query))
}
