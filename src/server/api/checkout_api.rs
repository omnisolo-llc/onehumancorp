use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;

#[derive(Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub tenant_id: String,
    pub r#type: String,
    pub amount_cents: i64,
    pub device_id: Option<String>,
    pub cart_payload: Option<serde_json::Value>,
    pub discount_code: Option<String>,
    pub target_currency: Option<String>,
}

#[derive(Serialize)]
pub struct CreateCheckoutSessionResponse {
    pub session_id: String,
    pub success: bool,
    pub error_message: Option<String>,
}

pub async fn create_checkout_session_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    req_data: axum::extract::Json<CreateCheckoutSessionRequest>,
) -> axum::response::Response {
    let session_id = uuid::Uuid::new_v4().to_string();
    let tenant_id = req_data.tenant_id.clone();

    let mut db_tx = match hub.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error_message": e.to_string()}))).into_response()
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error_message": e.to_string()}))).into_response()
    }


    let mut reserved_locks = Vec::new();
    let mut updated_cart_payload = req_data.cart_payload.clone();

    if let Some(cart) = &mut updated_cart_payload {
        if let Some(items) = cart.as_array_mut() {
            let inventory_service = crate::services::inventory::InventoryService::new(hub.redis_client.clone());
            let ttl = if req_data.r#type == "IN_PERSON" { 15 } else { 300 };

            for item in items {
                if let (Some(product_obj), Some(quantity)) = (item.get("product").cloned(), item.get("quantity").and_then(|q| q.as_i64())) {
                    if let Some(product_id) = product_obj.get("id").and_then(|id| id.as_str()) {
                        let reserve_result = inventory_service.reserve_inventory(&tenant_id, product_id, quantity as i32, ttl).await;
                        match reserve_result {
                            Ok(res) if res.success => {
                                reserved_locks.push((product_id.to_string(), res.lock_id.clone(), quantity as i32));
                                if let Some(obj) = item.as_object_mut() {
                                    obj.insert("lock_id".to_string(), serde_json::Value::String(res.lock_id));
                                }
                            }
                            _ => {
                                // Rollback reserved locks if we failed midway
                                for (pid, lid, qty) in reserved_locks {
                                    let _ = inventory_service.release_inventory(&tenant_id, &pid, qty, &lid).await;
                                }
                                let _ = db_tx.rollback().await;
                                return (StatusCode::CONFLICT, Json(CreateCheckoutSessionResponse {
                                    session_id: "".to_string(),
                                    success: false,
                                    error_message: Some("Item is currently being checked out by another customer.".to_string()),
                                })).into_response();
                            }
                        }
                    }
                }
            }
        }
    }

    let mut final_amount = req_data.amount_cents;
    if let Some(discount_code) = &req_data.discount_code {
        let is_valid: Result<bool, sqlx::Error> = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM reward_claims WHERE tenant_id = $1 AND discount_code = $2 AND status = 'Active')"
        )
        .bind(&tenant_id)
        .bind(discount_code)
        .fetch_one(&mut *db_tx)
        .await;

        if let Ok(true) = is_valid {
            final_amount = (final_amount as f64 * 0.85) as i64; // 15% discount
            let _ = sqlx::query("UPDATE reward_claims SET status = 'Used' WHERE tenant_id = $1 AND discount_code = $2")
                .bind(&tenant_id)
                .bind(discount_code)
                .execute(&mut *db_tx)
                .await;
        }
    }

    // Redlock inventory reservation in checkout flow
    if let Some(cart) = &req_data.cart_payload {
        if let Some(items) = cart.get("items").and_then(|i| i.as_array()) {
            let service = crate::services::inventory::InventoryService::new(hub.redis_client.clone());
            for item in items {
                if let Some(product_id) = item.get("product_id").and_then(|p| p.as_str()) {
                    let quantity = item.get("quantity").and_then(|q| q.as_i64()).unwrap_or(1) as i32;
                    // Redlock 5 minutes for online checkout cart
                    match service.reserve_inventory(&tenant_id, product_id, quantity, 300).await {
                        Ok(res) if !res.success => {
                            let _ = db_tx.rollback().await;
                            return (StatusCode::BAD_REQUEST, Json(CreateCheckoutSessionResponse {
                                session_id: "".to_string(),
                                success: false,
                                error_message: Some(res.error_message),
                            })).into_response()
                        },
                        Err(e) => {
                            let _ = db_tx.rollback().await;
                            return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateCheckoutSessionResponse {
                                session_id: "".to_string(),
                                success: false,
                                error_message: Some(e),
                            })).into_response()
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    let mut rate_multiplier = 1.0;
    let mut applied_currency = "USD".to_string();

    if let Some(target_curr) = &req_data.target_currency {
        let base_currency = "USD";
        let fx_service = crate::services::localization::fx_cache::FxCacheService::new(hub.redis_client.clone(), hub.pool.clone());
        if let Ok(rate) = fx_service.get_rate(base_currency, target_curr).await {
            rate_multiplier = rate;
            applied_currency = target_curr.clone();
        }
    }

    let final_settlement_amount = (final_amount as f64 * rate_multiplier).round() as i64;

    let query = sqlx::query(
        "INSERT INTO checkout_sessions (id, tenant_id, type, amount_cents, settlement_amount_cents, settlement_currency, exchange_rate, device_id, cart_payload, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'PENDING')"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&req_data.r#type)
    .bind(final_amount)
    .bind(final_settlement_amount)
    .bind(&applied_currency)
    .bind(rate_multiplier)
    .bind(&req_data.device_id)
    .bind(&updated_cart_payload);

    match query.execute(&mut *db_tx).await {
        Ok(_) => {
            let _ = db_tx.commit().await;
            (StatusCode::OK, Json(CreateCheckoutSessionResponse {
                session_id,
                success: true,
                error_message: None,
            })).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateCheckoutSessionResponse {
                session_id: "".to_string(),
                success: false,
                error_message: Some(e.to_string()),
            })).into_response()
        }
    }
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new().route("/session", axum::routing::post(create_checkout_session_handler)).with_state(hub)
}
