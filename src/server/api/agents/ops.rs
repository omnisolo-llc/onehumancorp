use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use crate::db::DB;
use sqlx::Row;

pub fn router(db: Arc<DB>) -> Router {
    Router::new()
        .route("/alerts", get(get_inventory_alerts))
        .route("/restock/:order_id/approve", post(approve_restock_order))
        .with_state(db)
}

#[derive(serde::Serialize)]
struct InventoryAlert {
    order_id: String,
    product_id: String,
    product_name: String,
    suggested_quantity: i32,
    days_until_stockout: f64,
}

async fn get_inventory_alerts(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = auth_info.tenant_id;

    let rows = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                r#"
                SELECT o.id as order_id, o.product_id, p.name as product_name, o.quantity, f.days_until_stockout
                FROM supplier_orders o
                JOIN products p ON o.product_id = p.id
                JOIN stock_forecasts f ON o.product_id = f.product_id
                WHERE o.tenant_id = $1 AND o.status = 'DRAFT'
                "#
            )
            .bind(tenant_id)
            .fetch_all(&db.pool)
            .await
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                r#"
                SELECT o.id as order_id, o.product_id, p.name as product_name, o.quantity, f.days_until_stockout
                FROM supplier_orders o
                JOIN products p ON o.product_id = p.id
                JOIN stock_forecasts f ON o.product_id = f.product_id
                WHERE o.tenant_id = ? AND o.status = 'DRAFT'
                "#
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
        }
    };

    match rows {
        Ok(rows) => {
            let alerts: Vec<InventoryAlert> = rows.into_iter().map(|row| {
                InventoryAlert {
                    order_id: row.get("order_id"),
                    product_id: row.get("product_id"),
                    product_name: row.try_get("product_name").unwrap_or_else(|_| "Unknown Product".to_string()),
                    suggested_quantity: row.get("quantity"),
                    days_until_stockout: row.get("days_until_stockout"),
                }
            }).collect();
            (StatusCode::OK, Json(alerts)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch inventory alerts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to fetch alerts"}))).into_response()
        }
    }
}

async fn approve_restock_order(
    State(db): State<Arc<DB>>,
    Path(order_id): Path<String>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = auth_info.tenant_id;

    let result = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE supplier_orders SET status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                .bind(&order_id)
                .bind(tenant_id)
                .execute(&db.pool)
                .await
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE supplier_orders SET status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                .bind(&order_id)
                .bind(tenant_id)
                .execute(pool)
                .await
        }
    };

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to approve restock order: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to approve order"}))).into_response()
        }
    }
}
