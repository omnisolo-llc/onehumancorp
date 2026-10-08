use crate::hub::Hub;
use axum::http::HeaderMap;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"success": false, "error_message": e.to_string()})),
            )
                .into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"success": false, "error_message": e.to_string()})),
        )
            .into_response();
    }

    let mut reserved_locks = Vec::new();
    let mut updated_cart_payload = req_data.cart_payload.clone();

    if let Some(cart) = &mut updated_cart_payload
        && let Some(items) = cart.as_array_mut()
    {
        let inventory_service =
            crate::services::inventory::InventoryService::new(hub.redis_client());
        let ttl = if req_data.r#type == "IN_PERSON" {
            15
        } else {
            300
        };

        for item in items {
            if let (Some(product_obj), Some(quantity)) = (
                item.get("product").cloned(),
                item.get("quantity").and_then(|q| q.as_i64()),
            ) && let Some(product_id) = product_obj.get("id").and_then(|id| id.as_str())
            {
                let reserve_result = inventory_service
                    .reserve_inventory(&tenant_id, product_id, quantity as i32, ttl)
                    .await;
                match reserve_result {
                    Ok(res) if res.success => {
                        reserved_locks.push((
                            product_id.to_string(),
                            res.lock_id.clone(),
                            quantity as i32,
                        ));
                        if let Some(obj) = item.as_object_mut() {
                            obj.insert(
                                "lock_id".to_string(),
                                serde_json::Value::String(res.lock_id),
                            );
                        }
                    }
                    _ => {
                        // Rollback reserved locks if we failed midway
                        for (pid, lid, qty) in reserved_locks {
                            let _ = inventory_service
                                .release_inventory(&tenant_id, &pid, qty, &lid)
                                .await;
                        }
                        let _ = db_tx.rollback().await;
                        return (
                            StatusCode::CONFLICT,
                            Json(CreateCheckoutSessionResponse {
                                session_id: "".to_string(),
                                success: false,
                                error_message: Some(
                                    "Item is currently being checked out by another customer."
                                        .to_string(),
                                ),
                            }),
                        )
                            .into_response();
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
    if let Some(cart) = &req_data.cart_payload
        && let Some(items) = cart.get("items").and_then(|i| i.as_array())
    {
        let mut checkout_locks: Vec<(String, String, i32)> = Vec::new();
        let service = crate::services::inventory::InventoryService::new(hub.redis_client());
        for item in items {
            if let Some(product_id) = item.get("product_id").and_then(|p| p.as_str()) {
                let quantity = item.get("quantity").and_then(|q| q.as_i64()).unwrap_or(1) as i32;
                // Redlock 5 minutes for online checkout cart
                match service
                    .reserve_inventory(&tenant_id, product_id, quantity, 300)
                    .await
                {
                    Ok(res) if !res.success => {
                        for (pid, lid, qty) in checkout_locks {
                            let _ = service.release_inventory(&tenant_id, &pid, qty, &lid).await;
                        }
                        let _ = db_tx.rollback().await;
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(CreateCheckoutSessionResponse {
                                session_id: "".to_string(),
                                success: false,
                                error_message: Some(res.error_message),
                            }),
                        )
                            .into_response();
                    }
                    Err(e) => {
                        for (pid, lid, qty) in checkout_locks {
                            let _ = service.release_inventory(&tenant_id, &pid, qty, &lid).await;
                        }
                        let _ = db_tx.rollback().await;
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(CreateCheckoutSessionResponse {
                                session_id: "".to_string(),
                                success: false,
                                error_message: Some(e),
                            }),
                        )
                            .into_response();
                    }
                    Ok(res) => {
                        if res.success {
                            checkout_locks.push((product_id.to_string(), res.lock_id.clone(), quantity));
                        }
                    }
                }
            }
        }
        if !checkout_locks.is_empty() {
            if updated_cart_payload.is_none() {
                updated_cart_payload = Some(serde_json::json!({}));
            }
            if let Some(cart_obj) = updated_cart_payload.as_mut().and_then(|c| c.as_object_mut()) {
                let lock_ids: Vec<String> = checkout_locks.into_iter().map(|(_, lid, _)| lid).collect();
                if lock_ids.len() == 1 {
                    cart_obj.insert(
                        "inventory_lock_id".to_string(),
                        serde_json::Value::String(lock_ids[0].clone()),
                    );
                } else if lock_ids.len() > 1 {
                    cart_obj.insert(
                        "inventory_lock_ids".to_string(),
                        serde_json::json!(lock_ids),
                    );
                }
            }
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
            (
                StatusCode::OK,
                Json(CreateCheckoutSessionResponse {
                    session_id,
                    success: true,
                    error_message: None,
                }),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CreateCheckoutSessionResponse {
                session_id: "".to_string(),
                success: false,
                error_message: Some(e.to_string()),
            }),
        )
            .into_response(),
    }
}

pub fn router(
    hub: Arc<Hub>,
) -> axum::Router<Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route(
            "/session",
            axum::routing::post(create_checkout_session_handler),
        )
        .with_state(hub)
}
