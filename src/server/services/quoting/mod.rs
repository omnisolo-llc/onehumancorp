use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use axum::{
    extract::{State, Path},
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/v1/quotes", post(create_quote).get(list_quotes))
        .route("/v1/quotes/:id", get(get_quote))
        .route("/v1/quotes/:id/approve", patch(approve_quote))
        .route("/v1/pricing-rules", get(get_pricing_rules).post(create_pricing_rule))
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PricingRule {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub base_price_cents: i64,
    pub rules_json: sqlx::types::Json<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePricingRuleReq {
    pub name: String,
    pub base_price_cents: i64,
    pub rules_json: serde_json::Value,
}

async fn list_quotes(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Quote>>, axum::http::StatusCode> {
    let quotes = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(quotes))
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

async fn get_pricing_rules(
    State(pool): State<PgPool>,
    axum::extract::HeaderMap(headers): axum::extract::HeaderMap,
) -> Result<Json<Vec<PricingRule>>, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("test_tenant").to_string();
    let rules = sqlx::query_as::<_, PricingRule>("SELECT id, tenant_id, name, base_price_cents, rules_json as \"rules_json: sqlx::types::Json<serde_json::Value>\" FROM pricing_rules WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rules))
}

async fn create_pricing_rule(
    State(pool): State<PgPool>,
    axum::extract::HeaderMap(headers): axum::extract::HeaderMap,
    Json(payload): Json<CreatePricingRuleReq>,
) -> Result<Json<PricingRule>, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("test_tenant").to_string();
    let id = Uuid::new_v4();

    let rule = sqlx::query_as::<_, PricingRule>(
        "INSERT INTO pricing_rules (id, tenant_id, name, base_price_cents, rules_json) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, base_price_cents, rules_json as \"rules_json: sqlx::types::Json<serde_json::Value>\""
    )
    .bind(id)
    .bind(tenant_id)
    .bind(payload.name)
    .bind(payload.base_price_cents)
    .bind(sqlx::types::Json(payload.rules_json))
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rule))
}
