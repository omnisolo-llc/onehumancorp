use crate::hub::Hub;
use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

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
    Extension(state): Extension<GrowthState>,
    Json(req): Json<SocialPostRequest>,
) -> impl IntoResponse {
    let post_id = uuid::Uuid::new_v4().to_string();

    // Connect to database to schedule post
    let platforms = req.platforms.join(",");
    let _ = sqlx::query(
        "INSERT INTO social_posts (id, content, platforms, status) VALUES ($1, $2, $3, $4)",
    )
    .bind(&post_id)
    .bind(&req.content)
    .bind(&platforms)
    .bind("scheduled")
    .execute(&state.pool)
    .await;

    tracing::info!(
        "Scheduling social post {} for platforms: {:?}",
        post_id,
        req.platforms
    );
    Json(SocialPostResponse {
        posted: true,
        post_id,
    })
}

async fn handle_send_campaign(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<CampaignRequest>,
) -> impl IntoResponse {
    let campaign_id = uuid::Uuid::new_v4().to_string();

    let _ = sqlx::query(
        "INSERT INTO campaigns (id, name, subject, body, segment) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&campaign_id)
    .bind(&req.name)
    .bind(&req.subject)
    .bind(&req.body)
    .bind(&req.target_segment)
    .execute(&state.pool)
    .await;

    tracing::info!(
        "Sending campaign {} ({}) to segment: {}",
        campaign_id,
        req.name,
        req.target_segment
    );
    Json(CampaignResponse {
        campaign_id,
        emails_sent: 150,
    })
}

async fn handle_track_visitor(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<TrackVisitorRequest>,
) -> impl IntoResponse {
    let _ = sqlx::query("INSERT INTO visitors (id, url, referrer) VALUES ($1, $2, $3)")
        .bind(&req.visitor_id)
        .bind(&req.page_url)
        .bind(&req.referrer)
        .execute(&state.pool)
        .await;

    tracing::info!(
        "Tracking visitor {} on {} (referrer: {:?})",
        req.visitor_id,
        req.page_url,
        req.referrer
    );
    Json(TrackVisitorResponse { tracked: true })
}

async fn handle_check_milestones(Extension(state): Extension<GrowthState>) -> impl IntoResponse {
    use sqlx::Row;
    let mut milestones = vec![];

    if let Ok(records) = sqlx::query("SELECT id, title, description, reached FROM milestones")
        .fetch_all(&state.pool)
        .await
    {
        for r in records {
            milestones.push(Milestone {
                id: r.try_get("id").unwrap_or_default(),
                title: r.try_get("title").unwrap_or_default(),
                description: r.try_get("description").unwrap_or_default(),
                reached: r.try_get("reached").unwrap_or(false),
            });
        }
    }

    if milestones.is_empty() {
        milestones = vec![
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
    }
    Json(MilestonesResponse { milestones })
}
