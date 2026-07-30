use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Path},
    response::IntoResponse,
};
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<String>,
    State(state): State<Arc<super::hub::ChatHub>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state))
}

async fn handle_socket(socket: WebSocket, tenant_id: String, hub: Arc<super::hub::ChatHub>) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = hub.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Some(tenant) = msg.get("tenant_id").and_then(|t| t.as_str()) {
                if tenant == tenant_id {
                    if let Ok(text) = serde_json::to_string(&msg) {
                        if sender.send(Message::Text(text)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(_text))) = receiver.next().await {
            // Echo or handle client messages if needed
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
