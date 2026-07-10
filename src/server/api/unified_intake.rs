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
pub struct SimulateIntakeRequest {
    pub customer_name: String,
    pub channel: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApproveProposedTaskRequest {
    pub action_status: String, // "APPROVED" or "DISMISSED"
}

pub async fn simulate_intake_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<SimulateIntakeRequest>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);
    let intake_id = format!("in-{}", Uuid::new_v4());
    let job_id = format!("job-{}", Uuid::new_v4());

    match &db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query(
                "INSERT INTO intake_messages (id, tenant_id, customer_name, channel, message, status) VALUES ($1, $2, $3, $4, $5, 'pending_triage')"
            )
            .bind(&intake_id)
            .bind(&tenant_id)
            .bind(&payload.customer_name)
            .bind(&payload.channel)
            .bind(&payload.message)
            .execute(&db.pool).await;

            // Trigger AI Job
            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'UNIFIED_INTAKE_TRIAGE', $3, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(serde_json::json!({
                "intake_message_id": intake_id,
                "customer_name": payload.customer_name,
                "message": payload.message
            }))
            .execute(&db.pool).await;

            (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true, "intake_id": intake_id}))).into_response()
        },
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query(
                "INSERT INTO intake_messages (id, tenant_id, customer_name, channel, message, status) VALUES (?, ?, ?, ?, ?, 'pending_triage')"
            )
            .bind(&intake_id)
            .bind(&tenant_id)
            .bind(&payload.customer_name)
            .bind(&payload.channel)
            .bind(&payload.message)
            .execute(pool).await;

            // Trigger AI Job
            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'UNIFIED_INTAKE_TRIAGE', ?, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&tenant_id)
            .bind(serde_json::json!({
                "intake_message_id": intake_id,
                "customer_name": payload.customer_name,
                "message": payload.message
            }).to_string())
            .execute(pool).await;

            (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true, "intake_id": intake_id}))).into_response()
        }
    }
}

pub async fn get_triage_feed_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "SELECT t.id, t.status, t.draft_reply, i.message as context, i.customer_name, p.amount, b.scheduled_for FROM proposed_tasks t
                JOIN intake_messages i ON t.intake_message_id = i.id
                LEFT JOIN payment_links p ON t.payment_link_id = p.id
                LEFT JOIN booking_drafts b ON t.booking_draft_id = b.id
                WHERE t.tenant_id = $1 AND t.status = 'proposed' ORDER BY t.created_at DESC"
            ).bind(&tenant_id).fetch_all(&db.pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|e| {
                        let mut map = serde_json::Map::new();
                        map.insert("id".to_string(), serde_json::json!(e.try_get::<String, _>("id").unwrap_or_default()));
                        map.insert("status".to_string(), serde_json::json!(e.try_get::<String, _>("status").unwrap_or_default()));
                        map.insert("draft_reply".to_string(), serde_json::json!(e.try_get::<String, _>("draft_reply").unwrap_or_default()));
                        map.insert("context".to_string(), serde_json::json!(e.try_get::<String, _>("context").unwrap_or_default()));
                        map.insert("customer_name".to_string(), serde_json::json!(e.try_get::<String, _>("customer_name").unwrap_or_default()));

                        // Handle potential numeric
                        if let Ok(amount) = e.try_get::<sqlx::types::BigDecimal, _>("amount") {
                            use bigdecimal::ToPrimitive;
                            map.insert("deposit_amount".to_string(), serde_json::json!(amount.to_f64().unwrap_or(0.0)));
                        }

                        serde_json::Value::Object(map)
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
             let res = sqlx::query(
                "SELECT t.id, t.status, t.draft_reply, i.message as context, i.customer_name, p.amount, b.scheduled_for FROM proposed_tasks t
                JOIN intake_messages i ON t.intake_message_id = i.id
                LEFT JOIN payment_links p ON t.payment_link_id = p.id
                LEFT JOIN booking_drafts b ON t.booking_draft_id = b.id
                WHERE t.tenant_id = ? AND t.status = 'proposed' ORDER BY t.created_at DESC"
            ).bind(&tenant_id).fetch_all(pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|e| {
                        let mut map = serde_json::Map::new();
                        map.insert("id".to_string(), serde_json::json!(e.try_get::<String, _>("id").unwrap_or_default()));
                        map.insert("status".to_string(), serde_json::json!(e.try_get::<String, _>("status").unwrap_or_default()));
                        map.insert("draft_reply".to_string(), serde_json::json!(e.try_get::<String, _>("draft_reply").unwrap_or_default()));
                        map.insert("context".to_string(), serde_json::json!(e.try_get::<String, _>("context").unwrap_or_default()));
                        map.insert("customer_name".to_string(), serde_json::json!(e.try_get::<String, _>("customer_name").unwrap_or_default()));

                        if let Ok(amount) = e.try_get::<f64, _>("amount") {
                            map.insert("deposit_amount".to_string(), serde_json::json!(amount));
                        }

                        serde_json::Value::Object(map)
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}

pub async fn approve_proposed_task_handler(
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<ApproveProposedTaskRequest>,
) -> axum::response::Response {
    let tenant_id = ui_tenant_id(&query);
    let target_status = if payload.action_status == "DISMISSED" {
        "dismissed"
    } else {
        "approved"
    };

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE proposed_tasks SET status = $1 WHERE id = $2 AND tenant_id = $3"
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
                "UPDATE proposed_tasks SET status = ? WHERE id = ? AND tenant_id = ?"
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
