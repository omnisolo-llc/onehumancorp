use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repository::models::{Proposal, ProposalLineItem};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/{id}", get(get_proposal))
        .route("/{id}/approve_and_send", post(approve_and_send_proposal))
}

#[derive(Serialize)]
pub struct ProposalResponse {
    pub proposal: Proposal,
    pub line_items: Vec<ProposalLineItem>,
}

#[derive(Deserialize)]
pub struct ApproveAndSendRequest {
    pub scope: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
}

async fn get_proposal(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let proposal_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let (proposal_res, items_res) = tokio::join!(
        sqlx::query_as::<_, Proposal>("SELECT * FROM proposals WHERE id = $1")
            .bind(proposal_id)
            .fetch_optional(&pool),
        sqlx::query_as::<_, ProposalLineItem>("SELECT * FROM proposal_line_items WHERE proposal_id = $1")
            .bind(proposal_id)
            .fetch_all(&pool)
    );

    let proposal = match proposal_res {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch proposal: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch proposal line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (StatusCode::OK, Json(ProposalResponse { proposal, line_items })).into_response()
}

async fn approve_and_send_proposal(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<ApproveAndSendRequest>,
) -> impl IntoResponse {
    let proposal_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    // Note: Stripe integration would happen here
    let stripe_payment_link = format!("https://buy.stripe.com/test_{}", Uuid::new_v4().to_string().replace("-", "").chars().take(12).collect::<String>());

    match sqlx::query(
        "UPDATE proposals SET status = 'SENT', scope = $1, total_amount_cents = $2, required_deposit_cents = $3, stripe_payment_link = $4, updated_at = NOW() WHERE id = $5"
    )
        .bind(&payload.scope)
        .bind(payload.total_amount_cents)
        .bind(payload.required_deposit_cents)
        .bind(&stripe_payment_link)
        .bind(proposal_id)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true, "stripe_payment_link": stripe_payment_link}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to approve and send proposal: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response()
        }
    }
}
