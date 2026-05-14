use axum::{
    extract::{Form, State},
    response::IntoResponse,
};
use std::collections::HashMap;
use sqlx::PgPool;

pub async fn twilio_webhook_receive(
    State(pool): State<PgPool>,
    Form(payload): Form<HashMap<String, String>>
) -> impl IntoResponse {
    if let Some(body) = payload.get("Body") {
        if let Some(from) = payload.get("From") {
            let _ = sqlx::query(
                "INSERT INTO sms_inbox (sender, message) VALUES ($1, $2)"
            )
            .bind(from)
            .bind(body)
            .execute(&pool).await;
        }
    }
    "EVENT_RECEIVED".to_string()
}
