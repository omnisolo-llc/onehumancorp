use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::Row;

// We'll wrap our DbPool in a State structure
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub reputation_worker: Arc<crate::workers::reputation_worker::ReputationWorker>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/reputation/:tenant_id/reviews", get(list_reviews))
        .route("/api/reputation/:tenant_id/settings", get(get_settings).put(update_settings))
        .route("/api/reputation/webhook/sms-reply", post(handle_sms_reply))
        .with_state(state)
}

#[derive(Serialize)]
pub struct ReputationReview {
    pub id: String,
    pub customer_id: String,
    pub booking_id: Option<String>,
    pub rating: Option<i32>,
    pub feedback_text: Option<String>,
    pub sentiment: Option<String>,
    pub status: String,
}

async fn list_reviews(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> axum::response::Result<Json<Vec<ReputationReview>>, axum::http::StatusCode> {
    let pool = &state.db;
    let records: Vec<sqlx::postgres::PgRow> = sqlx::query(
        r#"
        SELECT id, customer_id, booking_id, rating, feedback_text, sentiment, status
        FROM reputation_reviews
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let reviews = records.into_iter().map(|row| ReputationReview {
        id: row.get("id"),
        customer_id: row.get("customer_id"),
        booking_id: row.get("booking_id"),
        rating: row.get("rating"),
        feedback_text: row.get("feedback_text"),
        sentiment: row.get("sentiment"),
        status: row.get("status"),
    }).collect();

    Ok(Json(reviews))
}

#[derive(Serialize, Deserialize)]
pub struct ReputationSettings {
    pub auto_request_enabled: bool,
    pub delay_hours: i32,
    pub google_review_link: Option<String>,
    pub yelp_review_link: Option<String>,
}

async fn get_settings(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> axum::response::Result<Json<ReputationSettings>, axum::http::StatusCode> {
    let pool = &state.db;
    let record: Option<sqlx::postgres::PgRow> = sqlx::query(
        r#"
        SELECT auto_request_enabled, delay_hours, google_review_link, yelp_review_link
        FROM reputation_settings
        WHERE tenant_id = $1
        "#
    )
    .bind(&tenant_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(row) = record {
        Ok(Json(ReputationSettings {
            auto_request_enabled: row.get("auto_request_enabled"),
            delay_hours: row.get("delay_hours"),
            google_review_link: row.get("google_review_link"),
            yelp_review_link: row.get("yelp_review_link"),
        }))
    } else {
        // Default
        Ok(Json(ReputationSettings {
            auto_request_enabled: true,
            delay_hours: 2,
            google_review_link: None,
            yelp_review_link: None,
        }))
    }
}

async fn update_settings(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<ReputationSettings>,
) -> axum::response::Result<Json<ReputationSettings>, axum::http::StatusCode> {
    let pool = &state.db;
    let id = format!("set_{}", uuid::Uuid::new_v4());

    let _: sqlx::postgres::PgQueryResult = sqlx::query(
        r#"
        INSERT INTO reputation_settings (id, tenant_id, auto_request_enabled, delay_hours, google_review_link, yelp_review_link)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (tenant_id) DO UPDATE SET
            auto_request_enabled = EXCLUDED.auto_request_enabled,
            delay_hours = EXCLUDED.delay_hours,
            google_review_link = EXCLUDED.google_review_link,
            yelp_review_link = EXCLUDED.yelp_review_link,
            updated_at = NOW()
        "#
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(payload.auto_request_enabled)
    .bind(payload.delay_hours)
    .bind(&payload.google_review_link)
    .bind(&payload.yelp_review_link)
    .execute(pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(payload))
}

#[derive(Deserialize)]
pub struct SmsReplyPayload {
    pub tenant_id: String,
    pub customer_id: String,
    pub reply_text: String,
}

async fn handle_sms_reply(
    State(state): State<AppState>,
    Json(payload): Json<SmsReplyPayload>,
) -> axum::response::Result<axum::http::StatusCode, axum::http::StatusCode> {
    state.reputation_worker
        .handle_customer_reply(&payload.tenant_id, &payload.customer_id, &payload.reply_text)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(axum::http::StatusCode::OK)
}
