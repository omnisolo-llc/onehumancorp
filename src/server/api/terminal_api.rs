use axum::{extract::State, Json};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;

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

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/token", axum::routing::get(get_terminal_connection_token_handler))
        .route("/intent", axum::routing::post(create_payment_intent_handler))
        .with_state(hub)
}

pub async fn get_terminal_connection_token_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<Result<TerminalTokenResponse, String>> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(Err("Unauthenticated".to_string()))
    };

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
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<Result<PaymentIntentResponse, String>> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(Err("Unauthenticated".to_string()))
    };

    let payload: Result<axum::extract::Json<PaymentIntentRequest>, _> = axum::extract::FromRequest::from_request(request, &()).await;
    let req_data = match payload {
        Ok(data) => data.0,
        Err(_) => return Json(Err("Invalid payload".to_string())),
    };

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
