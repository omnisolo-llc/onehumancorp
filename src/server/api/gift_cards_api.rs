use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use crate::db::{get_pool};
use ::server_common::auth_utils::get_org_context;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GiftCard {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub code: String,
    pub type_: String,
    pub initial_balance: f64,
    pub current_balance: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct IssueGiftCardRequest {
    pub customer_id: Option<String>,
    pub amount: f64,
    pub type_: String,
}

#[derive(Serialize)]
pub struct IssueGiftCardResponse {
    pub gift_card: GiftCard,
}

#[derive(Deserialize)]
pub struct RedeemGiftCardRequest {
    pub code: String,
    pub amount: f64,
    pub transaction_ref: Option<String>,
}

#[derive(Serialize)]
pub struct RedeemGiftCardResponse {
    pub new_balance: f64,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balance: f64,
    pub status: String,
}


pub async fn list_gift_cards(
    headers: HeaderMap,
) -> Result<Json<Vec<GiftCard>>, axum::http::StatusCode> {
    let tenant_id = get_org_context(&headers).unwrap_or_else(|| "default".to_string());
    let pool = get_pool();


    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query(format!("SET LOCAL app.current_tenant = '{}'", tenant_id).as_str()).execute(&mut *tx).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let rows = sqlx::query!(
        r#"
        SELECT id, tenant_id, customer_id, code, type as type_, initial_balance, current_balance, status, created_at, updated_at
        FROM gift_cards
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#,
        tenant_id
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch gift cards: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let cards = rows.into_iter().map(|r| GiftCard {
        id: r.id,
        tenant_id: r.tenant_id,
        customer_id: r.customer_id,
        code: r.code,
        type_: r.type_,
        initial_balance: {
            use sqlx::types::BigDecimal;
            r.initial_balance.to_string().parse::<f64>().unwrap_or(0.0)
        },
        current_balance: {
            use sqlx::types::BigDecimal;
            r.current_balance.to_string().parse::<f64>().unwrap_or(0.0)
        },
        status: r.status,
        created_at: r.created_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
        updated_at: r.updated_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
    }).collect();

    Ok(Json(cards))
}

pub async fn issue_gift_card(
    headers: HeaderMap,
    Json(payload): Json<IssueGiftCardRequest>,
) -> Result<Json<IssueGiftCardResponse>, axum::http::StatusCode> {
    let tenant_id = get_org_context(&headers).unwrap_or_else(|| "default".to_string());
    let pool = get_pool();
    let id = uuid::Uuid::new_v4().to_string();
    let code = format!("GC-{}", uuid::Uuid::new_v4().to_string().replace("-", "").to_uppercase()[..8].to_string());
    let type_ = if payload.type_ == "STORE_CREDIT" { "STORE_CREDIT" } else { "GIFT_CARD" };
    let status = "ACTIVE";

    if payload.amount <= 0.0 {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let amount_decimal: sqlx::types::BigDecimal = match payload.amount.to_string().parse() {
        Ok(d) => d,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query(format!("SET LOCAL app.current_tenant = '{}'", tenant_id).as_str()).execute(&mut *tx).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        r#"
        INSERT INTO gift_cards (id, tenant_id, customer_id, code, type, initial_balance, current_balance, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        id, tenant_id, payload.customer_id, code, type_, amount_decimal.clone(), amount_decimal.clone(), status
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to issue gift card: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let ledger_id = uuid::Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO gift_card_ledger_entries (id, tenant_id, gift_card_id, amount, transaction_ref)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        ledger_id, tenant_id, id, amount_decimal, "INITIAL_ISSUANCE"
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert ledger entry: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let gc = GiftCard {
        id,
        tenant_id,
        customer_id: payload.customer_id,
        code,
        type_: type_.to_string(),
        initial_balance: payload.amount,
        current_balance: payload.amount,
        status: status.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(IssueGiftCardResponse { gift_card: gc }))
}

pub async fn check_balance(
    Path(code): Path<String>,
    headers: HeaderMap,
) -> Result<Json<BalanceResponse>, axum::http::StatusCode> {
    let tenant_id = get_org_context(&headers).unwrap_or_else(|| "default".to_string());
    let pool = get_pool();


    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query(format!("SET LOCAL app.current_tenant = '{}'", tenant_id).as_str()).execute(&mut *tx).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let row = sqlx::query!(
        r#"
        SELECT current_balance, status
        FROM gift_cards
        WHERE code = $1 AND tenant_id = $2
        "#,
        code, tenant_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check balance: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match row {
        Some(r) => {
            let balance = r.current_balance.to_string().parse::<f64>().unwrap_or(0.0);
            Ok(Json(BalanceResponse { balance, status: r.status }))
        },
        None => Err(axum::http::StatusCode::NOT_FOUND)
    }
}

pub async fn redeem_gift_card(
    headers: HeaderMap,
    Json(payload): Json<RedeemGiftCardRequest>,
) -> Result<Json<RedeemGiftCardResponse>, axum::http::StatusCode> {
    let tenant_id = get_org_context(&headers).unwrap_or_else(|| "default".to_string());
    let pool = get_pool();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query(format!("SET LOCAL app.current_tenant = '{}'", tenant_id).as_str()).execute(&mut *tx).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Lock the row for update
    let gc = sqlx::query!(
        r#"
        SELECT id, current_balance, status
        FROM gift_cards
        WHERE code = $1 AND tenant_id = $2
        FOR UPDATE
        "#,
        payload.code, tenant_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch gift card for redemption: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let gc = match gc {
        Some(g) => g,
        None => return Err(axum::http::StatusCode::NOT_FOUND),
    };

    if gc.status != "ACTIVE" {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    if payload.amount <= 0.0 {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let current_balance = gc.current_balance.to_string().parse::<f64>().unwrap_or(0.0);
    if current_balance < payload.amount {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let new_balance = current_balance - payload.amount;
    let new_status = if new_balance <= 0.001 { "EXHAUSTED" } else { "ACTIVE" };

    let new_balance_decimal: sqlx::types::BigDecimal = match new_balance.to_string().parse() {
        Ok(d) => d,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    sqlx::query!(
        r#"
        UPDATE gift_cards
        SET current_balance = $1, status = $2, updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        "#,
        new_balance_decimal.clone(), new_status, gc.id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update balance: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let ledger_id = uuid::Uuid::new_v4().to_string();
    let redemption_amount = -payload.amount;
    let redemption_amount_decimal: sqlx::types::BigDecimal = match redemption_amount.to_string().parse() {
        Ok(d) => d,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    sqlx::query!(
        r#"
        INSERT INTO gift_card_ledger_entries (id, tenant_id, gift_card_id, amount, transaction_ref)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        ledger_id, tenant_id, gc.id, redemption_amount_decimal, payload.transaction_ref
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert ledger entry: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RedeemGiftCardResponse { new_balance }))
}

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(list_gift_cards).post(issue_gift_card))
        .route("/:code/balance", axum::routing::get(check_balance))
        .route("/redeem", axum::routing::post(redeem_gift_card))
}
