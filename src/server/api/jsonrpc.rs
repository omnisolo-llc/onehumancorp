use axum::{extract::State, response::IntoResponse, routing::post, Router};
use std::sync::Arc;
use ohc_builtin_agent::codex_runner::{AppServer, Runner, CodexCore};
use ohc_builtin_agent::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};

struct MockLlmClient;

#[async_trait::async_trait]
impl LlmClient for MockLlmClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut msg = String::new();
        msg.push_str("Chapter 1: Intro. Chapter 2: A. Chapter 3: B. Chapter 4: C. Chapter 5: D. Chapter 6: E. Chapter 7: F. Chapter 8: Conclusion. Chart: Market Trends. Analysis: Deep. ");
        msg.push_str(&"word ".repeat(20000));
        Ok(ChatResponse {
            message: Message::assistant(msg),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("mock-id".to_string()),
        })
    }
}

pub fn router() -> Router {
    let llm = Arc::new(MockLlmClient);
    let agent = Arc::new(Agent::new(llm, vec![]));
    let core = Arc::new(CodexCore::new(agent, AgentRunConfig::default()));
    let runner = Arc::new(Runner::new_with_core(core));
    let app_server = Arc::new(AppServer::new(runner));

    Router::new()
        .route("/", post(handle_rpc))
        .route("/rpc", post(handle_rpc))
        .with_state(app_server)
}

async fn handle_rpc(
    State(app_server): State<Arc<AppServer>>,
    body: String,
) -> impl IntoResponse {
    let result = app_server.handle_request(&body).await;
    axum::response::Json(serde_json::from_str::<serde_json::Value>(&result).unwrap_or(serde_json::Value::Null))
}
