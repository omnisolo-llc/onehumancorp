use axum::{
    extract::{State, Json},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct ResendWebhookPayload {
    pub r#type: String,
    pub created_at: String,
    pub data: serde_json::Value,
}

pub async fn resend_webhook_receive(
    State(pool): State<PgPool>,
    Json(payload): Json<ResendWebhookPayload>
) -> impl IntoResponse {
    if let Some(email_id) = payload.data.get("email_id").and_then(|v| v.as_str()) {
        let _ = sqlx::query(
            "UPDATE emails SET status = $1 WHERE provider_id = $2"
        )
        .bind(&payload.r#type)
        .bind(email_id)
        .execute(&pool).await;
    }
    "EVENT_RECEIVED".to_string()
}
