use axum::{Json, Router, routing::post};
use omnisolo_builtin_agent::perplexity::{PerplexityAgent, PerplexityLlmClient};
use omnisolo_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

struct E2EPerplexityLlm {
    responses: Mutex<Vec<String>>,
}

#[async_trait::async_trait]
impl PerplexityLlmClient for E2EPerplexityLlm {
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
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("mock-id".to_string()),
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
        responses: Mutex::new(vec![
            "According to source [1], the sky is blue. [1] https://example.com".to_string(),
        ]),
    });

    let agent = PerplexityAgent::new(llm, "default".to_string());

    match agent.execute_query(&req.query).await {
        Ok(answer) => Ok(Json(PerplexityQueryResp { answer })),
        Err(e) => Err(e),
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/query", post(handle_perplexity_query))
}
