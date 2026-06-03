use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DeliveryQuoteRequest {
    pub pickup_address: String,
    pub dropoff_address: String,
}

#[derive(Serialize)]
pub struct DeliveryQuoteResponse {
    pub fee: f64,
}

async fn handle_get_quote(
    Json(payload): Json<DeliveryQuoteRequest>,
) -> impl IntoResponse {
    // In a real app we would call doordash client here, but using mock for now as per instructions.
    let fee = 8.50;

    (StatusCode::OK, Json(DeliveryQuoteResponse { fee })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/quote", post(handle_get_quote))
}
