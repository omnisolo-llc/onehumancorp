use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Extension, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChatState {
    pub pool: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_type: String,
    pub content: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = ChatState { pool };
    Router::new()
        .route("/conversations", get(get_conversations))
        .route("/conversations/:id/messages", post(send_message))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn get_conversations(
    State(state): State<ChatState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> Result<Json<Vec<Conversation>>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();

    let mut tx = state.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let convs = sqlx::query_as!(
        Conversation,
        "SELECT id, tenant_id, inbox_id, contact_id, status FROM chat_conversations"
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(convs))
}

async fn send_message(
    State(state): State<ChatState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let msg_id = Uuid::new_v4();

    let mut tx = state.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let msg = sqlx::query_as!(
        Message,
        "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content, status) VALUES ($1, $2, $3, $4, $5, 'sent') RETURNING id, tenant_id, conversation_id, sender_type, content, status",
        msg_id, tenant_id, id, payload.sender_type, payload.content
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Error inserting message: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(pool) = crate::redis_pool::get_redis_pool() {
        if let Ok(mut conn) = pool.get_async_connection().await {
            let channel = format!("chat:tenant:{}", tenant_id);
            let msg_json = serde_json::to_string(&msg).unwrap_or_default();
            let _: Result<(), _> = redis::cmd("PUBLISH").arg(&channel).arg(&msg_json).query_async(&mut conn).await;
        }
    }

    Ok(Json(msg))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let redis_client_opt = crate::redis_pool::get_redis_client();

    let (ws_tx, mut ws_rx) = tokio::sync::mpsc::channel::<String>(256);

    let pubsub_task = tokio::spawn(async move {
        if let Some(client) = redis_client_opt {
            if let Ok(mut pubsub) = client.get_async_pubsub().await {
                let channel = format!("chat:tenant:{}", tenant_id);
                if pubsub.subscribe(&channel).await.is_ok() {
                    let mut stream = pubsub.into_on_message();
                    while let Some(msg) = stream.next().await {
                        if let Ok(payload) = msg.get_payload::<String>() {
                            if ws_tx.send(payload).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            }
        }
    });

    let send_task = tokio::spawn(async move {
        while let Some(msg) = ws_rx.recv().await {
            if sender.send(WsMessage::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let WsMessage::Close(_) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = pubsub_task => {},
        _ = recv_task => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
    };
    use tower::ServiceExt;
    use sqlx::PgPool;
    use ::server_common::Claims;

    async fn setup_test_db() -> PgPool {
        let pool = crate::db::get_pool().clone();

        let mut tx = pool.begin().await.unwrap();
        crate::common::auth_utils::set_org_context(&mut *tx, "tenant_maya").await.unwrap();
        let inbox_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO chat_inboxes (id, tenant_id, name, channel_type) VALUES ($1, 'tenant_maya', 'Instagram DMs', 'instagram') ON CONFLICT (id) DO NOTHING",
            inbox_id
        ).execute(&mut *tx).await.unwrap();

        let contact_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO chat_contacts (id, tenant_id, name, email) VALUES ($1, 'tenant_maya', 'Cake Lover', 'cake@example.com') ON CONFLICT (id) DO NOTHING",
            contact_id
        ).execute(&mut *tx).await.unwrap();

        let conv_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, 'tenant_maya', $2, $3, 'open') ON CONFLICT (id) DO NOTHING",
            conv_id, inbox_id, contact_id
        ).execute(&mut *tx).await.unwrap();

        tx.commit().await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_fetch_conversations() {
        let pool = setup_test_db().await;

        let claims = Claims {
            organization_id: Some("tenant_maya".to_string()),
            ..Default::default()
        };

        let app = router(pool).layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
            let claims = claims.clone();
            async move {
                req.extensions_mut().insert(claims);
                next.run(req).await
            }
        }));

        let request = Request::builder()
            .uri("/conversations")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_send_message() {
        let pool = setup_test_db().await;

        let claims = Claims {
            organization_id: Some("tenant_maya".to_string()),
            ..Default::default()
        };

        let app = router(pool.clone()).layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
            let claims = claims.clone();
            async move {
                req.extensions_mut().insert(claims);
                next.run(req).await
            }
        }));

        let mut tx = pool.begin().await.unwrap();
        crate::common::auth_utils::set_org_context(&mut *tx, "tenant_maya").await.unwrap();
        let conv = sqlx::query!("SELECT id FROM chat_conversations WHERE tenant_id = 'tenant_maya' LIMIT 1").fetch_one(&mut *tx).await.unwrap();

        let payload = serde_json::json!({
            "sender_type": "agent",
            "content": "Hello Maya, we received your order!"
        });

        let request = Request::builder()
            .method("POST")
            .uri(format!("/conversations/{}/messages", conv.id))
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ws_connection_upgrade() {
        let pool = setup_test_db().await;

        let claims = Claims {
            organization_id: Some("tenant_maya".to_string()),
            ..Default::default()
        };
        let app = router(pool).layer(axum::middleware::from_fn(move |mut req: axum::extract::Request, next: axum::middleware::Next| {
            let claims = claims.clone();
            async move {
                req.extensions_mut().insert(claims);
                next.run(req).await
            }
        }));

        let request = Request::builder()
            .method("GET")
            .uri("/ws")
            .header(header::CONNECTION, "upgrade")
            .header(header::UPGRADE, "websocket")
            .header(header::SEC_WEBSOCKET_VERSION, "13")
            .header(header::SEC_WEBSOCKET_KEY, "dGhlIHNhbXBsZSBub25jZQ==")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
    }
}
