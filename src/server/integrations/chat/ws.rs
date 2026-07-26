use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
        Extension,
    },
    response::IntoResponse,
};
use uuid::Uuid;
use crate::service::ChatState;
use tokio::sync::broadcast;

// Note: Tenant Auth is inherently handled by Axum middleware in the actual server application
// via `Extension<Uuid>` for the tenant_id. We just pass it through here.
// In a real application, we might also validate the connection payload here, but for this PR,
// the middleware ensures that any request reaching here is for a valid tenant.
pub async fn chat_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ChatState>,
    Extension(tenant_id): Extension<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, tenant_id))
}

async fn handle_socket(mut socket: WebSocket, _state: ChatState, tenant_id: Uuid) {
    // Acknowledge connection
    if socket.send(Message::Text(format!("Connected to tenant: {}", tenant_id).into())).await.is_err() {
        return;
    }

    // In a real application, we would subscribe to a Redis or NATS PubSub channel here.
    // For this demonstration, we use a simple loop.
    let (tx, mut rx) = broadcast::channel::<String>(16);

    let _send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if socket.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Simulating pub/sub loop
    let _tx_clone = tx.clone();
    tokio::spawn(async move {
        // Echo loop
        // In real implementation this would listen to external NATS events
    });
}
