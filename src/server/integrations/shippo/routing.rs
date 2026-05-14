use axum::{
    extract::{State, Json},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct ShippoWebhookPayload {
    pub event: String,
    pub data: serde_json::Value,
}

pub async fn shippo_webhook_receive(
    State(pool): State<PgPool>,
    Json(payload): Json<ShippoWebhookPayload>
) -> impl IntoResponse {
    if payload.event == "track_updated" {
        if let Some(tracking_number) = payload.data.get("tracking_number").and_then(|v| v.as_str()) {
            let _ = sqlx::query(
                "UPDATE orders SET tracking_status = $1 WHERE tracking_number = $2"
            )
            .bind("updated")
            .bind(tracking_number)
            .execute(&pool).await;
        }
    }
    "EVENT_RECEIVED".to_string()
}
