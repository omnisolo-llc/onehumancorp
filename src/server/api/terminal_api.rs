use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use crate::integrations::stripe::client::StripeClient;
use crate::integrations::stripe::terminal::{ConnectionToken, PaymentIntent};
use axum::http::StatusCode;

#[derive(Serialize)]
pub struct ConnectionTokenResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct PaymentIntentRequest {
    pub amount: i64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct PaymentIntentResponse {
    pub intent_id: String,
    pub amount: i64,
    pub status: String,
}

async fn handle_connection_token(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    tracing::info!("Generating terminal connection token for tenant: {}", tenant_id);

    let client = StripeClient::new("sk_test_123".to_string());
    match client.create_terminal_connection_token().await {
        Ok(token) => (StatusCode::OK, Json(ConnectionTokenResponse { token: token.secret })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", e))).into_response(),
    }
}

async fn handle_create_payment_intent(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
    Json(payload): Json<PaymentIntentRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    tracing::info!("Generating terminal payment intent for tenant: {}, amount: {}", tenant_id, payload.amount);

    let client = StripeClient::new("sk_test_123".to_string());
    match client.create_terminal_payment_intent(payload.amount, &payload.currency).await {
        Ok(intent) => (StatusCode::OK, Json(PaymentIntentResponse {
            intent_id: intent.id,
            amount: intent.amount,
            status: intent.status,
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", e))).into_response(),
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/terminal/token", post(handle_connection_token))
        .route("/terminal/intent", post(handle_create_payment_intent))
        .layer(Extension(hub))
}
