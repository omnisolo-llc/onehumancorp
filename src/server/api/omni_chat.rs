use axum::{
    extract::{Path, WebSocketUpgrade, ws::{Message as WsMessage, WebSocket}, Extension},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
};
use futures_util::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatConversation, ChatMessage};

pub fn router() -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/:id/messages", get(get_messages))
        .route("/conversations/:id/messages", post(send_message))
        .route("/widget/ws", get(web_widget_ws_handler))
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub tenant_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub tenant_id: Uuid,
}

async fn list_conversations(
    Extension(service): Extension<Arc<ChatService>>,
    axum::extract::Query(params): axum::extract::Query<QueryParams>,
) -> Result<Json<Vec<ChatConversation>>, StatusCode> {
    match service.list_conversations(params.tenant_id).await {
        Ok(convos) => Ok(Json(convos)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_messages(
    Extension(service): Extension<Arc<ChatService>>,
    Path(conversation_id): Path<Uuid>,
    axum::extract::Query(params): axum::extract::Query<QueryParams>,
) -> Result<Json<Vec<ChatMessage>>, StatusCode> {
    match service.get_messages(params.tenant_id, conversation_id).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn send_message(
    Extension(service): Extension<Arc<ChatService>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, StatusCode> {
    match service.send_message(
        payload.tenant_id,
        conversation_id,
        payload.sender_type,
        payload.sender_id,
        payload.content,
    ).await {
        Ok(msg) => Ok(Json(msg)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn web_widget_ws_handler(
    ws: WebSocketUpgrade,
    Extension(service): Extension<Arc<ChatService>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, service))
}

async fn handle_socket(mut socket: WebSocket, service: Arc<ChatService>) {
    while let Some(Ok(msg)) = socket.next().await {
        if let WsMessage::Text(text) = msg {
            if let Ok(req) = serde_json::from_str::<SendMessageRequest>(&text) {
                if service.handle_incoming_widget_message(req.tenant_id, req.content.clone()).await.is_ok() {
                    let reply = serde_json::json!({
                        "status": "delivered",
                        "content": req.content,
                    });
                    let _ = socket.send(WsMessage::Text(reply.to_string().into())).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_omni_chat() {
        assert!(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;
    use crate::db::isolated_omni_postgres_pool;

    #[tokio::test]
    async fn test_omni_chat_routes() {
        let Some((admin, pool, schema, role)) = isolated_omni_postgres_pool().await else {
            return;
        };

        let schema_sql = std::fs::read_to_string("src/server/migrations/217_native_omnichannel_chat.sql").unwrap();

        let mut tx = pool.begin().await.unwrap();
        for statement in schema_sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed).execute(&mut *tx).await.unwrap();
            }
        }
        tx.commit().await.unwrap();

        let service = Arc::new(ChatService::new(pool.clone()));
        let app = router().layer(Extension(service.clone()));

        let tenant_id = Uuid::new_v4();

        let req = Request::builder()
            .uri(format!("/conversations?tenant_id={}", tenant_id))
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let inbox = service.create_inbox(tenant_id, "Test".into()).await.unwrap();
        let contact = service.create_contact(tenant_id, None, None, None).await.unwrap();
        let convo = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();

        let payload = SendMessageRequest {
            tenant_id,
            sender_type: "contact".into(),
            sender_id: None,
            content: "Hello from test".into(),
        };

        let req = Request::builder()
            .uri(format!("/conversations/{}/messages", convo.id))
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::query(&format!("DROP ROLE {role}"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}
