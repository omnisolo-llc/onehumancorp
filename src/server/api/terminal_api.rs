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
        .route("/inventory_lock", axum::routing::post(acquire_inventory_lock_handler))
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
    match client.create_terminal_connection_token(&tenant_id).await {
        Ok(token) => Json(Ok(TerminalTokenResponse { token })),
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
                 VALUES ($1, $2, 'pos_offline_sync', $3::jsonb)"
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
    match client.create_terminal_payment_intent(&tenant_id, req_data.amount_cents, &req_data.currency).await {
        Ok(client_secret) => Json(Ok(PaymentIntentResponse { client_secret })),
        Err(e) => Json(Err(e)),
    }
}

#[derive(serde::Deserialize)]
pub struct InventoryLockRequest {
    pub product_id: String,
    pub session_id: String,
}

#[derive(serde::Serialize)]
pub struct InventoryLockResponse {
    pub success: bool,
    pub lock_id: Option<String>,
    pub error: Option<String>,
}

pub async fn acquire_inventory_lock_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<InventoryLockRequest>,
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

    info!(tenant_id = %tenant_id, product_id = %req_data.product_id, "Acquiring POS offline inventory lock");

    // Initialize the soft lock store
    // Use the global redis instance (if available in this context, otherwise default None/local for standalone)
    // To properly access redis, we'll try to use the shared redis client pattern from the repo.
    // In booking.rs, it uses `BookingSoftLockStore::for_service(redis_client)`.

    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| std::env::var("REDIS_URL").unwrap_or_default());
    let redis_client = if !redis_url.is_empty() {
        redis::Client::open(redis_url).ok()
    } else {
        None
    };

    let locks = crate::services::booking::BookingSoftLockStore::for_service(redis_client);

    // Retrieve product capacity
    let inventory_capacity = match crate::services::booking::NativeBookingService::product_inventory_capacity(&tenant_id, &req_data.product_id).await {
        Ok(c) => c,
        Err(_) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(InventoryLockResponse { success: false, lock_id: None, error: Some("Product not found or has no capacity".to_string()) })
            ).into_response();
        }
    };

    // Acquire lock (15s TTL)
    match locks.acquire_inventory_lock(
        &tenant_id,
        &req_data.product_id,
        &req_data.session_id,
        inventory_capacity,
        std::time::Duration::from_secs(15)
    ).await {
        Ok(Some(receipt)) => {
            (axum::http::StatusCode::OK, Json(InventoryLockResponse { success: true, lock_id: Some(receipt.key), error: None })).into_response()
        }
        Ok(None) => {
            (axum::http::StatusCode::CONFLICT, Json(InventoryLockResponse { success: false, lock_id: None, error: Some("Product inventory is currently fully held".to_string()) })).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to acquire inventory lock: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(InventoryLockResponse { success: false, lock_id: None, error: Some(e) })).into_response()
        }
    }
}
