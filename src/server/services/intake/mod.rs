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
        .route("/webhooks/intake", post(handle_inquiry_webhook))
        .route("/quote-drafts", get(get_quote_drafts))
        .route("/quote-drafts/{id}/approve", patch(approve_quote_draft))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntakeWebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub identifier: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inquiry {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub source: String,
    pub raw_message: String,
    pub parsed_intent: Option<String>,
    pub urgency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuoteDraft {
    pub id: Uuid,
    pub tenant_id: String,
    pub inquiry_id: Uuid,
    pub suggested_service: String,
    pub estimated_amount_cents: i64,
    pub suggested_time_slot: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

async fn handle_inquiry_webhook(
    State(pool): State<PgPool>,
    Json(payload): Json<IntakeWebhookPayload>,
) -> Result<Json<QuoteDraft>, axum::http::StatusCode> {
    let inquiry_id = Uuid::new_v4();
    let tenant_id = payload.tenant_id.clone();

    // Call intent classification AI
    let prompt = format!(
        "Classify the following customer inquiry intent as one of [SERVICE_REQUEST, COMPLAINT, QUESTION, OTHER]. Return ONLY the classification string. Inquiry: '{}'",
        payload.message
    );
    let prompt = crate::pricing::compression::reduce_tokens(&prompt);

    let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
        Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        Ok("minimax") => {
            if let Ok(api_key) = std::env::var("MINIMAX_API_KEY") {
                crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
            } else {
                crate::minimax::LocalLLMClient::new().reason(&prompt).await
            }
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
    };

    let intent = llm_res.unwrap_or_else(|_| "SERVICE_REQUEST".to_string());
    let intent = intent.trim().to_uppercase();
    let safe_intent = if ["SERVICE_REQUEST", "COMPLAINT", "QUESTION"].contains(&intent.as_str()) {
        intent
    } else {
        "OTHER".to_string()
    };

    // Step 1: Create the inquiry
    let _inquiry = sqlx::query_as::<_, Inquiry>(
        "INSERT INTO inquiries (id, tenant_id, source, raw_message, parsed_intent, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(inquiry_id)
    .bind(&tenant_id)
    .bind(&payload.source)
    .bind(&payload.message)
    .bind(&safe_intent)
    .bind("QUOTED")
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create inquiry: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 2: Sales Agent & Ops Agent Draft the Quote
    let draft_id = Uuid::new_v4();

    // Simulate AI determining quote details
    let suggested_service = "Basic Repair Service".to_string();
    let estimated_amount_cents = 15000; // $150
    let suggested_time_slot = Utc::now() + chrono::Duration::days(2);

    let draft = sqlx::query_as::<_, QuoteDraft>(
        "INSERT INTO quote_drafts (id, tenant_id, inquiry_id, suggested_service, estimated_amount_cents, suggested_time_slot, status) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(draft_id)
    .bind(&tenant_id)
    .bind(inquiry_id)
    .bind(&suggested_service)
    .bind(estimated_amount_cents)
    .bind(suggested_time_slot)
    .bind("PENDING_APPROVAL")
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create quote draft: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(draft))
}

async fn get_quote_drafts(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<server_common::Claims>,
) -> Result<Json<Vec<QuoteDraft>>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id;

    let drafts = sqlx::query_as::<_, QuoteDraft>(
        "SELECT * FROM quote_drafts WHERE tenant_id = $1 AND status = 'PENDING_APPROVAL' ORDER BY created_at DESC"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch quote drafts: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(drafts))
}

async fn approve_quote_draft(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    axum::extract::Extension(claims): axum::extract::Extension<server_common::Claims>,
) -> Result<Json<QuoteDraft>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id;

    let draft = sqlx::query_as::<_, QuoteDraft>(
        "UPDATE quote_drafts SET status = 'APPROVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2 RETURNING *"
    )
    .bind(id)
    .bind(&tenant_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to approve quote draft: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match draft {
        Some(d) => Ok(Json(d)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_draft_serialization() {
        let draft = QuoteDraft {
            id: Uuid::new_v4(),
            tenant_id: "test-tenant".to_string(),
            inquiry_id: Uuid::new_v4(),
            suggested_service: "Plumbing Repair".to_string(),
            estimated_amount_cents: 20000,
            suggested_time_slot: Some(Utc::now()),
            status: "PENDING_APPROVAL".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let serialized = serde_json::to_string(&draft).unwrap();
        assert!(serialized.contains("Plumbing Repair"));
        assert!(serialized.contains("estimated_amount_cents"));
    }
}
