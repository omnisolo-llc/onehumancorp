use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use crate::domain::repository::models::{CapitalOffer, CapitalAdvance};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Serialize)]
pub struct CapitalOfferResponse {
    pub offer: CapitalOffer,
}

#[derive(Serialize)]
pub struct CapitalAdvanceResponse {
    pub advance: CapitalAdvance,
}

#[derive(Deserialize)]
pub struct AcceptOfferRequest {
    pub merchant_id: String,
}

pub fn routes(db: Arc<DB>) -> Router {
    let state = AppState { db };
    Router::new()
        .route("/api/v1/capital/offers/:merchant_id", get(get_offers))
        .route("/api/v1/capital/offers/:offer_id/accept", post(accept_offer))
        .with_state(state)
}

async fn get_offers(
    State(state): State<AppState>,
    Path(merchant_id): Path<String>,
) -> Result<Json<Vec<CapitalOffer>>, StatusCode> {
    let offers = sqlx::query_as::<_, CapitalOffer>(
        "SELECT * FROM capital_offers WHERE merchant_id = $1 AND status = 'active' AND (expires_at IS NULL OR expires_at > NOW())",
    )
    .bind(&merchant_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(offers))
}

async fn accept_offer(
    State(state): State<AppState>,
    Path(offer_id): Path<String>,
    Json(payload): Json<AcceptOfferRequest>,
) -> Result<Json<CapitalAdvanceResponse>, StatusCode> {
    // 1. Verify offer
    let offer = sqlx::query_as::<_, CapitalOffer>(
        "SELECT * FROM capital_offers WHERE id = $1 AND merchant_id = $2 AND status = 'active'",
    )
    .bind(&offer_id)
    .bind(&payload.merchant_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // 2. Create advance
    let advance_id = Uuid::new_v4().to_string();
    let total_owed = offer.amount + offer.flat_fee;

    let mut tx = state.db.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let advance = sqlx::query_as::<_, CapitalAdvance>(
        r#"
        INSERT INTO capital_advances (id, tenant_id, offer_id, total_owed, total_repaid, status)
        VALUES ($1, $2, $3, $4, 0, 'active')
        RETURNING *
        "#,
    )
    .bind(&advance_id)
    .bind(&offer.tenant_id)
    .bind(&offer.id)
    .bind(total_owed)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Mark offer as accepted
    sqlx::query(
        "UPDATE capital_offers SET status = 'accepted' WHERE id = $1",
    )
    .bind(&offer_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CapitalAdvanceResponse { advance }))
}
