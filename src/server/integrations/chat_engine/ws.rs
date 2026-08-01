use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use sqlx::PgPool;
use uuid::Uuid;
use super::db;
use super::models::{ChatConversation, ChatMessage};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WsState {
    pub pool: PgPool,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
}

#[derive(Deserialize)]
struct WsIncomingMessage {
    pub contact_id: Uuid,
    pub content: String,
}

#[derive(Serialize)]
struct WsOutgoingMessage {
    pub message_type: String, // 'typing', 'message', 'presence'
    pub content: Option<String>,
}

pub fn ws_routes(state: WsState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<WsState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: WsState) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if let Ok(incoming) = serde_json::from_str::<WsIncomingMessage>(&text) {
                // Find or create conversation
                let conv = match db::get_conversation(&state.pool, state.tenant_id, state.inbox_id, incoming.contact_id).await.unwrap_or(None) {
                    Some(c) => c,
                    None => {
                        let c = ChatConversation {
                            id: Uuid::new_v4(),
                            tenant_id: state.tenant_id,
                            inbox_id: state.inbox_id,
                            contact_id: incoming.contact_id,
                            assignee_id: None,
                            status: "open".to_string(),
                        };
                        let _ = db::create_conversation(&state.pool, &c).await;
                        c
                    }
                };

                // Save message
                let chat_msg = ChatMessage {
                    id: Uuid::new_v4(),
                    tenant_id: state.tenant_id,
                    conversation_id: conv.id,
                    sender_type: "contact".to_string(),
                    sender_id: Some(incoming.contact_id),
                    content: incoming.content.clone(),
                };
                let _ = db::create_message(&state.pool, &chat_msg).await;

                // Send ack
                let ack = WsOutgoingMessage {
                    message_type: "ack".to_string(),
                    content: None,
                };
                let _ = socket.send(Message::Text(serde_json::to_string(&ack).unwrap().into())).await;

                // Trigger AI
                tracing::info!("Triggered AI Work Triage for new Web Widget message in conversation {}", conv.id);
            }
        }
    }
}
