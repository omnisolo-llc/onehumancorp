use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::omnichannel_chat::{IntentClassifier, CopilotDrafter, AutoResponder, HandoffManager, Intent, ChatMessage};

pub struct ChatAppState {
    pub intent_classifier: Arc<IntentClassifier>,
    pub copilot_drafter: Arc<CopilotDrafter>,
    pub auto_responder: Arc<AutoResponder>,
    pub handoff_manager: Arc<HandoffManager>,
}

#[derive(Deserialize)]
pub struct ClassifyRequest {
    message: String,
}

#[derive(Serialize)]
pub struct ClassifyResponse {
    intent: Intent,
}

#[derive(Deserialize)]
pub struct DraftRequest {
    history: Vec<ChatMessage>,
    tone: String,
}

#[derive(Serialize)]
pub struct DraftResponse {
    draft: String,
}

#[derive(Deserialize)]
pub struct AutoRespondRequest {
    message: String,
    intent: Intent,
}

#[derive(Serialize)]
pub struct AutoRespondResponse {
    action_type: String,
    content: String,
}

async fn classify_intent(
    State(state): State<Arc<ChatAppState>>,
    Json(payload): Json<ClassifyRequest>,
) -> Result<Json<ClassifyResponse>, (StatusCode, String)> {
    let intent = state.intent_classifier.classify(&payload.message).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(ClassifyResponse { intent }))
}

async fn draft_response(
    State(state): State<Arc<ChatAppState>>,
    Json(payload): Json<DraftRequest>,
) -> Result<Json<DraftResponse>, (StatusCode, String)> {
    let draft = state.copilot_drafter.draft_response(&payload.history, &payload.tone).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(DraftResponse { draft }))
}

async fn auto_respond(
    State(state): State<Arc<ChatAppState>>,
    Json(payload): Json<AutoRespondRequest>,
) -> Result<Json<AutoRespondResponse>, (StatusCode, String)> {
    let action = state.auto_responder.process(&payload.message, &payload.intent).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    match action {
        crate::omnichannel_chat::AutoResponderAction::Reply(content) => {
            Ok(Json(AutoRespondResponse {
                action_type: "reply".to_string(),
                content,
            }))
        }
        crate::omnichannel_chat::AutoResponderAction::Handoff(reason) => {
            Ok(Json(AutoRespondResponse {
                action_type: "handoff".to_string(),
                content: reason,
            }))
        }
    }
}

pub fn create_router(state: Arc<ChatAppState>) -> Router {
    Router::new()
        .route("/api/chat/classify", post(classify_intent))
        .route("/api/chat/draft", post(draft_response))
        .route("/api/chat/auto_respond", post(auto_respond))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
    use axum::http::Request;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use crate::llm::LlmClient;

    struct MockLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.response_text.clone()),
                usage: ohc_builtin_agent_core::types::Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    fn mock_state(response_text: &str) -> Arc<ChatAppState> {
        let llm = Arc::new(MockLlm { response_text: response_text.to_string() });
        Arc::new(ChatAppState {
            intent_classifier: Arc::new(IntentClassifier::new(llm.clone())),
            copilot_drafter: Arc::new(CopilotDrafter::new(llm.clone())),
            auto_responder: Arc::new(AutoResponder::new(llm.clone(), 0.9)),
            // Mock redis connection is hard to do here without setting up a real server, so we skip HandoffManager tests in API unit tests for now
            handoff_manager: Arc::new(HandoffManager::new("redis://127.0.0.1:6379").unwrap_or(HandoffManager::new("redis://localhost").unwrap())),
        })
    }

    #[tokio::test]
    async fn test_classify_intent_api() {
        let state = mock_state("Support");
        let app = create_router(state);

        let response = app.oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/chat/classify")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"message": "help"}"#))
                .unwrap()
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["intent"], "Support");
    }

    use serde_json::Value;
}
