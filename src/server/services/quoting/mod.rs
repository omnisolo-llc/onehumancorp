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
    Router::new()
        .route("/", post(create_quote))
        .route("/{id}", get(get_quote))
        .route("/{id}/approve", patch(approve_quote))
        // We'll keep pricing rules as well, just fixed the brackets.
        .route("/pricing-rules", get(get_pricing_rules))
        .route("/pricing-rules", post(create_pricing_rule))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub id: String,
    #[serde(skip_serializing)]
    pub tenant_id: String,
    #[serde(skip_serializing)]
    pub customer_id: String,
    pub customer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_photo_url: Option<String>,
    pub request_text: String,
    pub status: String,
    pub items: Vec<QuoteItem>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuoteItem {
    pub id: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
    pub is_optional: bool,
    pub selected: bool,
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

    #[derive(FromRow)]
    #[allow(dead_code)]
    struct DbQuote {
        id: Uuid,
        tenant_id: String,
        customer_id: Uuid,
        status: String,
    }

    let _q = sqlx::query_as::<_, DbQuote>(
        "INSERT INTO quotes (id, tenant_id, customer_id, status) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, customer_id, status"
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

    // We don't have the full details for the response so we just re-fetch it to construct a complete object
    get_quote(State(pool), Path(quote_id)).await
}

async fn get_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    #[derive(FromRow)]
    #[allow(dead_code)]
    struct DbQuote {
        id: Uuid,
        tenant_id: String,
        customer_id: Uuid,
        status: String,
    }

    let q = sqlx::query_as::<_, DbQuote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(q) = q {
        #[derive(FromRow)]
        struct DbQuoteLineItem {
            id: Uuid,
            description: String,
            unit_price_cents: i64,
            quantity: i32,
            is_optional: bool,
        }

        let items_rows = sqlx::query_as::<_, DbQuoteLineItem>(
            "SELECT id, description, unit_price_cents, quantity, is_optional FROM quote_line_items WHERE quote_id = $1"
        )
        .bind(id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut items = Vec::new();
        for row in items_rows {
            items.push(QuoteItem {
                id: row.id.to_string(),
                description: row.description,
                price: (row.unit_price_cents as f64) / 100.0,
                quantity: row.quantity,
                is_optional: row.is_optional,
                selected: !row.is_optional, // Default required items to selected
            });
        }

        // Use ID for name and empty string for request_text to avoid fake data
        let customer_name = q.customer_id.to_string();
        let request_text = String::new();

        let quote = Quote {
            id: q.id.to_string(),
            tenant_id: q.tenant_id,
            customer_id: q.customer_id.to_string(),
            customer_name,
            customer_photo_url: None,
            request_text,
            status: q.status,
            items,
        };

        Ok(Json(quote))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin tx: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    #[derive(FromRow)]
    #[allow(dead_code)]
    struct DbQuote {
        id: Uuid,
        tenant_id: String,
        customer_id: Uuid,
        status: String,
    }

    let quote = sqlx::query_as::<_, DbQuote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING id, tenant_id, customer_id, status"
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update quote: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(ref q) = quote {
        #[derive(FromRow)]
        struct DbQuoteLineItem {
            description: String,
            unit_price_cents: i64,
            quantity: i32,
            is_optional: bool,
        }

        let line_items = sqlx::query_as::<_, DbQuoteLineItem>(
            "SELECT description, unit_price_cents, quantity, is_optional FROM quote_line_items WHERE quote_id = $1"
        )
        .bind(q.id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch quote line items: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let mut total_amount_cents = 0;
        for item in &line_items {
            if !item.is_optional {
                total_amount_cents += item.unit_price_cents * (item.quantity as i64);
            }
        }

        let invoice_id = Uuid::new_v4().to_string();
        let current_timestamp = Utc::now().timestamp();
        let due_date = current_timestamp + 30 * 24 * 3600;
        // Fetch actual customer details (we assume they exist or we fallback gracefully without fake data if possible,
        // but for now we'll just use the customer ID for the name if we don't have it, or query the customers table.
        // Since we don't have a clear customer table here, we'll try to find an opportunity or lead)
        let client_name = q.customer_id.to_string(); // Do not use fake data

        // due_date is BIGINT in invoices table
        sqlx::query(
            "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, to_timestamp($9::double precision), to_timestamp($10::double precision))"
        )
        .bind(&invoice_id)
        .bind(&q.tenant_id)
        .bind(q.customer_id.to_string())
        .bind(&client_name)
        .bind("draft")
        .bind(due_date)
        .bind("USD")
        .bind((total_amount_cents as f64) / 100.0)
        .bind(current_timestamp as f64)
        .bind(current_timestamp as f64)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert invoice: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

        for item in line_items {
            if !item.is_optional {
                sqlx::query(
                    "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, to_timestamp($8::double precision), to_timestamp($9::double precision))"
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&q.tenant_id)
                .bind(&invoice_id)
                .bind(item.description)
                .bind(item.quantity)
                .bind((item.unit_price_cents as f64) / 100.0)
                .bind(((item.unit_price_cents * item.quantity as i64) as f64) / 100.0)
                .bind(current_timestamp as f64)
                .bind(current_timestamp as f64)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert invoice line item: {}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })?;
            }
        }
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit tx: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if quote.is_some() {
        get_quote(State(pool), Path(id)).await
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}
