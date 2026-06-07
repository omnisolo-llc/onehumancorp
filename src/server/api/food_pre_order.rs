use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ::server_common::Claims;
use crate::hub::Hub;

#[derive(Clone, Serialize, Deserialize)]
pub struct PreOrderRequest {
    pub customer_name: String,
    pub items: Vec<String>,
    pub notes: Option<String>,
    pub total_amount: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdatePreOrderStatusRequest {
    pub status: String,
}

pub fn router(_hub: Arc<Hub>) -> Router<Arc<crate::db::DB>> {
    Router::new()
        .route("/create", post(create_pre_order))
        .route("/list", get(list_pre_orders))
        .route("/{id}/status", post(update_pre_order_status))
}

async fn create_pre_order(
    State(db): State<Arc<crate::db::DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<PreOrderRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "default".to_string(),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let status = "pending";
    let notes = payload.notes.unwrap_or_default();

    // Convert items vector to JSON string to save in DB if we had a column,
    // but we can just use metadata or a separate table.
    // For simplicity, we just insert into `orders` table.

    let res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO orders (id, tenant_id, total_amount, status, notes) VALUES ($1, $2, $3, $4, $5)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(payload.total_amount)
                .bind(status)
                .bind(&notes)
                .execute(&db.pool)
                .await
        }
        crate::db::DbStore::Sqlite(_pool) => {
            sqlx::query("INSERT INTO orders (id, tenant_id, total_amount, status, notes) VALUES (?, ?, ?, ?, ?)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(payload.total_amount)
                .bind(status)
                .bind(&notes)
                .execute(&db.pool)
                .await
        }
    };

    if res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create order"}))).into_response();
    }

    // Publish event
    if let Some(bus) = crate::msgbus::get_bus() {
        let event_payload = serde_json::json!({
            "tenant_id": tenant_id,
            "order_id": id,
            "customer_name": payload.customer_name,
            "items": payload.items,
            "notes": notes,
        });

        let msg = crate::msgbus::Message {
            topic: "tenant.order.created".to_string(),
            payload: event_payload.to_string().into_bytes(),
        };
        let _ = bus.publish(msg).await;
    }

    (StatusCode::OK, Json(serde_json::json!({"id": id, "status": status}))).into_response()
}

async fn list_pre_orders(
    State(db): State<Arc<crate::db::DB>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "default".to_string(),
    };

    use sqlx::Row;

    let orders = match &db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query("SELECT id, COALESCE(total_amount, 0.0) AS total_amount, COALESCE(status, '') AS status, COALESCE(notes, '') AS notes, COALESCE(translated_notes, '') AS translated_notes, COALESCE(created_at::text, '') AS created_at FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 50")
                .bind(&tenant_id)
                .fetch_all(&db.pool)
                .await {
                Ok(rows) => Ok(rows.into_iter().map(|row| serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "total_amount": row.get::<f64, _>("total_amount"),
                    "status": row.get::<String, _>("status"),
                    "notes": row.get::<String, _>("notes"),
                    "translated_notes": row.get::<String, _>("translated_notes"),
                    "created_at": row.get::<String, _>("created_at"),
                })).collect::<Vec<_>>()),
                Err(_) => Err(()),
            }
        }
        crate::db::DbStore::Sqlite(_pool) => {
            match sqlx::query("SELECT id, COALESCE(total_amount, 0.0) AS total_amount, COALESCE(status, '') AS status, COALESCE(notes, '') AS notes, COALESCE(translated_notes, '') AS translated_notes, COALESCE(CAST(created_at AS TEXT), '') AS created_at FROM orders WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 50")
                .bind(&tenant_id)
                .fetch_all(&db.pool)
                .await {
                Ok(rows) => Ok(rows.into_iter().map(|row| serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "total_amount": row.get::<f64, _>("total_amount"),
                    "status": row.get::<String, _>("status"),
                    "notes": row.get::<String, _>("notes"),
                    "translated_notes": row.get::<String, _>("translated_notes"),
                    "created_at": row.get::<String, _>("created_at"),
                })).collect::<Vec<_>>()),
                Err(_) => Err(()),
            }
        }
    };

    match orders {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch orders"}))).into_response(),
    }
}

async fn update_pre_order_status(
    State(db): State<Arc<crate::db::DB>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdatePreOrderStatusRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "default".to_string(),
    };

    let res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(&payload.status)
                .bind(&id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await
        }
        crate::db::DbStore::Sqlite(_pool) => {
            sqlx::query("UPDATE orders SET status = ? WHERE id = ? AND tenant_id = ?")
                .bind(&payload.status)
                .bind(&id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await
        }
    };

    if res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update order"}))).into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}
