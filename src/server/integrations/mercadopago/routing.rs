use axum::{
    extract::{Query, Json, State},
    response::IntoResponse,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct MercadoPagoWebhookPayload {
    pub action: String,
    pub api_version: String,
    pub data: serde_json::Value,
    pub date_created: String,
    pub id: i64,
    pub live_mode: bool,
    pub r#type: String,
    pub user_id: i64,
}

pub async fn mercadopago_webhook_receive(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<MercadoPagoWebhookPayload>,
) -> impl IntoResponse {
    if let Some(topic) = params.get("topic") {
        if topic == "payment" {
            let payment_id = payload.data.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let _ = sqlx::query(
                "UPDATE orders SET status = 'paid' WHERE payment_intent_id = $1"
            )
            .bind(payment_id)
            .execute(&pool).await;
        }
    }
    "EVENT_RECEIVED".to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum MercadoPagoMethod {
    Pix,
    Boleto,
    CreditCard,
}
