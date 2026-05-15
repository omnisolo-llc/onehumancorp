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


#[derive(Debug, Serialize, Deserialize)]
pub struct SendCampaignResponse {
    pub success: bool,
    pub campaign_id: String,
    pub target_audience_count: i32,
    pub estimated_delivery_time: i32,
}

async fn handle_send_campaign(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<CampaignRequest>,
) -> impl IntoResponse {
    tracing::info!("Starting email campaign: {}", req.name);
    // Real implementation would queue jobs here
    Json(SendCampaignResponse {
        success: true,
        campaign_id: uuid::Uuid::new_v4().to_string(),
        target_audience_count: 450,
        estimated_delivery_time: 120,
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
