use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::hub::Hub;

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPostRequest {
    pub content: String,
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPostResponse {
    pub posted: bool,
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub target_segment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignResponse {
    pub campaign_id: String,
    pub emails_sent: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackVisitorRequest {
    pub page_url: String,
    pub referrer: Option<String>,
    pub visitor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackVisitorResponse {
    pub tracked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub reached: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestonesResponse {
    pub milestones: Vec<Milestone>,
}

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/social/posts", get(handle_get_social_posts))
        .route("/social/post/:id/status", post(handle_update_social_post))
        .route("/email/campaigns", get(handle_get_email_campaigns))
        .route("/email/campaign", post(handle_create_email_campaign))
        .route("/milestones", get(handle_get_milestones))
        .route("/milestones/:id/notified", post(handle_mark_milestone_notified))
        .route("/social/post", post(handle_social_post))
        .route("/campaign/send", post(handle_send_campaign))
        .route("/storefront/track", post(handle_track_visitor))
        .route("/milestones/check", get(handle_check_milestones))
        .layer(Extension(GrowthState { pool, hub }))
}

#[derive(Clone)]
struct GrowthState {
    pool: PgPool,
    hub: Arc<Hub>,
}

async fn handle_social_post(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<SocialPostRequest>,
) -> impl IntoResponse {
    Json(SocialPostResponse {
        posted: true,
        post_id: uuid::Uuid::new_v4().to_string(),
    })
}

async fn handle_send_campaign(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<CampaignRequest>,
) -> impl IntoResponse {
    Json(CampaignResponse {
        campaign_id: uuid::Uuid::new_v4().to_string(),
        emails_sent: 150,
    })
}

async fn handle_track_visitor(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<TrackVisitorRequest>,
) -> impl IntoResponse {
    Json(TrackVisitorResponse { tracked: true })
}

async fn handle_check_milestones(
    Extension(_state): Extension<GrowthState>,
) -> impl IntoResponse {
    let milestones = vec![
        Milestone {
            id: "1".to_string(),
            title: "First Teammate".to_string(),
            description: "Hire your first AI agent".to_string(),
            reached: true,
        },
        Milestone {
            id: "2".to_string(),
            title: "Global Reach".to_string(),
            description: "Connect to a partner organization".to_string(),
            reached: false,
        },
    ];
    Json(MilestonesResponse { milestones })
}

async fn handle_get_social_posts(
    Extension(state): Extension<GrowthState>,
) -> impl IntoResponse {
    let org_id = "default_org"; // In real scenario, extract from Auth claims
    match crate::services::growth::social_media::get_posts(&state.pool, org_id).await {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_update_social_post(
    Extension(state): Extension<GrowthState>,
    Path(id): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    let org_id = "default_org";
    let status = req.get("status").and_then(|v| v.as_str()).unwrap_or("DRAFT");

    let res = sqlx::query("UPDATE social_posts SET status = $1 WHERE id = $2 AND tenant_id = $3")
        .bind(status)
        .bind(id)
        .bind(org_id)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_get_email_campaigns(
    Extension(state): Extension<GrowthState>,
) -> impl IntoResponse {
    let org_id = "default_org";
    match crate::services::growth::email_marketing::get_campaigns(&state.pool, org_id).await {
        Ok(camps) => Json(camps).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_create_email_campaign(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<::server_ohc::orchestration::CreateEmailCampaignRequest>,
) -> impl IntoResponse {
    let org_id = "default_org";
    match crate::services::growth::email_marketing::create_campaign(&state.pool, org_id, req).await {
        Ok(camp) => Json(camp).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_get_milestones(
    Extension(state): Extension<GrowthState>,
) -> impl IntoResponse {
    let org_id = "default_org";
    let rows = sqlx::query("SELECT id, milestone_key, achieved_at, notified FROM business_milestones WHERE tenant_id = $1 AND notified = FALSE")
        .bind(org_id)
        .fetch_all(&state.pool)
        .await;

    match rows {
        Ok(rows) => {
            let milestones: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                use sqlx::Row;
                serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "milestone_key": row.get::<String, _>("milestone_key"),
                    "notified": row.get::<bool, _>("notified")
                })
            }).collect();
            Json(serde_json::json!({ "milestones": milestones })).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn handle_mark_milestone_notified(
    Extension(state): Extension<GrowthState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let org_id = "default_org";
    let _ = sqlx::query("UPDATE business_milestones SET notified = TRUE WHERE id = $1 AND tenant_id = $2")
        .bind(id)
        .bind(org_id)
        .execute(&state.pool)
        .await;
    StatusCode::OK
}
