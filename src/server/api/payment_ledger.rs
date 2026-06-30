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
    pub capture_method: Option<String>,
}

#[derive(Serialize)]
pub struct PaymentIntentResponse {
    pub payment_id: String,
    pub idempotency_key: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
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
    let capture_method = payload.capture_method.unwrap_or_else(|| "automatic".to_string());

    let res = sqlx::query(
        r#"
        INSERT INTO payment_intents (tenant_id, payment_id, idempotency_key, amount, currency, source, capture_method)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(&tenant_id)
    .bind(&payment_id)
    .bind(&idempotency_key)
    .bind(payload.amount)
    .bind(&payload.currency)
    .bind(&payload.source)
    .bind(&capture_method)
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

async fn stripe_webhook(
    State(_state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    if payload.type_field != "payment_intent.succeeded" {
        return StatusCode::OK.into_response();
    }

    let payment_intent = payload.data.object;

    // In a real app we'd verify the signature, for this implementation we rely on the metadata
    let tenant_id = payment_intent.metadata.get("tenant_id").cloned().unwrap_or_default();
    let idempotency_key = payment_intent.metadata.get("idempotency_key").cloned().unwrap_or_default();

    if tenant_id.is_empty() || idempotency_key.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let pool = crate::db::get_pool();

    // Check if we already processed it
    let existing: Option<(String,)> = sqlx::query_as("SELECT status FROM payment_intents WHERE idempotency_key = $1 AND tenant_id = $2")
        .bind(&idempotency_key)
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await.unwrap_or(None);

    if let Some((status,)) = existing {
        if status == "succeeded" {
            return StatusCode::OK.into_response();
        }
    } else {
        return StatusCode::NOT_FOUND.into_response();
    }

    // Process success
    let mut tx = pool.begin().await.unwrap();

    let update_res = sqlx::query("UPDATE payment_intents SET status = 'succeeded', stripe_payment_intent_id = $1 WHERE idempotency_key = $2 AND tenant_id = $3")
        .bind(&payment_intent.id)
        .bind(&idempotency_key)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await;

    if update_res.is_err() {
        let _ = tx.rollback().await;
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // Update Ledger (Create transaction and entry)
    let payment_info: (f64, String) = sqlx::query_as("SELECT amount, currency FROM payment_intents WHERE idempotency_key = $1")
        .bind(&idempotency_key)
        .fetch_one(&mut *tx)
        .await.unwrap();

    let tx_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO ledger_transactions (tenant_id, tx_id, amount, currency) VALUES ($1, $2, $3, $4)")
        .bind(&tenant_id)
        .bind(&tx_id)
        .bind(payment_info.0)
        .bind(&payment_info.1)
        .execute(&mut *tx)
        .await;

    // ensure account exists for tenant
    let account_id = "default_revenue";
    let _ = sqlx::query("INSERT INTO ledger_accounts (tenant_id, account_id, currency, balance) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
        .bind(&tenant_id)
        .bind(account_id)
        .bind(&payment_info.1)
        .bind(0.0)
        .execute(&mut *tx)
        .await;

    let entry_id = Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO ledger_entries (tenant_id, entry_id, tx_id, account_id, direction, amount) VALUES ($1, $2, $3, $4, 'CREDIT', $5)")
        .bind(&tenant_id)
        .bind(&entry_id)
        .bind(&tx_id)
        .bind(account_id)
        .bind(payment_info.0)
        .execute(&mut *tx)
        .await;

    let _ = sqlx::query("UPDATE ledger_accounts SET balance = balance + $1 WHERE tenant_id = $2 AND account_id = $3")
        .bind(payment_info.0)
        .bind(&tenant_id)
        .bind(account_id)
        .execute(&mut *tx)
        .await;

    // Notify Finance Agent
    let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'payment_ledger', 'finance', 'payment_succeeded', $3, 'pending')")
        .bind(Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(serde_json::json!({
            "event": "payment_succeeded",
            "amount": payment_info.0,
            "currency": payment_info.1,
            "idempotency_key": idempotency_key
        }))
        .execute(&mut *tx)
        .await;

    let _ = tx.commit().await;

    StatusCode::OK.into_response()
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
