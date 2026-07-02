use axum::{
    extract::{Query, State, Path},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::auth_utils::{UiTenantQuery, ui_tenant_id};
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
    let customer_info = serde_json::json!({"name": "Instagram DM", "message": "Do you have vegan chocolate cake available this weekend?"});
    let suggested_actions = serde_json::json!([
        {
            "action_type": "Draft Reply",
            "message": "Hi! Yes, we have 2 vegan chocolate cakes left for this weekend"
        }
    ]);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query(
                "INSERT INTO inbound_signals (id, tenant_id, source, raw_payload, status) VALUES ($1, $2, $3, $4, 'PROCESSED')"
            )
            .bind(&signal_id)
            .bind(&tenant_id)
            .bind(&payload.source)
            .bind(sqlx::types::Json(&payload.payload))
            .execute(&db.pool).await;

            let _ = sqlx::query(
                "INSERT INTO daily_work_items (id, tenant_id, signal_id, intent, customer_info, suggested_actions, status) VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(&signal_id)
            .bind(&intent)
            .bind(sqlx::types::Json(&customer_info))
            .bind(sqlx::types::Json(&suggested_actions))
            .execute(&db.pool).await;
        },
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query(
                "INSERT INTO inbound_signals (id, tenant_id, source, raw_payload, status) VALUES (?, ?, ?, ?, 'PROCESSED')"
            )
            .bind(&signal_id)
            .bind(&tenant_id)
            .bind(&payload.source)
            .bind(serde_json::to_string(&payload.payload).unwrap_or_else(|_| "{}".to_string()))
            .execute(pool).await;

            let _ = sqlx::query(
                "INSERT INTO daily_work_items (id, tenant_id, signal_id, intent, customer_info, suggested_actions, status) VALUES (?, ?, ?, ?, ?, ?, 'PENDING')"
            )
            .bind(&work_item_id)
            .bind(&tenant_id)
            .bind(&signal_id)
            .bind(&intent)
            .bind(serde_json::to_string(&customer_info).unwrap_or_else(|_| "{}".to_string()))
            .bind(serde_json::to_string(&suggested_actions).unwrap_or_else(|_| "{}".to_string()))
            .execute(pool).await;
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
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let cache_key = format!("daily_work:{}:mobile:{}", tenant_id, mobile_optimized);
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
                    let pool1 = db_bg.pool.clone();
                    let pool2 = db_bg.pool.clone();
                    let t_bg1 = t_bg.clone();
                    let t_bg2 = t_bg.clone();



            let pool_env = db_bg.pool.clone();
            let t_env = t_bg.clone();
            let (work_res, orders_res, env_res) = tokio::join!(
                sqlx::query(if mobile_optimized { "SELECT id, signal_id, intent, '{}'::jsonb as customer_info, '{}'::jsonb as suggested_actions, status FROM daily_work_items WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC" } else { "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC" }).bind(&t_bg1).fetch_all(&pool1),
                sqlx::query(if mobile_optimized { "SELECT id, status, 0.0 as total_amount FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5" } else { "SELECT id, status, total_amount FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5" }).bind(&t_bg2).fetch_all(&pool2),
                sqlx::query("SELECT id, current_department, status, payload, routing_history FROM task_envelopes WHERE tenant_id = $1 AND status != 'COMPLETED' ORDER BY created_at DESC").bind(&t_env).fetch_all(&pool_env)
            );

            work_res.map(|rows| {
use sqlx::Row;
                        let mut items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                            serde_json::json!({
                                "id": r.get::<String, _>("id"),
                                "signal_id": r.try_get::<Option<String>, _>("signal_id").ok().flatten(),
                                "intent": r.get::<String, _>("intent"),
                                "customer_info": r.try_get::<Option<serde_json::Value>, _>("customer_info").ok().flatten(),
                                "suggested_actions": r.try_get::<Option<serde_json::Value>, _>("suggested_actions").ok().flatten(),
                                "status": r.get::<String, _>("status")
                            })
                        }).collect();
                if let Ok(orders) = orders_res {
                    for o in orders {
                        items.push(serde_json::json!({
                            "id": o.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "recent_order",
                            "status": o.try_get::<String, _>("status").unwrap_or_default(),
                            "suggested_actions": null,
                        }));
                    }
                }
                if let Ok(envelopes) = env_res {
                    for e in envelopes {
                        let payload_str: String = e.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                        let payload_val: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
                        items.push(serde_json::json!({
                            "id": e.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "task_envelope",
                            "status": e.try_get::<String, _>("status").unwrap_or_default(),
                            "customer_info": { "department": e.try_get::<String, _>("current_department").unwrap_or_default() },
                            "suggested_actions": payload_val,
                        }));
                    }
                }
                items
            })

                },
                crate::db::DbStore::Sqlite(pool) => {
                    let pool1 = pool.clone();
                    let pool2 = pool.clone();
                    let t_bg1 = t_bg.clone();
                    let t_bg2 = t_bg.clone();



            let pool_env = pool.clone();
            let t_env = t_bg.clone();
            let (work_res, orders_res, env_res) = tokio::join!(
                sqlx::query(if mobile_optimized { "SELECT id, signal_id, intent, '{}' as customer_info, '{}' as suggested_actions, status FROM daily_work_items WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC" } else { "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC" }).bind(&t_bg1).fetch_all(&pool1),
                sqlx::query(if mobile_optimized { "SELECT id, status, 0.0 as total_amount FROM orders WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 5" } else { "SELECT id, status, total_amount FROM orders WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 5" }).bind(&t_bg2).fetch_all(&pool2),
                sqlx::query("SELECT id, current_department, status, payload, routing_history FROM task_envelopes WHERE tenant_id = ? AND status != 'COMPLETED' ORDER BY created_at DESC").bind(&t_env).fetch_all(&pool_env)
            );

            work_res.map(|rows| {
use sqlx::Row;
                        let mut items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
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
                if let Ok(orders) = orders_res {
                    for o in orders {
                        items.push(serde_json::json!({
                            "id": o.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "recent_order",
                            "status": o.try_get::<String, _>("status").unwrap_or_default(),
                            "suggested_actions": null,
                        }));
                    }
                }
                if let Ok(envelopes) = env_res {
                    for e in envelopes {
                        let payload_str: String = e.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                        let payload_val: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
                        items.push(serde_json::json!({
                            "id": e.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "task_envelope",
                            "status": e.try_get::<String, _>("status").unwrap_or_default(),
                            "customer_info": { "department": e.try_get::<String, _>("current_department").unwrap_or_default() },
                            "suggested_actions": payload_val,
                        }));
                    }
                }
                items
            })

                }
            };
            if let Ok(items) = res {
                if let Some(c) = DAILY_WORK_CACHE.get() {
                    c.set(&cache_key_bg, items, std::time::Duration::from_secs(30)).await;
                }
            }
        });

        return (axum::http::StatusCode::OK, Json(serde_json::json!({"items": cached}))).into_response();
    }

    let res = match &db.store {
        crate::db::DbStore::Postgres => {
            let pool1 = db.pool.clone();
            let pool2 = db.pool.clone();
            let t_bg1 = tenant_id.clone();
            let t_bg2 = tenant_id.clone();



            let pool_env = db.pool.clone();
            let t_env = tenant_id.clone();
            let (work_res, orders_res, env_res) = tokio::join!(
                sqlx::query(if mobile_optimized { "SELECT id, signal_id, intent, '{}'::jsonb as customer_info, '{}'::jsonb as suggested_actions, status FROM daily_work_items WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC" } else { "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC" }).bind(&t_bg1).fetch_all(&pool1),
                sqlx::query(if mobile_optimized { "SELECT id, status, 0.0 as total_amount FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5" } else { "SELECT id, status, total_amount FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5" }).bind(&t_bg2).fetch_all(&pool2),
                sqlx::query("SELECT id, current_department, status, payload, routing_history FROM task_envelopes WHERE tenant_id = $1 AND status != 'COMPLETED' ORDER BY created_at DESC").bind(&t_env).fetch_all(&pool_env)
            );

            work_res.map(|rows| {
use sqlx::Row;
                let mut items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                    serde_json::json!({
                        "id": r.get::<String, _>("id"),
                        "signal_id": r.try_get::<Option<String>, _>("signal_id").ok().flatten(),
                        "intent": r.get::<String, _>("intent"),
                        "customer_info": r.try_get::<Option<serde_json::Value>, _>("customer_info").ok().flatten(),
                        "suggested_actions": r.try_get::<Option<serde_json::Value>, _>("suggested_actions").ok().flatten(),
                        "status": r.get::<String, _>("status")
                    })
                }).collect();
                if let Ok(orders) = orders_res {
                    for o in orders {
                        items.push(serde_json::json!({
                            "id": o.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "recent_order",
                            "status": o.try_get::<String, _>("status").unwrap_or_default(),
                            "suggested_actions": null,
                        }));
                    }
                }
                if let Ok(envelopes) = env_res {
                    for e in envelopes {
                        let payload_str: String = e.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                        let payload_val: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
                        items.push(serde_json::json!({
                            "id": e.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "task_envelope",
                            "status": e.try_get::<String, _>("status").unwrap_or_default(),
                            "customer_info": { "department": e.try_get::<String, _>("current_department").unwrap_or_default() },
                            "suggested_actions": payload_val,
                        }));
                    }
                }
                items
            })

        },
        crate::db::DbStore::Sqlite(pool) => {
            let pool1 = pool.clone();
            let pool2 = pool.clone();
            let t_bg1 = tenant_id.clone();
            let t_bg2 = tenant_id.clone();



            let pool_env = pool.clone();
            let t_env = tenant_id.clone();
            let (work_res, orders_res, env_res) = tokio::join!(
                sqlx::query(if mobile_optimized { "SELECT id, signal_id, intent, '{}' as customer_info, '{}' as suggested_actions, status FROM daily_work_items WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC" } else { "SELECT id, signal_id, intent, customer_info, suggested_actions, status FROM daily_work_items WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC" }).bind(&t_bg1).fetch_all(&pool1),
                sqlx::query(if mobile_optimized { "SELECT id, status, 0.0 as total_amount FROM orders WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 5" } else { "SELECT id, status, total_amount FROM orders WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 5" }).bind(&t_bg2).fetch_all(&pool2),
                sqlx::query("SELECT id, current_department, status, payload, routing_history FROM task_envelopes WHERE tenant_id = ? AND status != 'COMPLETED' ORDER BY created_at DESC").bind(&t_env).fetch_all(&pool_env)
            );

            work_res.map(|rows| {
use sqlx::Row;
                let mut items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
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
                if let Ok(orders) = orders_res {
                    for o in orders {
                        items.push(serde_json::json!({
                            "id": o.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "recent_order",
                            "status": o.try_get::<String, _>("status").unwrap_or_default(),
                            "suggested_actions": null,
                        }));
                    }
                }
                if let Ok(envelopes) = env_res {
                    for e in envelopes {
                        let payload_str: String = e.try_get("payload").unwrap_or_else(|_| "{}".to_string());
                        let payload_val: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
                        items.push(serde_json::json!({
                            "id": e.try_get::<String, _>("id").unwrap_or_default(),
                            "intent": "task_envelope",
                            "status": e.try_get::<String, _>("status").unwrap_or_default(),
                            "customer_info": { "department": e.try_get::<String, _>("current_department").unwrap_or_default() },
                            "suggested_actions": payload_val,
                        }));
                    }
                }
                items
            })

        }
    };

    match res {
        Ok(items) => {
            cache.set(&cache_key, items.clone(), std::time::Duration::from_secs(30)).await;
            (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
        },
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to load daily work");
            tracing::error!("Failed to load daily work: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response()
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

            let res2 = sqlx::query(
                "UPDATE task_envelopes SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2"
            )
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool).await;

            if res.is_ok() || res2.is_ok() {
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

            let res2 = sqlx::query(
                "UPDATE task_envelopes SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
            )
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool).await;

            if res.is_ok() || res2.is_ok() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }

}

pub async fn list_action_cards_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "SELECT id, tenant_id, message_id, card_type, content_json, status, CAST(created_at AS text) as created_at FROM action_cards WHERE tenant_id = $1 AND status = 'pending' ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool)
            .await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        serde_json::json!({
                            "id": r.get::<String, _>("id"),
                            "tenant_id": r.get::<String, _>("tenant_id"),
                            "message_id": r.try_get::<Option<String>, _>("message_id").ok().flatten(),
                            "card_type": r.get::<String, _>("card_type"),
                            "content_json": r.get::<String, _>("content_json"),
                            "status": r.get::<String, _>("status"),
                            "created_at": r.try_get::<String, _>("created_at").unwrap_or_default(),
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to list action cards: {}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "SELECT id, tenant_id, message_id, card_type, content_json, status, CAST(created_at AS text) as created_at FROM action_cards WHERE tenant_id = ? AND status = 'pending' ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        serde_json::json!({
                            "id": r.get::<String, _>("id"),
                            "tenant_id": r.get::<String, _>("tenant_id"),
                            "message_id": r.try_get::<Option<String>, _>("message_id").ok().flatten(),
                            "card_type": r.get::<String, _>("card_type"),
                            "content_json": r.get::<String, _>("content_json"),
                            "status": r.get::<String, _>("status"),
                            "created_at": r.try_get::<String, _>("created_at").unwrap_or_default(),
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to list action cards: {}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
                }
            }
        }
    }
}

pub async fn approve_action_card_handler(
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<ApproveDailyWorkRequest>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);
    let target_status = if payload.action_status == "DISMISSED" {
        "discarded"
    } else {
        "approved"
    };

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE action_cards SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool)
            .await;

            match res {
                Ok(_) => (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
                Err(e) => {
                    tracing::error!("Failed to update action card: {}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE action_cards SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool)
            .await;

            match res {
                Ok(_) => (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
                Err(e) => {
                    tracing::error!("Failed to update action card: {}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"}))).into_response()
                }
            }
        }
    }
}
