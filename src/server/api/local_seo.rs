use axum::{
    extract::Path,
    Extension,
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

fn tenant_id(claims: &Claims) -> String {
    claims
        .organization_id
        .clone()
        .unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant())
}

fn google_business_api_base() -> String {
    std::env::var("GOOGLE_BUSINESS_API_BASE")
        .unwrap_or_else(|_| "https://mybusiness.googleapis.com".to_string())
        .trim_end_matches('/')
        .to_string()
}

fn percent_encode(input: &str) -> String {
    input.bytes().map(|byte| match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (byte as char).to_string(),
        _ => format!("%{byte:02X}"),
    }).collect()
}

fn google_business_token() -> Option<String> {
    std::env::var("GOOGLE_BUSINESS_ACCESS_TOKEN")
        .ok()
        .filter(|token| !token.trim().is_empty())
}

fn google_business_location() -> Option<(String, String)> {
    let account_id = std::env::var("GOOGLE_BUSINESS_ACCOUNT_ID").ok()?;
    let location_id = std::env::var("GOOGLE_BUSINESS_LOCATION_ID").ok()?;
    if account_id.trim().is_empty() || location_id.trim().is_empty() {
        return None;
    }
    Some((account_id, location_id))
}

pub async fn connect_google_business(Extension(claims): Extension<Claims>) -> Json<serde_json::Value> {
    let client_id = match std::env::var("GOOGLE_BUSINESS_CLIENT_ID") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return Json(serde_json::json!({
            "status": "error",
            "message": "Google Business Profile OAuth is not configured"
        })),
    };
    let redirect_uri = match std::env::var("GOOGLE_BUSINESS_REDIRECT_URI") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return Json(serde_json::json!({
            "status": "error",
            "message": "Google Business Profile OAuth is not configured"
        })),
    };

    let redirect_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&scope={}&response_type=code&access_type=offline&prompt=consent&state={}",
        percent_encode(&client_id),
        percent_encode(&redirect_uri),
        percent_encode("https://www.googleapis.com/auth/business.manage"),
        percent_encode(&tenant_id(&claims)),
    );

    Json(serde_json::json!({
        "status": "success",
        "redirect_url": redirect_url
    }))
}

pub async fn get_connection_status(Extension(_claims): Extension<Claims>) -> Json<ConnectionStatusResponse> {
    Json(ConnectionStatusResponse {
        connected: google_business_token().is_some() && google_business_location().is_some(),
    })
}

pub async fn get_pending_reviews(Extension(_claims): Extension<Claims>) -> Json<Vec<LocalReview>> {
    let token = match google_business_token() {
        Some(token) => token,
        None => return Json(vec![]),
    };
    let (account_id, location_id) = match google_business_location() {
        Some(location) => location,
        None => return Json(vec![]),
    };

    let url = format!(
        "{}/v4/accounts/{}/locations/{}/reviews",
        google_business_api_base(),
        percent_encode(&account_id),
        percent_encode(&location_id),
    );

    let resp = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await;
    let body: serde_json::Value = match resp {
        Ok(response) if response.status().is_success() => match response.json().await {
            Ok(body) => body,
            Err(_) => return Json(vec![]),
        },
        _ => return Json(vec![]),
    };

    let reviews = body.get("reviews")
        .and_then(|value| value.as_array())
        .map(|items| {
            items.iter().filter_map(|item| {
                let review_id = item.get("reviewId")
                    .or_else(|| item.get("name"))
                    .and_then(|value| value.as_str())?
                    .to_string();
                let reviewer_name = item.get("reviewer")
                    .and_then(|value| value.get("displayName"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("Google reviewer")
                    .to_string();
                let star_rating = item.get("starRating")
                    .and_then(|value| value.as_str())
                    .and_then(|rating| match rating {
                        "ONE" => Some(1),
                        "TWO" => Some(2),
                        "THREE" => Some(3),
                        "FOUR" => Some(4),
                        "FIVE" => Some(5),
                        _ => None,
                    })
                    .unwrap_or(0);
                let comment = item.get("comment").and_then(|value| value.as_str()).map(|value| value.to_string());
                let reply_status = if item.get("reviewReply").is_some() { "REPLIED" } else { "PENDING" }.to_string();
                Some(LocalReview {
                    review_id,
                    reviewer_name,
                    star_rating,
                    comment,
                    ai_draft_reply: None,
                    reply_status,
                })
            }).collect()
        })
        .unwrap_or_default();

    Json(reviews)
}

pub async fn approve_and_reply(
    Extension(_claims): Extension<Claims>,
    Path(review_id): Path<String>,
    Json(payload): Json<ApproveReplyRequest>,
) -> Json<serde_json::Value> {
    let token = match google_business_token() {
        Some(token) => token,
        None => return Json(serde_json::json!({
            "status": "error",
            "message": "Google Business Profile is not connected"
        })),
    };
    let (account_id, location_id) = match google_business_location() {
        Some(location) => location,
        None => return Json(serde_json::json!({
            "status": "error",
            "message": "Google Business Profile is not connected"
        })),
    };
    if payload.reply_content.trim().is_empty() {
        return Json(serde_json::json!({
            "status": "error",
            "message": "Reply content is required"
        }));
    }

    let url = format!(
        "{}/v4/accounts/{}/locations/{}/reviews/{}/reply",
        google_business_api_base(),
        percent_encode(&account_id),
        percent_encode(&location_id),
        percent_encode(&review_id),
    );
    let resp = reqwest::Client::new()
        .put(url)
        .bearer_auth(token)
        .json(&serde_json::json!({ "comment": payload.reply_content }))
        .send()
        .await;

    match resp {
        Ok(response) if response.status().is_success() => Json(serde_json::json!({ "status": "success", "review_id": review_id })),
        Ok(response) => Json(serde_json::json!({ "status": "error", "message": format!("Google Business Profile API error: {}", response.status()) })),
        Err(err) => Json(serde_json::json!({ "status": "error", "message": format!("Google Business Profile request failed: {err}") })),
    }
}

pub async fn webhook_ingest(Json(_payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    // In a real implementation, we parse the webhook payload, insert to `ohc_local_reviews` and trigger AI
    Json(serde_json::json!({ "status": "received" }))
}

#[derive(serde::Serialize)]
pub struct DiscoveryReport {
    pub id: uuid::Uuid,
    pub month: String,
    pub plain_language_summary: String,
    pub metrics: serde_json::Value,
}

pub async fn get_discovery_report(
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(claims): Extension<Claims>,
) -> Json<Vec<DiscoveryReport>> {
    let tenant_id = tenant_id(&claims);
    let Ok(uuid) = uuid::Uuid::parse_str(&tenant_id) else {
        return Json(vec![]);
    };

    let mut conn = match pool.acquire().await {
        Ok(c) => c,
        Err(_) => return Json(vec![]),
    };

    let _ = ::server_common::auth_utils::set_org_context(&mut *conn, &tenant_id).await;

    let rows = sqlx::query_as::<_, (uuid::Uuid, String, String, Option<serde_json::Value>)>(
        "SELECT id, month, plain_language_summary, metrics FROM seo_discovery_reports WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(uuid)
    .fetch_all(&mut *conn)
    .await
    .unwrap_or_default();

    let reports = rows.into_iter().map(|(id, month, plain_language_summary, metrics)| DiscoveryReport {
        id,
        month,
        plain_language_summary,
        metrics: metrics.unwrap_or_else(|| serde_json::json!({})),
    }).collect();

    Json(reports)
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/connect", post(connect_google_business))
        .route("/status", get(get_connection_status))
        .route("/reviews/pending", get(get_pending_reviews))
        .route("/reviews/{review_id}/approve", post(approve_and_reply))
        .route("/webhook", post(webhook_ingest))
        .route("/discovery_report", get(get_discovery_report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_common::Claims;
    use axum::Json;

    fn mock_claims() -> Claims {
        Claims {
            sub: "user123".to_string(),
            exp: 9999999999,
            iat: 1,
            organization_id: Some("tenant123".to_string()),
            username: "owner".to_string(),
            email: "owner@example.com".to_string(),
            roles: vec!["owner".to_string()],
            session_id: None,
            jti: "jti123".to_string(),
        }
    }

    #[tokio::test]
    async fn test_connect_google_business() {
        unsafe {
            std::env::set_var("GOOGLE_BUSINESS_CLIENT_ID", "client-123.apps.googleusercontent.com");
            std::env::set_var("GOOGLE_BUSINESS_REDIRECT_URI", "https://ohc.example/oauth/google-business/callback");
        }

        let claims = mock_claims();
        let Json(response) = connect_google_business(Extension(claims)).await;
        let redirect_url = response["redirect_url"].as_str().unwrap();
        assert_eq!(response["status"], "success");
        assert!(redirect_url.contains("client-123.apps.googleusercontent.com"));
        assert!(redirect_url.contains("https%3A%2F%2Fohc.example%2Foauth%2Fgoogle-business%2Fcallback"));
        assert!(redirect_url.contains("state=tenant123"));
        assert!(!redirect_url.contains("MOCK"));

        unsafe {
            std::env::remove_var("GOOGLE_BUSINESS_CLIENT_ID");
            std::env::remove_var("GOOGLE_BUSINESS_REDIRECT_URI");
        }
    }

    #[tokio::test]
    async fn test_connect_google_business_requires_oauth_configuration() {
        unsafe {
            std::env::remove_var("GOOGLE_BUSINESS_CLIENT_ID");
            std::env::remove_var("GOOGLE_BUSINESS_REDIRECT_URI");
        }
        let claims = mock_claims();
        let Json(response) = connect_google_business(Extension(claims)).await;
        assert_eq!(response["status"], "error");
        assert_eq!(response["message"], "Google Business Profile OAuth is not configured");
    }

    #[tokio::test]
    async fn test_get_connection_status() {
        unsafe {
            std::env::remove_var("GOOGLE_BUSINESS_ACCESS_TOKEN");
            std::env::remove_var("GOOGLE_BUSINESS_ACCOUNT_ID");
            std::env::remove_var("GOOGLE_BUSINESS_LOCATION_ID");
        }
        let claims = mock_claims();
        let Json(response) = get_connection_status(Extension(claims)).await;
        assert!(!response.connected);
    }

    #[tokio::test]
    async fn test_get_pending_reviews() {
        let claims = mock_claims();
        let Json(response) = get_pending_reviews(Extension(claims)).await;
        assert!(response.is_empty());
    }

    #[tokio::test]
    async fn test_approve_and_reply() {
        unsafe {
            std::env::remove_var("GOOGLE_BUSINESS_ACCESS_TOKEN");
            std::env::remove_var("GOOGLE_BUSINESS_ACCOUNT_ID");
            std::env::remove_var("GOOGLE_BUSINESS_LOCATION_ID");
        }
        let claims = mock_claims();
        let review_id = Path("rev1".to_string());
        let payload = Json(ApproveReplyRequest {
            reply_content: "Thanks!".to_string(),
        });
        let Json(response) = approve_and_reply(Extension(claims), review_id, payload).await;
        assert_eq!(response["status"], "error");
        assert_eq!(response["message"], "Google Business Profile is not connected");
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
