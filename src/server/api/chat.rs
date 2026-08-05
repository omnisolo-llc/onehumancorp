use axum::Json;
use serde_json::Value;

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

use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::db::DB;

#[derive(Clone)]
pub struct ChatAppState {
    pub db: Arc<DB>,
}

pub async fn list_chat_conversations_handler(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match crate::strict_ui_claim_tenant(&claims) {
        Some(t) => {
            if let Ok(uid) = Uuid::parse_str(&t) {
                uid
            } else {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid tenant ID format" }))).into_response();
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let chat_service = match &state.db.store {
        crate::db::DbStore::Postgres => ChatService::new(state.db.pool.clone()),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Unsupported database store" }))).into_response(),
    };

    match chat_service.get_conversations(tenant_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get chat conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal server error" }))).into_response()
        }
    }
}

pub async fn list_chat_messages_handler(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<String>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match crate::strict_ui_claim_tenant(&claims) {
        Some(t) => {
            if let Ok(uid) = Uuid::parse_str(&t) {
                uid
            } else {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid tenant ID format" }))).into_response();
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let conv_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid conversation ID format" }))).into_response(),
    };

    let chat_service = match &state.db.store {
        crate::db::DbStore::Postgres => ChatService::new(state.db.pool.clone()),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Unsupported database store" }))).into_response(),
    };

    match chat_service.get_messages(tenant_id, conv_uuid).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get chat messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal server error" }))).into_response()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SendMessagePayload {
    pub content: String,
    pub sender_type: String, // "contact" or "agent"
    pub sender_id: Option<String>,
}

pub async fn send_chat_message_handler(
    State(state): State<ChatAppState>,
    Path(conversation_id): Path<String>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    let tenant_id = match crate::strict_ui_claim_tenant(&claims) {
        Some(t) => {
            if let Ok(uid) = Uuid::parse_str(&t) {
                uid
            } else {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid tenant ID format" }))).into_response();
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let conv_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid conversation ID format" }))).into_response(),
    };

    let sender_uuid = match payload.sender_id {
        Some(s) => match Uuid::parse_str(&s) {
            Ok(id) => Some(id),
            Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid sender ID format" }))).into_response(),
        },
        None => None,
    };

    let chat_service = match &state.db.store {
        crate::db::DbStore::Postgres => ChatService::new(state.db.pool.clone()),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Unsupported database store" }))).into_response(),
    };

    match chat_service.send_message(tenant_id, conv_uuid, payload.sender_type, sender_uuid, payload.content).await {
        Ok(message) => (StatusCode::OK, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to send chat message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal server error" }))).into_response()
        }
    }
}

pub async fn chat_webhook_ingress_handler(
    State(state): State<ChatAppState>,
    Path(_channel): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let tenant_id_str = payload.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("");
    let tenant_id = match Uuid::parse_str(tenant_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Missing or invalid tenant_id in payload" }))).into_response(),
    };

    let chat_service = match &state.db.store {
        crate::db::DbStore::Postgres => ChatService::new(state.db.pool.clone()),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Unsupported database store" }))).into_response(),
    };

    // Store in webhook ingress table
    match chat_service.ingest_webhook(tenant_id, payload.clone()).await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to ingest webhook payload: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to store webhook payload" }))).into_response();
        }
    }

    // Now, if it's an SMS or something, mock the conversation creation and message send for E2E
    let content = payload.get("message").and_then(|v| v.as_str()).unwrap_or("No content");
    let sender_id_str = payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("");
    let sender_id = Uuid::parse_str(sender_id_str).ok();

    // In a real app we'd look up the conversation ID by contact/channel, but for demonstration we'll just create one or pick the first
    if let Ok(convs) = chat_service.get_conversations(tenant_id).await {
        let conv_id = if let Some(conv) = convs.first() {
            conv.id
        } else {
            // Need an inbox and contact to create a conversation
            if let Ok(inbox) = chat_service.create_inbox(tenant_id, "Main Inbox".to_string()).await {
                if let Ok(contact) = chat_service.create_contact(tenant_id, Some("Webhook Contact".to_string()), None, None).await {
                     if let Ok(conv) = chat_service.start_conversation(tenant_id, inbox.id, contact.id, None).await {
                         conv.id
                     } else {
                         return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to start conversation" }))).into_response();
                     }
                } else {
                     return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to create contact" }))).into_response();
                }
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to create inbox" }))).into_response();
            }
        };

        match chat_service.send_message(tenant_id, conv_id, "contact".to_string(), sender_id, content.to_string()).await {
            Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
            Err(e) => {
                 tracing::error!("Failed to send chat message from webhook: {}", e);
                 (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal server error" }))).into_response()
            }
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal server error" }))).into_response()
    }
}
