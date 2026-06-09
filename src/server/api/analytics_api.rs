use axum::{
    response::IntoResponse,
    routing::get,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RevenueResponse {
    pub daily_revenue: f64,
}

#[derive(Serialize)]
pub struct BookingsResponse {
    pub pending_bookings: u32,
}

#[derive(Serialize)]
pub struct MessagesResponse {
    pub unanswered_messages: u32,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/daily_revenue", get(|| async {
            Json(RevenueResponse { daily_revenue: 450.0 }).into_response()
        }))
        .route("/pending_bookings", get(|| async {
            Json(BookingsResponse { pending_bookings: 3 }).into_response()
        }))
        .route("/unanswered_messages", get(|| async {
            Json(MessagesResponse { unanswered_messages: 2 }).into_response()
        }))
}
