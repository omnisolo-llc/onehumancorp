use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    Json,
    routing::{get, post},
    Router,
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::DB, orchestration::departments::types::DepartmentEvent};

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub db: DB,
    pub orchestrator: std::sync::Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>,
}

#[derive(Debug, Deserialize)]
pub struct OmnichannelPayload {
    pub tenant_id: String,
    pub source: String,
    pub message: String,
    pub sender_id: String,
    pub target_language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub tenant_id: String,
    pub source: String,
    pub original_content: String,
    pub translated_content: String,
    pub draft_reply: String,
    pub status: String,
    pub sender_id: String,
    pub created_at: String,
}

pub fn router(db: std::sync::Arc<DB>, orchestrator: std::sync::Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>) -> Router {
    let state = OmnichannelWebhookState {
        db: (*db).clone(),
        orchestrator: orchestrator.clone(),
    };

    Router::new()
        .route("/conversations/:tenant_id", get(get_conversations))
        .route(
            "/messages/:tenant_id/:conversation_id",
            get(get_messages),
        )
        .route("/webhook", post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn get_conversations(
    State(state): State<OmnichannelWebhookState>,
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Path(tenant_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&tenant_id) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(Vec::<ConversationResponse>::new()),
        )
            .into_response();
    }
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let query = "SELECT id, tenant_id, contact_id as customer_id, (SELECT channel_type FROM inboxes WHERE id = conversations.inbox_id) as channel, status, CAST(created_at AS text) as created_at FROM conversations WHERE tenant_id = $1 ORDER BY created_at DESC";
            match sqlx::query(query)
                .bind(&tenant_id)
                .fetch_all(&state.db.pool)
                .await
            {
                Ok(rows) => {
                    let conversations: Vec<ConversationResponse> = rows
                        .into_iter()
                        .map(|row| ConversationResponse {
                            id: sqlx::Row::get(&row, "id"),
                            tenant_id: sqlx::Row::get(&row, "tenant_id"),
                            customer_id: sqlx::Row::try_get(&row, "customer_id").ok(),
                            channel: sqlx::Row::get(&row, "channel"),
                            status: sqlx::Row::get(&row, "status"),
                            created_at: sqlx::Row::try_get(&row, "created_at")
                                .unwrap_or_default(),
                        })
                        .collect();
                    (StatusCode::OK, Json(conversations)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to fetch conversations: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Vec::<ConversationResponse>::new()),
                    )
                        .into_response()
                }
            }
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let query = "SELECT id, tenant_id, contact_id as customer_id, (SELECT channel_type FROM inboxes WHERE id = conversations.inbox_id) as channel, status, CAST(created_at AS text) as created_at FROM conversations WHERE tenant_id = ? ORDER BY created_at DESC";
            match sqlx::query(query).bind(&tenant_id).fetch_all(sqlite_pool).await {
                Ok(rows) => {
                    let conversations: Vec<ConversationResponse> = rows
                        .into_iter()
                        .map(|row| ConversationResponse {
                            id: sqlx::Row::get(&row, "id"),
                            tenant_id: sqlx::Row::get(&row, "tenant_id"),
                            customer_id: sqlx::Row::try_get(&row, "customer_id").ok(),
                            channel: sqlx::Row::get(&row, "channel"),
                            status: sqlx::Row::get(&row, "status"),
                            created_at: sqlx::Row::try_get(&row, "created_at")
                                .unwrap_or_default(),
                        })
                        .collect();
                    (StatusCode::OK, Json(conversations)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to fetch conversations: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Vec::<ConversationResponse>::new()),
                    )
                        .into_response()
                }
            }
        }
    }
}

pub async fn get_messages(
    State(state): State<OmnichannelWebhookState>,
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Path((tenant_id, conversation_id)): axum::extract::Path<(String, String)>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&tenant_id) {
        return (StatusCode::UNAUTHORIZED, Json(Vec::<MessageResponse>::new())).into_response();
    }
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let query = "SELECT id, tenant_id, (SELECT channel_type FROM inboxes WHERE id = (SELECT inbox_id FROM conversations WHERE id = messages.conversation_id)) as source, content as original_content, content as translated_content, COALESCE(draft_reply, '') as draft_reply, 'unread' as status, sender_type as sender_id, CAST(created_at AS text) as created_at FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC";
            match sqlx::query(query)
                .bind(&tenant_id)
                .bind(&conversation_id)
                .fetch_all(&state.db.pool)
                .await
            {
                Ok(rows) => {
                    let messages: Vec<MessageResponse> = rows
                        .into_iter()
                        .map(|row| MessageResponse {
                            id: sqlx::Row::get(&row, "id"),
                            tenant_id: sqlx::Row::get(&row, "tenant_id"),
                            source: sqlx::Row::get(&row, "source"),
                            original_content: sqlx::Row::get(&row, "original_content"),
                            translated_content: sqlx::Row::get(&row, "translated_content"),
                            draft_reply: sqlx::Row::try_get(&row, "draft_reply")
                                .unwrap_or_default(),
                            status: sqlx::Row::get(&row, "status"),
                            sender_id: sqlx::Row::try_get(&row, "sender_id").unwrap_or_default(),
                            created_at: sqlx::Row::try_get(&row, "created_at")
                                .unwrap_or_default(),
                        })
                        .collect();
                    (StatusCode::OK, Json(messages)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to fetch messages: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Vec::<MessageResponse>::new()),
                    )
                        .into_response()
                }
            }
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let query = "SELECT id, tenant_id, (SELECT channel_type FROM inboxes WHERE id = (SELECT inbox_id FROM conversations WHERE id = messages.conversation_id)) as source, content as original_content, content as translated_content, COALESCE(draft_reply, '') as draft_reply, 'unread' as status, sender_type as sender_id, CAST(created_at AS text) as created_at FROM messages WHERE tenant_id = ? AND conversation_id = ? ORDER BY created_at ASC";
            match sqlx::query(query)
                .bind(&tenant_id)
                .bind(&conversation_id)
                .fetch_all(sqlite_pool)
                .await
            {
                Ok(rows) => {
                    let messages: Vec<MessageResponse> = rows
                        .into_iter()
                        .map(|row| MessageResponse {
                            id: sqlx::Row::get(&row, "id"),
                            tenant_id: sqlx::Row::get(&row, "tenant_id"),
                            source: sqlx::Row::get(&row, "source"),
                            original_content: sqlx::Row::get(&row, "original_content"),
                            translated_content: sqlx::Row::get(&row, "translated_content"),
                            draft_reply: sqlx::Row::try_get(&row, "draft_reply")
                                .unwrap_or_default(),
                            status: sqlx::Row::get(&row, "status"),
                            sender_id: sqlx::Row::try_get(&row, "sender_id").unwrap_or_default(),
                            created_at: sqlx::Row::try_get(&row, "created_at")
                                .unwrap_or_default(),
                        })
                        .collect();
                    (StatusCode::OK, Json(messages)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to fetch messages: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(Vec::<MessageResponse>::new()),
                    )
                        .into_response()
                }
            }
        }
    }
}

pub async fn handle_omnichannel_webhook(
    State(state): State<OmnichannelWebhookState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (
            StatusCode::FORBIDDEN,
            Json(WebhookResponse {
                success: false,
                message_id: None,
            }),
        )
            .into_response();
    }

    let pool = &state.db.pool;

    let (_inbox_id, _contact_id, conversation_id) = match &state.db.store {
        crate::db::DbStore::Postgres => {
            // Find or create Inbox
            let inbox_id: String = match sqlx::query("SELECT id FROM inboxes WHERE tenant_id = $1 AND channel_type = $2 LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&payload.source)
                .fetch_optional(pool).await.unwrap_or(None) {
                    Some(row) => sqlx::Row::get(&row, "id"),
                    None => {
                        let new_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO inboxes (id, tenant_id, name, channel_type) VALUES ($1, $2, 'Default', $3)")
                            .bind(&new_id).bind(&payload.tenant_id).bind(&payload.source).execute(pool).await;
                        new_id
                    }
                };

            // Find or create Contact
            let contact_id: String = match sqlx::query("SELECT id FROM contacts WHERE tenant_id = $1 AND phone = $2 LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&payload.sender_id)
                .fetch_optional(pool).await.unwrap_or(None) {
                    Some(row) => sqlx::Row::get(&row, "id"),
                    None => {
                        let new_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO contacts (id, tenant_id, name, phone) VALUES ($1, $2, 'Unknown', $3)")
                            .bind(&new_id).bind(&payload.tenant_id).bind(&payload.sender_id).execute(pool).await;
                        new_id
                    }
                };

            // Find or create Conversation
            let conversation_id: String = match sqlx::query("SELECT id FROM conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&inbox_id)
                .bind(&contact_id)
                .fetch_optional(pool).await.unwrap_or(None) {
                    Some(row) => sqlx::Row::get(&row, "id"),
                    None => {
                        let new_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')")
                            .bind(&new_id).bind(&payload.tenant_id).bind(&inbox_id).bind(&contact_id).execute(pool).await;
                        new_id
                    }
                };

            (inbox_id, contact_id, conversation_id)
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            // Find or create Inbox
            let inbox_id: String = match sqlx::query("SELECT id FROM inboxes WHERE tenant_id = ? AND channel_type = ? LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&payload.source)
                .fetch_optional(sqlite_pool).await.unwrap_or(None) {
                    Some(row) => sqlx::Row::get(&row, "id"),
                    None => {
                        let new_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO inboxes (id, tenant_id, name, channel_type) VALUES (?, ?, 'Default', ?)")
                            .bind(&new_id).bind(&payload.tenant_id).bind(&payload.source).execute(sqlite_pool).await;
                        new_id
                    }
                };

            // Find or create Contact
            let contact_id: String = match sqlx::query("SELECT id FROM contacts WHERE tenant_id = ? AND phone = ? LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&payload.sender_id)
                .fetch_optional(sqlite_pool).await.unwrap_or(None) {
                    Some(row) => sqlx::Row::get(&row, "id"),
                    None => {
                        let new_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO contacts (id, tenant_id, name, phone) VALUES (?, ?, 'Unknown', ?)")
                            .bind(&new_id).bind(&payload.tenant_id).bind(&payload.sender_id).execute(sqlite_pool).await;
                        new_id
                    }
                };

            // Find or create Conversation
            let conversation_id: String = match sqlx::query("SELECT id FROM conversations WHERE tenant_id = ? AND inbox_id = ? AND contact_id = ? LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&inbox_id)
                .bind(&contact_id)
                .fetch_optional(sqlite_pool).await.unwrap_or(None) {
                    Some(row) => sqlx::Row::get(&row, "id"),
                    None => {
                        let new_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, 'open')")
                            .bind(&new_id).bind(&payload.tenant_id).bind(&inbox_id).bind(&contact_id).execute(sqlite_pool).await;
                        new_id
                    }
                };

            (inbox_id, contact_id, conversation_id)
        }
    };

    let id = Uuid::new_v4().to_string();
    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                r#"
                INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type, created_at)
                VALUES ($1, $2, $3, $4, $5, NOW())
                "#,
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&conversation_id)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .execute(pool)
            .await
            .map(|_| ())
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type, created_at)
                VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
                "#,
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&conversation_id)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .execute(sqlite_pool)
            .await
            .map(|_| ())
        }
    };

    let payload_json = serde_json::json!({
        "message_id": id,
        "inbox_message_id": id,
        "conversation_id": conversation_id,
        "source": payload.source,
        "content": payload.message,
        "sender_id": payload.sender_id
    });
    let job_id = Uuid::new_v4().to_string();

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert into messages: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WebhookResponse {
                success: false,
                message_id: None,
            }),
        )
            .into_response();
    }

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&payload.tenant_id)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&payload.tenant_id)
                .bind(payload_json.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
        tracing::error!("Failed to enqueue message_triage job: {}", e);
    }

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json.clone(),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => (
            StatusCode::OK,
            Json(WebhookResponse {
                success: true,
                message_id: Some(id),
            }),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(WebhookResponse {
                success: false,
                message_id: None,
            }),
        )
            .into_response(),
    }
}
