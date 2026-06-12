use axum::{
    extract::{State, Query},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/rules", get(get_pricing_rules).post(create_pricing_rule))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PricingRule {
    pub id: Uuid,
    pub tenant_id: String,
    pub service_category: String,
    pub rule_name: String,
    pub base_price_cents: i64,
    pub modifiers: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePricingRuleReq {
    pub service_category: String,
    pub rule_name: String,
    pub base_price_cents: i64,
    pub modifiers: serde_json::Value,
}

async fn get_pricing_rules(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<PricingRule>>, axum::http::StatusCode> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let _ = sqlx::query("SET app.current_tenant TO $1")
        .bind(&tenant_id)
        .execute(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let rules = sqlx::query_as::<_, PricingRule>(
        "SELECT * FROM pricing_rules WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rules))
}

async fn create_pricing_rule(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreatePricingRuleReq>,
) -> Result<Json<PricingRule>, axum::http::StatusCode> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let _ = sqlx::query("SET app.current_tenant TO $1")
        .bind(&tenant_id)
        .execute(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let rule = sqlx::query_as::<_, PricingRule>(
        "INSERT INTO pricing_rules (id, tenant_id, service_category, rule_name, base_price_cents, modifiers) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&tenant_id)
    .bind(payload.service_category)
    .bind(payload.rule_name)
    .bind(payload.base_price_cents)
    .bind(payload.modifiers)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rule))
}
