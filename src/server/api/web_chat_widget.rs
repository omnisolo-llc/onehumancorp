use axum::{
    extract::{State, Json, Query},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::identity_resolution::IdentityResolver;
use sqlx::Row;

#[derive(Clone)]
pub struct WebChatWidgetState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct WebChatIngestPayload {
    pub tenant_id: String,
    pub session_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct WebChatIngestResponse {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct WebChatHistoryQuery {
    pub tenant_id: String,
    pub session_id: String,
}

#[derive(Serialize)]
pub struct WebChatMessage {
    pub id: String,
    pub sender_type: String,
    pub content: String,
    pub created_at: String,
}

pub async fn web_chat_ingest_handler(
    State(state): State<WebChatWidgetState>,
    Json(payload): Json<WebChatIngestPayload>,
) -> impl IntoResponse {
    if payload.message.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(WebChatIngestResponse { success: false })).into_response();
    }

    let tenant_id = payload.tenant_id;
    let session_id = payload.session_id;
    let message = payload.message;
    let source = "web_chat_widget".to_string();

    let resolver = IdentityResolver::new(state.db.clone());
    let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &session_id, &source).await;

    if let Ok(ref customer_id) = customer_id_result {
        if payload.name.is_some() || payload.email.is_some() {
             match &state.db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query("UPDATE customers SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3 AND tenant_id = $4")
                        .bind(payload.name)
                        .bind(payload.email)
                        .bind(customer_id)
                        .bind(&tenant_id)
                        .execute(&state.db.pool)
                        .await;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query("UPDATE customers SET name = COALESCE(?, name), email = COALESCE(?, email) WHERE id = ? AND tenant_id = ?")
                        .bind(payload.name)
                        .bind(payload.email)
                        .bind(customer_id)
                        .bind(&tenant_id)
                        .execute(sqlite_pool)
                        .await;
                }
             };
        }
    }

    let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());
    let inbox_id = Uuid::new_v4().to_string();

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&message)
            .bind(&message)
            .bind(&session_id)
            .bind(customer_id)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&message)
            .bind(&message)
            .bind(&session_id)
            .bind(customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omni_inbox_message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebChatIngestResponse { success: false })).into_response();
    }

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let job_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, created_at, updated_at) VALUES ($1, $2, 'message_triage', $3, 'pending', NOW(), NOW())"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(serde_json::to_string(&serde_json::json!({
                "message_id": inbox_id,
                "source": source,
            })).unwrap())
            .execute(&state.db.pool)
            .await;
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let job_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, created_at, updated_at) VALUES (?, ?, 'message_triage', ?, 'pending', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(serde_json::to_string(&serde_json::json!({
                "message_id": inbox_id,
                "source": source,
            })).unwrap())
            .execute(sqlite_pool)
            .await;
        }
    };

    (StatusCode::OK, Json(WebChatIngestResponse { success: true })).into_response()
}

pub async fn web_chat_history_handler(
    State(state): State<WebChatWidgetState>,
    Query(query): Query<WebChatHistoryQuery>,
) -> impl IntoResponse {

    let mut messages = Vec::new();

    match &state.db.store {
         crate::db::DbStore::Postgres => {
             let history_res = sqlx::query(
                 "SELECT id, original_content, status, CAST(created_at AS text) as created_at FROM omni_inbox_messages WHERE tenant_id = $1 AND sender_id = $2 AND source = 'web_chat_widget' ORDER BY created_at ASC LIMIT 100"
             )
             .bind(&query.tenant_id)
             .bind(&query.session_id)
             .fetch_all(&state.db.pool).await;

             if let Ok(rows) = history_res {
                 for row in rows {
                     let id: String = row.get("id");
                     let content: String = row.get("original_content");
                     let created_at: String = row.try_get("created_at").unwrap_or_default();

                     messages.push(WebChatMessage {
                         id,
                         sender_type: "customer".to_string(),
                         content,
                         created_at,
                     });
                 }
             }
         },
         crate::db::DbStore::Sqlite(sqlite_pool) => {
             let history_res = sqlx::query(
                 "SELECT id, original_content, status, CAST(created_at AS text) as created_at FROM omni_inbox_messages WHERE tenant_id = ? AND sender_id = ? AND source = 'web_chat_widget' ORDER BY created_at ASC LIMIT 100"
             )
             .bind(&query.tenant_id)
             .bind(&query.session_id)
             .fetch_all(sqlite_pool).await;

             if let Ok(rows) = history_res {
                 for row in rows {
                     let id: String = row.get("id");
                     let content: String = row.get("original_content");
                     let created_at: String = row.try_get("created_at").unwrap_or_default();

                     messages.push(WebChatMessage {
                         id,
                         sender_type: "customer".to_string(),
                         content,
                         created_at,
                     });
                 }
             }
         }
    };

    (StatusCode::OK, Json(messages)).into_response()
}
