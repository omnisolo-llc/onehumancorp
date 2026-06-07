use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
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
    pub product_ids: Option<Vec<String>>,
}

#[derive(serde::Serialize)]
pub struct PaymentIntentResponse {
    pub intent_id: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/token", axum::routing::post(get_terminal_connection_token_handler))
        .route("/intent", axum::routing::post(create_payment_intent_handler))
        .with_state(hub)
}


pub async fn get_terminal_connection_token_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Unauthenticated: Missing tenant ID".to_string() })).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Unauthenticated".to_string() })).into_response()
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
        Ok(token) => (StatusCode::OK, Json(TerminalTokenResponse { token })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e })).into_response(),
    }
}



pub async fn create_payment_intent_handler(
    _headers: HeaderMap,
    State(_hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    req_data: axum::extract::Json<PaymentIntentRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Unauthenticated: Missing tenant ID".to_string() })).into_response();
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Unauthenticated".to_string() })).into_response()
    };

    info!(tenant_id = %tenant_id, amount = req_data.amount_cents, currency = %req_data.currency, "Creating Stripe Terminal Payment Intent");

    // Attempt to acquire Redis Redlock for inventory constraints before generating payment intent
    if let Some(product_ids) = &req_data.product_ids {
        if !product_ids.is_empty() {
            let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
            if let Ok(redis_client) = redis::Client::open(redis_url) {
                let redis_lock = crate::orchestration::locks::RedisLock::new(redis_client);
                use crate::orchestration::locks::DistributedLock;
                for product_id in product_ids {
                    match redis_lock.acquire_resource(&tenant_id, "inventory", product_id).await {
                        Ok(_lock_val) => {
                            // Lock acquired successfully
                        }
                        _ => {
                            tracing::warn!("Failed to acquire inventory lock for product: {}", product_id);
                            return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Out of stock".to_string() })).into_response();
                        }
                    }
                }
            } else {
                tracing::warn!("Could not initialize Redis client to check inventory limits.");
            }
        }
    }

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
        Ok(intent_id) => (StatusCode::OK, Json(PaymentIntentResponse { intent_id })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e })).into_response(),
    }
}
