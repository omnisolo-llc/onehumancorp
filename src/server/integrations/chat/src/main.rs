use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    // Shared state would go here (e.g., db connection pool)
}

#[derive(Serialize, Deserialize, Clone)]
struct Conversation {
    id: Uuid,
    tenant_id: Uuid,
    inbox_id: Uuid,
    contact_id: Uuid,
    status: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatMessage {
    id: Uuid,
    tenant_id: Uuid,
    conversation_id: Uuid,
    content: String,
    sender_type: String,
    sender_id: Option<Uuid>,
}

#[derive(Deserialize)]
struct CreateMessageRequest {
    tenant_id: Uuid,
    conversation_id: Uuid,
    content: String,
    sender_type: String,
    sender_id: Option<Uuid>,
}

async fn get_conversations(
    Path(tenant_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Json<Vec<Conversation>> {
    // In a real app, fetch from DB
    Json(vec![])
}

async fn create_message(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateMessageRequest>,
) -> Json<ChatMessage> {
    // In a real app, save to DB
    let msg = ChatMessage {
        id: Uuid::new_v4(),
        tenant_id: payload.tenant_id,
        conversation_id: payload.conversation_id,
        content: payload.content,
        sender_type: payload.sender_type,
        sender_id: payload.sender_id,
    };
    Json(msg)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("Client sent str: {:?}", t);
                    // Echo it back
                    if socket
                        .send(Message::Text(format!("Server echo: {}", t)))
                        .await
                        .is_err()
                    {
                        println!("Client disconnected");
                        return;
                    }
                }
                Message::Binary(_) => {
                    println!("Client sent binary data");
                }
                Message::Ping(_) => {
                    println!("Socket ping");
                }
                Message::Pong(_) => {
                    println!("Socket pong");
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    return;
                }
            }
        } else {
            println!("Client disconnected");
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {});

    let app = Router::new()
        .route("/api/v1/tenants/:tenant_id/conversations", get(get_conversations))
        .route("/api/v1/messages", post(create_message))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 3000");
    axum::serve(listener, app).await.unwrap();
}
