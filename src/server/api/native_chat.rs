use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message as WsMessage}, Path, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: Arc<ChatService>,
    pub tx: broadcast::Sender<String>, // We will use this to broadcast messages for simplicity
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub inbox_id: Uuid,
    pub sender_name: String,
    pub content: String,
}

pub fn router(chat_service: Arc<ChatService>) -> Router {
    let (tx, _rx) = broadcast::channel(100);
    let state = ChatAppState {
        chat_service,
        tx,
    };

    Router::new()
        .route("/ws/:tenant_id", get(ws_handler))
        .route("/webhook/:tenant_id", post(webhook_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(state): State<ChatAppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, tenant_id))
}

async fn handle_socket(mut socket: WebSocket, state: ChatAppState, tenant_id: Uuid) {
    let mut rx = state.tx.subscribe();

    tokio::select! {
        _ = async {
            while let Ok(msg) = rx.recv().await {
                if socket.send(WsMessage::Text(msg)).await.is_err() {
                    break;
                }
            }
        } => {}
        _ = async {
            while let Some(Ok(_msg)) = socket.recv().await {
                // Client to server not fully implemented for this simple mockup
            }
        } => {}
    }
}

async fn webhook_handler(
    Path(tenant_id): Path<Uuid>,
    State(state): State<ChatAppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    let contact = match state.chat_service.create_contact(tenant_id, Some(payload.sender_name.clone()), None, None).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Error creating contact: {}", e);
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let conv = match state.chat_service.start_conversation(tenant_id, payload.inbox_id, contact.id, None).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Error starting conversation: {}", e);
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let msg = match state.chat_service.send_message(tenant_id, conv.id, "contact".to_string(), Some(contact.id), payload.content.clone()).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Error sending message: {}", e);
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Trigger AI drafting in background
    let chat_service = state.chat_service.clone();
    let tx = state.tx.clone();
    let content = payload.content.clone();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await; // simulate AI delay
        let draft_reply = format!("AI Draft: Thank you for your message: '{}'. We will get back to you shortly.", content);

        let _ = chat_service.send_message(
            tenant_id,
            conv.id,
            "agent_draft".to_string(),
            None,
            draft_reply.clone(),
        ).await;

        let _ = tx.send(serde_json::json!({
            "type": "new_message",
            "conversation_id": conv.id,
            "message": draft_reply,
        }).to_string());
    });

    let _ = state.tx.send(serde_json::json!({
        "type": "new_message",
        "conversation_id": conv.id,
        "message": payload.content,
    }).to_string());

    axum::http::StatusCode::OK.into_response()
}
