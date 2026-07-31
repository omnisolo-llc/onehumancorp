use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::stream::StreamExt;
use std::sync::Arc;
use uuid::Uuid;
use super::service::ChatService;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(service): State<Arc<ChatService>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, service))
}

async fn handle_socket(mut socket: WebSocket, tenant_id: Uuid, _service: Arc<ChatService>) {
    // Basic websocket that listens to a redis channel
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to connect to redis: {}", e);
            return;
        }
    };

    #[allow(deprecated)]
    let mut con = match client.get_async_connection().await {
        Ok(c) => c.into_pubsub(),
        Err(e) => {
            tracing::error!("Failed to get redis pubsub connection: {}", e);
            return;
        }
    };

    let channel = format!("chat:tenant:{}", tenant_id);
    if let Err(e) = con.subscribe(&channel).await {
        tracing::error!("Failed to subscribe to channel {}: {}", channel, e);
        return;
    }

    let mut pubsub_stream = con.into_on_message();

    loop {
        tokio::select! {
            msg = pubsub_stream.next() => {
                if let Some(msg) = msg {
                    if let Ok(payload) = msg.get_payload::<String>() {
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
            client_msg = socket.next() => {
                if let Some(Ok(Message::Close(_))) = client_msg {
                    break;
                }
            }
        }
    }
}

pub fn websocket_routes(service: Arc<ChatService>) -> Router {
    Router::new()
        .route("/ws/:tenant_id", get(ws_handler))
        .with_state(service)
}
