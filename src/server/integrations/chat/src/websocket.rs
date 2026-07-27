use axum::{
    extract::{
        ws::{Message as AxumWsMessage, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, debug};

use crate::models::{WsEvent, Message as ChatMessage};
use crate::service::ChatService;

pub struct AppState {
    pub tx: broadcast::Sender<WsEvent>,
    pub service: Arc<ChatService>,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // Spawn a task to receive messages from the broadcast channel and send them to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(AxumWsMessage::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn a task to receive messages from the client and handle them
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let AxumWsMessage::Text(text) = msg {
                debug!("Received message from client: {}", text);

                // Try to parse the incoming message as a ChatMessage create request
                if let Ok(incoming) = serde_json::from_str::<ChatMessage>(&text) {
                     // In a real app we'd validate tenant_id from auth context, for now we pass it through
                     let _ = state_clone.service.create_message(
                         incoming.tenant_id,
                         incoming.conversation_id,
                         incoming.content,
                         incoming.sender_type,
                     ).await;
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    info!("WebSocket context closed");
}
