use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use axum::http::HeaderMap;

fn get_tenant_id(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get("x-spiffe-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|val| ::server_auth::parse_spiffe_id(val).ok())
        .map(|(t, _)| t)
        .or_else(|| headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).map(|s| s.to_string()))
}


pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/rules", get(get_rules).post(upsert_rules))
        .route("/rules/:service_id", get(get_rules_for_service))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DynamicPricingRule {
    pub id: Uuid,
    pub tenant_id: String,
    pub service_id: String,
    pub base_price_cents: i64,
    pub rules_json: sqlx::types::JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertRulesReq {
    pub service_id: String,
    pub base_price_cents: i64,
    pub rules_json: serde_json::Value,
}

async fn get_rules(headers: HeaderMap, State(pool): State<PgPool>) -> Result<Json<Vec<DynamicPricingRule>>, axum::http::StatusCode> {
    let tenant_id = match get_tenant_id(&headers) {
        Some(t) => t,
        None => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    let rules = sqlx::query_as::<_, DynamicPricingRule>(
        "SELECT * FROM dynamic_pricing_rules WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get dynamic pricing rules: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rules))
}

async fn get_rules_for_service(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Path(service_id): Path<String>,
) -> Result<Json<DynamicPricingRule>, axum::http::StatusCode> {
    let tenant_id = match get_tenant_id(&headers) {
        Some(t) => t,
        None => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    let rule = sqlx::query_as::<_, DynamicPricingRule>(
        "SELECT * FROM dynamic_pricing_rules WHERE tenant_id = $1 AND service_id = $2"
    )
    .bind(tenant_id)
    .bind(service_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get dynamic pricing rule: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match rule {
        Some(r) => Ok(Json(r)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn upsert_rules(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Json(payload): Json<UpsertRulesReq>,
) -> Result<Json<DynamicPricingRule>, axum::http::StatusCode> {
    let tenant_id = match get_tenant_id(&headers) {
        Some(t) => t,
        None => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    let existing = sqlx::query_as::<_, DynamicPricingRule>(
        "SELECT * FROM dynamic_pricing_rules WHERE tenant_id = $1 AND service_id = $2"
    )
    .bind(&tenant_id)
    .bind(&payload.service_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check existing dynamic pricing rule: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let rule = if let Some(e) = existing {
        sqlx::query_as::<_, DynamicPricingRule>(
            "UPDATE dynamic_pricing_rules SET base_price_cents = $1, rules_json = $2, updated_at = NOW() WHERE id = $3 RETURNING *"
        )
        .bind(payload.base_price_cents)
        .bind(payload.rules_json)
        .bind(e.id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update dynamic pricing rule: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        sqlx::query_as::<_, DynamicPricingRule>(
            "INSERT INTO dynamic_pricing_rules (id, tenant_id, service_id, base_price_cents, rules_json) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(Uuid::new_v4())
        .bind(&tenant_id)
        .bind(&payload.service_id)
        .bind(payload.base_price_cents)
        .bind(payload.rules_json)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert dynamic pricing rule: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
    };

    Ok(Json(rule))
}
