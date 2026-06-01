use axum::{extract::State, Json};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;
use serde::Serialize;
use sqlx::Row;

#[derive(Serialize)]
pub struct CapitalAdvanceOffer {
    pub id: String,
    pub amount_cents: i64,
    pub fee_cents: i64,
    pub total_repayment_cents: i64,
    pub repayment_percentage: f64,
    pub status: String,
}

#[derive(serde::Deserialize)]
pub struct AcceptAdvanceRequest {
    pub advance_id: String,
    pub amount_cents: i64,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/api/capital/offers", axum::routing::get(get_offers_handler))
        .route("/api/capital/accept", axum::routing::post(accept_offer_handler))
        .with_state(hub)
}

pub async fn get_offers_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<Vec<CapitalAdvanceOffer>> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(vec![])
    };

    let pool = &hub.pool;

    // Use ::FLOAT8 mapping directly in query
    let rows = sqlx::query(
        r#"
        SELECT id, amount_cents, fee_cents, total_repayment_cents, repayment_percentage::FLOAT8 as repayment_percentage, status
        FROM capital_advances
        WHERE tenant_id = $1 AND status = 'offered'
        "#
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let offers = rows.into_iter().map(|row| {
        let repayment_percentage: f64 = row.get("repayment_percentage");

        CapitalAdvanceOffer {
            id: row.get("id"),
            amount_cents: row.get("amount_cents"),
            fee_cents: row.get("fee_cents"),
            total_repayment_cents: row.get("total_repayment_cents"),
            repayment_percentage,
            status: row.get("status"),
        }
    }).collect();

    Json(offers)
}

pub async fn accept_offer_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Json<bool> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return Json(false)
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), usize::MAX).await.unwrap();
    let payload: AcceptAdvanceRequest = serde_json::from_slice(&body_bytes).unwrap();

    let pool = &hub.pool;

    // Scale fees properly. 10% fee means * 1.1 total.
    let fee_cents = (payload.amount_cents as f64 * 0.1) as i64;
    let total_repayment_cents = payload.amount_cents + fee_cents;

    let result = sqlx::query(
        r#"
        UPDATE capital_advances
        SET status = 'accepted', updated_at = CURRENT_TIMESTAMP, amount_cents = $1, fee_cents = $2, total_repayment_cents = $3
        WHERE id = $4 AND tenant_id = $5 AND status = 'offered'
        "#
    )
    .bind(payload.amount_cents)
    .bind(fee_cents)
    .bind(total_repayment_cents)
    .bind(payload.advance_id)
    .bind(tenant_id)
    .execute(pool)
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Json(true),
        _ => Json(false),
    }
}
