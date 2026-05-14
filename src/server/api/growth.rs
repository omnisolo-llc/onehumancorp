use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Extension,
};
use axum::http::HeaderMap;
use axum::extract::FromRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::hub::Hub;
use crate::services::growth::social::SocialMediaService;
use crate::services::growth::email::EmailMarketingService;
use crate::services::growth::milestones::MilestonesService;

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPostRequest {
    pub content: String,
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPostResponse {
    pub posted: bool,
    pub post_id: String,
    pub error: Option<String>,
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
    pub error: Option<String>,
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
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestonesResponse {
    pub milestones: Vec<crate::services::growth::milestones::MilestoneData>,
    pub error: Option<String>,
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
    #[allow(dead_code)]
    hub: Arc<Hub>,
}

async fn handle_social_post(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let state = req.extensions().get::<GrowthState>().unwrap().clone();

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: SocialPostRequest = match axum::extract::Json::<SocialPostRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return Json(SocialPostResponse { posted: false, post_id: "".to_string(), error: Some("Invalid payload".to_string()) }),
    };

    let service = SocialMediaService::new(state.pool);

    let platform = payload.platforms.first().map(|s| s.as_str()).unwrap_or("social");

    match service.schedule_post(&tenant_id, platform, &payload.content).await {
        Ok(post_id) => Json(SocialPostResponse {
            posted: true,
            post_id,
            error: None,
        }),
        Err(e) => Json(SocialPostResponse {
            posted: false,
            post_id: String::new(),
            error: Some(e),
        }),
    }
}

async fn handle_send_campaign(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let state = req.extensions().get::<GrowthState>().unwrap().clone();

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: CampaignRequest = match axum::extract::Json::<CampaignRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return Json(CampaignResponse { campaign_id: "".to_string(), emails_sent: 0, error: Some("Invalid payload".to_string()) }),
    };

    let service = EmailMarketingService::new(state.pool);

    match service.send_campaign(&tenant_id, &payload.name, &payload.subject, &payload.body, &payload.target_segment).await {
        Ok((campaign_id, emails_sent)) => Json(CampaignResponse {
            campaign_id,
            emails_sent,
            error: None,
        }),
        Err(e) => Json(CampaignResponse {
            campaign_id: String::new(),
            emails_sent: 0,
            error: Some(e),
        }),
    }
}

async fn handle_track_visitor(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let state = req.extensions().get::<GrowthState>().unwrap().clone();

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: TrackVisitorRequest = match axum::extract::Json::<TrackVisitorRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return Json(TrackVisitorResponse { tracked: false, error: Some("Invalid payload".to_string()) }),
    };

    let service = MilestonesService::new(state.pool);

    match service.track_visitor(&tenant_id, &payload.visitor_id, &payload.page_url, payload.referrer.as_deref()).await {
        Ok(tracked) => Json(TrackVisitorResponse { tracked, error: None }),
        Err(e) => Json(TrackVisitorResponse { tracked: false, error: Some(e) }),
    }
}

async fn handle_check_milestones(
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let state = req.extensions().get::<GrowthState>().unwrap().clone();

    let service = MilestonesService::new(state.pool);

    match service.get_milestones(&tenant_id).await {
        Ok(milestones) => Json(MilestonesResponse { milestones, error: None }),
        Err(e) => Json(MilestonesResponse { milestones: vec![], error: Some(e) }),
    }
}
