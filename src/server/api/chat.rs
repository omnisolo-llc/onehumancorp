use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use sqlx::PgPool;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: Arc<ChatService>,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = ChatAppState {
        chat_service: Arc::new(ChatService::new(pool)),
    };

    Router::new()
        .route("/api/v1/chat/inboxes/{inbox_id}/conversations", post(create_conversation))
        .route("/api/v1/chat/conversations/{conversation_id}/messages", post(send_message))
        .route("/api/v1/chat/inboxes/{tenant_id}/{inbox_id}/conversations", get(list_conversations)) // Using tenant_id for RLS/security if needed
        .with_state(state)
}

async fn create_conversation(
    State(state): State<ChatAppState>,
    Path(inbox_id): Path<Uuid>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    if inbox_id != payload.inbox_id {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    match state.chat_service.start_conversation(
        payload.tenant_id,
        payload.inbox_id,
        payload.contact_id,
        payload.assignee_id,
    ).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn send_message(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    if conversation_id != payload.conversation_id {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    match state.chat_service.send_message(
        payload.tenant_id,
        payload.conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    ).await {
        Ok(message) => Ok(Json(message)),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_conversations(
    State(state): State<ChatAppState>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    // I need to implement this in ChatService.
    match state.chat_service.list_conversations(tenant_id, inbox_id).await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => {
            tracing::error!("Failed to list conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


#[derive(serde::Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

pub async fn help_chat_handler(Json(req): Json<ChatRequest>) -> Json<Value> {
    let articles = crate::api::docs::get_articles();
    let query = req.message.to_lowercase();

    // We fetch articles once.
    let mut prompt = String::from("You are the OneHumanCorp (OHC) AI Help Agent. Answer the user's question concisely using ONLY the provided help center knowledge. Include a recommendation to read the full article if relevant.\
\
Help Center Knowledge:\
");
    for a in &articles {
        prompt.push_str(&format!("Title: {}\
Category: {}\
Content: {}\
---\
", a.title, a.category, a.desc));
    }
    prompt.push_str(&format!("\
User Question: {}\
", query));

    // Default fallback URL and Title
    let mut link_url = "/help/getting-started-1".to_string();
    let link_title = "Read the full article →".to_string();

    // Guess the best link in case LLM doesn't output it
    for article in &articles {
        if query.contains(&article.title.to_lowercase()) || query.contains(&article.category.to_lowercase()) {
            link_url = article.link.clone();
            break;
        }
    }

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "dummy_key".to_string());
    let client = crate::minimax::MinimaxClient::new(api_key);

    let reply = match client.reason(&prompt).await {
        Ok(res) => res,
        Err(_) => {
             // Fallback to simple matching if LLM fails
             let mut fb = "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.".to_string();
             for article in &articles {
                 if query.contains(&article.title.to_lowercase()) || query.contains(&article.category.to_lowercase()) {
                     fb = format!("Based on our help center: {}", article.desc);
                     link_url = article.link.clone();
                     break;
                 }
             }
             fb
        }
    };

    Json(serde_json::json!({
        "reply": reply,
        "link": {
            "title": link_title,
            "url": link_url
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;

    #[tokio::test]
    async fn test_help_chat_handler_fallback() {
        let req = ChatRequest {
            message: "getting started".to_string(),
        };

        // This will fall back due to 'dummy_key'
        let response = help_chat_handler(Json(req)).await.0;

        assert!(response["reply"].as_str().unwrap().contains("Welcome to OneHumanCorp"));
        assert_eq!(response["link"]["url"].as_str().unwrap(), "/help/getting-started-1");
    }
}
