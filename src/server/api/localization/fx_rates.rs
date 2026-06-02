use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FxRate {
    pub id: String,
    pub base_currency: String,
    pub target_currency: String,
    pub exchange_rate: f64,
}

#[derive(Deserialize)]
pub struct UpsertFxRateRequest {
    pub base_currency: String,
    pub target_currency: String,
    pub exchange_rate: f64,
}

#[derive(Deserialize)]
pub struct GetFxRatesQuery {
    pub base_currency: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .with_state(state)
        .route("/api/localization/fx", get(get_fx_rates))
        .route("/api/localization/fx", post(upsert_fx_rate))
}

async fn get_fx_rates(
    State(state): State<AppState>,
    Query(query): Query<GetFxRatesQuery>,
) -> impl IntoResponse {
    let rows = match sqlx::query!(
        "SELECT id, base_currency, target_currency, exchange_rate
         FROM offline_fx_rates
         WHERE base_currency = $1",
        query.base_currency
    )
    .fetch_all(&state.db.pool)
    .await {
        Ok(rows) => rows,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rates: Vec<FxRate> = rows.into_iter().map(|row| FxRate {
        id: row.id,
        base_currency: row.base_currency,
        target_currency: row.target_currency,
        exchange_rate: row.exchange_rate.to_string().parse::<f64>().unwrap_or(0.0),
    }).collect();

    (StatusCode::OK, Json(rates)).into_response()
}

async fn upsert_fx_rate(
    State(state): State<AppState>,
    Json(payload): Json<UpsertFxRateRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query!(
        "INSERT INTO offline_fx_rates (id, base_currency, target_currency, exchange_rate)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (base_currency, target_currency)
         DO UPDATE SET exchange_rate = EXCLUDED.exchange_rate, fetched_at = CURRENT_TIMESTAMP",
        id, payload.base_currency, payload.target_currency, sqlx::types::BigDecimal::try_from(payload.exchange_rate).unwrap_or_default()
    )
    .execute(&state.db.pool)
    .await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    StatusCode::OK.into_response()
}
