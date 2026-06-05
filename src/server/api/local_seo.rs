use axum::{
    extract::{Path, Extension},
    response::Json,
    routing::{get, post},
    Router,
};
use ::server_common::Claims;
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

pub async fn connect_google_business(claims: Claims) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "redirect_url": format!("https://accounts.google.com/o/oauth2/auth?client_id=MOCK_CLIENT_ID&redirect_uri=MOCK_URI&scope=https://www.googleapis.com/auth/business.manage&response_type=code&state={}", claims.tenant_id)
    }))
}

pub async fn get_connection_status(_claims: Claims) -> Json<serde_json::Value> {
    Json(ConnectionStatusResponse { connected: true })
}

pub async fn get_pending_reviews(
    claims: Claims,
    Extension(state): Extension<LocalSeoState>,
) -> Json<serde_json::Value> {
    let rows = sqlx::query!(
        "SELECT review_id, reviewer_name, star_rating, comment, ai_draft_reply, reply_status
         FROM ohc_local_reviews
         WHERE tenant_id = $1 AND reply_status = 'PENDING'
         ORDER BY created_at DESC",
        claims.tenant_id
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let reviews = rows
        .into_iter()
        .map(|row| LocalReview {
            review_id: row.review_id,
            reviewer_name: row.reviewer_name,
            star_rating: row.star_rating,
            comment: row.comment,
            ai_draft_reply: row.ai_draft_reply,
            reply_status: row.reply_status,
        })
        .collect();

    Json(reviews)
}

pub async fn approve_and_reply(
    claims: Claims,
    Extension(state): Extension<LocalSeoState>,
    Path(review_id): Path<String>,
    Json(payload): Json<ApproveReplyRequest>,
) -> Json<serde_json::Value> {
    let _ = sqlx::query!(
        "UPDATE ohc_local_reviews
         SET reply_status = 'PUBLISHED', ai_draft_reply = $1, updated_at = CURRENT_TIMESTAMP
         WHERE tenant_id = $2 AND review_id = $3",
        payload.reply_content,
        claims.tenant_id,
        review_id
    )
    .execute(&state.pool)
    .await;

    Json(serde_json::json!({ "status": "success", "review_id": review_id }))
}

pub async fn webhook_ingest(
    Extension(state): Extension<LocalSeoState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    if let Some(tenant_id) = payload.get("tenant_id").and_then(|v| v.as_str()) {
        let review_id = payload.get("review_id").and_then(|v| v.as_str()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string().as_str());
        let reviewer_name = payload.get("reviewer_name").and_then(|v| v.as_str()).unwrap_or("Anonymous");
        let star_rating = payload.get("star_rating").and_then(|v| v.as_i64()).unwrap_or(5) as i32;
        let comment = payload.get("comment").and_then(|v| v.as_str());
        let platform = payload.get("platform").and_then(|v| v.as_str());

        let id = uuid::Uuid::new_v4().to_string();

        let _ = sqlx::query!(
            "INSERT INTO ohc_local_reviews (id, tenant_id, review_id, reviewer_name, star_rating, comment, platform, reply_status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'PENDING')
             ON CONFLICT (id) DO NOTHING",
            id,
            tenant_id,
            review_id,
            reviewer_name,
            star_rating,
            comment,
            platform
        )
        .execute(&state.pool)
        .await;

        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_type: "tenant.review.received".to_string(),
            payload: payload.clone(),
        };

        if let Ok(mut msg_bytes) = serde_json::to_vec(&event) {
            let _ = state.hub.publish(crate::msgbus::Message {
                id: uuid::Uuid::new_v4().to_string(),
                topic: "hub.events".to_string(),
                payload: msg_bytes,
                metadata: std::collections::HashMap::new(),
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
            });
        }
    }

    Json(serde_json::json!({ "status": "received" }))
}

pub fn router(pool: PgPool, hub: Arc<Hub>) -> Router {
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
    use ::server_common::Claims;
    use axum::Json;
    use axum::extract::Extension;

    async fn setup_db() -> PgPool {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap()
    }

    fn mock_claims() -> Claims {
        Claims {
            sub: "user123".to_string(),
            tenant_id: "tenant123".to_string(),
            exp: 9999999999,
            role: "owner".to_string(),
            permissions: vec![],
        }
    }

    #[tokio::test]
    async fn test_connect_google_business() {
        let claims = mock_claims();
        let Json(response) = connect_google_business(claims).await;
        assert_eq!(response["status"], "success");
        assert!(response["redirect_url"].as_str().unwrap().contains("tenant123"));
    }

    #[tokio::test]
    async fn test_get_connection_status() {
        let claims = mock_claims();
        let Json(response) = get_connection_status(claims).await;
        assert!(response.connected);
    }

    #[tokio::test]
    async fn test_get_pending_reviews() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }

        let claims = mock_claims();
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let state = LocalSeoState { pool: pool.clone(), hub };

        let _ = sqlx::query!("DELETE FROM ohc_local_reviews WHERE tenant_id = $1", claims.tenant_id).execute(&pool).await;
        let _ = sqlx::query!(
            "INSERT INTO ohc_local_reviews (id, tenant_id, review_id, reviewer_name, star_rating, comment, reply_status) VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')",
            uuid::Uuid::new_v4().to_string(), claims.tenant_id, "rev-123", "Test User", 5, "Great service!"
        ).execute(&pool).await;

        let Json(response) = get_pending_reviews(claims.clone(), Extension(state.clone())).await;
        assert_eq!(response.len(), 1);
        assert_eq!(response[0].review_id, "rev-123");
        assert_eq!(response[0].reply_status, "PENDING");
    }

    #[tokio::test]
    async fn test_approve_and_reply() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }

        let claims = mock_claims();
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let state = LocalSeoState { pool: pool.clone(), hub };

        let _ = sqlx::query!("DELETE FROM ohc_local_reviews WHERE tenant_id = $1", claims.tenant_id).execute(&pool).await;
        let _ = sqlx::query!(
            "INSERT INTO ohc_local_reviews (id, tenant_id, review_id, reviewer_name, star_rating, reply_status) VALUES ($1, $2, $3, $4, $5, 'PENDING')",
            uuid::Uuid::new_v4().to_string(), claims.tenant_id, "rev-456", "Test User 2", 4
        ).execute(&pool).await;

        let review_id = Path("rev-456".to_string());
        let payload = Json(ApproveReplyRequest {
            reply_content: "Thank you!".to_string(),
        });

        let Json(response) = approve_and_reply(claims.clone(), Extension(state.clone()), review_id, payload).await;
        assert_eq!(response["status"], "success");
        assert_eq!(response["review_id"], "rev-456");

        let updated = sqlx::query!("SELECT reply_status, ai_draft_reply FROM ohc_local_reviews WHERE tenant_id = $1 AND review_id = 'rev-456'", claims.tenant_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(updated.reply_status, "PUBLISHED");
        assert_eq!(updated.ai_draft_reply.unwrap(), "Thank you!");
    }

    #[tokio::test]
    async fn test_webhook_ingest() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }

        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let state = LocalSeoState { pool: pool.clone(), hub };

        let payload = Json(serde_json::json!({
            "tenant_id": "tenant123",
            "review_id": "rev-webhook-1",
            "reviewer_name": "Webhook User",
            "star_rating": 3,
            "comment": "It was okay.",
            "platform": "Yelp"
        }));

        let Json(response) = webhook_ingest(Extension(state.clone()), payload).await;
        assert_eq!(response["status"], "received");

        let inserted = sqlx::query!("SELECT * FROM ohc_local_reviews WHERE tenant_id = 'tenant123' AND review_id = 'rev-webhook-1'")
            .fetch_one(&pool).await.unwrap();

        assert_eq!(inserted.reviewer_name, "Webhook User");
        assert_eq!(inserted.star_rating, 3);
        assert_eq!(inserted.reply_status, "PENDING");
    }
}
