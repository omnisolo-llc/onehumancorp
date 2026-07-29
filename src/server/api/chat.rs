use axum::{
    extract::{Extension, State, Path, WebSocketUpgrade},
    response::IntoResponse,
    http::StatusCode,
    routing::{post, get},
    Router,
    Json,
};
use axum::extract::ws::{Message, WebSocket};
use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use ::server_common::Claims;
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

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

#[derive(serde::Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct CreateChannelRequest {
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Value,
}

#[derive(serde::Deserialize)]
pub struct CreateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub omnichannel_ids: Value,
}

#[derive(serde::Deserialize)]
pub struct StartConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

#[derive(serde::Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub status: Option<String>,
}

#[derive(Clone)]
pub struct ChatApiState {
    pub chat_service: Arc<ChatService>,
    pub tx: broadcast::Sender<String>,
}

pub fn native_chat_router<S>(chat_service: Arc<ChatService>, tx: broadcast::Sender<String>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ChatApiState { chat_service, tx };
    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/channels", post(create_channel))
        .route("/contacts", post(create_contact))
        .route("/conversations", post(start_conversation))
        .route("/messages", post(send_message))
        .route("/messages/:message_id/approve", post(approve_message))
        .route("/ws", get(chat_ws_handler))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.chat_service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create inbox").into_response(),
    }
}

async fn create_channel(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.chat_service.create_channel(tenant_id, payload.inbox_id, payload.channel_type, payload.config).await {
        Ok(channel) => (StatusCode::CREATED, Json(channel)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create channel").into_response(),
    }
}

async fn create_contact(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateContactRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.chat_service.create_contact(tenant_id, payload.name, payload.email, payload.phone, payload.omnichannel_ids).await {
        Ok(contact) => (StatusCode::CREATED, Json(contact)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create contact").into_response(),
    }
}

async fn start_conversation(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<StartConversationRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.chat_service.start_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conv) => (StatusCode::CREATED, Json(conv)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start conversation").into_response(),
    }
}

async fn send_message(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let status = payload.status.unwrap_or_else(|| "sent".to_string());

    match state.chat_service.send_message(tenant_id, payload.conversation_id, payload.sender_type, payload.sender_id, payload.content, status).await {
        Ok(msg) => {
            let _ = state.tx.send(serde_json::to_string(&msg).unwrap_or_default());
            (StatusCode::CREATED, Json(msg)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send message").into_response(),
    }
}

async fn approve_message(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
    Path(message_id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match state.chat_service.approve_message(tenant_id, message_id).await {
        Ok(msg) => {
            let _ = state.tx.send(serde_json::to_string(&msg).unwrap_or_default());
            (StatusCode::OK, Json(msg)).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to approve message").into_response(),
    }
}

async fn chat_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ChatApiState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id) => Uuid::parse_str(id).unwrap_or(Uuid::nil()),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, tenant_id))
}

async fn handle_socket(socket: WebSocket, state: ChatApiState, tenant_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Simplified check, in reality, you'd parse `msg` and check tenant_id
            if msg.contains(&tenant_id.to_string()) {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(_text))) = receiver.next().await {
            // Handle incoming WS messages if needed
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
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
