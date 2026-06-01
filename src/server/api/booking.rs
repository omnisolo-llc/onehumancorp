use axum::{
    extract::{State, Json, Extension},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::services::booking::{BookingService, BookingRecord};
use chrono::Utc;
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct BookingChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct BookingChatResponse {
    pub reply: String,
    pub success: bool,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/chat", post(handle_chat))
}

async fn handle_chat(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<BookingChatRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "system".to_string(),
    };
    let customer_id = "default_customer".to_string(); // Assuming customer lookup logic will go here

    let lower_msg = payload.message.to_lowercase();

    if lower_msg.contains("confirm") || lower_msg.contains("book") || lower_msg.contains("tomorrow") {
        let start_time = Utc::now() + chrono::Duration::days(1);
        let end_time = start_time + chrono::Duration::hours(1);

        let booking = BookingRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            customer_id,
            product_id: "default_service".to_string(),
            start_time,
            end_time: Some(end_time),
            status: "confirmed".to_string(),
        };

        match BookingService::create_booking(booking).await {
            Ok(_) => {
                (StatusCode::OK, Json(BookingChatResponse {
                    reply: "Your appointment is confirmed for tomorrow! Looking forward to it.".to_string(),
                    success: true,
                })).into_response()
            },
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(BookingChatResponse {
                    reply: format!("Failed to create booking: {}", e),
                    success: false,
                })).into_response()
            }
        }
    } else {
        (StatusCode::OK, Json(BookingChatResponse {
            reply: "Hello! I can help you book an appointment. What time works best for you?".to_string(),
            success: true,
        })).into_response()
    }
}
