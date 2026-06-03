use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

use crate::services::capital::gift_card_ledger::{GiftCard, GiftCardLedger, GiftCardLedgerEntry};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

#[derive(Deserialize)]
pub struct IssueGiftCardRequest {
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub code: String,
    pub card_type: String, // 'GIFT_CARD' or 'STORE_CREDIT'
    pub initial_amount: i64,
    pub transaction_ref: Option<String>,
    #[serde(default)]
    pub is_offline_sync: bool,
}

#[derive(Deserialize)]
pub struct RedeemGiftCardRequest {
    pub tenant_id: String,
    pub amount: i64, // Positive value representing amount to deduct
    pub transaction_ref: Option<String>,
    #[serde(default)]
    pub is_offline_sync: bool,
}

#[derive(Deserialize)]
pub struct GetBalanceQuery {
    pub tenant_id: String,
}

pub fn router(pool: Arc<PgPool>) -> Router {
    let state = AppState { db: pool };
    Router::new()
        .route("/issue", post(issue_gift_card))
        .route("/:code/redeem", post(redeem_gift_card))
        .route("/:code/balance", get(get_gift_card_balance))
        .with_state(state)
}

async fn issue_gift_card(
    State(state): State<AppState>,
    Json(payload): Json<IssueGiftCardRequest>,
) -> impl IntoResponse {
    let ledger = GiftCardLedger::new(state.db);

    match ledger
        .issue_card(
            &payload.tenant_id,
            payload.customer_id,
            &payload.code,
            &payload.card_type,
            payload.initial_amount,
            payload.transaction_ref,
            payload.is_offline_sync,
        )
        .await
    {
        Ok(card) => (StatusCode::CREATED, Json(card)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}

async fn redeem_gift_card(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Json(payload): Json<RedeemGiftCardRequest>,
) -> impl IntoResponse {
    let ledger = GiftCardLedger::new(state.db);

    // Amount to deduct, must pass negative to apply_transaction
    let deduction = if payload.amount > 0 {
        -payload.amount
    } else {
        payload.amount
    };

    match ledger
        .apply_transaction(
            &payload.tenant_id,
            &code,
            deduction,
            payload.transaction_ref,
            payload.is_offline_sync,
        )
        .await
    {
        Ok(entry) => (StatusCode::OK, Json(entry)).into_response(),
        Err(e) => {
            let status = if e == "Insufficient balance" {
                StatusCode::BAD_REQUEST
            } else if e.starts_with("Gift card with code") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(serde_json::json!({ "error": e }))).into_response()
        }
    }
}

async fn get_gift_card_balance(
    State(state): State<AppState>,
    Path(code): Path<String>,
    axum::extract::Query(query): axum::extract::Query<GetBalanceQuery>,
) -> impl IntoResponse {
    let ledger = GiftCardLedger::new(state.db);

    match ledger.get_card_by_code(&query.tenant_id, &code).await {
        Ok(Some(card)) => (StatusCode::OK, Json(card)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Gift card not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}
