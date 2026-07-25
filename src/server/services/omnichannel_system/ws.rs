use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::{info, warn};

pub async fn omnichannel_ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    info!("New WebSocket connection for tenant: {}", tenant_id);
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    // In a real implementation, we would subscribe to a Redis pub/sub channel here
    // using the tenant_id, and forward messages to the `sender`.

    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    info!("Received message from tenant {}: {}", tenant_id, text);
                    // Echo back for now
                    if let Err(e) = sender
                        .send(Message::Text(format!("Echo: {}", text)))
                        .await
                    {
                        warn!("Failed to send message to client: {}", e);
                        break;
                    }
                }
                Message::Close(_) => {
                    info!("Client disconnected for tenant {}", tenant_id);
                    break;
                }
                _ => {}
            }
        }
    });
}
