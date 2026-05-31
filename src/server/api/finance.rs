use axum::{
    http::Request,
    body::Body,
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Clone)]
pub struct FinanceAppState {
    pub db_pool: PgPool,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CashflowForecast {
    pub forecast_id: String,
    pub tenant_id: String,
    pub target_date: NaiveDate,
    pub expected_inflow: f64,
    pub expected_outflow: f64,
    pub net_position: f64,
    pub risk_level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CapitalOffer {
    pub offer_id: String,
    pub tenant_id: String,
    pub forecast_id: Option<String>,
    pub amount: f64,
    pub fee_percentage: f64,
    pub repayment_rate: f64,
    pub status: String,
}

pub fn finance_routes(pool: PgPool) -> Router {
    let state = FinanceAppState { db_pool: pool };
    Router::new()
        .route("/api/v1/finance/forecasts", get(get_forecasts))
        .route("/api/v1/finance/offers", get(get_offers))
        .route("/api/v1/finance/offers/:offer_id/accept", post(accept_offer))
        .with_state(state)
}

async fn get_forecasts(State(state): State<FinanceAppState>, request: Request<Body>) -> Json<Vec<CashflowForecast>> {
    let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>();
    let tenant_id_str = match auth_info {
        Some(info) => info.org_id.clone(),
        None => "default-tenant".to_string(),
    };
    let tenant_id = tenant_id_str.as_str();
    let _ = sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(tenant_id)
        .execute(&state.db_pool)
        .await;

    let forecasts = sqlx::query_as::<_, CashflowForecast>(
        "
        SELECT forecast_id, tenant_id, target_date, expected_inflow, expected_outflow, net_position, risk_level
        FROM cashflow_forecasts
        WHERE tenant_id = $1
        "
    ).bind(tenant_id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    Json(forecasts)
}

async fn get_offers(State(state): State<FinanceAppState>, request: Request<Body>) -> Json<Vec<CapitalOffer>> {
    let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>();
    let tenant_id_str = match auth_info {
        Some(info) => info.org_id.clone(),
        None => "default-tenant".to_string(),
    };
    let tenant_id = tenant_id_str.as_str();
    let _ = sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(tenant_id)
        .execute(&state.db_pool)
        .await;

    let offers = sqlx::query_as::<_, CapitalOffer>(
        "
        SELECT offer_id, tenant_id, forecast_id, amount, fee_percentage, repayment_rate, status
        FROM capital_offers
        WHERE tenant_id = $1
        "
    ).bind(tenant_id)
    .fetch_all(&state.db_pool)
    .await
    .unwrap_or_default();

    Json(offers)
}

#[derive(Deserialize)]
pub struct AcceptOfferRequest {}

async fn accept_offer(
    State(state): State<FinanceAppState>,
    Path(offer_id): Path<String>,
    request: Request<Body>,
) -> Json<serde_json::Value> {
    let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>();
    let tenant_id_str = match auth_info {
        Some(info) => info.org_id.clone(),
        None => "default-tenant".to_string(),
    };
    let tenant_id = tenant_id_str.as_str();
    let _ = sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(tenant_id)
        .execute(&state.db_pool)
        .await;

    let update_result = sqlx::query(
        "
        UPDATE capital_offers
        SET status = 'ACCEPTED', updated_at = CURRENT_TIMESTAMP
        WHERE offer_id = $1 AND tenant_id = $2 AND status = 'PENDING'
        RETURNING amount
        "
    ).bind(offer_id).bind(tenant_id)
    .fetch_optional(&state.db_pool)
    .await;

    match update_result {
        Ok(Some(row)) => {
            Json(serde_json::json!({
                "status": "success",
                "message": "Offer accepted",
                "credited_amount": row.get::<f64, _>("amount")
            }))
        }
        _ => {
            Json(serde_json::json!({
                "status": "error",
                "message": "Offer not found or already accepted"
            }))
        }
    }
}
