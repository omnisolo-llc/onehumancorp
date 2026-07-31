use axum::{
    extract::{Path, State, WebSocketUpgrade, ws::{WebSocket, Message as WsMessage}, Extension},
    routing::{get, post, put, delete},
    Json, Router, response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use ::server_common::Claims;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub chat_service: Arc<ChatService>,
    pub redis_url: Option<String>,
}

pub fn router(pool: PgPool, redis_url: Option<String>) -> Router {
    let chat_service = Arc::new(ChatService::new(pool, redis_url.clone()));
    let state = ChatAppState { chat_service, redis_url };

    Router::new()
        .route("/inboxes", post(create_inbox).get(get_inboxes))
        .route("/inboxes/:id", get(get_inbox).put(update_inbox).delete(delete_inbox))
        .route("/conversations", post(start_conversation).get(get_conversations))
        .route("/conversations/:id", get(get_conversation).put(update_conversation).delete(delete_conversation))
        .route("/conversations/:id/messages", post(send_message).get(get_messages))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

// --- DTOs ---
#[derive(Deserialize)]
pub struct CreateInboxReq { pub name: String }
#[derive(Deserialize)]
pub struct UpdateInboxReq { pub name: String }
#[derive(Deserialize)]
pub struct StartConversationReq { pub inbox_id: Uuid, pub contact_id: Uuid, pub assignee_id: Option<Uuid> }
#[derive(Deserialize)]
pub struct UpdateConversationReq { pub status: String, pub assignee_id: Option<Uuid> }
#[derive(Deserialize)]
pub struct SendMessageReq { pub sender_type: String, pub sender_id: Option<Uuid>, pub content: String }


// --- HANDLERS ---

async fn create_inbox(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_inboxes(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ChatInbox>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.get_inboxes(tenant_id).await {
        Ok(inboxes) => Ok(Json(inboxes)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_inbox(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.get_inbox(tenant_id, id).await {
        Ok(Some(inbox)) => Ok(Json(inbox)),
        Ok(None) => Err(axum::http::StatusCode::NOT_FOUND),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_inbox(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateInboxReq>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.update_inbox(tenant_id, id, payload.name).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_inbox(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.delete_inbox(tenant_id, id).await {
        Ok(0) => Err(axum::http::StatusCode::NOT_FOUND),
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn start_conversation(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<StartConversationReq>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.start_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conv) => Ok(Json(conv)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_conversations(
    State(state): State<ChatAppState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let inbox_id = params.get("inbox_id").and_then(|s| Uuid::parse_str(s).ok());
    match state.chat_service.get_conversations(tenant_id, inbox_id).await {
        Ok(convs) => Ok(Json(convs)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_conversation(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.get_conversation(tenant_id, id).await {
        Ok(Some(conv)) => Ok(Json(conv)),
        Ok(None) => Err(axum::http::StatusCode::NOT_FOUND),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_conversation(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateConversationReq>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.update_conversation(tenant_id, id, payload.status, payload.assignee_id).await {
        Ok(conv) => Ok(Json(conv)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_conversation(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.delete_conversation(tenant_id, id).await {
        Ok(0) => Err(axum::http::StatusCode::NOT_FOUND),
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn send_message(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.send_message(tenant_id, id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(msg) => Ok(Json(msg)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_messages(
    State(state): State<ChatAppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ChatMessage>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.get_messages(tenant_id, id).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// --- WEBSOCKET HANDLER ---

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
    State(state): State<ChatAppState>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state.redis_url))
}

async fn handle_socket(socket: WebSocket, tenant_id: String, redis_url: Option<String>) {
    let (mut sender, mut _receiver) = socket.split();

    if let Some(url) = redis_url {
        if let Ok(client) = redis::Client::open(url) {
            if let Ok(mut pubsub_conn) = client.get_async_connection().await {
                let channel = format!("chat_events:{}", tenant_id);

                // Spawn task to subscribe and forward
                tokio::spawn(async move {
                    let mut pubsub = pubsub_conn.into_pubsub();
                    if pubsub.subscribe(&channel).await.is_ok() {
                        let mut stream = pubsub.on_message();
                        while let Some(msg) = stream.next().await {
                            if let Ok(payload) = msg.get_payload::<String>() {
                                if sender.send(WsMessage::Text(payload)).await.is_err() {
                                    break; // Client disconnected
                                }
                            }
                        }
                    }
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;
    use serde_json::json;
    use uuid::Uuid;
    use crate::services::chat::service::ChatService;
    use std::sync::Arc;

    async fn setup_db() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_and_get_inbox_api() {
        let pool = setup_db().await;
        let tenant_id = Uuid::new_v4();
        let app = router(pool, None).layer(axum::middleware::from_fn(move |req, next| async move {
            let claims = ::server_common::Claims {
                sub: "user-1".to_string(),
                role: "owner".to_string(),
                organization_id: Some(tenant_id.to_string()),
                exp: 9999999999,
            };
            let mut req = req;
            req.extensions_mut().insert(claims);
            next.run(req).await
        }));

        let req_body = serde_json::to_vec(&json!({"name": "API Inbox"})).unwrap();
        let request = Request::builder()
            .method("POST")
            .uri("/inboxes")
            .header("content-type", "application/json")
            .body(Body::from(req_body))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_inbox: ChatInbox = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(created_inbox.name, "API Inbox");

        // Get inboxes
        let get_req = Request::builder().method("GET").uri("/inboxes").body(Body::empty()).unwrap();
        let get_resp = app.clone().oneshot(get_req).await.unwrap();
        assert_eq!(get_resp.status(), axum::http::StatusCode::OK);

        let get_bytes = axum::body::to_bytes(get_resp.into_body(), usize::MAX).await.unwrap();
        let inboxes: Vec<ChatInbox> = serde_json::from_slice(&get_bytes).unwrap();
        assert!(!inboxes.is_empty());
        assert_eq!(inboxes[0].name, "API Inbox");
    }
}
