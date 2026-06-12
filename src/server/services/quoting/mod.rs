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
        .route("/quotes", post(create_quote))
        .route("/quotes/:id", get(get_quote))
        .route("/quotes/:id/approve", patch(approve_quote))
        .route("/dynamic_pricing_rules", post(create_pricing_rules))
        .route("/dynamic_pricing_rules/:product_id", get(get_pricing_rules))
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
pub struct DynamicPricingRule {
    pub id: Uuid,
    pub tenant_id: String,
    pub product_id: String,
    pub base_price_cents: i64,
    pub rules: sqlx::types::Json<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDynamicPricingRuleReq {
    pub product_id: String,
    pub base_price_cents: i64,
    pub rules: serde_json::Value,
}

async fn create_pricing_rules(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateDynamicPricingRuleReq>,
) -> Result<Json<DynamicPricingRule>, axum::http::StatusCode> {
    let tenant_id = user.organization_id.unwrap_or_else(|| "default".to_string());

    // Set tenant context for RLS
    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let rule_id = Uuid::new_v4();

    let rule = sqlx::query_as::<_, DynamicPricingRule>(
        "INSERT INTO dynamic_pricing_rules (id, tenant_id, product_id, base_price_cents, rules) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(rule_id)
    .bind(&tenant_id)
    .bind(&payload.product_id)
    .bind(payload.base_price_cents)
    .bind(sqlx::types::Json(payload.rules))
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create pricing rule: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rule))
}

async fn get_pricing_rules(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(product_id): Path<String>,
) -> Result<Json<DynamicPricingRule>, axum::http::StatusCode> {
    let tenant_id = user.organization_id.unwrap_or_else(|| "default".to_string());

    // Set tenant context for RLS
    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let rule = sqlx::query_as::<_, DynamicPricingRule>(
        "SELECT * FROM dynamic_pricing_rules WHERE product_id = $1"
    )
    .bind(product_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get pricing rule: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match rule {
        Some(r) => Ok(Json(r)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
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
