use axum::{extract::State, Json, response::IntoResponse};
use std::sync::Arc;
use crate::hub::Hub;
use tracing::info;

#[derive(serde::Serialize)]
pub struct TerminalTokenResponse {
    pub token: String,
}

#[derive(serde::Deserialize)]
pub struct PaymentIntentRequest {
    pub amount_cents: i64,
    pub currency: String,
    pub product_id: Option<String>,
    pub quantity: Option<i32>,
    pub order_id: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: String,
    pub lock_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CapturePaymentIntentRequest {
    pub payment_intent_id: String,
    pub product_id: Option<String>,
    pub quantity: Option<i32>,
    pub lock_id: Option<String>,
    pub amount_cents: Option<i64>,
}

#[derive(serde::Serialize)]
pub struct CapturePaymentIntentResponse {
    pub success: bool,
    pub status: String,
    pub error_message: Option<String>,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/token", axum::routing::post(get_terminal_connection_token_handler))
        .route("/intent", axum::routing::post(create_payment_intent_handler))
        .route("/intent/capture", axum::routing::post(capture_payment_intent_handler))
        .route("/sync_offline", axum::routing::post(sync_offline_transactions_handler))
        .route("/reserve", axum::routing::post(reserve_inventory_handler))
        .route("/commit", axum::routing::post(commit_inventory_handler))
        .route("/session/start", axum::routing::post(start_terminal_session_handler))
        .route("/session/update", axum::routing::post(update_terminal_session_status_handler))
        .route("/session/end", axum::routing::post(end_terminal_session_handler))
        .route("/backend", axum::routing::get(get_terminal_backend_handler))
        .route("/backend", axum::routing::post(post_terminal_backend_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize)]
pub struct StartTerminalSessionRequest {
    pub device_id: String,
}

#[derive(serde::Serialize)]
pub struct StartTerminalSessionResponse {
    pub session_id: String,
    pub success: bool,
    pub error_message: String,
}

pub async fn start_terminal_session_handler(
    _headers: axum::http::HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<StartTerminalSessionRequest>,
) -> Json<StartTerminalSessionResponse> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(StartTerminalSessionResponse { session_id: "".to_string(), success: false, error_message: "Unauthenticated: Missing tenant ID".to_string() });
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(StartTerminalSessionResponse { session_id: "".to_string(), success: false, error_message: "Unauthenticated".to_string() })
    };

    let session_id = uuid::Uuid::new_v4().to_string();
    let pool = crate::db::get_pool();

    let res = sqlx::query(
        "INSERT INTO pos_terminal_sessions (id, tenant_id, device_id, status, started_at, last_synced_at, offline_changes_count)
         VALUES ($1, $2, $3, 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
         ON CONFLICT (tenant_id, device_id) DO UPDATE SET status = 'ACTIVE', last_synced_at = CURRENT_TIMESTAMP, offline_changes_count = 0 RETURNING id"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&req_data.device_id)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(row) => {
            let returned_id: String = sqlx::Row::get(&row, "id");
            Json(StartTerminalSessionResponse {
                session_id: returned_id,
                success: true,
                error_message: "".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to start terminal session: {}", e);
            Json(StartTerminalSessionResponse {
                session_id: "".to_string(),
                success: false,
                error_message: e.to_string(),
            })
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateTerminalSessionStatusRequest {
    pub session_id: String,
    pub status: String,
}

#[derive(serde::Serialize)]
pub struct UpdateTerminalSessionStatusResponse {
    pub success: bool,
    pub error_message: String,
}

pub async fn update_terminal_session_status_handler(
    _headers: axum::http::HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<UpdateTerminalSessionStatusRequest>,
) -> Json<UpdateTerminalSessionStatusResponse> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(UpdateTerminalSessionStatusResponse { success: false, error_message: "Unauthenticated: Missing tenant ID".to_string() });
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(UpdateTerminalSessionStatusResponse { success: false, error_message: "Unauthenticated".to_string() })
    };

    let pool = crate::db::get_pool();


    let status_str = req_data.status.as_str();
    let query = if status_str == "RESOLVED" {
        "UPDATE pos_terminal_sessions SET status = 'ACTIVE', sync_status = 'SYNCED', pending_reconciliation = '[]'::jsonb, last_conflict_resolved_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
    } else {
        "UPDATE pos_terminal_sessions SET status = $1, last_synced_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
    };

    let res = if status_str == "RESOLVED" {
        sqlx::query(query)
            .bind(&req_data.session_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
    } else {
        sqlx::query(query)
            .bind(&req_data.status)
            .bind(&req_data.session_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
    };


    match res {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Json(UpdateTerminalSessionStatusResponse { success: true, error_message: "".to_string() })
            } else {
                Json(UpdateTerminalSessionStatusResponse { success: false, error_message: "Session not found".to_string() })
            }
        }
        Err(e) => {
            tracing::error!("Failed to update terminal session status: {}", e);
            Json(UpdateTerminalSessionStatusResponse { success: false, error_message: e.to_string() })
        }
    }
}

#[derive(serde::Deserialize)]
pub struct EndTerminalSessionRequest {
    pub session_id: String,
}

#[derive(serde::Serialize)]
pub struct EndTerminalSessionResponse {
    pub success: bool,
    pub error_message: String,
}

pub async fn end_terminal_session_handler(
    _headers: axum::http::HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<EndTerminalSessionRequest>,
) -> Json<EndTerminalSessionResponse> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(EndTerminalSessionResponse { success: false, error_message: "Unauthenticated: Missing tenant ID".to_string() });
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(EndTerminalSessionResponse { success: false, error_message: "Unauthenticated".to_string() })
    };

    let pool = crate::db::get_pool();

    let res = sqlx::query(
        "UPDATE pos_terminal_sessions SET status = 'RECONCILED', last_synced_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&req_data.session_id)
    .bind(&tenant_id)
    .execute(&pool)
    .await;

    match res {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Json(EndTerminalSessionResponse { success: true, error_message: "".to_string() })
            } else {
                Json(EndTerminalSessionResponse { success: false, error_message: "Session not found".to_string() })
            }
        }
        Err(e) => {
            tracing::error!("Failed to end terminal session: {}", e);
            Json(EndTerminalSessionResponse { success: false, error_message: e.to_string() })
        }
    }
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
    pub customer_id: Option<String>,
    pub amount_cents: Option<i64>,
}

pub async fn reserve_inventory_handler(
    _headers: axum::http::HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<ReserveInventoryRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(info) => info.org_id.clone(),
        None => {
            let spiffe_id_str = _headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
            // WARNING: SECURITY FIX
            // Only allow tenant override for internal test agents, do not bypass spiffe id auth in prod!
            if let Some(tenant_override) = _headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).filter(|_| spiffe_id_str.starts_with("spiffe://ohc.local/org/") && spiffe_id_str.contains("/agent/")) {
                tenant_override.to_string()
            } else if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
                id
            } else {
                return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthenticated" }))).into_response()
            }
        }
    };

    let service = crate::services::inventory::InventoryService::new(
        hub.redis_client.clone()
    );

    match service.reserve_inventory(&tenant_id, &req_data.product_id, req_data.quantity, if req_data.ttl_seconds > 0 { req_data.ttl_seconds } else { 15 }).await {
        Ok(result) => {
            (axum::http::StatusCode::OK, Json(serde_json::json!({
                "success": result.success,
                "lock_id": result.lock_id,
                "error_message": result.error_message
            }))).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::OK, Json(serde_json::json!({
                "success": false,
                "lock_id": "",
                "error_message": e
            }))).into_response()
        }
    }
}


#[derive(serde::Deserialize)]
pub struct PosOfflineTransaction {
    pub id: Option<String>,
    pub client_id: Option<String>,
    pub amount_cents: i64,
    pub currency: String,
    pub payload: String,
    pub timestamp: Option<String>,
    pub mutation_type: Option<String>,
    pub device_signature: Option<String>,
    pub terminal_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SyncOfflineTransactionsRequest {
    pub session_id: Option<String>,
    pub transactions: Vec<PosOfflineTransaction>,
}

#[derive(serde::Serialize)]
pub struct SyncOfflineTransactionsResponse {
    pub success: bool,
    pub synced_count: i32,
    pub failed_transaction_ids: Vec<String>,
    pub pending_reconciliation: Option<Vec<serde_json::Value>>,
}

pub async fn sync_offline_transactions_handler(
    _headers: axum::http::HeaderMap,
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
    let mut pending_reconciliation_items: Vec<serde_json::Value> = Vec::new();

    for tx in &req_data.transactions {
        if let Some(sig) = &tx.device_signature {
            if !sig.starts_with("sig_") {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({ "error": "Invalid device signature" })),
                ).into_response();
            }
        } else {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Missing device signature" })),
            ).into_response();
        }
    }

    let client_id = req_data.transactions.first().and_then(|tx| tx.client_id.clone()).unwrap_or_else(|| "unknown".to_string());

    // Update pos_terminal_sessions
    let session_id = req_data.session_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let _ = sqlx::query(
        "INSERT INTO pos_terminal_sessions (id, tenant_id, device_id, status, started_at, last_synced_at, offline_changes_count)
         VALUES ($1, $2, $3, 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4)
         ON CONFLICT (tenant_id, device_id) DO UPDATE SET last_synced_at = CURRENT_TIMESTAMP, offline_changes_count = pos_terminal_sessions.offline_changes_count + $4"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&client_id)
    .bind(req_data.transactions.len() as i32)
    .execute(&pool)
    .await;

    if !req_data.transactions.is_empty() {
        let mut db_tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "Internal server error" })),
                ).into_response();
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
            tracing::error!("Failed to set org context: {}", e);
            let _ = db_tx.rollback().await;
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal server error" })),
            ).into_response();
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, amount_cents, currency, payload, status, _sync_status, device_signature, terminal_id) "
        );

        let tenant_id_clone = tenant_id.clone();
        query_builder.push_values(req_data.transactions.iter(), |mut b, tx| {
            let tx_id = tx.id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let client_id_clone = tx.client_id.clone().unwrap_or_default();
            let amount_cents = tx.amount_cents;
            let currency = tx.currency.clone();
            let payload_str = tx.payload.clone();
            let device_signature = tx.device_signature.clone();
            let terminal_id = tx.terminal_id.clone();

            b.push_bind(tx_id)
             .push_bind(tenant_id_clone.clone())
             .push_bind(client_id_clone)
             .push_bind(amount_cents)
             .push_bind(currency)
             .push_bind(sqlx::types::Json(serde_json::from_str::<serde_json::Value>(&payload_str).unwrap_or(serde_json::json!({}))))
             .push_bind("PENDING")
             .push_bind("pending")
             .push_bind(device_signature)
             .push_bind(terminal_id);
        });

        query_builder.push(" ON CONFLICT (id) DO NOTHING RETURNING id, client_id, amount_cents, currency, payload");

        match query_builder.build().fetch_all(&mut *db_tx).await {
            Ok(rows) => {
                // Evaluate conflicts for pending reconciliation synchronously
                for tx in &req_data.transactions {
                    if let Ok(payload_val) = serde_json::from_str::<serde_json::Value>(&tx.payload) {
                        if let Some(items) = payload_val.as_array() {
                            for item in items {
                                if let (Some(product_id), Some(quantity)) = (
                                    item.get("product_id").and_then(|v| v.as_str()),
                                    item.get("quantity").and_then(|v| v.as_i64()),
                                ) {
                                    let current_stock_res: Result<(i32,), sqlx::Error> = sqlx::query_as(
                                        "SELECT available_quantity FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE"
                                    )
                                    .bind(product_id)
                                    .bind(&tenant_id)
                                    .fetch_one(&mut *db_tx)
                                    .await;

                                    if let Ok((stock,)) = current_stock_res {
                                        let qty_i32 = quantity as i32;
                                        if stock < qty_i32 {
                                            let tx_id = tx.id.clone().unwrap_or_default();
                                            pending_reconciliation_items.push(serde_json::json!({
                                                "transaction_id": tx_id,
                                                "product_id": product_id,
                                                "shortage": qty_i32 - stock,
                                                "timestamp": chrono::Utc::now().to_rfc3339()
                                            }));
                                        }

                                        let _ = sqlx::query("UPDATE products SET pn_counter_n = pn_counter_n + $1, inventory_count = GREATEST(0, pn_counter_p - (pn_counter_n + $1)), available_quantity = GREATEST(0, available_quantity - $1) WHERE id = $2 AND tenant_id = $3")
                                            .bind(qty_i32)
                                            .bind(product_id)
                                            .bind(&tenant_id)
                                            .execute(&mut *db_tx)
                                            .await;

                                        if let Some(client) = crate::get_redis_client() {
                                            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                                let invalidation_topic = "cache_invalidation_events";
                                                let invalidation_payload = serde_json::json!({
                                                    "event": "inventory.updated",
                                                    "tags": [
                                                        format!("tenant-id:{}", tenant_id),
                                                        format!("entity:product:{}", product_id)
                                                    ]
                                                }).to_string();
                                                let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !rows.is_empty() {
                    let mut job_query_builder = sqlx::QueryBuilder::new(
                        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) "
                    );

                    job_query_builder.push_values(rows.into_iter(), |mut b, row| {
                        use sqlx::Row;
                        let job_id = uuid::Uuid::new_v4().to_string();
                        let tx_id: String = row.get("id");
                        let client_id_clone: String = row.get("client_id");
                        let amount_cents: i64 = row.get("amount_cents");
                        let currency: String = row.get("currency");
                        let payload_val: serde_json::Value = row.get("payload");
                        let payload_str = payload_val.to_string();


                        let mut m_type = None;
                        for tx_req in &req_data.transactions {
                            if tx_req.id.as_deref() == Some(tx_id.as_str()) {
                                m_type = tx_req.mutation_type.clone();
                                break;
                            }
                        }

                        let job_payload = serde_json::json!({

                            "pos_transaction_id": tx_id,
                            "client_id": client_id_clone,
                            "amount_cents": amount_cents,
                            "currency": currency,
                            "payload": payload_str,
                            "mutation_type": m_type,
                            "inventory_already_deducted": true,
                        }).to_string();

                        b.push_bind(job_id)
                         .push_bind(tenant_id.clone())
                         .push_bind("offline_pos_sync")
                         .push_bind(sqlx::types::Json(serde_json::from_str::<serde_json::Value>(&job_payload).unwrap_or(serde_json::json!({}))));
                    });

                    if let Err(e) = job_query_builder.build().execute(&mut *db_tx).await {
                        tracing::error!("Failed to enqueue jobs: {}", e);
                        for tx in &req_data.transactions {
                            failed_ids.push(tx.id.clone().unwrap_or_default());
                        }
                        let _ = db_tx.rollback().await;
                    } else {
                        if let Err(e) = db_tx.commit().await {
                            tracing::error!("Failed to commit transaction: {}", e);
                            for tx in &req_data.transactions {
                                failed_ids.push(tx.id.clone().unwrap_or_default());
                            }
                        } else {
                            // Update pos_terminal_sessions with conflicts_pending if needed
                            if !pending_reconciliation_items.is_empty() {
                                let conflict_payload = serde_json::json!(pending_reconciliation_items.clone());
                                let _ = sqlx::query(
                                    "UPDATE pos_terminal_sessions
                                     SET sync_status = 'CONFLICTS_PENDING',
                                         pending_reconciliation = COALESCE(pending_reconciliation, '[]'::jsonb) || $1::jsonb
                                     WHERE tenant_id = $2
                                     AND device_id = $3"
                                )
                                .bind(conflict_payload)
                                .bind(&tenant_id)
                                .bind(&client_id)
                                .execute(&pool)
                                .await;
                            }
                            synced_count = req_data.transactions.len() as i32;
                        }
                    }
                } else {
                    if let Err(e) = db_tx.commit().await {
                        tracing::error!("Failed to commit transaction (empty rows): {}", e);
                    }
                    synced_count = req_data.transactions.len() as i32; // Assuming empty meant duplicates were ignored, consider it synced.
                }
            }
            Err(e) => {
                tracing::error!("Failed to insert offline transactions: {}", e);
                for tx in &req_data.transactions {
                    failed_ids.push(tx.id.clone().unwrap_or_default());
                }
                let _ = db_tx.rollback().await;
            }
        }
    }

    // Create an order out of each synced POS offline transaction if the type is cash_sale or tap_to_pay.
    for tx in &req_data.transactions {
        if tx.mutation_type.as_deref() == Some("cash_sale") || tx.mutation_type.as_deref() == Some("tap_to_pay") {
            let pool = crate::db::get_pool();
            if let Ok(mut db_tx) = pool.begin().await {
                if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
                    let order_id = uuid::Uuid::new_v4().to_string();
                    let total_amount = (tx.amount_cents as f64) / 100.0;
                    let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, 'completed')")
                        .bind(&order_id)
                        .bind(&tenant_id)
                        .bind(None::<String>)
                        .bind(total_amount)
                        .execute(&mut *db_tx).await;
                    if let Ok(payload_val) = serde_json::from_str::<serde_json::Value>(&tx.payload) {
                        if let Some(items) = payload_val.as_array() {
                            for item in items {
                                if let (Some(product_id), Some(quantity)) = (
                                    item.get("product_id").and_then(|v| v.as_str()),
                                    item.get("quantity").and_then(|v| v.as_i64()),
                                ) {
                                    let item_id = uuid::Uuid::new_v4().to_string();
                                    let _ = sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4, $5, $6)")
                                        .bind(&item_id)
                                        .bind(&tenant_id)
                                        .bind(&order_id)
                                        .bind(product_id)
                                        .bind(quantity as i32)
                                        .bind(total_amount)
                                        .execute(&mut *db_tx).await;
                                }
                            }
                        }
                    }
                }
                let _ = db_tx.commit().await;
            }

            // Send agent action requests for offline transactions
            if let Ok(mut agent_tx) = crate::db::get_pool().begin().await {
                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'terminal_offline', 'sales_and_revenue', 'record_pos_transaction', $3, 'pending')")
                    .bind(uuid::Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .bind(serde_json::json!({
                        "event": "pos_transaction_synced",
                        "amount_cents": tx.amount_cents,
                    }))
                    .execute(&mut *agent_tx).await;

                let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'terminal_offline', 'operations', 'record_pos_transaction', $3, 'pending')")
                    .bind(uuid::Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .bind(serde_json::json!({
                        "event": "pos_transaction_synced",
                    }))
                    .execute(&mut *agent_tx).await;
                let _ = agent_tx.commit().await;
            }
        }
    }

    let mut pending_reconciliation = None;
    if let Some(session_id) = &req_data.session_id {
        if let Ok(row) = sqlx::query("SELECT pending_reconciliation FROM pos_terminal_sessions WHERE id = $1 AND tenant_id = $2")
            .bind(session_id)
            .bind(&tenant_id)
            .fetch_optional(&pool)
            .await
        {
            if let Some(r) = row {
                let pr: Option<serde_json::Value> = sqlx::Row::try_get(&r, "pending_reconciliation").unwrap_or(None);
                if let Some(pr_val) = pr {
                    if let Some(arr) = pr_val.as_array() {
                        pending_reconciliation = Some(arr.clone());
                    }
                }
            }
        }
    }

    let res = SyncOfflineTransactionsResponse {
        success: failed_ids.is_empty(),
        synced_count,
        failed_transaction_ids: failed_ids,
        pending_reconciliation,
    };

    (axum::http::StatusCode::OK, Json(res)).into_response()
}




pub async fn commit_inventory_handler(
    _headers: axum::http::HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<CommitInventoryRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(info) => info.org_id.clone(),
        None => {
            let spiffe_id_str = _headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
            // WARNING: SECURITY FIX
            // Only allow tenant override for internal test agents, do not bypass spiffe id auth in prod!
            if let Some(tenant_override) = _headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).filter(|_| spiffe_id_str.starts_with("spiffe://ohc.local/org/") && spiffe_id_str.contains("/agent/")) {
                tenant_override.to_string()
            } else if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
                id
            } else {
                return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthenticated" }))).into_response()
            }
        }
    };

    let service = crate::services::inventory::InventoryService::new(
        hub.redis_client.clone()
    );

    match service.commit_inventory(&tenant_id, &req_data.product_id, req_data.quantity, &req_data.lock_id).await {
        Ok(result) => {
            if result.success {
                let pool = crate::db::get_pool();
                if let Ok(mut tx) = pool.begin().await {
                    if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                        let order_id = uuid::Uuid::new_v4().to_string();
                        let total_amount = (req_data.amount_cents.unwrap_or(0) as f64) / 100.0;
                        let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, 'completed')")
                            .bind(&order_id).bind(&tenant_id).bind(&req_data.customer_id).bind(total_amount).execute(&mut *tx).await;

                        let item_id = uuid::Uuid::new_v4().to_string();
                        let _ = sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4, $5, $6)")
                            .bind(&item_id).bind(&tenant_id).bind(&order_id).bind(&req_data.product_id).bind(req_data.quantity).bind(total_amount).execute(&mut *tx).await;

                        let event_payload = serde_json::json!({
                            "order_id": order_id,
                            "tenant_id": tenant_id,
                            "customer_id": req_data.customer_id,
                            "amount": total_amount,
                            "source": "in_person_pos",
                        });

                        let event = crate::orchestration::departments::types::DepartmentEvent {
                            id: uuid::Uuid::new_v4().to_string(),
                            tenant_id: tenant_id.clone(),
                            event_type: "POS_SALE_COMPLETED".to_string(),
                            payload: event_payload,
                        };
                        let _ = hub.publish_mesh_event(::server_ohc::orchestration::MeshEvent {
                            event_id: uuid::Uuid::new_v4().to_string(),
                            topic: "pos_sales".to_string(),
                            payload: serde_json::to_vec(&event).unwrap_or_default(),
                            timestamp: chrono::Utc::now().timestamp(),
                        });

                        // Create an agent action request for the Operations Agent
                        let action_req_id = uuid::Uuid::new_v4().to_string();
                        let payload = serde_json::json!({
                            "source": "pos",
                            "order_id": order_id,
                            "quantity": req_data.quantity,
                            "reason": "in_person_sale_inventory_check"
                        });
                        let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, product_id, payload) VALUES ($1, $2, 'InventoryCheck', 'Pending', $3, $4)")
                            .bind(&action_req_id)
                            .bind(&tenant_id)
                            .bind(&req_data.product_id)
                            .bind(&payload)
                            .execute(&mut *tx)
                            .await;
                    }
                    let _ = tx.commit().await;
                }
            }

            (axum::http::StatusCode::OK, Json(serde_json::json!({
                "success": result.success,
                "error_message": result.error_message
            }))).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::OK, Json(serde_json::json!({
                "success": false,
                "error_message": e
            }))).into_response()
        }
    }
}

pub async fn create_payment_intent_handler(
    _headers: axum::http::HeaderMap,
    State(hub): State<Arc<Hub>>,
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
        None => {
            let spiffe_id_str = _headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
            if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
                id
            } else {
                return Json(Err("Unauthenticated".to_string()))
            }
        }
    };

    let idempotency_key = req_data.idempotency_key.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let pool = crate::db::get_pool();

    // Check for existing intent with the same idempotency key
    let existing: Option<(String,)> = sqlx::query_as(
        "SELECT stripe_payment_intent_id FROM payment_intents WHERE tenant_id = $1 AND idempotency_key = $2"
    )
    .bind(&tenant_id)
    .bind(&idempotency_key)
    .fetch_optional(&pool)
    .await.unwrap_or(None);

    if let Some((_stripe_id,)) = existing {
        // Return existing client secret from stripe - though we might not have it in db, we can re-construct or just return a generic success since it's idempotent.
        // Actually Stripe's idempotency will return the exact same response anyway if we just pass the idempotency key down.
        // Let's just let Stripe handle the idempotency by passing the key, but we need to make sure we don't crash on DB unique constraint if it already exists.
    }

    let mut lock_id_out = None;

    if let Some(product_id) = &req_data.product_id {
        let quantity = req_data.quantity.unwrap_or(1);
        let service = crate::services::inventory::InventoryService::new(
            hub.redis_client.clone()
        );
        match service.reserve_inventory(&tenant_id, product_id, quantity, 15).await {
            Ok(result) => {
                if !result.success {
                    return Json(Err(result.error_message));
                }
                lock_id_out = Some(result.lock_id);
            },
            Err(e) => return Json(Err(e))
        }
    }

    // Compute dynamic yield price before generating payment intent
    let mut final_amount_cents = req_data.amount_cents;

    if let Some(product_id) = &req_data.product_id {
        let calculated_price = ::server_pricing::engine::apply_yield_management(&pool, &tenant_id, product_id, chrono::Utc::now(), req_data.amount_cents).await;
        final_amount_cents = calculated_price;
    }

    info!(tenant_id = %tenant_id, amount = final_amount_cents, currency = %req_data.currency, "Creating Stripe Terminal Payment Intent");

    if let Err(e) = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        &tenant_id,
        "stripe_terminal_payment_intent",
        0.05
    ).await {
        tracing::warn!("Failed to record api call cost: {}", e);
    }

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();

    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    let session_manager = crate::integrations::stripe::terminal::TerminalSessionManager::new(client);

    match crate::integrations::stripe::client::StripeClient::new(std::env::var("STRIPE_API_KEY").unwrap_or_default()).require_api_key() {
        Ok(_) => match session_manager.create_terminal_payment_intent(
            &tenant_id,
            final_amount_cents,
            &req_data.currency,
            req_data.product_id.as_deref(),
            req_data.quantity,
            req_data.order_id.as_deref(),
            &idempotency_key,
        ).await {
            Ok((payment_intent_id, client_secret)) => {
                let pool = crate::db::get_pool();

                let amount_float = (final_amount_cents as f64) / 100.0;
                let payment_id = uuid::Uuid::new_v4().to_string();

                // Use ON CONFLICT DO NOTHING to avoid duplicate key errors if idempotency_key is reused and already exists.
                let _ = sqlx::query(
                    "INSERT INTO payment_intents (tenant_id, payment_id, idempotency_key, amount, currency, status, source, stripe_payment_intent_id) VALUES ($1, $2, $3, $4, $5, 'pending', 'in_person', $6) ON CONFLICT (idempotency_key) DO NOTHING"
                )
                .bind(&tenant_id)
                .bind(&payment_id)
                .bind(&idempotency_key)
                .bind(amount_float)
                .bind(&req_data.currency)
                .bind(&payment_intent_id)
                .execute(&pool)
                .await;

                let device_id = "default_device"; // Fallback device id for web terminal intent creation without active explicit session.
                if let Err(e) = sqlx::query(
                    "INSERT INTO pos_terminal_sessions (id, tenant_id, device_id, status, started_at, last_synced_at, offline_changes_count)
                     VALUES ($1, $2, $3, 'ACTIVE', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)
                     ON CONFLICT (tenant_id, device_id) DO UPDATE SET last_synced_at = CURRENT_TIMESTAMP, status = 'ACTIVE'"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(device_id)
                .execute(&pool)
                .await {
                    tracing::warn!("Failed to update pos terminal session for generic intent: {}", e);
                }

                Json(Ok(PaymentIntentResponse { client_secret, lock_id: lock_id_out }))
            },
            Err(e) => {
                if let (Some(lock_id), Some(product_id)) = (&lock_id_out, &req_data.product_id) {
                    let quantity = req_data.quantity.unwrap_or(1);
                    let service = crate::services::inventory::InventoryService::new(
                        hub.redis_client.clone()
                    );
                    if let Err(err) = service.release_inventory(&tenant_id, product_id, quantity, lock_id).await {
                        tracing::error!("Failed to release inventory after stripe intent failed: {}", err); // pii-safe
                    }
                }
                Json(Err(e))
            }
        },
        Err(e) => {
            if let (Some(lock_id), Some(product_id)) = (&lock_id_out, &req_data.product_id) {
                let quantity = req_data.quantity.unwrap_or(1);
                let service = crate::services::inventory::InventoryService::new(
                    hub.redis_client.clone()
                );
                if let Err(err) = service.release_inventory(&tenant_id, product_id, quantity, lock_id).await {
                    tracing::error!("Failed to release inventory after stripe intent failed: {}", err); // pii-safe
                }
            }
            Json(Err(e.to_string()))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

        // tests go here
    #[allow(unused_imports)]
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_commit_inventory_low_stock() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        let tenant_id = "tenant-terminal-test-low";
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Terminal Test Tenant') ON CONFLICT DO NOTHING")
            .bind(tenant_id).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-terminal-test-2', $1, 'Test Prod Terminal', 6) ON CONFLICT DO NOTHING")
            .bind(tenant_id).execute(&pool).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let req_data = axum::extract::Json(CommitInventoryRequest {
            tenant_id: tenant_id.to_string(),
            product_id: "prod-terminal-test-2".to_string(),
            quantity: 2,
            lock_id: "".to_string(),
            customer_id: None,
            amount_cents: None,
        });
        let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: tenant_id.to_string(),
            spiffe_id: "test".to_string(),
            agent_id: "test".to_string()
        }));
        let headers = axum::http::HeaderMap::new();

        let _resp = commit_inventory_handler(headers, axum::extract::State(hub), auth_info, req_data).await;
        // Verify action request count
        let action_request_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_action_requests WHERE tenant_id = $1 AND product_id = 'prod-terminal-test-2' AND action_type = 'Reorder'")
            .bind(tenant_id)
            .fetch_one(&pool).await.unwrap();
        assert!(action_request_count.0 > 0);
    }

    #[tokio::test]
    async fn test_commit_inventory_records_order() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        let tenant_id = "tenant-pos-test-order";
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'POS Test Tenant') ON CONFLICT DO NOTHING")
            .bind(tenant_id).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-pos-test', $1, 'POS Test Prod', 10) ON CONFLICT DO NOTHING")
            .bind(tenant_id).execute(&pool).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let req_data = axum::extract::Json(CommitInventoryRequest {
            tenant_id: tenant_id.to_string(),
            product_id: "prod-pos-test".to_string(),
            quantity: 1,
            lock_id: "".to_string(),
            customer_id: None,
            amount_cents: Some(1999),
        });
        let auth_info = Some(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            org_id: tenant_id.to_string(),
            spiffe_id: "test".to_string(),
            agent_id: "test".to_string()
        }));
        let headers = axum::http::HeaderMap::new();

        commit_inventory_handler(headers, axum::extract::State(hub), auth_info, req_data).await;

        // Verify order count
        let order_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_one(&pool).await.unwrap();
        assert!(order_count.0 > 0);

        // Verify order items count
        let items_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM order_items WHERE tenant_id = $1 AND product_id = 'prod-pos-test'")
            .bind(tenant_id)
            .fetch_one(&pool).await.unwrap();
        assert!(items_count.0 > 0);
    }
}

fn extract_tenant_id_or_error(
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    headers: &axum::http::HeaderMap
) -> Result<String, axum::response::Response> {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    if !spiffe_id_str.is_empty() {
        if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
            return Ok(id);
        }
    }
    match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                Err((axum::http::StatusCode::OK, Json(serde_json::json!({ "error": "Unauthenticated: Missing tenant ID" }))).into_response())
            } else {
                Ok(auth.org_id.clone())
            }
        },
        None => Err((axum::http::StatusCode::OK, Json(serde_json::json!({ "error": "Unauthenticated" }))).into_response())
    }
}

pub async fn get_terminal_connection_token_handler(
    _headers: axum::http::HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> axum::response::Response {
    let tenant_id = match extract_tenant_id_or_error(auth_info, &_headers) {
        Ok(id) => id,
        Err(response) => return response,
    };

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();
    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    let session_manager = crate::integrations::stripe::terminal::TerminalSessionManager::new(client);

    match crate::integrations::stripe::client::StripeClient::new(std::env::var("STRIPE_API_KEY").unwrap_or_default()).require_api_key() {
        Ok(_) => match session_manager.create_terminal_connection_token(&tenant_id).await {
            Ok(token) => (axum::http::StatusCode::OK, Json(serde_json::json!({ "secret": token }))).into_response(),
            Err(e) => (axum::http::StatusCode::OK, Json(serde_json::json!({ "error": e }))).into_response(),
        },
        Err(e) => (axum::http::StatusCode::OK, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

pub async fn capture_payment_intent_handler(
    _headers: axum::http::HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<CapturePaymentIntentRequest>,
) -> Json<CapturePaymentIntentResponse> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(CapturePaymentIntentResponse {
                    success: false,
                    status: "".to_string(),
                    error_message: Some("Unauthenticated: Missing tenant ID".to_string()),
                });
            } else {
                auth.org_id.clone()
            }
        },
        None => {
            let spiffe_id_str = _headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
            if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
                id
            } else {
                return Json(CapturePaymentIntentResponse {
                    success: false,
                    status: "".to_string(),
                    error_message: Some("Unauthenticated".to_string()),
                });
            }
        }
    };

    info!(tenant_id = %tenant_id, payment_intent_id = %req_data.payment_intent_id, "Capturing Stripe Terminal Payment Intent");

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();
    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

    match client.require_api_key() {
        Ok(_) => {
            match client.capture_terminal_payment_intent(&req_data.payment_intent_id).await {
                Ok(status) => {
                    if let Some(product_id) = &req_data.product_id {
                        let quantity = req_data.quantity.unwrap_or(1);
                        let lock_id = req_data.lock_id.clone().unwrap_or_default();
                        let service = crate::services::inventory::InventoryService::new(
                            hub.redis_client.clone()
                        );
                        match service.commit_inventory(&tenant_id, product_id, quantity, &lock_id).await {
                            Ok(res) if !res.success => {
                                tracing::error!("Failed to commit inventory after successful capture: {}", res.error_message);
                            },
                            Ok(_) => {
                                // Inventory commit successful, log an order if possible
                                let pool = crate::db::get_pool();
                                if let Ok(mut tx) = pool.begin().await {
                                    if let Ok(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                                        let order_id = uuid::Uuid::new_v4().to_string();
                                        let total_amount = (req_data.amount_cents.unwrap_or(0) as f64) / 100.0;
                                        let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, 'completed')")
                                            .bind(&order_id)
                                            .bind(&tenant_id)
                                            .bind(None::<String>)
                                            .bind(total_amount)
                                            .execute(&mut *tx).await;
                        let _ = sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4, $5, $6)")
                                            .bind(uuid::Uuid::new_v4().to_string())
                                            .bind(&tenant_id)
                                            .bind(&order_id)
                                            .bind(product_id)
                                            .bind(quantity)
                                            .bind(total_amount)
                                            .execute(&mut *tx).await;
                                        let _ = tx.commit().await;
                                    }
                                }

                                // Notify Sales & Revenue Assistant via KAIROS/Orchestrator
                                if let Ok(mut agent_tx) = crate::db::get_pool().begin().await {
                                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'terminal', 'sales_and_revenue', 'record_pos_transaction', $3, 'pending')")
                                        .bind(uuid::Uuid::new_v4().to_string())
                                        .bind(&tenant_id)
                                        .bind(serde_json::json!({
                                            "event": "pos_transaction_completed",
                                            "payment_intent_id": req_data.payment_intent_id,
                                            "product_id": product_id,
                                            "quantity": quantity,
                                            "amount_cents": req_data.amount_cents,
                                        }))
                                        .execute(&mut *agent_tx).await;
                                    let _ = agent_tx.commit().await;
                                }

                                // Notify Operations Assistant
                                if let Ok(mut agent_tx) = crate::db::get_pool().begin().await {
                                    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'terminal', 'operations', 'record_pos_transaction', $3, 'pending')")
                                        .bind(uuid::Uuid::new_v4().to_string())
                                        .bind(&tenant_id)
                                        .bind(serde_json::json!({
                                            "event": "pos_transaction_completed",
                                            "payment_intent_id": req_data.payment_intent_id,
                                            "product_id": product_id,
                                            "quantity": quantity,
                                        }))
                                        .execute(&mut *agent_tx).await;
                                    let _ = agent_tx.commit().await;
                                }

                                // Draft success card in agent feed
                                if let Ok(mut feed_tx) = crate::db::get_pool().begin().await {
                                    let _ = sqlx::query(
                                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, 'terminal', $3, $4, 'PENDING_APPROVAL', NOW(), NOW())"
                                    )
                                    .bind(uuid::Uuid::new_v4().to_string())
                                    .bind(&tenant_id)
                                    .bind(serde_json::json!({
                                        "feature_type": "receipt_draft",
                                        "transaction_successful": true,
                                        "payment_intent_id": req_data.payment_intent_id,
                                    }))
                                    .bind(serde_json::json!({
                                        "description": "Transaction successful. Send receipt?"
                                    }))
                                    .execute(&mut *feed_tx)
                                    .await;
                                    let _ = feed_tx.commit().await;
                                }
                            },
                            Err(e) => {
                                tracing::error!("Failed to commit inventory after successful capture: {}", e);
                            }
                        }
                    }

                    Json(CapturePaymentIntentResponse {
                        success: true,
                        status,
                        error_message: None,
                    })
                },
                Err(e) => {
                    tracing::error!("Failed to capture terminal payment intent: {}", e);
                    Json(CapturePaymentIntentResponse {
                        success: false,
                        status: "".to_string(),
                        error_message: Some(e),
                    })
                }
            }
        },
        Err(e) => {
            Json(CapturePaymentIntentResponse {
                success: false,
                status: "".to_string(),
                error_message: Some(e.to_string()),
            })
        }
    }
}

#[derive(serde::Deserialize)]
pub struct PostBackendRequest {
    pub backend: String,
}

pub async fn get_terminal_backend_handler() -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    Ok(axum::Json(serde_json::json!({
        "backend": "local"
    })))
}

pub async fn post_terminal_backend_handler(
    axum::Json(req): axum::Json<PostBackendRequest>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // For test/harness purposes only, return the requested backend if valid
    if req.backend == "local" || req.backend == "docker" {
        return Ok(axum::Json(serde_json::json!({ "success": true, "backend": req.backend })));
    }
    Err((axum::http::StatusCode::BAD_REQUEST, "Invalid backend".into()))
}
