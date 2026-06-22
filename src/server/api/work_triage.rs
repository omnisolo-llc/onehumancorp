use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;


use crate::db::DB;
use crate::harness::{UiTenantQuery, ui_tenant_id};
use axum::extract::Query;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulateInboundSignalRequest {
    pub source: String, // e.g. "INSTAGRAM_DM", "MISSED_CALL"
    pub raw_content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveDailyWorkRequest {
    pub action_status: String, // "APPROVED" or "DISMISSED"
}

pub async fn simulate_inbound_signal_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<SimulateInboundSignalRequest>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let tenant_id = ui_tenant_id(&query);
    let signal_id = format!("sig-{}", Uuid::new_v4());
    let work_item_id = format!("wi-{}", Uuid::new_v4());

    match &db.store {
        crate::db::DbStore::Postgres => {
            // First create a work item
            let _ = sqlx::query(
                "INSERT INTO work_items (id, tenant_id, title, description, status) VALUES ($1, $2, $3, $4, 'OPEN')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(format!("Inbound from {}", payload.source))
            .bind(&payload.raw_content)
            .execute(&db.pool).await;

            // Then queue the AI job (simulated here by direct draft creation)
            let draft_id = format!("draft-{}", Uuid::new_v4());
            let proposed_action = serde_json::json!({
                "action_type": "Draft Reply",
                "description": format!("Auto-generated reply to {}", payload.source),
            });
            let context = serde_json::json!({
                "signal_id": signal_id,
                "original_text": payload.raw_content
            });

            let res = sqlx::query(
                "INSERT INTO agent_drafts (id, tenant_id, work_item_id, proposed_action, context, status) VALUES ($1, $2, $3, $4, $5, 'PENDING')"
            )
            .bind(&draft_id)
            .bind(&tenant_id)
            .bind(&work_item_id)
            .bind(&proposed_action)
            .bind(&context)
            .execute(&db.pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::CREATED, Json(serde_json::json!({"success": true, "draft_id": draft_id}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
             // First create a work item
             let _ = sqlx::query(
                "INSERT INTO work_items (id, tenant_id, title, description, status) VALUES (?, ?, ?, ?, 'OPEN')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(format!("Inbound from {}", payload.source))
            .bind(&payload.raw_content)
            .execute(pool).await;

            // Then queue the AI job (simulated here by direct draft creation)
            let draft_id = format!("draft-{}", Uuid::new_v4());
            let proposed_action = serde_json::json!({
                "action_type": "Draft Reply",
                "description": format!("Auto-generated reply to {}", payload.source),
            });
            let context = serde_json::json!({
                "signal_id": signal_id,
                "original_text": payload.raw_content
            });

            let res = sqlx::query(
                "INSERT INTO agent_drafts (id, tenant_id, work_item_id, proposed_action, context, status) VALUES (?, ?, ?, ?, ?, 'PENDING')"
            )
            .bind(&draft_id)
            .bind(&tenant_id)
            .bind(&work_item_id)
            .bind(serde_json::to_string(&proposed_action).unwrap_or_default())
            .bind(serde_json::to_string(&context).unwrap_or_default())
            .execute(pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::CREATED, Json(serde_json::json!({"success": true, "draft_id": draft_id}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}

// Temporary in-memory cache for SQLite mock data.
static DAILY_WORK_CACHE: std::sync::OnceLock<::server_utils::cache::HybridCache<String, Vec<serde_json::Value>>> = std::sync::OnceLock::new();

pub async fn get_daily_work_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let tenant_id = ui_tenant_id(&query);
    let cache_key = format!("daily_work:{}", tenant_id);
    let cache = DAILY_WORK_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "SELECT d.id, d.work_item_id, d.proposed_action, d.context, d.status, w.title, w.description
                 FROM agent_drafts d
                 JOIN work_items w ON d.work_item_id = w.id
                 WHERE d.tenant_id = $1 AND d.status = 'PENDING'
                 ORDER BY d.created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let id: String = r.get("id");
                        let work_item_id: String = r.get("work_item_id");
                        let proposed_action_str: Option<String> = r.try_get("proposed_action").ok();
                        let proposed_action: Option<serde_json::Value> = proposed_action_str.and_then(|s| serde_json::from_str(&s).ok());
                        let context_str: Option<String> = r.try_get("context").ok();
                        let context: Option<serde_json::Value> = context_str.and_then(|s| serde_json::from_str(&s).ok());
                        let status: String = r.get("status");
                        let title: String = r.get("title");
                        let description: String = r.get("description");

                        serde_json::json!({
                            "id": id,
                            "work_item_id": work_item_id,
                            "proposed_action": proposed_action,
                            "context": context,
                            "status": status,
                            "title": title,
                            "description": description
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            if let Ok(Some(cached_data)) = cache.get(&cache_key).await {
                return (axum::http::StatusCode::OK, Json(serde_json::json!({"items": cached_data}))).into_response();
            }

            let res = sqlx::query(
                "SELECT d.id, d.work_item_id, d.proposed_action, d.context, d.status, w.title, w.description
                 FROM agent_drafts d
                 JOIN work_items w ON d.work_item_id = w.id
                 WHERE d.tenant_id = ? AND d.status = 'PENDING'
                 ORDER BY d.created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let mut items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                         let proposed_action_str: Option<String> = r.try_get("proposed_action").ok();
                        let proposed_action: Option<serde_json::Value> = proposed_action_str.and_then(|s| serde_json::from_str(&s).ok());

                        let context_str: Option<String> = r.try_get("context").ok();
                        let context: Option<serde_json::Value> = context_str.and_then(|s| serde_json::from_str(&s).ok());

                        let id: String = r.get("id");
                        let work_item_id: String = r.get("work_item_id");
                        let status: String = r.get("status");
                        let title: String = r.get("title");
                        let description: String = r.get("description");

                        serde_json::json!({
                            "id": id,
                            "work_item_id": work_item_id,
                            "proposed_action": proposed_action,
                            "context": context,
                            "status": status,
                            "title": title,
                            "description": description
                        })
                    }).collect();

                    if items.is_empty() {
                         items = vec![
                            serde_json::json!({
                                "id": "mock-draft-1",
                                "work_item_id": "mock-wi-1",
                                "status": "PENDING",
                                "title": "Missed Call from Carlos",
                                "description": "Customer called 10 mins ago. Left no voicemail.",
                                "proposed_action": {
                                    "action_type": "Draft Reply",
                                    "description": "Send SMS: 'Hi, sorry I missed you. Need an estimate?'"
                                },
                                "context": {
                                    "customer_name": "Carlos",
                                    "phone": "+1234567890"
                                }
                            }),
                            serde_json::json!({
                                "id": "mock-draft-2",
                                "work_item_id": "mock-wi-2",
                                "status": "PENDING",
                                "title": "Instagram DM Inquiry",
                                "description": "Maya asked about custom cake pricing.",
                                "proposed_action": {
                                    "action_type": "Draft Quote",
                                    "description": "Send Estimate for $150 (Custom Tier 2)"
                                },
                                "context": {
                                    "customer_name": "Maya",
                                    "platform": "Instagram"
                                }
                            })
                        ];
                        let _ = cache.set(cache_key.clone(), items.clone(), std::time::Duration::from_secs(300)).await;
                    }

                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        }
    }
}

pub async fn approve_daily_work_handler(
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<ApproveDailyWorkRequest>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let tenant_id = ui_tenant_id(&query);

    let target_status = if payload.action_status == "DISMISSED" {
        "DISMISSED"
    } else {
        "APPROVED"
    };

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE agent_drafts SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool).await;

            if res.is_ok() {
                // Remove from cache to force refresh
                let cache_key = format!("daily_work:{}", tenant_id);
                let cache = DAILY_WORK_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));
                let _ = cache.remove(&cache_key).await;

                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE agent_drafts SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool).await;

            if res.is_ok() {
                 let cache_key = format!("daily_work:{}", tenant_id);
                let cache = DAILY_WORK_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));
                if let Ok(Some(mut cached_data)) = cache.get(&cache_key).await {
                    cached_data.retain(|item| item["id"].as_str() != Some(&id));
                    let _ = cache.set(cache_key.clone(), cached_data, std::time::Duration::from_secs(300)).await;
                }

                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}
