use axum::{extract::State, Json, response::IntoResponse};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;
use tracing::info;

#[derive(serde::Serialize)]
pub struct TerminalTokenResponse {
    pub token: String,
}

#[derive(serde::Deserialize)]
pub struct PaymentIntentRequest {
    pub amount_cents: i64,
    pub currency: String,
}

#[derive(serde::Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: String,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/token", axum::routing::post(get_terminal_connection_token_handler))
        .route("/intent", axum::routing::post(create_payment_intent_handler))
        .route("/sync_offline", axum::routing::post(sync_offline_transactions_handler))
        .route("/reserve", axum::routing::post(reserve_inventory_handler))
        .route("/commit", axum::routing::post(commit_inventory_handler))
        .with_state(hub)
}




#[derive(serde::Deserialize)]
pub struct ReserveInventoryRequest {
    pub tenant_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub ttl_seconds: i32,
}

#[derive(serde::Deserialize)]
pub struct CommitInventoryRequest {
    pub tenant_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub lock_id: String,
}

pub async fn reserve_inventory_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<ReserveInventoryRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(info) => info.org_id.clone(),
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthenticated" }))).into_response()
    };

    let lock_id = uuid::Uuid::new_v4().to_string();
    let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, req_data.product_id);

    if let Some(client) = &hub.redis_client {
        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
            let ttl = if req_data.ttl_seconds > 0 { req_data.ttl_seconds } else { 300 };
            let acquired: bool = redis::cmd("SET")
                .arg(&lock_key).arg(&lock_id).arg("EX").arg(ttl).arg("NX")
                .query_async(&mut conn).await.unwrap_or(false);

            if !acquired {
                return (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": false,
                    "lock_id": "",
                    "error_message": "Item is currently being checked out by another customer"
                }))).into_response();
            }

            // Verify capacity AFTER acquiring the lock within a transaction
            let pool = crate::db::get_pool();
            if let Ok(mut tx) = pool.begin().await {
                if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                    let current_stock: Option<i32> = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                        .bind(&req_data.product_id)
                        .bind(&tenant_id)
                        .fetch_optional(&mut *tx)
                        .await
                        .unwrap_or(None);

                    if let Some(stock) = current_stock {
                        if stock < req_data.quantity {
                            let _ = tx.rollback().await;
                            let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                            return (axum::http::StatusCode::OK, Json(serde_json::json!({
                                "success": false,
                                "lock_id": "",
                                "error_message": format!("Insufficient inventory. Available: {}", stock)
                            }))).into_response();
                        }
                    } else {
                        let _ = tx.rollback().await;
                        let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
                        return (axum::http::StatusCode::OK, Json(serde_json::json!({
                            "success": false,
                            "lock_id": "",
                            "error_message": "Product not found"
                        }))).into_response();
                    }
                }
                let _ = tx.commit().await;
            }
        }
    } else {
        // Fallback if no redis
        let pool = crate::db::get_pool();
        if let Ok(mut tx) = pool.begin().await {
            if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                let current_stock: Option<i32> = sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                    .bind(&req_data.product_id)
                    .bind(&tenant_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .unwrap_or(None);

                if let Some(stock) = current_stock {
                    if stock < req_data.quantity {
                        let _ = tx.rollback().await;
                        return (axum::http::StatusCode::OK, Json(serde_json::json!({
                            "success": false,
                            "lock_id": "",
                            "error_message": format!("Insufficient inventory. Available: {}", stock)
                        }))).into_response();
                    }
                } else {
                    let _ = tx.rollback().await;
                    return (axum::http::StatusCode::OK, Json(serde_json::json!({
                        "success": false,
                        "lock_id": "",
                        "error_message": "Product not found"
                    }))).into_response();
                }
            }
            let _ = tx.commit().await;
        }
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "lock_id": lock_id,
        "error_message": ""
    }))).into_response()
}

pub async fn commit_inventory_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<CommitInventoryRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(info) => info.org_id.clone(),
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthenticated" }))).into_response()
    };

    let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, req_data.product_id);

    if let Some(client) = &hub.redis_client {
        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
            let current_lock_id: Option<String> = redis::cmd("GET").arg(&lock_key).query_async(&mut conn).await.unwrap_or(None);
            if let Some(cid) = current_lock_id {
                if cid != req_data.lock_id && !req_data.lock_id.is_empty() {
                    return (axum::http::StatusCode::OK, Json(serde_json::json!({
                        "success": false,
                        "error_message": "Lock ID mismatch. Reservation may have expired."
                    }))).into_response();
                }
            }
            let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut conn).await.unwrap_or(());
        }
    }

    let pool = crate::db::get_pool();
    if let Ok(mut tx) = pool.begin().await {
        if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
            let current_stock = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                .bind(&req_data.product_id).bind(&tenant_id).fetch_optional(&mut *tx).await.unwrap_or(None);

            if let Some(row) = current_stock {
                let stock: i32 = sqlx::Row::get(&row, "inventory_count");
                if stock < req_data.quantity {
                    let _ = tx.rollback().await;
                    return (axum::http::StatusCode::OK, Json(serde_json::json!({
                        "success": false,
                        "error_message": format!("Insufficient inventory. Available: {}", stock)
                    }))).into_response();
                }

                let new_stock = stock - req_data.quantity;
                let _ = sqlx::query("UPDATE products SET inventory_count = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(new_stock).bind(&req_data.product_id).bind(&tenant_id).execute(&mut *tx).await;

                if new_stock <= 5 {
                    let job_id = uuid::Uuid::new_v4().to_string();
                    let job_payload = serde_json::json!({
                        "product_id": req_data.product_id,
                        "remaining_stock": new_stock,
                        "threshold": 5,
                        "message": format!("Stock for product {} has dropped to {}.", req_data.product_id, new_stock)
                    }).to_string();

                    let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'LowStockAlert', $3::jsonb, 'PENDING')")
                        .bind(job_id).bind(&tenant_id).bind(&job_payload).execute(&mut *tx).await;

                    let approval_id = uuid::Uuid::new_v4().to_string();
                    let approval_payload = serde_json::json!({
                        "feature_type": "low_stock_restock",
                        "product_id": req_data.product_id,
                        "remaining_stock": new_stock,
                        "suggested_action": "Restock Item"
                    }).to_string();
                    let description = format!("Inventory for {} is low ({} remaining). Draft a restock order.", req_data.product_id, new_stock);
                    let _ = sqlx::query("INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, 'operations', $3, 'DRAFT', 'LOW', $4::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                        .bind(&approval_id).bind(&tenant_id).bind(&description).bind(&approval_payload).execute(&mut *tx).await;
                }

                let _ = tx.commit().await;
                return (axum::http::StatusCode::OK, Json(serde_json::json!({
                    "success": true,
                    "error_message": ""
                }))).into_response();
            }
        }
        let _ = tx.rollback().await;
    }

    (axum::http::StatusCode::OK, Json(serde_json::json!({
        "success": false,
        "error_message": "Database error"
    }))).into_response()
}
pub async fn get_terminal_connection_token_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> Json<Result<TerminalTokenResponse, String>> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(Err("Unauthenticated: Missing tenant ID".to_string()));
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(Err("Unauthenticated".to_string()))
    };

    info!(tenant_id = %tenant_id, "Generating Stripe Terminal Connection Token");

    let _ = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        &tenant_id,
        "stripe_terminal_connection_token",
        0.05
    ).await;

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();

    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    match client.require_api_key() {
        Ok(_) => match client.create_terminal_connection_token(&tenant_id).await {
            Ok(token) => Json(Ok(TerminalTokenResponse { token })),
            Err(e) => Json(Err(e)),
        },
        Err(e) => Json(Err(e)),
    }
}

#[derive(serde::Deserialize)]
pub struct PosOfflineTransaction {
    pub client_id: Option<String>,
    pub amount_cents: i64,
    pub currency: String,
    pub payload: String,
    pub timestamp: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SyncOfflineTransactionsRequest {
    pub transactions: Vec<PosOfflineTransaction>,
}

#[derive(serde::Serialize)]
pub struct SyncOfflineTransactionsResponse {
    pub success: bool,
    pub synced_count: i32,
    pub failed_transaction_ids: Vec<String>,
}

pub async fn sync_offline_transactions_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<SyncOfflineTransactionsRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({ "error": "Unauthenticated: Missing tenant ID" })),
                )
                    .into_response();
            } else {
                auth.org_id.clone()
            }
        }
        None => {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Unauthenticated" })),
            )
                .into_response();
        }
    };

    info!(tenant_id = %tenant_id, tx_count = req_data.transactions.len(), "Syncing offline POS transactions");

    let pool = crate::db::get_pool();
    let mut synced_count = 0;
    let mut failed_ids = Vec::new();
    let mut futures = Vec::new();

    for tx in &req_data.transactions {
        let pool_clone = pool.clone();
        let tenant_id_clone = tenant_id.clone();
        let client_id_clone = tx.client_id.clone().unwrap_or_default();
        let tx_id = uuid::Uuid::new_v4().to_string();

        let amount_cents = tx.amount_cents;
        let currency = tx.currency.clone();
        let payload_str = tx.payload.clone();

        futures.push(tokio::spawn(async move {
            let mut db_tx = match pool_clone.begin().await {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {}", e);
                    return Err(tx_id);
                }
            };

            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id_clone).await {
                tracing::error!("Failed to set org context: {}", e);
                return Err(tx_id);
            }

            let insert_res = sqlx::query(
                "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status)
                 VALUES ($1, $2, $3, $4, $5, $6::jsonb, 'PENDING')"
            )
            .bind(&tx_id)
            .bind(&tenant_id_clone)
            .bind(&client_id_clone)
            .bind(amount_cents)
            .bind(&currency)
            .bind(&payload_str)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = insert_res {
                tracing::error!("Failed to insert offline transaction: {}", e);
                return Err(tx_id);
            }

            let job_id = uuid::Uuid::new_v4().to_string();
            let job_payload = serde_json::json!({
                "pos_transaction_id": tx_id,
                "client_id": client_id_clone,
                "amount_cents": amount_cents,
                "currency": currency,
                "payload": payload_str,
            }).to_string();

            let job_res = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                 VALUES ($1, $2, 'offline_pos_sync', $3::jsonb)"
            )
            .bind(&job_id)
            .bind(&tenant_id_clone)
            .bind(&job_payload)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = job_res {
                tracing::error!("Failed to enqueue job: {}", e);
                return Err(tx_id);
            }

            if let Err(e) = db_tx.commit().await {
                tracing::error!("Failed to commit transaction: {}", e);
                return Err(tx_id);
            }

            Ok(())
        }));
    }

    let results = futures::future::join_all(futures).await;

    for res in results {
        match res {
            Ok(Ok(())) => {
                synced_count += 1;
            }
            Ok(Err(id)) => {
                failed_ids.push(id);
            }
            Err(e) => {
                tracing::error!("Task failed to execute: {}", e);
            }
        }
    }

    let res = SyncOfflineTransactionsResponse {
        success: failed_ids.is_empty(),
        synced_count,
        failed_transaction_ids: failed_ids,
    };

    (axum::http::StatusCode::OK, Json(res)).into_response()
}



pub async fn create_payment_intent_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<PaymentIntentRequest>,
) -> Json<Result<PaymentIntentResponse, String>> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(Err("Unauthenticated: Missing tenant ID".to_string()));
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(Err("Unauthenticated".to_string()))
    };

    info!(tenant_id = %tenant_id, amount = req_data.amount_cents, currency = %req_data.currency, "Creating Stripe Terminal Payment Intent");

    let _ = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        &tenant_id,
        "stripe_terminal_payment_intent",
        0.05
    ).await;

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();

    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    match client.require_api_key() {
        Ok(_) => match client.create_terminal_payment_intent(&tenant_id, req_data.amount_cents, &req_data.currency).await {
            Ok(client_secret) => Json(Ok(PaymentIntentResponse { client_secret })),
            Err(e) => Json(Err(e)),
        },
        Err(e) => Json(Err(e)),
    }
}
