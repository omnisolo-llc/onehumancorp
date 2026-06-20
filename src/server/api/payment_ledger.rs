use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;
use crate::hub::Hub;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
    pub hub: Arc<Hub>,
}

#[derive(Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub amount: f64,
    pub currency: String,
    pub source: String,
}

#[derive(Serialize)]
pub struct PaymentIntentResponse {
    pub payment_id: String,
    pub idempotency_key: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    #[serde(rename = "type")]
    pub type_field: String, // e.g. "payment_intent.succeeded"
    pub data: WebhookData,
}

#[derive(Deserialize)]
pub struct WebhookData {
    pub object: StripePaymentIntent,
}

#[derive(Deserialize)]
pub struct StripePaymentIntent {
    pub id: String,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub tenant_id: String,
    pub total_revenue: f64,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/payments/intent", post(create_payment_intent))
        .route("/api/payments/webhook", post(stripe_webhook))
        .route("/api/ledger/balance", get(get_balance))
}

async fn create_payment_intent(
    State(_state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<CreatePaymentIntentRequest>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id;
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Missing tenant ID").into_response();
    }

    let payment_id = Uuid::new_v4().to_string();
    let idempotency_key = Uuid::new_v4().to_string();

    let pool = crate::db::get_pool();
    let res = sqlx::query(
        r#"
        INSERT INTO payment_intents (tenant_id, payment_id, idempotency_key, amount, currency, source)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(&tenant_id)
    .bind(&payment_id)
    .bind(&idempotency_key)
    .bind(payload.amount)
    .bind(&payload.currency)
    .bind(&payload.source)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(PaymentIntentResponse {
            payment_id,
            idempotency_key,
            status: "pending".to_string()
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

use crate::integrations::stripe::webhooks::StripeWebhookEvent;
use crate::integrations::stripe::webhooks::enqueue_webhook_event;

use serde_json::Value;

async fn stripe_webhook(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let r_type = payload.get("type").and_then(|v| v.as_str()).unwrap_or_default();
    if r_type != "payment_intent.succeeded" {
        return StatusCode::OK.into_response();
    }

    let object = payload.get("data").and_then(|d| d.get("object"));

    // In a real app we'd verify the signature, for this implementation we rely on the metadata
    let tenant_id = object.and_then(|o| o.get("metadata")).and_then(|m| m.get("tenant_id")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let idempotency_key = object.and_then(|o| o.get("metadata")).and_then(|m| m.get("idempotency_key")).and_then(|v| v.as_str()).unwrap_or_default().to_string();

    if tenant_id.is_empty() || idempotency_key.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let pool = crate::db::get_pool();

    let event = StripeWebhookEvent {
        id: payload.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()),
        r#type: Some(r_type.to_string()),
        data: payload.get("data").map(|d| crate::integrations::stripe::webhooks::StripeWebhookData {
            object: d.get("object").cloned(),
        })
    };

    match enqueue_webhook_event(&pool, &tenant_id, &event).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_balance(
    State(_state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id;
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Missing tenant ID").into_response();
    }

    let pool = crate::db::get_pool();
    let balance: Option<(f64,)> = sqlx::query_as("SELECT balance FROM ledger_accounts WHERE tenant_id = $1 AND account_id = 'default_revenue'")
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await.unwrap_or(None);

    let total_revenue = match balance {
        Some((b,)) => b,
        None => 0.0
    };

    (StatusCode::OK, Json(BalanceResponse {
        tenant_id,
        total_revenue,
    })).into_response()
}
