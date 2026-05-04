use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::integrations::stripe::client::StripeClient;
use reqwest::StatusCode;

#[derive(Deserialize)]
pub struct CreateCheckoutRequest {
    pub plan_id: String,
    pub customer_id: String,
}

#[derive(Serialize)]
pub struct CreateCheckoutResponse {
    pub url: String,
}

pub fn router<S>(stripe_client: Arc<StripeClient>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // A temporary, smaller router specifically for this nested route, without the State type constraint on S.
    // It's then converted to Router<S> to match the parent app's state requirement when nesting.
    Router::new()
        .route("/checkout", post(create_checkout_session))
        .with_state(stripe_client)
}

async fn create_checkout_session(
    State(client): State<Arc<StripeClient>>,
    Json(payload): Json<CreateCheckoutRequest>,
) -> impl IntoResponse {
    match client.create_checkout_session(&payload.plan_id, &payload.customer_id).await {
        Ok(url) => (StatusCode::OK, Json(CreateCheckoutResponse { url })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
