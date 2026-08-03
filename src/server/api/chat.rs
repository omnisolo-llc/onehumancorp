use axum::{extract::{State, Path}, Json};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatConversation, ChatContact, ChatMessage};
use server_common::Claims;
use axum::response::IntoResponse;

#[derive(serde::Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct ConversationWithContactResponse {
    pub conversation: ChatConversation,
    pub contact: ChatContact,
}

#[derive(serde::Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub async fn list_conversations_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
) -> axum::response::Response {
    let tenant_id = match claims.organization_id.and_then(|id| Uuid::parse_str(&id).ok()) {
        Some(id) => id,
        None => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let chat_service = ChatService::new(db.pool.clone());
    match chat_service.get_open_conversations_with_contacts(tenant_id).await {
        Ok(records) => {
            let res: Vec<ConversationWithContactResponse> = records
                .into_iter()
                .map(|(conversation, contact)| ConversationWithContactResponse {
                    conversation,
                    contact,
                })
                .collect();
            (axum::http::StatusCode::OK, Json(res)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch open conversations: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_conversation_messages_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
    Path(conversation_id): Path<Uuid>,
) -> axum::response::Response {
    let tenant_id = match claims.organization_id.and_then(|id| Uuid::parse_str(&id).ok()) {
        Some(id) => id,
        None => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let chat_service = ChatService::new(db.pool.clone());
    match chat_service.get_conversation_messages(tenant_id, conversation_id).await {
        Ok(messages) => (axum::http::StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch conversation messages: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn send_message_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> axum::response::Response {
    let tenant_id = match claims.organization_id.and_then(|id| Uuid::parse_str(&id).ok()) {
        Some(id) => id,
        None => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let chat_service = ChatService::new(db.pool.clone());
    match chat_service
        .send_message(
            tenant_id,
            conversation_id,
            payload.sender_type,
            payload.sender_id,
            payload.content,
        )
        .await
    {
        Ok(message) => (axum::http::StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
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
