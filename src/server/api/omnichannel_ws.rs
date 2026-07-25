use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Extension},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
use futures_util::{sink::SinkExt, stream::StreamExt};

#[derive(Clone)]
pub struct OmniWsState {
    pub hub: Arc<crate::hub::Hub>,
}

pub fn router(state: OmniWsState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<OmniWsState>,
    Extension(claims): Extension<crate::common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    ws.on_upgrade(move |socket| handle_socket(socket, state, tenant_id))
}

async fn handle_socket(socket: WebSocket, state: OmniWsState, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to tenant-specific message channel
    let topic = format!("tenant.{}.omnichannel.messages", tenant_id);
    let mut rx = state.hub.subscribe(topic).await;

    // Task to forward messages from Hub to WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Task to handle incoming messages from WebSocket
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(_text))) = receiver.next().await {
            // Echo back or process actions like "mark read"
            // For now just ignore
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
