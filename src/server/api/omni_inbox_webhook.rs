use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::identity_resolution::IdentityResolver;
use crate::Hub;
use sqlx::Row;

#[derive(Clone)]
pub struct OmniInboxWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<crate::db::DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct OmniInboxPayload {
    pub tenant_id: String,
    pub source: String,
    pub sender_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
}

pub async fn omni_inbox_post_handler(
    State(state): State<OmniInboxWebhookState>,
    Json(payload): Json<OmniInboxPayload>,
) -> impl IntoResponse {
    if payload.message.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false })).into_response();
    }

    let tenant_id_str = payload.tenant_id;
    let source = payload.source.to_lowercase();
    let sender_id = payload.sender_id;
    let message = payload.message;

    // 1. Identity Resolution
    let resolver = IdentityResolver::new(state.db.clone());
    let customer_id_result = resolver.resolve_or_create_customer(&tenant_id_str, &sender_id, &source).await;

    if let Err(e) = customer_id_result {
         tracing::error!("Failed to resolve identity: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
    }

    // 2. Insert into chat_messages
    let customer_id_str = customer_id_result.as_ref().ok().map(|s| s.as_str()).unwrap_or_default();

    // Convert to UUID for DB
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid tenant_id format: {}", tenant_id_str);
            return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false })).into_response();
        }
    };

    let contact_id = match Uuid::parse_str(customer_id_str) {
        Ok(id) => id,
        Err(_) => Uuid::new_v4(), // Fallback if no valid customer ID was created
    };

    // We try to get an existing conversation or create a new one using sqlx::query (not macro) to avoid compile-time issues
    let mut conversation_id_opt: Option<Uuid> = None;

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            if let Ok(Some(row)) = sqlx::query("SELECT id FROM chat_conversations WHERE contact_id = $1 AND tenant_id = $2 AND status = 'open' LIMIT 1")
                .bind(contact_id)
                .bind(tenant_id)
                .fetch_optional(&state.db.pool).await {
                conversation_id_opt = Some(row.get("id"));
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            if let Ok(Some(row)) = sqlx::query("SELECT id FROM chat_conversations WHERE contact_id = ? AND tenant_id = ? AND status = 'open' LIMIT 1")
                .bind(contact_id.to_string())
                .bind(tenant_id.to_string())
                .fetch_optional(pool).await {
                conversation_id_opt = Uuid::parse_str(row.get("id")).ok();
            }
        }
    }

    let conversation_id = if let Some(id) = conversation_id_opt {
        id
    } else {
        let mut inbox_id_opt: Option<Uuid> = None;
        match &state.db.store {
            crate::db::DbStore::Postgres => {
                if let Ok(Some(row)) = sqlx::query("SELECT id FROM chat_inboxes WHERE tenant_id = $1 LIMIT 1")
                    .bind(tenant_id)
                    .fetch_optional(&state.db.pool).await {
                    inbox_id_opt = Some(row.get("id"));
                }
            },
            crate::db::DbStore::Sqlite(pool) => {
                if let Ok(Some(row)) = sqlx::query("SELECT id FROM chat_inboxes WHERE tenant_id = ? LIMIT 1")
                    .bind(tenant_id.to_string())
                    .fetch_optional(pool).await {
                    inbox_id_opt = Uuid::parse_str(row.get("id")).ok();
                }
            }
        }

        let inbox_id = if let Some(id) = inbox_id_opt {
            id
        } else {
            let new_id = Uuid::new_v4();
            match &state.db.store {
                crate::db::DbStore::Postgres => {
                    if let Err(e) = sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, 'Primary Inbox')")
                        .bind(new_id).bind(tenant_id).execute(&state.db.pool).await {
                        tracing::error!("Failed to insert chat_inboxes pg: {}", e);
                    }
                },
                crate::db::DbStore::Sqlite(pool) => {
                    if let Err(e) = sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES (?, ?, 'Primary Inbox')")
                        .bind(new_id.to_string()).bind(tenant_id.to_string()).execute(pool).await {
                        tracing::error!("Failed to insert chat_inboxes sqlite: {}", e);
                    }
                }
            }
            new_id
        };

        let conv_id = Uuid::new_v4();
        match &state.db.store {
            crate::db::DbStore::Postgres => {
                if let Err(e) = sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')")
                    .bind(conv_id).bind(tenant_id).bind(inbox_id).bind(contact_id).execute(&state.db.pool).await {
                    tracing::error!("Failed to insert chat_conversations pg: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
                }
            },
            crate::db::DbStore::Sqlite(pool) => {
                if let Err(e) = sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, 'open')")
                    .bind(conv_id.to_string()).bind(tenant_id.to_string()).bind(inbox_id.to_string()).bind(contact_id.to_string()).execute(pool).await {
                    tracing::error!("Failed to insert chat_conversations sqlite: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
                }
            }
        }
        conv_id
    };

    let message_id = Uuid::new_v4();
    let empty_json: serde_json::Value = serde_json::json!([]);
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            if let Err(e) = sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, attachments) VALUES ($1, $2, $3, 'contact', $4, $5, $6)")
                .bind(message_id).bind(tenant_id).bind(conversation_id).bind(contact_id).bind(&message).bind(&empty_json).execute(&state.db.pool).await {
                tracing::error!("Failed to insert chat_messages pg: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            if let Err(e) = sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, attachments) VALUES (?, ?, ?, 'contact', ?, ?, ?)")
                .bind(message_id.to_string()).bind(tenant_id.to_string()).bind(conversation_id.to_string()).bind(contact_id.to_string()).bind(&message).bind(empty_json.to_string()).execute(pool).await {
                tracing::error!("Failed to insert chat_messages sqlite: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
            }
        }
    }

    // 3. Enqueue to ohc_job_queue
    let job_id = Uuid::new_v4().to_string();
    let mut payload_json = serde_json::json!({
        "message_id": message_id.to_string(),
        "inbox_message_id": message_id.to_string(),
        "conversation_id": conversation_id.to_string(),
        "source": source,
        "content": message,
        "sender_id": sender_id
    });

    if let Ok(c_id) = &customer_id_result {
        payload_json["customer_id"] = serde_json::json!(c_id);
    }

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id_str)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id_str)
                .bind(payload_json.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
        tracing::error!("Failed to enqueue message_triage job: {}", e);
    }

    // Broadcast message created over redis
    let envelope = crate::api::unified_ws::build_envelope("chat", &format!("chat:{}", tenant_id_str), serde_json::json!({"action": "message.created", "message_id": message_id.to_string(), "conversation_id": conversation_id.to_string()}), 0);
    let _ = crate::api::unified_ws::get_broadcast_tx().send(envelope);

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id_str.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json,
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    (StatusCode::OK, Json(WebhookResponse { success: true })).into_response()
}
