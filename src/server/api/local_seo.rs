use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use ::server_common::Claims;
use serde::{Deserialize, Serialize};

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

pub async fn connect_google_business(axum::extract::Extension(claims): axum::extract::Extension<Claims>) -> Json<serde_json::Value> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());
    Json(serde_json::json!({
        "status": "success",
        "redirect_url": format!("https://accounts.google.com/o/oauth2/auth?client_id=MOCK_CLIENT_ID&redirect_uri=MOCK_URI&scope=https://www.googleapis.com/auth/business.manage&response_type=code&state={}", tenant_id)
    }))
}

pub async fn get_connection_status(axum::extract::Extension(_claims): axum::extract::Extension<Claims>) -> Json<ConnectionStatusResponse> {
    // In a real implementation we would check the DB for `ohc_google_business_profiles`
    Json(ConnectionStatusResponse { connected: true })
}

pub async fn get_pending_reviews(axum::extract::Extension(_claims): axum::extract::Extension<Claims>) -> Json<Vec<LocalReview>> {
    Json(vec![])
}

pub async fn approve_and_reply(
    axum::extract::Extension(_claims): axum::extract::Extension<Claims>,
    Path(review_id): Path<String>,
    Json(_payload): Json<ApproveReplyRequest>,
) -> Json<serde_json::Value> {
    // Here we would use GoogleBusinessClientWrapper to post the review
    // For now we just return success
    Json(serde_json::json!({ "status": "success", "review_id": review_id }))
}

pub async fn webhook_ingest(Json(_payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    // In a real implementation, we parse the webhook payload, insert to `ohc_local_reviews` and trigger AI
    Json(serde_json::json!({ "status": "received" }))
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/connect", post(connect_google_business))
        .route("/status", get(get_connection_status))
        .route("/reviews/pending", get(get_pending_reviews))
        .route("/reviews/{review_id}/approve", post(approve_and_reply))
        .route("/webhook", post(webhook_ingest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_common::Claims;
    use axum::Json;

    fn mock_claims() -> Claims {
        Claims {
            sub: "user123".to_string(),
            organization_id: Some("tenant123".to_string()),
            exp: 9999999999,
            iat: 0,
            username: "owner".to_string(),
            email: "owner@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: "jti123".to_string(),
        }
    }

    #[tokio::test]
    async fn test_connect_google_business() {
        let claims = mock_claims();
        let Json(response) = connect_google_business(axum::extract::Extension(claims)).await;
        assert_eq!(response["status"], "success");
        assert!(response["redirect_url"].as_str().unwrap().contains("tenant123"));
    }

    #[tokio::test]
    async fn test_get_connection_status() {
        let claims = mock_claims();
        let Json(response) = get_connection_status(axum::extract::Extension(claims)).await;
        assert!(response.connected);
    }

    #[tokio::test]
    async fn test_get_pending_reviews() {
        let claims = mock_claims();
        let Json(response) = get_pending_reviews(axum::extract::Extension(claims)).await;
        assert_eq!(response.len(), 0);
    }

    #[tokio::test]
    async fn test_approve_and_reply() {
        let claims = mock_claims();
        let review_id = Path("rev1".to_string());
        let payload = Json(ApproveReplyRequest {
            reply_content: "Thanks!".to_string(),
        });
        let Json(response) = approve_and_reply(axum::extract::Extension(claims), review_id, payload).await;
        assert_eq!(response["status"], "success");
        assert_eq!(response["review_id"], "rev1");
    }

    #[tokio::test]
    async fn test_webhook_ingest() {
        let payload = Json(serde_json::json!({
            "review": "test"
        }));
        let Json(response) = webhook_ingest(payload).await;
        assert_eq!(response["status"], "received");
    }
}
