use axum::{
    extract::{Query, State, Path},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::auth::{ui_tenant_id, UiTenantQuery};
use crate::db::DB;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulateInboundSignalRequest {
    pub source: String,
    pub payload: serde_json::Value,
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
    let tenant_id = ui_tenant_id(&query);
    let signal_id = format!("sig-{}", Uuid::new_v4());
    let work_item_id = format!("wi-{}", Uuid::new_v4());

    // Basic LLM simulation
    let intent = "inquiry".to_string();
    let customer_info = serde_json::json!({"name": "Simulated Customer"});
    let suggested_actions = serde_json::json!([
        {
            "action_type": "Draft Reply",
            "message": "This is an AI-generated draft based on the inbound signal."
        }
    ]);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to start transaction: {}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
                }
            };

            if let Err(e) = sqlx::query(
                "INSERT INTO inbound_signals (id, tenant_id, source, raw_payload, status) VALUES ($1, $2, $3, $4, 'PROCESSED')"
            )
            .bind(&signal_id)
            .bind(&tenant_id)
            .bind(&payload.source)
            .bind(sqlx::types::Json(&payload.payload))
            .execute(&mut *tx).await {
                tracing::error!("Failed to insert inbound signal: {}", e);
                let _ = tx.rollback().await;
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }

            if let Err(e) = sqlx::query(
                "INSERT INTO daily_work_items (id, tenant_id, signal_id, intent, customer_info, suggested_actions, status) VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(&signal_id)
            .bind(&intent)
            .bind(sqlx::types::Json(&customer_info))
            .bind(sqlx::types::Json(&suggested_actions))
            .execute(&mut *tx).await {
                tracing::error!("Failed to insert daily work item: {}", e);
                let _ = tx.rollback().await;
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }

            let job_id = format!("job-{}", Uuid::new_v4());
            let payload_json = serde_json::json!({
                "work_item_id": work_item_id,
                "signal_id": signal_id,
                "intent": intent,
                "customer_info": customer_info,
                "suggested_actions": suggested_actions
            });
            if let Err(e) = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'work_triage', $3, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(sqlx::types::Json(payload_json))
            .execute(&mut *tx).await {
                tracing::error!("Failed to enqueue background job: {}", e);
                let _ = tx.rollback().await;
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }

            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let mut tx = match pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to start transaction (sqlite): {}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
                }
            };

            if let Err(e) = sqlx::query(
                "INSERT INTO inbound_signals (id, tenant_id, source, raw_payload, status) VALUES (?, ?, ?, ?, 'PROCESSED')"
            )
            .bind(&signal_id)
            .bind(&tenant_id)
            .bind(&payload.source)
            .bind(serde_json::to_string(&payload.payload).unwrap())
            .execute(&mut *tx).await {
                tracing::error!("Failed to insert inbound signal (sqlite): {}", e);
                let _ = tx.rollback().await;
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }

            if let Err(e) = sqlx::query(
                "INSERT INTO daily_work_items (id, tenant_id, signal_id, intent, customer_info, suggested_actions, status) VALUES (?, ?, ?, ?, ?, ?, 'PENDING')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(&signal_id)
            .bind(&intent)
            .bind(serde_json::to_string(&customer_info).unwrap())
            .bind(serde_json::to_string(&suggested_actions).unwrap())
            .execute(&mut *tx).await {
                tracing::error!("Failed to insert daily work item (sqlite): {}", e);
                let _ = tx.rollback().await;
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }

            let job_id = format!("job-{}", Uuid::new_v4());
            let payload_json = serde_json::json!({
                "work_item_id": work_item_id,
                "signal_id": signal_id,
                "intent": intent,
                "customer_info": customer_info,
                "suggested_actions": suggested_actions
            });
            if let Err(e) = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'work_triage', ?, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(serde_json::to_string(&payload_json).unwrap())
            .execute(&mut *tx).await {
                tracing::error!("Failed to enqueue background job (sqlite): {}", e);
                let _ = tx.rollback().await;
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }

            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction (sqlite): {}", e);
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
            }
        }
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({"id": work_item_id, "success": true}))).into_response()
}

#[derive(Serialize, sqlx::FromRow)]
pub struct DailyWorkItemRow {
    pub id: String,
    pub signal_id: Option<String>,
    pub intent: String,
    pub customer_info: Option<serde_json::Value>,
    pub suggested_actions: Option<serde_json::Value>,
    pub status: String,
}


static DAILY_WORK_CACHE: std::sync::OnceLock<::server_utils::cache::HybridCache<Vec<serde_json::Value>>> = std::sync::OnceLock::new();

pub async fn get_daily_work_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);
    let cache_key = format!("daily_work:{}", tenant_id);
    let cache = DAILY_WORK_CACHE.get_or_init(|| ::server_utils::cache::HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (axum::http::StatusCode::OK, Json(serde_json::json!({"items": cached}))).into_response();
        }

        let db_bg = db.clone();
        let t_bg = tenant_id.clone();
        let cache_key_bg = cache_key.clone();

        tokio::spawn(async move {
            let res = match &db_bg.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query!(
                        "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC",
                        t_bg
                    ).fetch_all(&db_bg.pool).await.map(|rows| {
                        rows.into_iter().map(|r| {
                            serde_json::json!({
                                "id": r.id,
                                "signal_id": r.signal_id,
                                "intent": r.intent,
                                "customer_info": r.customer_info,
                                "suggested_actions": r.suggested_actions,
                                "status": r.status
                            })
                        }).collect::<Vec<_>>()
                    })
                },
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC"
                    ).bind(&t_bg).fetch_all(pool).await.map(|rows| {
                        use sqlx::Row;
                        rows.into_iter().map(|r| {
                            let customer_info_str: Option<String> = r.try_get("customer_info").ok();
                            let customer_info: Option<serde_json::Value> = customer_info_str.and_then(|s| serde_json::from_str(&s).ok());

                            let suggested_actions_str: Option<String> = r.try_get("suggested_actions").ok();
                            let suggested_actions: Option<serde_json::Value> = suggested_actions_str.and_then(|s| serde_json::from_str(&s).ok());

                            let id: String = r.get("id");
                            let signal_id: Option<String> = r.try_get("signal_id").ok().flatten();
                            let intent: String = r.get("intent");
                            let status: String = r.get("status");

                            serde_json::json!({
                                "id": id,
                                "signal_id": signal_id,
                                "intent": intent,
                                "customer_info": customer_info,
                                "suggested_actions": suggested_actions,
                                "status": status
                            })
                        }).collect::<Vec<_>>()
                    })
                }
            };
            if let Ok(items) = res {
                if let Some(c) = DAILY_WORK_CACHE.get() {
                    let _ = c.set(&cache_key_bg, items, std::time::Duration::from_secs(10)).await;
                }
            }
        });
        return (axum::http::StatusCode::OK, Json(serde_json::json!({"items": cached}))).into_response();
    }

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query!(
                "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC",
                tenant_id
            )
            .fetch_all(&db.pool).await;

            match res {
                Ok(rows) => {
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        serde_json::json!({
                            "id": r.id,
                            "signal_id": r.signal_id,
                            "intent": r.intent,
                            "customer_info": r.customer_info,
                            "suggested_actions": r.suggested_actions,
                            "status": r.status
                        })
                    }).collect();
                    let _ = cache.set(&cache_key, items.clone(), std::time::Duration::from_secs(10)).await;
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
             let res = sqlx::query(
                "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let customer_info_str: Option<String> = r.try_get("customer_info").ok();
                        let customer_info: Option<serde_json::Value> = customer_info_str.and_then(|s| serde_json::from_str(&s).ok());

                        let suggested_actions_str: Option<String> = r.try_get("suggested_actions").ok();
                        let suggested_actions: Option<serde_json::Value> = suggested_actions_str.and_then(|s| serde_json::from_str(&s).ok());

                        let id: String = r.get("id");
                        let signal_id: Option<String> = r.try_get("signal_id").ok().flatten();
                        let intent: String = r.get("intent");
                        let status: String = r.get("status");

                        serde_json::json!({
                            "id": id,
                            "signal_id": signal_id,
                            "intent": intent,
                            "customer_info": customer_info,
                            "suggested_actions": suggested_actions,
                            "status": status
                        })
                    }).collect();
                    let _ = cache.set(&cache_key, items.clone(), std::time::Duration::from_secs(10)).await;
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
    let tenant_id = ui_tenant_id(&query);

    let target_status = if payload.action_status == "DISMISSED" {
        "DISMISSED"
    } else {
        "APPROVED"
    };

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE daily_work_items SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE daily_work_items SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}
