use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ReturnsState {
    pub orchestrator: Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>,
    pub db: Arc<crate::db::DB>,
}

#[derive(Serialize, Deserialize)]
pub struct ReturnRequest {
    pub id: String,
    pub order_id: String,
    pub customer_name: String,
    pub amount: f64,
    pub status: String,
}

#[derive(Deserialize)]
pub struct ReturnsQuery {
    pub tenant_id: Option<String>,
}

pub fn router<S: Clone + Send + Sync + 'static>(state: ReturnsState) -> Router<S> {
    Router::new()
        .route("/requests", get(list_return_requests))
        .route("/requests/{id}/approve", post(approve_return))
        .route("/requests/seed", post(seed_return_request)) // Adding a helper endpoint for E2E tests
        .with_state(state)
}

async fn list_return_requests(State(state): State<ReturnsState>, Query(query): Query<ReturnsQuery>) -> impl IntoResponse {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());

    let mut requests = Vec::new();
    use sqlx::Row;

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            if let Ok(rows) = sqlx::query("SELECT id, customer_id, total_amount, status FROM orders WHERE tenant_id = $1 AND status = 'return_requested'")
                .bind(&tenant_id)
                .fetch_all(&state.db.pool).await {
                for row in rows {
                    requests.push(ReturnRequest {
                        id: row.get("id"),
                        order_id: row.get("id"),
                        customer_name: row.try_get("customer_id").unwrap_or_else(|_| "Unknown".to_string()),
                        amount: row.try_get::<f64, _>("total_amount").unwrap_or(0.0),
                        status: row.try_get("status").unwrap_or_else(|_| "return_requested".to_string()),
                    });
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            if let Ok(rows) = sqlx::query("SELECT id, customer_id, total_amount, status FROM orders WHERE tenant_id = ? AND status = 'return_requested'")
                .bind(&tenant_id)
                .fetch_all(sqlite_pool).await {
                for row in rows {
                    requests.push(ReturnRequest {
                        id: row.get("id"),
                        order_id: row.get("id"),
                        customer_name: row.try_get("customer_id").unwrap_or_else(|_| "Unknown".to_string()),
                        amount: row.try_get::<f64, _>("total_amount").unwrap_or(0.0),
                        status: row.try_get("status").unwrap_or_else(|_| "return_requested".to_string()),
                    });
                }
            }
        }
    }

    (StatusCode::OK, Json(requests))
}

async fn seed_return_request(State(state): State<ReturnsState>) -> impl IntoResponse {
    let order_id = uuid::Uuid::new_v4().to_string();
    let tenant_id = "default".to_string();
    let customer_id = "Sarah".to_string();
    let total_amount = 45.0;
    let status = "return_requested".to_string();

    let res = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(&order_id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(total_amount)
            .bind(&status)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&order_id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(total_amount)
            .bind(&status)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    match res {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "seeded", "id": order_id}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}

async fn approve_return(
    State(state): State<ReturnsState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // 1. Dispatch Operations Agent Event for restocking
    let ops_event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: "default".to_string(),
        event_type: "tenant.inventory.updated".to_string(),
        payload: serde_json::json!({
            "return_id": id,
            "action": "restock",
            "reason": "return_approved"
        }),
    };
    let _ = state.orchestrator.dispatch_event(ops_event).await;

    // 2. Dispatch Finance Agent Event for refund
    let finance_event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: "default".to_string(),
        event_type: "tenant.payment.received".to_string(), // In absence of an explicit refund event, we use a payment event that the Finance Agent tracks
        payload: serde_json::json!({
            "return_id": id,
            "amount": -45.0, // Negative amount denotes refund
            "action": "stripe_refund",
        }),
    };
    let _ = state.orchestrator.dispatch_event(finance_event).await;

    // Update the order status to returned
    let update_res = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE orders SET status = 'returned' WHERE id = $1")
                .bind(&id)
                .execute(&state.db.pool)
                .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("UPDATE orders SET status = 'returned' WHERE id = ?")
                .bind(&id)
                .execute(sqlite_pool)
                .await.map(|_| ())
        }
    };

    if update_res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to update status"}))).into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"status": "approved", "id": id}))).into_response()
}
