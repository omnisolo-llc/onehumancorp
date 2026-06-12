use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use axum::{
    extract::{State, Path},
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    axum::Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes/{id}", get(get_quote))
        .route("/quotes/{id}/approve", patch(approve_quote))
        .route("/pricing-rules", get(get_pricing_rules))
        .route("/pricing-rules", post(create_pricing_rule))
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
    pub rules_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePricingRuleReq {
    pub name: String,
    pub base_price_cents: i64,
    pub rules_json: serde_json::Value,
}

async fn get_pricing_rules(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
) -> Result<Json<Vec<PricingRule>>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id;

    let rules = sqlx::query_as::<_, PricingRule>(
        "SELECT id, tenant_id, name, base_price_cents, rules_json, created_at, updated_at FROM pricing_rules WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch pricing rules: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rules))
}

async fn create_pricing_rule(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
    Json(payload): Json<CreatePricingRuleReq>,
) -> Result<Json<PricingRule>, axum::http::StatusCode> {
    let rule_id = Uuid::new_v4();
    let tenant_id = claims.organization_id;

    let rule = sqlx::query_as::<_, PricingRule>(
        "INSERT INTO pricing_rules (id, tenant_id, name, base_price_cents, rules_json) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(rule_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(payload.base_price_cents)
    .bind(&payload.rules_json)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert pricing rule: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rule))
}

async fn create_quote(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
    Json(payload): Json<CreateQuoteReq>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote_id = Uuid::new_v4();
    let tenant_id = claims.organization_id;

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
    .map_err(|e| {
        tracing::error!("Failed to create quote: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

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
        .map_err(|e| {
            tracing::error!("Failed to create quote line item: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
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
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to accept quote: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let q = match quote {
        Some(q) => q,
        None => return Err(axum::http::StatusCode::NOT_FOUND),
    };

    // Agentic Quote-to-Cash Workflow:
    // Once a quote is accepted, we automatically generate an Invoice.
    let invoice_id = uuid::Uuid::new_v4().to_string();
    let stripe_payment_link = format!("https://checkout.stripe.com/pay/cs_test_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

    // Set tenant context for RLS
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&q.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to set RLS context: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get the quote line items to calculate total and migrate items
    #[derive(FromRow)]
    struct LocalQuoteLineItem {
        description: String,
        unit_price_cents: i64,
        quantity: i32,
    }

    let line_items = sqlx::query_as::<_, LocalQuoteLineItem>(
        "SELECT description, unit_price_cents, quantity FROM quote_line_items WHERE quote_id = $1"
    )
    .bind(id)
    .fetch_all(&mut *tx)
    .await
    .unwrap_or_default();

    let mut total_amount: f64 = 0.0;
    for item in &line_items {
        total_amount += (item.unit_price_cents as f64 / 100.0) * (item.quantity as f64);
        let item_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&item_id)
        .bind(&q.tenant_id)
        .bind(&invoice_id)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(item.unit_price_cents as f64 / 100.0)
        .bind((item.unit_price_cents as f64 / 100.0) * (item.quantity as f64))
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create invoice line item: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    let status = "open".to_string();
    let due_date = chrono::Utc::now().timestamp() + (30 * 24 * 3600); // Net 30

    sqlx::query(
        "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_payment_link)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(&invoice_id)
    .bind(&q.tenant_id)
    .bind(q.customer_id.to_string())
    .bind("Client") // Ideally we'd join and fetch the client name
    .bind(&status)
    .bind(due_date)
    .bind("USD")
    .bind(total_amount)
    .bind(&stripe_payment_link)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create invoice: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(q))
}
