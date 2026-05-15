use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

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

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/social/post", post(handle_social_post))
        .route("/campaign/send", post(handle_send_campaign))
        .route("/storefront/track", post(handle_track_visitor))
        .route("/milestones/check", get(handle_check_milestones))
}

async fn handle_social_post(
    Json(req): Json<SocialPostRequest>,
) -> impl IntoResponse {
    tracing::info!("Received social post request: {} platforms", req.platforms.len());
    Json(SocialPostResponse {
        posted: true,
        post_id: uuid::Uuid::new_v4().to_string(),
    })
}

async fn handle_send_campaign(
    Json(req): Json<CampaignRequest>,
) -> impl IntoResponse {
    tracing::info!("Received campaign request: {} targeting {}", req.name, req.target_segment);
    Json(CampaignResponse {
        campaign_id: uuid::Uuid::new_v4().to_string(),
        emails_sent: 150,
    })
}

async fn handle_track_visitor(
    Json(req): Json<TrackVisitorRequest>,
) -> impl IntoResponse {
    tracing::info!("Tracking visitor for page: {}", req.page_url);
    Json(TrackVisitorResponse { tracked: true })
}

async fn handle_check_milestones() -> impl IntoResponse {
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
