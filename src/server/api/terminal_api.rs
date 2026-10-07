use crate::hub::Hub;
use axum::{Json, extract::State, http::HeaderMap, response::IntoResponse};
use std::sync::Arc;
use tracing::info;

#[path = "terminal_offline_sync.rs"]
mod offline_sync;

#[path = "terminal_cash_receipts.rs"]
mod cash_receipts;

#[path = "terminal_payment_identity.rs"]
mod terminal_payment_identity;

#[derive(serde::Serialize)]
pub struct TerminalTokenResponse {
    pub token: String,
}

fn default_terminal_currency() -> String {
    "usd".to_string()
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaymentIntentRequest {
    pub amount_cents: Option<i64>,
    #[serde(default = "default_terminal_currency")]
    pub currency: String,
    pub product_id: Option<String>,
    pub quantity: Option<i32>,
    pub order_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub total: Option<f64>,
    pub reader_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: String,
    pub lock_id: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
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

pub fn router(
    hub: Arc<Hub>,
) -> axum::Router<Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route(
            "/token",
            axum::routing::post(get_terminal_connection_token_handler),
        )
        .route(
            "/intent",
            axum::routing::post(create_payment_intent_handler),
        )
        .route(
            "/intent/capture",
            axum::routing::post(capture_payment_intent_handler),
        )
        .route(
            "/sync_offline",
            axum::routing::post(sync_offline_transactions_handler),
        )
        .route(
            "/edge_sync",
            axum::routing::post(sync_edge_ledger_transactions_handler),
        )
        .route("/reserve", axum::routing::post(reserve_inventory_handler))
        .route("/commit", axum::routing::post(commit_inventory_handler))
        .route(
            "/commit/{operation_id}",
            axum::routing::get(read_cash_receipt_handler),
        )
        .route(
            "/session/start",
            axum::routing::post(start_terminal_session_handler),
        )
        .route(
            "/session/update",
            axum::routing::post(update_terminal_session_status_handler),
        )
        .route(
            "/session/end",
            axum::routing::post(end_terminal_session_handler),
        )
        .route("/backend", axum::routing::get(get_terminal_backend_handler))
        .route(
            "/backend",
            axum::routing::post(post_terminal_backend_handler),
        )
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
                return Json(StartTerminalSessionResponse {
                    session_id: "".to_string(),
                    success: false,
                    error_message: "Unauthenticated: Missing tenant ID".to_string(),
                });
            } else {
                auth.org_id.clone()
            }
        }
        None => {
            return Json(StartTerminalSessionResponse {
                session_id: "".to_string(),
                success: false,
                error_message: "Unauthenticated".to_string(),
            });
        }
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
                return Json(UpdateTerminalSessionStatusResponse {
                    success: false,
                    error_message: "Unauthenticated: Missing tenant ID".to_string(),
                });
            } else {
                auth.org_id.clone()
            }
        }
        None => {
            return Json(UpdateTerminalSessionStatusResponse {
                success: false,
                error_message: "Unauthenticated".to_string(),
            });
        }
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
                Json(UpdateTerminalSessionStatusResponse {
                    success: true,
                    error_message: "".to_string(),
                })
            } else {
                Json(UpdateTerminalSessionStatusResponse {
                    success: false,
                    error_message: "Session not found".to_string(),
                })
            }
        }
        Err(e) => {
            tracing::error!("Failed to update terminal session status: {}", e);
            Json(UpdateTerminalSessionStatusResponse {
                success: false,
                error_message: e.to_string(),
            })
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
                return Json(EndTerminalSessionResponse {
                    success: false,
                    error_message: "Unauthenticated: Missing tenant ID".to_string(),
                });
            } else {
                auth.org_id.clone()
            }
        }
        None => {
            return Json(EndTerminalSessionResponse {
                success: false,
                error_message: "Unauthenticated".to_string(),
            });
        }
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
                Json(EndTerminalSessionResponse {
                    success: true,
                    error_message: "".to_string(),
                })
            } else {
                Json(EndTerminalSessionResponse {
                    success: false,
                    error_message: "Session not found".to_string(),
                })
            }
        }
        Err(e) => {
            tracing::error!("Failed to end terminal session: {}", e);
            Json(EndTerminalSessionResponse {
                success: false,
                error_message: e.to_string(),
            })
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
    pub operation_id: Option<String>,
    pub tenant_id: String,
    pub items: Option<Vec<cash_receipts::CashItem>>,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub quantity: i32,
    #[serde(default)]
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
            let spiffe_id_str = _headers
                .get("x-spiffe-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            // WARNING: SECURITY FIX
            // Only allow tenant override for internal test agents, do not bypass spiffe id auth in prod!
            if let Some(tenant_override) = _headers
                .get("x-tenant-id")
                .and_then(|v| v.to_str().ok())
                .filter(|_| {
                    spiffe_id_str.starts_with("spiffe://ohc/org/")
                        && spiffe_id_str.contains("/agent/")
                })
            {
                tenant_override.to_string()
            } else if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
                id
            } else {
                return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({ "error": "unauthenticated" })),
                )
                    .into_response();
            }
        }
    };

    let service = crate::services::inventory::InventoryService::new(hub.redis_client());

    match service
        .reserve_inventory(
            &tenant_id,
            &req_data.product_id,
            req_data.quantity,
            if req_data.ttl_seconds > 0 {
                req_data.ttl_seconds
            } else {
                15
            },
        )
        .await
    {
        Ok(result) => (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({
                "success": result.success,
                "lock_id": result.lock_id,
                "error_message": result.error_message
            })),
        )
            .into_response(),
        Err(e) => (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({
                "success": false,
                "lock_id": "",
                "error_message": e
            })),
        )
            .into_response(),
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
    /// Durable acceptance for processing, not proof of a completed payment.
    pub acknowledged_transaction_ids: Vec<String>,
    pub already_processed_transaction_ids: Vec<String>,
    pub reconciliation_required_transaction_ids: Vec<String>,
    pub outcomes: Vec<offline_sync::TerminalOutcome>,
}

#[derive(serde::Deserialize)]
pub struct EdgeLedgerTransaction {
    pub transaction_id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub status: String,
    pub device_signature: Option<String>,
    pub payload: String,
}

#[derive(serde::Deserialize)]
pub struct SyncEdgeLedgerTransactionsRequest {
    pub transactions: Vec<EdgeLedgerTransaction>,
}

#[derive(serde::Serialize)]
pub struct SyncEdgeLedgerTransactionsResponse {
    pub success: bool,
    pub synced_count: i32,
    pub failed_transaction_ids: Vec<String>,
}

pub async fn sync_edge_ledger_transactions_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<SyncEdgeLedgerTransactionsRequest>,
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
            let spiffe_id_str = _headers
                .get("x-spiffe-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            if let Some(tenant_override) = _headers.get("x-tenant-id").and_then(|v| v.to_str().ok())
            {
                tenant_override.to_string()
            } else if let Ok((id, _)) = ::server_auth::parse_spiffe_id(spiffe_id_str) {
                id
            } else {
                return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({ "error": "unauthenticated" })),
                )
                    .into_response();
            }
        }
    };

    let pool = crate::db::get_pool();
    let mut failed_ids = Vec::new();
    let mut synced_count = 0;

    if !req_data.transactions.is_empty() {
        let mut db_tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({ "error": "Internal server error" })),
                )
                    .into_response();
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await
        {
            tracing::error!("Failed to set org context: {}", e);
            let _ = db_tx.rollback().await;
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal server error" })),
            )
                .into_response();
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO edge_ledger_transactions (id, tenant_id, transaction_id, amount_cents, currency, status, device_signature, payload, synced_at) ",
        );

        let tenant_id_clone = tenant_id.clone();
        query_builder.push_values(req_data.transactions.iter(), |mut b, tx| {
            let id = uuid::Uuid::new_v4().to_string();
            let transaction_id = tx.transaction_id.clone();
            let amount_cents = tx.amount_cents;
            let currency = tx.currency.clone();
            let status = tx.status.clone();
            let device_signature = tx.device_signature.clone();
            let payload_str = tx.payload.clone();

            b.push_bind(id)
                .push_bind(tenant_id_clone.clone())
                .push_bind(transaction_id)
                .push_bind(amount_cents)
                .push_bind(currency)
                .push_bind(status)
                .push_bind(device_signature)
                .push_bind(sqlx::types::Json(
                    serde_json::from_str::<serde_json::Value>(&payload_str)
                        .unwrap_or(serde_json::json!({})),
                ))
                .push_bind(sqlx::types::chrono::Utc::now());
        });

        query_builder.push(" ON CONFLICT (tenant_id, transaction_id) DO NOTHING");

        match query_builder.build().execute(&mut *db_tx).await {
            Ok(result) => {
                synced_count = result.rows_affected() as i32;
                if let Err(e) = db_tx.commit().await {
                    tracing::error!("Failed to commit edge ledger transaction: {}", e);
                    for tx in &req_data.transactions {
                        failed_ids.push(tx.transaction_id.clone());
                    }
                }
            }
            Err(e) => {
                tracing::error!("Failed to insert edge ledger transactions: {}", e);
                for tx in &req_data.transactions {
                    failed_ids.push(tx.transaction_id.clone());
                }
                let _ = db_tx.rollback().await;
            }
        }
    }

    Json(SyncEdgeLedgerTransactionsResponse {
        success: failed_ids.is_empty(),
        synced_count,
        failed_transaction_ids: failed_ids,
    })
    .into_response()
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

    offline_sync::sync_offline_response(
        &crate::db::get_pool(),
        &tenant_id,
        &req_data,
        crate::get_redis_client(),
    )
    .await
}

pub async fn commit_inventory_handler(
    _headers: axum::http::HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<CommitInventoryRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(info) if !info.org_id.trim().is_empty() => info.org_id.clone(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"success":false,"status":"rejected","error_message":"Unauthenticated"}))).into_response(),
    };
    cash_receipts::commit(&hub, &tenant_id, &req_data).await
}

pub async fn read_cash_receipt_handler(
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    axum::extract::Path(operation_id): axum::extract::Path<String>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(info) if !info.org_id.trim().is_empty() => info.org_id.clone(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"success":false,"status":"rejected","error_message":"Unauthenticated"}))).into_response(),
    };
    cash_receipts::read(&hub, &tenant_id, &operation_id).await
}

pub async fn create_payment_intent_handler(
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    axum::extract::Json(input): axum::extract::Json<PaymentIntentRequest>,
) -> axum::response::Response {
    let tenant = match auth_info {
        Some(info)
            if !info.org_id.trim().is_empty()
                && !info.org_id.trim().eq_ignore_ascii_case("system") =>
        {
            info.org_id.clone()
        }
        _ => {
            return terminal_payment_identity::Error(
                axum::http::StatusCode::UNAUTHORIZED,
                "rejected",
                "Authentication required.",
            )
            .into_response();
        }
    };
    // Never treat a caller's aggregate cart price as one catalog product. The
    // existing reservation path cannot bind every line durably, so fail closed.
    if input.product_id.is_some()
        || input.quantity.is_some()
        || input.order_id.is_some()
        || input.total.is_some()
    {
        return terminal_payment_identity::Error(axum::http::StatusCode::CONFLICT,"rejected","Catalog/cart card payment requires a persisted reservation contract. No card payment was started.").into_response();
    }
    let (Some(operation_id), Some(amount_cents)) = (input.idempotency_key, input.amount_cents)
    else {
        return terminal_payment_identity::Error(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            "rejected",
            "A stable idempotency_key and amount_cents are required.",
        )
        .into_response();
    };
    let client = match terminal_payment_client(&hub.pool, &tenant).await {
        Ok(client) => client,
        Err(error) => return error.into_response(),
    };
    let fingerprint = match terminal_payment_identity::connection_fingerprint(&client) {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };
    match terminal_payment_identity::create(
        &hub.pool,
        &tenant,
        terminal_payment_identity::IntentInput {
            operation_id,
            amount_cents,
            currency: input.currency,
            reader_id: input.reader_id,
        },
        &fingerprint,
        &client,
    )
    .await
    {
        Ok(receipt) => terminal_json_response(receipt),
        Err(error) => error.into_response(),
    }
}

fn terminal_json_response(value: impl serde::Serialize) -> axum::response::Response {
    let mut response = Json(value).into_response();
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("private, no-store"),
    );
    response
}

async fn terminal_payment_client(
    pool: &sqlx::PgPool,
    tenant: &str,
) -> Result<crate::integrations::stripe::client::StripeClient, terminal_payment_identity::Error> {
    let db = crate::db::DB {
        pool: pool.clone(),
        store: crate::db::DbStore::Postgres,
    };
    let key = crate::api::tool_integrations::stripe_key_for_tenant(&db, tenant)
        .await
        .map_err(|_| {
            terminal_payment_identity::Error(
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "rejected",
                "A verified tenant payment connection is required.",
            )
        })?;
    Ok(crate::integrations::stripe::client::StripeClient::new(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests go here
    #[allow(unused_imports)]
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_commit_inventory_low_stock() {
        let database_url = std::env::var("OMNISOLO_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options()
            .connect(&database_url)
            .await
            .unwrap();

        let tenant_id = "tenant-terminal-test-low";
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Terminal Test Tenant') ON CONFLICT DO NOTHING")
            .bind(tenant_id).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, price_cents, inventory_count, available_quantity) VALUES ('prod-terminal-test-2', $1, 'Test Prod Terminal', 100, 6, 6) ON CONFLICT (id) DO UPDATE SET price_cents=100,inventory_count=6,available_quantity=6,locked_quantity=0")
            .bind(tenant_id).execute(&pool).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let req_data = axum::extract::Json(CommitInventoryRequest {
            operation_id: Some(uuid::Uuid::new_v4().to_string()),
            items: None,
            tenant_id: tenant_id.to_string(),
            product_id: "prod-terminal-test-2".to_string(),
            quantity: 2,
            lock_id: "".to_string(),
            customer_id: None,
            amount_cents: Some(200),
        });
        let auth_info = Some(axum::extract::Extension(
            ::server_auth::orchestration::AuthInfo {
                org_id: tenant_id.to_string(),
                spiffe_id: "test".to_string(),
                agent_id: "test".to_string(),
            },
        ));
        let headers = axum::http::HeaderMap::new();

        let resp =
            commit_inventory_handler(headers, axum::extract::State(hub), auth_info, req_data).await;
        assert_eq!(resp.status(), axum::http::StatusCode::OK);
        // Verify action request count
        let action_request_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_action_requests WHERE tenant_id = $1 AND product_id = 'prod-terminal-test-2' AND action_type = 'Reorder'")
            .bind(tenant_id)
            .fetch_one(&pool).await.unwrap();
        assert!(action_request_count.0 > 0);
    }

    #[tokio::test]
    async fn test_commit_inventory_records_order() {
        let database_url = std::env::var("OMNISOLO_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options()
            .connect(&database_url)
            .await
            .unwrap();

        let tenant_id = "tenant-pos-test-order";
        sqlx::query(
            "INSERT INTO tenants (id, name) VALUES ($1, 'POS Test Tenant') ON CONFLICT DO NOTHING",
        )
        .bind(tenant_id)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, price_cents, inventory_count, available_quantity) VALUES ('prod-pos-test', $1, 'POS Test Prod', 1999, 10, 10) ON CONFLICT (id) DO UPDATE SET price_cents=1999,inventory_count=10,available_quantity=10,locked_quantity=0")
            .bind(tenant_id).execute(&pool).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let req_data = axum::extract::Json(CommitInventoryRequest {
            operation_id: Some(uuid::Uuid::new_v4().to_string()),
            items: None,
            tenant_id: tenant_id.to_string(),
            product_id: "prod-pos-test".to_string(),
            quantity: 1,
            lock_id: "".to_string(),
            customer_id: None,
            amount_cents: Some(1999),
        });
        let auth_info = Some(axum::extract::Extension(
            ::server_auth::orchestration::AuthInfo {
                org_id: tenant_id.to_string(),
                spiffe_id: "test".to_string(),
                agent_id: "test".to_string(),
            },
        ));
        let headers = axum::http::HeaderMap::new();

        let response =
            commit_inventory_handler(headers, axum::extract::State(hub), auth_info, req_data).await;
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        // Verify order count
        let order_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM orders WHERE tenant_id = $1")
                .bind(tenant_id)
                .fetch_one(&pool)
                .await
                .unwrap();
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
    _headers: &axum::http::HeaderMap,
) -> Result<String, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Transport headers are untrusted data. Only the verified middleware
    // extension may select a tenant's payment connection.
    match auth_info {
        Some(auth)
            if !auth.org_id.trim().is_empty()
                && !auth.org_id.trim().eq_ignore_ascii_case("system") =>
        {
            Ok(auth.org_id.clone())
        }
        _ => Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error":"Authentication required."})),
        )),
    }
}

pub async fn get_terminal_connection_token_handler(
    _headers: axum::http::HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> axum::response::Response {
    let tenant_id = match extract_tenant_id_or_error(auth_info, &_headers) {
        Ok(id) => id,
        Err(response) => return response.into_response(),
    };

    let client = match terminal_payment_client(&hub.pool, &tenant_id).await {
        Ok(client) => client,
        Err(error) => return error.into_response(),
    };
    match client.create_terminal_connection_token(&tenant_id).await {
        Ok(secret) => terminal_json_response(serde_json::json!({"secret":secret})),
        Err(_) => (
            axum::http::StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error":"Terminal connection is unavailable."})),
        )
            .into_response(),
    }
}

pub async fn capture_payment_intent_handler(
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    axum::extract::Json(input): axum::extract::Json<CapturePaymentIntentRequest>,
) -> axum::response::Response {
    let tenant = match auth_info {
        Some(info)
            if !info.org_id.trim().is_empty()
                && !info.org_id.trim().eq_ignore_ascii_case("system") =>
        {
            info.org_id.clone()
        }
        _ => {
            return terminal_payment_identity::Error(
                axum::http::StatusCode::UNAUTHORIZED,
                "rejected",
                "Authentication required.",
            )
            .into_response();
        }
    };
    if input.product_id.is_some()
        || input.quantity.is_some()
        || input
            .lock_id
            .as_deref()
            .is_some_and(|value| !value.is_empty())
    {
        return terminal_payment_identity::Error(
            axum::http::StatusCode::CONFLICT,
            "rejected",
            "Catalog/cart capture has no persisted reservation binding and is unavailable.",
        )
        .into_response();
    }
    if let Err(error) =
        terminal_payment_identity::require_owned(&hub.pool, &tenant, &input.payment_intent_id).await
    {
        return error.into_response();
    }
    let client = match terminal_payment_client(&hub.pool, &tenant).await {
        Ok(client) => client,
        Err(error) => return error.into_response(),
    };
    let fingerprint = match terminal_payment_identity::connection_fingerprint(&client) {
        Ok(value) => value,
        Err(error) => return error.into_response(),
    };
    match terminal_payment_identity::capture(
        &hub.pool,
        &tenant,
        terminal_payment_identity::CaptureInput {
            payment_intent_id: input.payment_intent_id,
            amount_cents: input.amount_cents,
        },
        &fingerprint,
        &client,
    )
    .await
    {
        Ok(receipt) => terminal_json_response(receipt),
        Err(error) => error.into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct PostBackendRequest {
    pub backend: String,
}

pub async fn get_terminal_backend_handler()
-> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    Ok(axum::Json(serde_json::json!({
        "backend": "local"
    })))
}

pub async fn post_terminal_backend_handler(
    axum::Json(req): axum::Json<PostBackendRequest>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // For test/harness purposes only, return the requested backend if valid
    if req.backend == "local" || req.backend == "docker" {
        return Ok(axum::Json(
            serde_json::json!({ "success": true, "backend": req.backend }),
        ));
    }
    Err((
        axum::http::StatusCode::BAD_REQUEST,
        "Invalid backend".into(),
    ))
}
