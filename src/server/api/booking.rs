use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::services::booking::BookingService;
use crate::orchestration::locks::DistributedLock;

#[derive(Deserialize)]
pub struct ReserveSlotRequest {
    pub customer_id: String,
    pub product_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ReserveSlotResponse {
    pub success: bool,
    pub booking_id: Option<String>,
    pub error: Option<String>,
}

async fn handle_reserve_slot(
    Extension(lock_manager): Extension<Arc<dyn DistributedLock>>,
    Extension(claims): Extension<::server_auth::common::Claims>,
    Json(payload): Json<ReserveSlotRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    match BookingService::reserve_time_slot(
        lock_manager,
        &tenant_id,
        &payload.customer_id,
        &payload.product_id,
        payload.start_time,
        payload.end_time
    ).await {
        Ok(booking_id) => (
            StatusCode::OK,
            Json(ReserveSlotResponse {
                success: true,
                booking_id: Some(booking_id),
                error: None,
            })
        ).into_response(),
        Err(e) => (
            StatusCode::CONFLICT,
            Json(ReserveSlotResponse {
                success: false,
                booking_id: None,
                error: Some(e),
            })
        ).into_response()
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(lock_manager: Arc<dyn DistributedLock>) -> Router<S> {
    Router::new()
        .route("/reserve", post(handle_reserve_slot))
        .layer(Extension(lock_manager))
}
