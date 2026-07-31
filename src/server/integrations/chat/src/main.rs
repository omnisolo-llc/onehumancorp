use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: PgPool,
    redis_client: redis::Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contact_id: Option<Uuid>,
    pub channel_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String, // customer, agent, ai
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct WebhookPayload {
    tenant_id: Uuid,
    channel_type: String,
    sender_id: String,
    content: String,
}

#[allow(dead_code)] // To suppress warning for channel_type and sender_id
impl WebhookPayload {
    pub fn channel_type(&self) -> &str {
        &self.channel_type
    }

    pub fn sender_id(&self) -> &str {
        &self.sender_id
    }
}


async fn ingest_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    let conversation_id = Uuid::new_v4();
    let msg_id = Uuid::new_v4();

    let message = ChatMessage {
        id: msg_id,
        conversation_id,
        sender_type: "customer".to_string(),
        content: payload.content.clone(),
        created_at: Utc::now(),
    };

    let db_clone = state.db.clone();
    let _ = db_clone; // suppress warning

    let mut con = state.redis_client.get_multiplexed_async_connection().await.unwrap();
    let msg_json = serde_json::to_string(&message).unwrap();
    let _: () = redis::cmd("PUBLISH")
        .arg(format!("tenant:{}:messages", payload.tenant_id))
        .arg(&msg_json)
        .query_async(&mut con)
        .await
        .unwrap();

    Json(message)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    let tenant_id = Uuid::nil();

    let state_clone = state.clone();
    tokio::spawn(async move {
        // we use deprecated method due to issue in pubsub implementation missing in multiplexed
        #[allow(deprecated)]
        let mut pubsub_con = state_clone.redis_client.get_async_connection().await.unwrap().into_pubsub();
        pubsub_con.subscribe(format!("tenant:{}:messages", tenant_id)).await.unwrap();

        let mut pubsub_stream = pubsub_con.on_message();
        while let Some(msg) = pubsub_stream.next().await {
            let payload: String = msg.get_payload().unwrap();
            if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(_msg)) = receiver.next().await {
        // Handle incoming client messages
    }
}


#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap_or_else(|_| PgPoolOptions::new().connect_lazy(&db_url).unwrap());

    let redis_client = redis::Client::open(redis_url).unwrap();

    let state = Arc::new(AppState { db: pool, redis_client });

    let app = Router::new()
        .route("/webhooks/ingest", post(ingest_message))
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_payload() {
        let payload = WebhookPayload {
            tenant_id: Uuid::nil(),
            channel_type: "whatsapp".to_string(),
            sender_id: "user123".to_string(),
            content: "hello world".to_string(),
        };

        assert_eq!(payload.channel_type(), "whatsapp");
        assert_eq!(payload.sender_id(), "user123");
    }
}
