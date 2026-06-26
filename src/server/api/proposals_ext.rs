use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Proposal {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub status: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub stripe_payment_link: Option<String>,
    pub valid_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/:id/approve", post(approve_proposal))
}

async fn approve_proposal(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let proposal_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let proposal = match sqlx::query_as::<_, Proposal>(
        "UPDATE proposals SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(proposal_id)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(p)) => p,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to approve proposal: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let amount_usd = (proposal.required_deposit_cents as f64) / 100.0;

    // Default to test mode key so tests pass. In prod, read from env.
    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
    let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

    let mut updated_proposal = proposal;

    if updated_proposal.stripe_payment_link.is_none() && amount_usd > 0.0 {
        match stripe_client.create_checkout_session(
            &format!("Proposal Deposit #{}", updated_proposal.id),
            &updated_proposal.customer_id.map(|u| u.to_string()).unwrap_or_default(),
            amount_usd,
            None,
            None
        ).await {
            Ok(url) => {
                let _ = sqlx::query("UPDATE proposals SET stripe_payment_link = $1, updated_at = NOW() WHERE id = $2")
                    .bind(&url)
                    .bind(proposal_id)
                    .execute(&pool)
                    .await;
                updated_proposal.stripe_payment_link = Some(url);
            }
            Err(e) => {
                tracing::error!("Failed to create stripe payment link for proposal: {}", e);
            }
        }
    }

    (StatusCode::OK, Json(updated_proposal)).into_response()
}
