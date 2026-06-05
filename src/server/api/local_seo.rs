use sqlx::Row;
use axum::{
    extract::{Path, Extension},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::hub::Hub;

#[derive(Serialize, Deserialize, Clone)]
pub struct LocalReview {
    pub review_id: String,
    pub reviewer_name: String,
    pub star_rating: i32,
    pub comment: Option<String>,
    pub ai_draft_reply: Option<String>,
    pub reply_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApproveReplyRequest {
    pub reply_content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionStatusResponse {
    pub connected: bool,
}

#[derive(Clone)]
pub struct LocalSeoState {
    pub pool: PgPool,
    pub hub: Arc<Hub>,
}

pub async fn connect_google_business() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "redirect_url": format!("https://accounts.google.com/o/oauth2/auth?client_id=MOCK_CLIENT_ID&redirect_uri=MOCK_URI&scope=https://www.googleapis.com/auth/business.manage&response_type=code&state={}", "tenant123")
    }))
}

pub async fn get_connection_status() -> Json<ConnectionStatusResponse> {
    Json(ConnectionStatusResponse { connected: true })
}

pub async fn get_pending_reviews(
    Extension(state): Extension<LocalSeoState>,
) -> Json<Vec<LocalReview>> {
    let rows = sqlx::query(
        "SELECT review_id, reviewer_name, star_rating, comment, ai_draft_reply, reply_status
         FROM ohc_local_reviews
         WHERE tenant_id = $1 AND reply_status = 'PENDING'
         ORDER BY created_at DESC"
    )
    .bind("tenant123")
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let reviews = rows
        .into_iter()
        .map(|row| LocalReview {
            review_id: row.try_get("review_id").unwrap_or_default(),
            reviewer_name: row.try_get("reviewer_name").unwrap_or_default(),
            star_rating: row.try_get("star_rating").unwrap_or_default(),
            comment: row.try_get("comment").ok(),
            ai_draft_reply: row.try_get("ai_draft_reply").ok(),
            reply_status: row.try_get("reply_status").unwrap_or_default(),
        })
        .collect();

    Json(reviews)
}

pub async fn approve_and_reply(
    Path(review_id): Path<String>,
    Extension(state): Extension<LocalSeoState>,
    Json(payload): Json<ApproveReplyRequest>,
) -> Json<serde_json::Value> {
    let _ = sqlx::query(
        "UPDATE ohc_local_reviews
         SET reply_status = 'PUBLISHED', ai_draft_reply = $1, updated_at = CURRENT_TIMESTAMP
         WHERE tenant_id = $2 AND review_id = $3"
    )
    .bind(payload.reply_content)
    .bind("tenant123")
    .bind(&review_id)
    .execute(&state.pool)
    .await;

    Json(serde_json::json!({ "status": "success", "review_id": review_id }))
}

pub async fn webhook_ingest(
    Extension(state): Extension<LocalSeoState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    if let Some(tenant_id) = payload.get("tenant_id").and_then(|v| v.as_str()) {
        let generated_uuid = uuid::Uuid::new_v4().to_string();
        let review_id = payload.get("review_id").and_then(|v| v.as_str()).unwrap_or(&generated_uuid);
        let reviewer_name = payload.get("reviewer_name").and_then(|v| v.as_str()).unwrap_or("Anonymous");
        let star_rating = payload.get("star_rating").and_then(|v| v.as_i64()).unwrap_or(5) as i32;
        let comment = payload.get("comment").and_then(|v| v.as_str());
        let platform = payload.get("platform").and_then(|v| v.as_str());

        let id = uuid::Uuid::new_v4().to_string();

        let _ = sqlx::query(
            "INSERT INTO ohc_local_reviews (id, tenant_id, review_id, reviewer_name, star_rating, comment, platform, reply_status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'PENDING')
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(&review_id)
        .bind(reviewer_name)
        .bind(star_rating)
        .bind(comment)
        .bind(platform)
        .execute(&state.pool)
        .await;

        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_type: "tenant.review.received".to_string(),
            payload: payload.clone(),
        };

        if let Ok(msg_bytes) = serde_json::to_vec(&event) {
                        let _ = state.hub.publish(::server_ohc::orchestration::Message {
                id: uuid::Uuid::new_v4().to_string(),
                from_agent: "system".to_string(),
                to_agent: "publicist".to_string(),
                r#type: "event".to_string(),
                content: String::from_utf8_lossy(&msg_bytes).to_string(),
                meeting_id: "".to_string(),
                occurred_at_unix: chrono::Utc::now().timestamp_millis(),
            });
        }
    }

    Json(serde_json::json!({ "status": "received" }))
}

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = LocalSeoState { pool, hub };
    Router::new()
        .route("/connect", post(connect_google_business))
        .route("/status", get(get_connection_status))
        .route("/reviews/pending", get(get_pending_reviews))
        .route("/reviews/:review_id/approve", post(approve_and_reply))
        .route("/webhook", post(webhook_ingest))
        .layer(axum::extract::Extension(state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::extract::Extension;

    async fn setup_db() -> PgPool {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_connect_google_business() {
        let Json(response) = connect_google_business().await;
        assert_eq!(response["status"], "success");
        assert!(response["redirect_url"].as_str().unwrap().contains("tenant123"));
    }

    #[tokio::test]
    async fn test_get_connection_status() {
        let Json(response) = get_connection_status().await;
        assert!(response.connected);
    }
}
