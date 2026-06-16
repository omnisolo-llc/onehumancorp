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
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub stripe_payment_link: Option<String>,
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

    let total_amount_cents = payload.line_items.iter().map(|li| li.unit_price_cents * li.quantity as i64).sum::<i64>();
    let required_deposit_cents = total_amount_cents / 3; // Default 33% deposit

    let quote = sqlx::query_as::<_, Quote>(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(quote_id)
    .bind(&tenant_id)
    .bind(payload.customer_id)
    .bind(&payload.status)
    .bind(total_amount_cents)
    .bind(required_deposit_cents)
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
    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Integrate Stripe deposit logic
    if let Some(mut q) = quote {
        if q.status == "ACCEPTED" {
            let amount_usd = (q.total_amount_cents as f64) / 100.0;
            // Use Stripe client to create a checkout session
            let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
            let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

            match stripe_client.create_checkout_session(
                &format!("Quote #{}", q.id),
                &q.customer_id.to_string(),
                amount_usd,
                false
            ).await {
                Ok(url) => {
                    q.stripe_payment_link = Some(url.clone());
                    let _ = sqlx::query("UPDATE quotes SET stripe_payment_link = $1 WHERE id = $2")
                        .bind(&url)
                        .bind(q.id)
                        .execute(&pool)
                        .await;
                },
                Err(e) => {
                    tracing::error!("Failed to create Stripe checkout session: {}", e);
                }
            }
        }
        Ok(Json(q))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_quote_struct_serialization() {
        let quote = Quote {
            id: Uuid::new_v4(),
            tenant_id: "test-tenant".to_string(),
            customer_id: Uuid::new_v4(),
            status: "DRAFT".to_string(),
            valid_until: None,
            total_amount_cents: 1000,
            required_deposit_cents: 333,
            stripe_payment_link: Some("http://stripe.com".to_string()),
        };
        let serialized = serde_json::to_string(&quote).unwrap();
        assert!(serialized.contains("total_amount_cents"));
    }
}
