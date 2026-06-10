use axum::{
    routing::get,
    Json, Router,
    extract::Extension,
    http::StatusCode,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::server_core::Hub;

pub fn pos_routes() -> Router {
    Router::new()
        .route("/orders", get(get_orders))
        .route("/inventory", get(get_inventory))
        .route("/catalog", get(get_catalog))
}

async fn get_orders() -> Json<Value> {
    Json(json!({
        "orders": [
            {
                "id": "ord_1",
                "status": "pending",
                "items": [{"name": "Coffee", "quantity": 2}]
            }
        ]
    }))
}

async fn get_inventory() -> Json<Value> {
    Json(json!({
        "inventory": [
            {
                "id": "inv_1",
                "name": "Coffee Beans",
                "stock": 100
            }
        ]
    }))
}

async fn get_catalog(
    auth_info: Option<Extension<::server_auth::orchestration::AuthInfo>>,
) -> (StatusCode, Json<Value>) {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthenticated: Missing tenant ID" })));
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthenticated" })))
    };

    let pool = crate::db::get_pool();
    let rows = sqlx::query!("SELECT id, COALESCE(title, name) as name, price_cents FROM products WHERE tenant_id = $1 OR organization_id = $1 LIMIT 100", tenant_id)
        .fetch_all(&pool)
        .await;

    match rows {
        Ok(records) => {
            let mut items = Vec::new();
            for rec in records {
                items.push(json!({
                    "id": rec.id,
                    "name": rec.name.unwrap_or_else(|| "Unnamed Item".to_string()),
                    "price_cents": rec.price_cents.unwrap_or(0),
                }));
            }
            (StatusCode::OK, Json(json!(items)))
        },
        Err(e) => {
            tracing::error!("Failed to fetch pos catalog: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Internal Server Error" })))
        }
    }
}
