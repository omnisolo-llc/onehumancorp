use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::Row;

#[derive(Clone)]
pub struct MarketingCampaignState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct CreateCampaignRequest {
    pub subject: String,
    pub body: String,
    pub target_audience_tag: String,
}

#[derive(Serialize)]
pub struct CreateCampaignResponse {
    pub success: bool,
}

pub async fn create_campaign_handler(
    State(state): State<MarketingCampaignState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.org_id.clone(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), usize::MAX).await.unwrap_or_default();
    let payload: CreateCampaignRequest = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid payload").into_response(),
    };

    // Extract API key for tenant
    let api_key: Option<String> = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT sendgrid_api_key FROM tenants WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.get("sendgrid_api_key"))
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT sendgrid_api_key FROM tenants WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.get("sendgrid_api_key"))
        }
    };

    let api_key = match api_key {
        Some(k) => k,
        None => return (StatusCode::BAD_REQUEST, "SendGrid API key not configured").into_response(),
    };

    let client = crate::integrations::sendgrid::client::RealSendGridClient::new(api_key);

    // Query DB for customers matching the tag
    let target_emails: Vec<String> = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT email FROM customers WHERE tenant_id = ? AND tags LIKE ?")
                .bind(&tenant_id)
                .bind(format!("%{}%", payload.target_audience_tag))
                .fetch_all(pool)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|r| r.get("email"))
                .collect()
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT email FROM customers WHERE tenant_id = $1 AND tags LIKE $2")
                .bind(&tenant_id)
                .bind(format!("%{}%", payload.target_audience_tag))
                .fetch_all(&state.db.pool)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|r| r.get("email"))
                .collect()
        }
    };

    use crate::integrations::sendgrid::client::SendGridClientWrapper;
    for email in target_emails {
        let _ = client.send_email(&email, &payload.subject, &payload.body).await;
    }

    (StatusCode::OK, Json(CreateCampaignResponse { success: true })).into_response()
}
