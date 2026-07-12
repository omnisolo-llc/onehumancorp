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

    let query = sqlx::query(
        "INSERT INTO checkout_sessions (id, tenant_id, type, amount_cents, device_id, cart_payload, status)
         VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&req_data.r#type)
    .bind(final_amount)
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
