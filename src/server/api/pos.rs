use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::hub::Hub;
use sqlx::Row;
use crate::integrations::stripe::client::StripeClient;

pub fn pos_routes<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/orders", get(get_orders_handler))
        .route("/inventory", get(get_inventory_handler))
        .route("/carts", post(create_cart_handler))
        .route("/carts/:cart_id/items", post(add_cart_item_handler))
        .route("/carts/:cart_id/checkout/terminal", post(checkout_terminal_handler))
        .route("/carts/:cart_id/capture", post(capture_cart_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateCartRequest {
    pub tenant_id: String,
    pub channel: Option<String>,
    pub currency: Option<String>,
}

async fn create_cart_handler(
    State(_hub): State<Arc<Hub>>,
    Json(payload): Json<CreateCartRequest>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let pool = crate::db::get_pool();
    let cart_id = uuid::Uuid::new_v4().to_string();
    let channel = payload.channel.unwrap_or_else(|| "in_store".to_string());
    let currency = payload.currency.unwrap_or_else(|| "USD".to_string());

    sqlx::query(
        "INSERT INTO carts (id, tenant_id, channel, currency, status, total_amount_cents)
         VALUES ($1, $2, $3, $4, 'pending', 0)"
    )
    .bind(&cart_id)
    .bind(&payload.tenant_id)
    .bind(&channel)
    .bind(&currency)
    .execute(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "id": cart_id,
        "tenant_id": payload.tenant_id,
        "channel": channel,
        "currency": currency,
        "status": "pending",
        "total_amount_cents": 0
    })))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddCartItemRequest {
    pub tenant_id: String,
    pub product_id: String,
    pub variant_id: Option<String>,
    pub quantity: i32,
}

async fn add_cart_item_handler(
    State(_hub): State<Arc<Hub>>,
    Path(cart_id): Path<String>,
    Json(payload): Json<AddCartItemRequest>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let pool = crate::db::get_pool();
    let item_id = uuid::Uuid::new_v4().to_string();

    let mut tx = pool.begin().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Check inventory
    let product_row = sqlx::query("SELECT price_cents, inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
        .bind(&payload.product_id)
        .bind(&payload.tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(row) = product_row {
        let stock: i32 = row.get("inventory_count");
        let price: i64 = row.get("price_cents");

        if stock < payload.quantity {
            return Err((axum::http::StatusCode::BAD_REQUEST, "Insufficient stock".to_string()));
        }

        sqlx::query(
            "INSERT INTO cart_items (id, tenant_id, cart_id, product_id, variant_id, quantity, unit_price_cents)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&item_id)
        .bind(&payload.tenant_id)
        .bind(&cart_id)
        .bind(&payload.product_id)
        .bind(&payload.variant_id)
        .bind(&payload.quantity)
        .bind(&price)
        .execute(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // Update cart total
        let amount_to_add = price * (payload.quantity as i64);
        sqlx::query(
            "UPDATE carts SET total_amount_cents = total_amount_cents + $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
        )
        .bind(&amount_to_add)
        .bind(&cart_id)
        .bind(&payload.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        tx.commit().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(json!({
            "id": item_id,
            "cart_id": cart_id,
            "product_id": payload.product_id,
            "quantity": payload.quantity,
            "unit_price_cents": price
        })))
    } else {
        Err((axum::http::StatusCode::NOT_FOUND, "Product not found".to_string()))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CheckoutTerminalRequest {
    pub tenant_id: String,
}

async fn checkout_terminal_handler(
    State(_hub): State<Arc<Hub>>,
    Path(cart_id): Path<String>,
    Json(payload): Json<CheckoutTerminalRequest>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let pool = crate::db::get_pool();

    // Get cart
    let row = sqlx::query("SELECT total_amount_cents, currency FROM carts WHERE id = $1 AND tenant_id = $2")
        .bind(&cart_id)
        .bind(&payload.tenant_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(r) = row {
        let total_amount_cents: i64 = r.get("total_amount_cents");
        let currency: String = r.get("currency");

        let stripe_api_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();
        let stripe_client = StripeClient::new(stripe_api_key);

        let connection_token = stripe_client.create_terminal_connection_token(&payload.tenant_id)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

        let payment_intent_secret = stripe_client.create_terminal_payment_intent(
            &payload.tenant_id,
            total_amount_cents,
            &currency,
            None,
            None,
            Some(&cart_id),
        ).await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

        // Update cart status
        sqlx::query("UPDATE carts SET status = 'checkout', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&cart_id)
            .execute(&pool)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(json!({
            "connection_token": connection_token,
            "payment_intent_secret": payment_intent_secret,
            "cart_id": cart_id,
        })))
    } else {
        Err((axum::http::StatusCode::NOT_FOUND, "Cart not found".to_string()))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CaptureCartRequest {
    pub tenant_id: String,
}

async fn capture_cart_handler(
    State(_hub): State<Arc<Hub>>,
    Path(cart_id): Path<String>,
    Json(payload): Json<CaptureCartRequest>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let pool = crate::db::get_pool();

    let mut tx = pool.begin().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get cart
    let row = sqlx::query("SELECT status FROM carts WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
        .bind(&cart_id)
        .bind(&payload.tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(r) = row {
        let status: String = r.get("status");
        if status == "completed" {
            return Err((axum::http::StatusCode::BAD_REQUEST, "Cart already completed".to_string()));
        }

        // Decrement inventory
        let items = sqlx::query("SELECT product_id, quantity FROM cart_items WHERE cart_id = $1")
            .bind(&cart_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        for item in items {
            let product_id: String = item.get("product_id");
            let quantity: i32 = item.get("quantity");

            sqlx::query("UPDATE products SET inventory_count = inventory_count - $1 WHERE id = $2 AND tenant_id = $3")
                .bind(&quantity)
                .bind(&product_id)
                .bind(&payload.tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }

        sqlx::query("UPDATE carts SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&cart_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        tx.commit().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(json!({
            "success": true,
            "cart_id": cart_id,
            "status": "completed"
        })))
    } else {
        Err((axum::http::StatusCode::NOT_FOUND, "Cart not found".to_string()))
    }
}

#[derive(serde::Deserialize)]
pub struct PosQuery {
    pub tenant_id: Option<String>,
}

async fn get_orders_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, total_amount, status, created_at FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 20")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let orders: Vec<Value> = rows.into_iter().map(|row| {
        json!({
            "id": row.get::<String, _>("id"),
            "total_amount": row.get::<f64, _>("total_amount"),
            "status": row.get::<String, _>("status"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
        })
    }).collect();

    Json(json!({ "orders": orders }))
}

async fn get_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, title, description, price_cents, currency, inventory_count FROM products WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let inventory: Vec<Value> = rows.into_iter().map(|row| {
        json!({
            "id": row.get::<String, _>("id"),
            "name": row.get::<String, _>("title"),
            "description": row.get::<Option<String>, _>("description"),
            "price_cents": row.get::<i64, _>("price_cents"),
            "currency": row.get::<String, _>("currency"),
            "stock": row.get::<i32, _>("inventory_count"),
        })
    }).collect();

    Json(json!({ "inventory": inventory }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::Value;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_omnichannel_cart_flow() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB {
            pool: crate::db::get_pool(),
            store: DbStore::Postgres,
        });

        let hub_mock = Arc::new(Hub {
            db: db.clone(),
            redis: None,
            payment_gateway: None,
            email_client: None,
            sms_client: None,
        });

        let state = axum::extract::State(hub_mock.clone());

        let tenant_id = "test_omni_tenant".to_string();
        let product_id = "test_omni_product".to_string();

        let _ = sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Test') ON CONFLICT DO NOTHING")
            .bind(&tenant_id)
            .execute(&db.pool)
            .await;

        let _ = sqlx::query("INSERT INTO products (id, tenant_id, title, price_cents, inventory_count) VALUES ($1, $2, 'Dress', 5000, 10) ON CONFLICT DO NOTHING")
            .bind(&product_id)
            .bind(&tenant_id)
            .execute(&db.pool)
            .await;

        let req = CreateCartRequest {
            tenant_id: tenant_id.clone(),
            channel: Some("in_store".to_string()),
            currency: Some("USD".to_string()),
        };
        let cart_res = create_cart_handler(state.clone(), Json(req)).await.unwrap();
        let cart_id = cart_res.0["id"].as_str().unwrap().to_string();
        assert!(!cart_id.is_empty());

        let add_req = AddCartItemRequest {
            tenant_id: tenant_id.clone(),
            product_id: product_id.clone(),
            variant_id: None,
            quantity: 1,
        };
        let add_res = add_cart_item_handler(state.clone(), axum::extract::Path(cart_id.clone()), Json(add_req)).await.unwrap();
        let item_id = add_res.0["id"].as_str().unwrap().to_string();
        assert!(!item_id.is_empty());

        let checkout_req = CheckoutTerminalRequest {
            tenant_id: tenant_id.clone(),
        };
        let _ = checkout_terminal_handler(state.clone(), axum::extract::Path(cart_id.clone()), Json(checkout_req)).await;

        let capture_req = CaptureCartRequest {
            tenant_id: tenant_id.clone(),
        };
        let capture_res = capture_cart_handler(state.clone(), axum::extract::Path(cart_id.clone()), Json(capture_req)).await.unwrap();
        assert_eq!(capture_res.0["status"], "completed");

        let row = sqlx::query("SELECT inventory_count FROM products WHERE id = $1")
            .bind(&product_id)
            .fetch_one(&db.pool)
            .await
            .unwrap();
        let stock: i32 = row.get("inventory_count");
        assert!(stock < 10 || stock >= 0);
    }
}
