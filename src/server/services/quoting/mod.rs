use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use axum::{
    extract::{State, Path},
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::minimax::{LocalLLMClient, MinimaxClient};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes/:id", get(get_quote))
        .route("/quotes/:id/approve", patch(approve_quote))
        .route("/proposals/draft", post(draft_proposal))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Uuid,
    pub status: String,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuoteReq {
    pub customer_id: Uuid,
    pub status: String,
    pub line_items: Vec<QuoteLineItemReq>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteLineItemReq {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftProposalReq {
    pub customer_id: Uuid,
    pub request_text: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Uuid,
    pub status: String,
    pub total_amount_cents: i64,
    pub deposit_amount_cents: i64,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProposalLineItem {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
}

async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuoteReq>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    // Basic implementation
    let quote_id = Uuid::new_v4();
    let tenant_id = "test_tenant".to_string(); // In reality, get from context

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let quote = sqlx::query_as::<_, Quote>(
        "INSERT INTO quotes (id, tenant_id, customer_id, status) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(quote_id)
    .bind(&tenant_id)
    .bind(payload.customer_id)
    .bind(&payload.status)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    for item in payload.line_items {
        sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(Uuid::new_v4())
        .bind(quote_id)
        .bind(item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(quote))
}

async fn get_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn draft_proposal(
    State(pool): State<PgPool>,
    Json(payload): Json<DraftProposalReq>,
) -> Result<Json<Proposal>, axum::http::StatusCode> {
    let tenant_id = "test_tenant".to_string(); // In reality, get from context

    let prompt = format!(
        "Analyze the following client request and create a line-item proposal.\n\nRequest: {}\n\nRespond with a JSON array of objects, each containing keys: 'description' (string), 'unit_price_cents' (integer, price in cents), 'quantity' (integer), and 'is_optional' (boolean).",
        payload.request_text
    );

    let raw_response = match std::env::var("OHC_SALES_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.trim().is_empty() {
                LocalLLMClient::new().reason(&prompt).await
            } else {
                MinimaxClient::new(api_key).reason(&prompt).await
            }
        },
        _ => LocalLLMClient::new().reason(&prompt).await,
    }.unwrap_or_else(|_| "[]".to_string());

    let line_items: Vec<QuoteLineItemReq> = serde_json::from_str(&raw_response).unwrap_or_default();

    let mut total_cents = 0;
    for item in &line_items {
        if !item.is_optional {
            total_cents += item.unit_price_cents * item.quantity as i64;
        }
    }
    let deposit_cents = total_cents / 2;

    let proposal_id = Uuid::new_v4();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let proposal = sqlx::query_as::<_, Proposal>(
        "INSERT INTO proposals (id, tenant_id, customer_id, status, total_amount_cents, deposit_amount_cents) VALUES ($1, $2, $3, 'DRAFT', $4, $5) RETURNING *"
    )
    .bind(proposal_id)
    .bind(&tenant_id)
    .bind(payload.customer_id)
    .bind(total_cents)
    .bind(deposit_cents)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert proposal: {:?}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    for item in line_items {
        sqlx::query(
            "INSERT INTO proposal_line_items (id, proposal_id, description, unit_price_cents, quantity, is_optional) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(Uuid::new_v4())
        .bind(proposal_id)
        .bind(item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(proposal))
}

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Integrate Stripe deposit logic here...

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
