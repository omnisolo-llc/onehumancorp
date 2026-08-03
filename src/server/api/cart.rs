use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;

#[derive(Deserialize)]
pub struct CreateCartRequest {
    pub customer_id: Option<String>,
    pub channel: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize)]
pub struct CartResponse {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub status: String,
    pub total_amount_cents: i64,
    pub currency: String,
    pub items: Vec<CartItemResponse>,
}

#[derive(Serialize)]
pub struct CartItemResponse {
    pub id: String,
    pub product_id: String,
    pub variant_id: Option<String>,
    pub quantity: i32,
    pub unit_price_cents: i64,
}

#[derive(Deserialize)]
pub struct AddCartItemRequest {
    pub product_id: String,
    pub variant_id: Option<String>,
    pub quantity: i32,
    pub unit_price_cents: i64,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/", axum::routing::post(create_cart_handler))
        .route("/{cart_id}", axum::routing::get(get_cart_handler))
        .route("/{cart_id}/items", axum::routing::post(add_cart_item_handler))
        .route("/{cart_id}/status", axum::routing::post(update_cart_status_handler))
        .with_state(hub)
}

pub async fn create_cart_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<CreateCartRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated: Missing tenant ID" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    let cart_id = uuid::Uuid::new_v4().to_string();
    let channel = req_data.channel.clone().unwrap_or_else(|| "online".to_string());
    let currency = req_data.currency.clone().unwrap_or_else(|| "usd".to_string());

    let pool = crate::db::get_pool();

    let res = sqlx::query(
        "INSERT INTO carts (id, tenant_id, customer_id, channel, status, currency) VALUES ($1, $2, $3, $4, 'active', $5)"
    )
    .bind(&cart_id)
    .bind(&tenant_id)
    .bind(&req_data.customer_id)
    .bind(&channel)
    .bind(&currency)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => {
            let response = CartResponse {
                id: cart_id,
                tenant_id,
                customer_id: req_data.customer_id.clone(),
                channel,
                status: "active".to_string(),
                total_amount_cents: 0,
                currency,
                items: vec![],
            };
            (axum::http::StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create cart: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to create cart" }))).into_response()
        }
    }
}

pub async fn get_cart_handler(
    Path(cart_id): Path<String>,
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    let pool = crate::db::get_pool();

    let (cart_res, items_res) = tokio::join!(
        async {
            sqlx::query("SELECT id, customer_id, channel, status, total_amount_cents, currency FROM carts WHERE id = $1 AND tenant_id = $2")
                .bind(&cart_id)
                .bind(&tenant_id)
                .fetch_optional(&pool)
                .await
        },
        async {
            sqlx::query("SELECT id, product_id, variant_id, quantity, unit_price_cents FROM cart_items WHERE cart_id = $1 AND tenant_id = $2")
                .bind(&cart_id)
                .bind(&tenant_id)
                .fetch_all(&pool)
                .await
        }
    );

    let cart_row = match cart_res {
        Ok(Some(row)) => row,
        Ok(None) => return (axum::http::StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Cart not found" }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch cart: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to fetch cart" }))).into_response();
        }
    };

    let items_rows = match items_res {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch cart items: {}", e);
            vec![]
        }
    };

    let items: Vec<CartItemResponse> = items_rows.into_iter().map(|row| {
        use sqlx::Row;
        CartItemResponse {
            id: row.get("id"),
            product_id: row.get("product_id"),
            variant_id: row.try_get("variant_id").unwrap_or(None),
            quantity: row.get("quantity"),
            unit_price_cents: row.get("unit_price_cents"),
        }
    }).collect();

    use sqlx::Row;
    let response = CartResponse {
        id: cart_row.get("id"),
        tenant_id,
        customer_id: cart_row.try_get("customer_id").unwrap_or(None),
        channel: cart_row.get("channel"),
        status: cart_row.get("status"),
        total_amount_cents: cart_row.get("total_amount_cents"),
        currency: cart_row.get("currency"),
        items,
    };

    (axum::http::StatusCode::OK, Json(response)).into_response()
}

pub async fn add_cart_item_handler(
    Path(cart_id): Path<String>,
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<AddCartItemRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    // First check if cart exists and is active
    let pool = crate::db::get_pool();
    let status: Option<String> = match sqlx::query_scalar("SELECT status FROM carts WHERE id = $1 AND tenant_id = $2")
        .bind(&cart_id)
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await
    {
        Ok(s) => s,
        Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
    };

    match status {
        Some(s) if s != "active" => return (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Cart is not active" }))).into_response(),
        None => return (axum::http::StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Cart not found" }))).into_response(),
        _ => {}
    }

    // Check inventory - this creates a soft reservation or just checks bounds depending on your system,
    // but typically we can do a reserve here if needed.
    // For simplicity, let's just do a reserve
    let inventory_service = crate::services::inventory::InventoryService::new(hub.redis_client());
    let reserve_result = inventory_service.reserve_inventory(&tenant_id, &req_data.product_id, req_data.quantity, 900).await;

    match reserve_result {
        Ok(res) if !res.success => {
            return (axum::http::StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Item is currently being checked out" }))).into_response();
        },
        Err(e) => {
            tracing::error!("Inventory service error: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Inventory check failed" }))).into_response();
        },
        _ => {} // Reserved
    }

    let item_id = uuid::Uuid::new_v4().to_string();
    let res = sqlx::query(
        "INSERT INTO cart_items (id, tenant_id, cart_id, product_id, variant_id, quantity, unit_price_cents) VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(&item_id)
    .bind(&tenant_id)
    .bind(&cart_id)
    .bind(&req_data.product_id)
    .bind(&req_data.variant_id)
    .bind(&req_data.quantity)
    .bind(&req_data.unit_price_cents)
    .execute(&pool)
    .await;

    if res.is_err() {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to add item to cart" }))).into_response();
    }

    // Apply Dynamic Pricing
    let final_unit_price = crate::pricing::engine::apply_dynamic_pricing(
        &pool,
        &tenant_id,
        &req_data.product_id,
        req_data.unit_price_cents,
    ).await;

    // Update cart total
    let _ = sqlx::query(
        "UPDATE carts SET total_amount_cents = total_amount_cents + $1 WHERE id = $2 AND tenant_id = $3"
    )
    .bind(final_unit_price * (req_data.quantity as i64))
    .bind(&cart_id)
    .bind(&tenant_id)
    .execute(&pool)
    .await;

    (axum::http::StatusCode::OK, Json(serde_json::json!({ "success": true, "item_id": item_id }))).into_response()
}

#[derive(Deserialize)]
pub struct UpdateCartStatusRequest {
    pub status: String,
}

pub async fn update_cart_status_handler(
    Path(cart_id): Path<String>,
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<UpdateCartStatusRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response()
    };

    let pool = crate::db::get_pool();
    let res = sqlx::query("UPDATE carts SET status = $1 WHERE id = $2 AND tenant_id = $3")
        .bind(&req_data.status)
        .bind(&cart_id)
        .bind(&tenant_id)
        .execute(&pool)
        .await;

    match res {
        Ok(result) => {
            if result.rows_affected() > 0 {
                (axum::http::StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response()
            } else {
                (axum::http::StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Cart not found" }))).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to update cart status: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Failed to update cart status" }))).into_response()
        }
    }
}
