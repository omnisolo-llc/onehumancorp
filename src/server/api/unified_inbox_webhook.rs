use axum::{
    extract::{State, Json, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use sqlx::Row;
use crate::db::DB;
use crate::orchestration::router::{SemanticRouter, SemanticRoutingRequest};
use crate::orchestration::departments::types::DepartmentType;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
    pub semantic_router: Arc<SemanticRouter>,
}

#[derive(Debug, Deserialize)]
pub struct UnifiedWebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub identifier: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftedResponse {
    pub customer_id: String,
    pub context_summary: String,
    pub draft_reply: String,
}

#[derive(Serialize)]
pub struct UnifiedThread {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct UnifiedMessage {
    pub id: String,
    pub tenant_id: String,
    pub thread_id: String,
    pub sender_type: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct UnifiedTriageAction {
    pub id: String,
    pub tenant_id: String,
    pub thread_id: String,
    pub action_type: String,
    pub action_payload: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct UnifiedFeedItem {
    pub thread: UnifiedThread,
    pub messages: Vec<UnifiedMessage>,
    pub triage_actions: Vec<UnifiedTriageAction>,
}

#[derive(Deserialize)]
pub struct LocalUiTenantQuery {
    pub tenant_id: Option<String>,
    pub tenant: Option<String>,
}

fn get_ui_tenant_id(query: &LocalUiTenantQuery) -> String {
    query.tenant_id.clone().or(query.tenant.clone()).unwrap_or_else(|| "default".to_string())
}

pub fn router(db: Arc<DB>, semantic_router: Arc<SemanticRouter>) -> Router {
    let state = AppState { db, semantic_router };
    Router::new()
        .route("/api/v1/webhooks/unified_inbox", post(handle_unified_webhook))
        .route("/api/ui/unified_inbox_feed", get(get_unified_feed))
        .with_state(state)
}

pub async fn handle_unified_webhook(
    State(state): State<AppState>,
    Json(payload): Json<UnifiedWebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = &payload.tenant_id;

    if let Some(redis_client) = crate::get_redis_client() {
        if let Ok(mut conn) = redis_client.get_multiplexed_async_connection().await {
            let lock_key = format!("ohc:lock:unified_inbox:{}:{}", tenant_id, payload.identifier);
            let locked: redis::RedisResult<Option<String>> = redis::cmd("SET")
                .arg(&lock_key)
                .arg("1")
                .arg("NX")
                .arg("EX")
                .arg(30)
                .query_async(&mut conn)
                .await;

            if locked.is_err() || locked.unwrap().is_none() {
                tracing::warn!("Failed to acquire lock for tenant {} identifier {}", tenant_id, payload.identifier);
            }
        }
    }

    let routing_req = SemanticRoutingRequest {
        tenant_id: tenant_id.clone(),
        prompt: payload.message.clone(),
        embedding: None,
    };
    let target_department = match state.semantic_router.route(&routing_req) {
        Ok(res) => res.target_department,
        Err(_) => DepartmentType::CustomerSuccess,
    };

    let customer_id = format!("cust-{}", Uuid::new_v4());
    let thread_id = format!("thread-{}", Uuid::new_v4());
    let message_id = format!("msg-{}", Uuid::new_v4());
    let action_id = format!("action-{}", Uuid::new_v4());

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO unified_threads (id, tenant_id, customer_id, channel, status) VALUES ($1, $2, $3, $4, 'open') ON CONFLICT DO NOTHING")
                .bind(&thread_id)
                .bind(tenant_id)
                .bind(&customer_id)
                .bind(&payload.source)
                .execute(&state.db.pool).await;

            let _ = sqlx::query("INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES ($1, $2, $3, 'customer', $4)")
                .bind(&message_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&payload.message)
                .execute(&state.db.pool).await;

            let draft_reply = generate_omni_context_draft(target_department, &payload.message);
            let action_payload = serde_json::to_string(&DraftedResponse {
                customer_id: customer_id.clone(),
                context_summary: "Customer inquiry received.".to_string(),
                draft_reply,
            }).unwrap();

            let _ = sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES ($1, $2, $3, 'DRAFT_REPLY', $4, 'pending')")
                .bind(&action_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&action_payload)
                .execute(&state.db.pool).await;
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let _ = sqlx::query("INSERT OR IGNORE INTO unified_threads (id, tenant_id, customer_id, channel, status) VALUES (?, ?, ?, ?, 'open')")
                .bind(&thread_id)
                .bind(tenant_id)
                .bind(&customer_id)
                .bind(&payload.source)
                .execute(sqlite_pool).await;

            let _ = sqlx::query("INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES (?, ?, ?, 'customer', ?)")
                .bind(&message_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&payload.message)
                .execute(sqlite_pool).await;

            let draft_reply = generate_omni_context_draft(target_department, &payload.message);
            let action_payload = serde_json::to_string(&DraftedResponse {
                customer_id: customer_id.clone(),
                context_summary: "Customer inquiry received.".to_string(),
                draft_reply,
            }).unwrap();

            let _ = sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES (?, ?, ?, 'DRAFT_REPLY', ?, 'pending')")
                .bind(&action_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&action_payload)
                .execute(sqlite_pool).await;
        }
    }

    (StatusCode::OK, axum::Json(serde_json::json!({"success": true, "thread_id": thread_id}))).into_response()
}

pub fn generate_omni_context_draft(department: DepartmentType, message: &str) -> String {
    let context = match department {
        DepartmentType::Sales => "Sales Agent Context: Attached pricing and latest quote link.",
        DepartmentType::Operations => "Operations Agent Context: Checked schedule. Next delivery slot is tomorrow.",
        _ => "Customer Agent Context: Reviewed past interactions.",
    };
    format!("[Drafted by {} Agent] Hi there! Thanks for your message: '{}'. {} How can we help?", department, message, context)
}

pub async fn get_unified_feed(
    State(state): State<AppState>,
    Query(query): Query<LocalUiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = get_ui_tenant_id(&query);

    let mut feed_items: Vec<UnifiedFeedItem> = vec![];

    let mut threads_res_mapped: Result<Vec<UnifiedThread>, sqlx::Error> = Ok(vec![]);
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query("SELECT id, tenant_id, customer_id, channel, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_threads WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 50")
                .bind(&tenant_id)
                .fetch_all(&state.db.pool).await;
            threads_res_mapped = res.map(|rows| {
                rows.into_iter().map(|row| {
                    UnifiedThread {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        customer_id: row.try_get("customer_id").ok(),
                        channel: row.get("channel"),
                        status: row.get("status"),
                        created_at: row.try_get("created_at").unwrap_or_default(),
                        updated_at: row.try_get("updated_at").unwrap_or_default(),
                    }
                }).collect()
            });
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query("SELECT id, tenant_id, customer_id, channel, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_threads WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 50")
                .bind(&tenant_id)
                .fetch_all(sqlite_pool).await;
            threads_res_mapped = res.map(|rows| {
                rows.into_iter().map(|row| {
                    UnifiedThread {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        customer_id: row.try_get("customer_id").ok(),
                        channel: row.get("channel"),
                        status: row.get("status"),
                        created_at: row.try_get("created_at").unwrap_or_default(),
                        updated_at: row.try_get("updated_at").unwrap_or_default(),
                    }
                }).collect()
            });
        }
    }

    if let Ok(threads_rows) = threads_res_mapped {
        for thread in threads_rows {
            let thread_id = thread.id.clone();

            let mut messages: Vec<UnifiedMessage> = vec![];
            let mut triage_actions: Vec<UnifiedTriageAction> = vec![];

            match &state.db.store {
                crate::db::DbStore::Postgres => {
                    if let Ok(msg_rows) = sqlx::query("SELECT id, tenant_id, thread_id, sender_type, content, CAST(created_at AS text) as created_at FROM unified_messages WHERE thread_id = $1 ORDER BY created_at ASC")
                        .bind(&thread_id).fetch_all(&state.db.pool).await {
                        for m_row in msg_rows {
                            messages.push(UnifiedMessage {
                                id: m_row.get("id"),
                                tenant_id: m_row.get("tenant_id"),
                                thread_id: m_row.get("thread_id"),
                                sender_type: m_row.get("sender_type"),
                                content: m_row.get("content"),
                                created_at: m_row.try_get("created_at").unwrap_or_default(),
                            });
                        }
                    }

                    if let Ok(action_rows) = sqlx::query("SELECT id, tenant_id, thread_id, action_type, action_payload, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_triage_actions WHERE thread_id = $1")
                        .bind(&thread_id).fetch_all(&state.db.pool).await {
                        for a_row in action_rows {
                            triage_actions.push(UnifiedTriageAction {
                                id: a_row.get("id"),
                                tenant_id: a_row.get("tenant_id"),
                                thread_id: a_row.get("thread_id"),
                                action_type: a_row.get("action_type"),
                                action_payload: a_row.try_get("action_payload").ok(),
                                status: a_row.get("status"),
                                created_at: a_row.try_get("created_at").unwrap_or_default(),
                                updated_at: a_row.try_get("updated_at").unwrap_or_default(),
                            });
                        }
                    }
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    if let Ok(msg_rows) = sqlx::query("SELECT id, tenant_id, thread_id, sender_type, content, CAST(created_at AS text) as created_at FROM unified_messages WHERE thread_id = ? ORDER BY created_at ASC")
                        .bind(&thread_id).fetch_all(sqlite_pool).await {
                        for m_row in msg_rows {
                            messages.push(UnifiedMessage {
                                id: m_row.get("id"),
                                tenant_id: m_row.get("tenant_id"),
                                thread_id: m_row.get("thread_id"),
                                sender_type: m_row.get("sender_type"),
                                content: m_row.get("content"),
                                created_at: m_row.try_get("created_at").unwrap_or_default(),
                            });
                        }
                    }

                    if let Ok(action_rows) = sqlx::query("SELECT id, tenant_id, thread_id, action_type, action_payload, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_triage_actions WHERE thread_id = ?")
                        .bind(&thread_id).fetch_all(sqlite_pool).await {
                        for a_row in action_rows {
                            triage_actions.push(UnifiedTriageAction {
                                id: a_row.get("id"),
                                tenant_id: a_row.get("tenant_id"),
                                thread_id: a_row.get("thread_id"),
                                action_type: a_row.get("action_type"),
                                action_payload: a_row.try_get("action_payload").ok(),
                                status: a_row.get("status"),
                                created_at: a_row.try_get("created_at").unwrap_or_default(),
                                updated_at: a_row.try_get("updated_at").unwrap_or_default(),
                            });
                        }
                    }
                }
            }

            feed_items.push(UnifiedFeedItem {
                thread,
                messages,
                triage_actions,
            });
        }
    }

    (StatusCode::OK, axum::Json(feed_items)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::types::DepartmentType;

    #[test]
    fn test_generate_omni_context_draft_sales() {
        let msg = "Can you send me a quote?";
        let draft = generate_omni_context_draft(DepartmentType::Sales, msg);
        assert!(draft.contains("[Drafted by sales Agent]"));
        assert!(draft.contains("Sales Agent Context: Attached pricing and latest quote link."));
        assert!(draft.contains(msg));
    }

    #[test]
    fn test_generate_omni_context_draft_ops() {
        let msg = "Where is my order?";
        let draft = generate_omni_context_draft(DepartmentType::Operations, msg);
        assert!(draft.contains("[Drafted by operations Agent]"));
        assert!(draft.contains("Operations Agent Context: Checked schedule. Next delivery slot is tomorrow."));
        assert!(draft.contains(msg));
    }

    #[test]
    fn test_generate_omni_context_draft_customer_success() {
        let msg = "I have a question about my account.";
        let draft = generate_omni_context_draft(DepartmentType::CustomerSuccess, msg);
        assert!(draft.contains("[Drafted by customer_success Agent]"));
        assert!(draft.contains("Customer Agent Context: Reviewed past interactions."));
        assert!(draft.contains(msg));
    }
}
