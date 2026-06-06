use axum::{extract::State, Json};
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
    pub intent_id: String,
}

#[derive(serde::Deserialize)]
pub struct ReserveInventoryRequest {
    pub product_id: String,
    pub ttl_secs: u64,
}

#[derive(serde::Serialize)]
pub struct ReserveInventoryResponse {
    pub success: bool,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/token", axum::routing::post(get_terminal_connection_token_handler))
        .route("/intent", axum::routing::post(create_payment_intent_handler))
        .route("/reserve", axum::routing::post(reserve_inventory_handler))
        .with_state(hub)
}

pub async fn reserve_inventory_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<ReserveInventoryRequest>,
) -> Json<ReserveInventoryResponse> {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Json(ReserveInventoryResponse { success: false });
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(ReserveInventoryResponse { success: false })
    };

    if let Some(client) = &hub.redis_client {
        let lock_key = format!("ohc:lock:{}:inventory:{}", tenant_id, req_data.product_id);
        if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
            let acquired: bool = redis::cmd("SET")
                .arg(&lock_key)
                .arg("1")
                .arg("NX")
                .arg("EX")
                .arg(req_data.ttl_secs)
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            return Json(ReserveInventoryResponse { success: acquired });
        }
    }

    // Fail closed if Redis is unavailable to prevent double booking
    Json(ReserveInventoryResponse { success: false })
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

    let stripe_key = match std::env::var("STRIPE_API_KEY") {
        Ok(k) => k,
        Err(_) => "sk_test_123".to_string(), // Fallback for dev/test
    };

    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    match client.create_terminal_connection_token(&tenant_id).await {
        Ok(token) => Json(Ok(TerminalTokenResponse { token })),
        Err(e) => Json(Err(e)),
    }
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

    let stripe_key = match std::env::var("STRIPE_API_KEY") {
        Ok(k) => k,
        Err(_) => "sk_test_123".to_string(), // Fallback for dev/test
    };

    let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
    match client.create_terminal_payment_intent(&tenant_id, req_data.amount_cents, &req_data.currency).await {
        Ok(intent_id) => Json(Ok(PaymentIntentResponse { intent_id })),
        Err(e) => Json(Err(e)),
    }
}
