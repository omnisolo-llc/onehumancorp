use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct CalWebhookPayload {
    pub triggerEvent: String,
    pub payload: serde_json::Value,
}

pub async fn cal_webhook_receive(
    State(pool): State<PgPool>,
    Json(payload): Json<CalWebhookPayload>
) -> impl IntoResponse {
    if let Some(booking_id) = payload.payload.get("uid").and_then(|v| v.as_str()) {
        let _ = sqlx::query(
            "UPDATE bookings SET status = $1 WHERE provider_id = $2"
        )
        .bind(&payload.triggerEvent)
        .bind(booking_id)
        .execute(&pool).await;
    }
    "EVENT_RECEIVED".to_string()
}

pub async fn cal_oauth_callback(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(_code) = params.get("code") {
        return "Cal OAuth Successful".to_string();
    }
    "Failed".to_string()
}
