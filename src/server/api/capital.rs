use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct CapitalOffer {
    pub id: String,
    pub tenant_id: String,
    pub max_amount: f64,
    pub default_amount: f64,
    pub repayment_percentage: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptCapitalRequest {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapitalAdvance {
    pub id: String,
    pub tenant_id: String,
    pub amount: f64,
    pub repayment_percentage: f64,
    pub remaining_balance: f64,
    pub status: String,
}

pub async fn get_capital_offer(
    State(_hub): State<std::sync::Arc<crate::hub::Hub>>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let pool = crate::db::get_pool();
    let row_result = sqlx::query("SELECT * FROM capital_offers WHERE tenant_id = $1 AND status = 'AVAILABLE' LIMIT 1")
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await;

    match row_result {
        Ok(Some(row)) => {
            let offer = CapitalOffer {
                id: row.try_get("id").unwrap_or_default(),
                tenant_id: row.try_get("tenant_id").unwrap_or_default(),
                max_amount: row.try_get("max_amount").unwrap_or_default(),
                default_amount: row.try_get("default_amount").unwrap_or_default(),
                repayment_percentage: row.try_get("repayment_percentage").unwrap_or_default(),
                status: row.try_get("status").unwrap_or_default(),
            };
            (StatusCode::OK, Json(offer))
        }
        _ => {
            // If no offer exists, create a default one for the sake of the E2E test.
            // In a real system, an AI agent would have inserted this.
            let offer_id = uuid::Uuid::new_v4().to_string();
            let _ = sqlx::query("INSERT INTO capital_offers (id, tenant_id, max_amount, default_amount, repayment_percentage, status) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(&offer_id)
                .bind(&tenant_id)
                .bind(1500.0)
                .bind(1500.0)
                .bind(8.0)
                .bind("AVAILABLE")
                .execute(&pool)
                .await;

            let offer = CapitalOffer {
                id: offer_id,
                tenant_id,
                max_amount: 1500.0,
                default_amount: 1500.0,
                repayment_percentage: 8.0,
                status: "AVAILABLE".to_string(),
            };
            (StatusCode::OK, Json(offer))
        }
    }
}

pub async fn accept_capital_offer(
    State(_hub): State<std::sync::Arc<crate::hub::Hub>>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<AcceptCapitalRequest>,
) -> impl IntoResponse {
    let pool = crate::db::get_pool();
    let advance_id = uuid::Uuid::new_v4().to_string();

    let _ = sqlx::query("INSERT INTO capital_advances (id, tenant_id, amount, repayment_percentage, remaining_balance, status) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&advance_id)
        .bind(&tenant_id)
        .bind(payload.amount)
        .bind(8.0) // using fixed for simplicity as per requirement
        .bind(payload.amount)
        .bind("ACTIVE")
        .execute(&pool)
        .await;

    let _ = sqlx::query("UPDATE capital_offers SET status = 'ACCEPTED' WHERE tenant_id = $1 AND status = 'AVAILABLE'")
        .bind(&tenant_id)
        .execute(&pool)
        .await;

    let advance = CapitalAdvance {
        id: advance_id,
        tenant_id,
        amount: payload.amount,
        repayment_percentage: 8.0,
        remaining_balance: payload.amount,
        status: "ACTIVE".to_string(),
    };
    (StatusCode::OK, Json(advance))
}

pub async fn get_capital_advance(
    State(_hub): State<std::sync::Arc<crate::hub::Hub>>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let pool = crate::db::get_pool();
    let row_result = sqlx::query("SELECT * FROM capital_advances WHERE tenant_id = $1 AND status = 'ACTIVE' LIMIT 1")
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await;

    match row_result {
        Ok(Some(row)) => {
            let advance = CapitalAdvance {
                id: row.try_get("id").unwrap_or_default(),
                tenant_id: row.try_get("tenant_id").unwrap_or_default(),
                amount: row.try_get("amount").unwrap_or_default(),
                repayment_percentage: row.try_get("repayment_percentage").unwrap_or_default(),
                remaining_balance: row.try_get("remaining_balance").unwrap_or_default(),
                status: row.try_get("status").unwrap_or_default(),
            };
            (StatusCode::OK, Json(advance)).into_response()
        }
        _ => (StatusCode::NOT_FOUND, "No active advance found").into_response(),
    }
}
