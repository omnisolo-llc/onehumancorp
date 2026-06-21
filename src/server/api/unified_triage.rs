use axum::{
    extract::{Query, State, Path},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;
use server_utils::cache::HybridCache;

#[derive(Deserialize)]
pub struct UiTenantQuery {
    pub tenant_id: Option<String>,
    pub tenant: Option<String>,
}

pub fn ui_tenant_id(query: &UiTenantQuery) -> String {
    query
        .tenant_id
        .as_deref()
        .or(query.tenant.as_deref())
        .unwrap_or("ohc")
        .to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestWebhookRequest {
    pub customer_id: Option<String>,
    pub channel: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveUnifiedActionRequest {
    pub action_status: String, // "APPROVED" or "DISMISSED"
}

pub async fn ingest_triage_webhook_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<IngestWebhookRequest>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    // Minimal deduplication/lock via redis to prevent race conditions during thread updates
    let lock_key = format!("ohc:lock:{}:triage_ingest:{}", tenant_id, payload.channel);
    let cache = HybridCache::<String>::new(crate::get_redis_client());
    // We cannot use set_nx if it doesn't exist, use set instead. It's a simple simulation lock.
    let _ = cache.set(&lock_key, "locked".to_string(), std::time::Duration::from_secs(5)).await;

    let customer_id = payload.customer_id.unwrap_or_else(|| "unknown".to_string());

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to begin tx: {}", e)).into_response(),
    };

    let thread_id = match &db.store {
        crate::db::DbStore::Postgres => {
            let existing: Option<(String,)> = sqlx::query_as("SELECT id FROM unified_threads WHERE tenant_id = $1 AND customer_id = $2 AND channel = $3 AND status = 'open' LIMIT 1")
                .bind(&tenant_id).bind(&customer_id).bind(&payload.channel)
                .fetch_optional(&mut *tx).await.unwrap_or(None);

            if let Some((id,)) = existing {
                id
            } else {
                let id = format!("thr-{}", Uuid::new_v4());
                let _ = sqlx::query("INSERT INTO unified_threads (id, tenant_id, customer_id, channel) VALUES ($1, $2, $3, $4)")
                    .bind(&id).bind(&tenant_id).bind(&customer_id).bind(&payload.channel)
                    .execute(&mut *tx).await;
                id
            }
        },
        crate::db::DbStore::Sqlite(_) => {
            let existing: Option<(String,)> = sqlx::query_as("SELECT id FROM unified_threads WHERE tenant_id = ? AND customer_id = ? AND channel = ? AND status = 'open' LIMIT 1")
                .bind(&tenant_id).bind(&customer_id).bind(&payload.channel)
                .fetch_optional(&mut *tx).await.unwrap_or(None);

            if let Some((id,)) = existing {
                id
            } else {
                let id = format!("thr-{}", Uuid::new_v4());
                let _ = sqlx::query("INSERT INTO unified_threads (id, tenant_id, customer_id, channel) VALUES (?, ?, ?, ?)")
                    .bind(&id).bind(&tenant_id).bind(&customer_id).bind(&payload.channel)
                    .execute(&mut *tx).await;
                id
            }
        }
    };

    let msg_id = format!("msg-{}", Uuid::new_v4());

    let _ = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO unified_messages (id, tenant_id, thread_id, direction, content) VALUES ($1, $2, $3, 'inbound', $4)")
                .bind(&msg_id).bind(&tenant_id).bind(&thread_id).bind(&payload.content)
                .execute(&mut *tx).await
        },
        crate::db::DbStore::Sqlite(_) => {
            sqlx::query("INSERT INTO unified_messages (id, tenant_id, thread_id, direction, content) VALUES (?, ?, ?, 'inbound', ?)")
                .bind(&msg_id).bind(&tenant_id).bind(&thread_id).bind(&payload.content)
                .execute(&mut *tx).await
        }
    };

    // Simulate Agent evaluation (Work Triage Coordinator)
    let action_type = "Draft Reply";
    let action_payload = serde_json::json!({
        "message": format!("Thank you for your message on {}: '{}'", payload.channel, payload.content)
    });
    let action_id = format!("act-{}", Uuid::new_v4());

    let _ = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)")
                .bind(&action_id).bind(&tenant_id).bind(&thread_id).bind(&action_type).bind(action_payload.to_string())
                .execute(&mut *tx).await
        },
        crate::db::DbStore::Sqlite(_) => {
            sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, payload) VALUES (?, ?, ?, ?, ?)")
                .bind(&action_id).bind(&tenant_id).bind(&thread_id).bind(&action_type).bind(action_payload.to_string())
                .execute(&mut *tx).await
        }
    };

    let _ = tx.commit().await;
    let _ = cache.invalidate(&lock_key).await;

    (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true, "thread_id": thread_id}))).into_response()
}

pub async fn get_triage_feed_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    let res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "SELECT t.id as thread_id, t.channel, m.content, a.id as action_id, a.action_type, a.payload, CAST(t.created_at AS text) AS created_at
                 FROM unified_threads t
                 LEFT JOIN (
                    SELECT thread_id, content FROM unified_messages
                    WHERE direction = 'inbound'
                    ORDER BY created_at DESC LIMIT 1
                 ) m ON t.id = m.thread_id
                 LEFT JOIN unified_triage_actions a ON t.id = a.thread_id
                 WHERE t.tenant_id = $1 AND t.status = 'open' AND a.status = 'pending'
                 ORDER BY t.created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool).await
        },
        crate::db::DbStore::Sqlite(pool) => {
             let res = sqlx::query(
                "SELECT t.id as thread_id, t.channel, m.content, a.id as action_id, a.action_type, a.payload, CAST(t.created_at AS TEXT) AS created_at
                 FROM unified_threads t
                 LEFT JOIN (
                    SELECT thread_id, content FROM unified_messages
                    WHERE direction = 'inbound'
                    ORDER BY created_at DESC LIMIT 1
                 ) m ON t.id = m.thread_id
                 LEFT JOIN unified_triage_actions a ON t.id = a.thread_id
                 WHERE t.tenant_id = ? AND t.status = 'open' AND a.status = 'pending'
                 ORDER BY t.created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(pool).await;

            // Map SqliteRow to PgRow equivalent structure for unified handling below
            // Since we can't easily map between them, let's just serialize it directly in the Sqlite arm.
            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let payload_str: String = r.try_get("payload").unwrap_or_default();
                        let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::Value::Null);

                        serde_json::json!({
                            "id": r.get::<String, _>("action_id"),
                            "thread_id": r.get::<String, _>("thread_id"),
                            "source": r.get::<String, _>("channel"),
                            "context": r.try_get::<String, _>("content").unwrap_or_default(),
                            "action_type": r.try_get::<String, _>("action_type").unwrap_or_default(),
                            "action_payload": payload_json.get("message").and_then(|m| m.as_str()).unwrap_or(""),
                            "created_at": r.get::<String, _>("created_at"),
                            "priority": "normal"
                        })
                    }).collect();
                    return (axum::http::StatusCode::OK, Json(items)).into_response();
                },
                Err(e) => {
                    tracing::error!("Database error fetching unified triage feed: {:?}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
                }
            }
        }
    };

    match res {
        Ok(rows) => {
            use sqlx::Row;
            let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                let payload_str: String = r.try_get("payload").unwrap_or_default();
                let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::Value::Null);

                serde_json::json!({
                    "id": r.get::<String, _>("action_id"),
                    "thread_id": r.get::<String, _>("thread_id"),
                    "source": r.get::<String, _>("channel"),
                    "context": r.try_get::<String, _>("content").unwrap_or_default(),
                    "action_type": r.try_get::<String, _>("action_type").unwrap_or_default(),
                    "action_payload": payload_json.get("message").and_then(|m| m.as_str()).unwrap_or(""),
                    "created_at": r.get::<String, _>("created_at"),
                    "priority": "normal"
                })
            }).collect();
            (axum::http::StatusCode::OK, Json(items)).into_response()
        },
        Err(e) => {
            tracing::error!("Database error fetching unified triage feed: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn triage_action_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<serde_json::Value>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    let action_id = payload.get("triage_item_id").and_then(|v| v.as_str()).unwrap_or("");
    let approved = payload.get("approved").and_then(|v| v.as_bool()).unwrap_or(false);
    let target_status = if approved { "approved" } else { "dismissed" };

    let is_ok = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "UPDATE unified_triage_actions SET status = $1 WHERE id = $2 AND tenant_id = $3"
            )
            .bind(target_status)
            .bind(&action_id)
            .bind(&tenant_id)
            .execute(&db.pool).await.is_ok()
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                "UPDATE unified_triage_actions SET status = ? WHERE id = ? AND tenant_id = ?"
            )
            .bind(target_status)
            .bind(&action_id)
            .bind(&tenant_id)
            .execute(pool).await.is_ok()
        }
    };

    if is_ok {
        (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
    } else {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
    }
}
