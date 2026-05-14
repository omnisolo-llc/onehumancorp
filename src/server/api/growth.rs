use axum::{
    extract::{Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Extension,
};
use serde::{Deserialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::hub::Hub;
use std::sync::OnceLock;
use regex::Regex;

fn email_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap())
}

use crate::domain::growth::services::GrowthService;
use crate::domain::growth::repository::PgGrowthRepo;

#[derive(Clone)]
pub struct GrowthAppState {
    pub service: Arc<GrowthService>,
}

pub fn router<S>(pool: PgPool, _hub: Arc<Hub>) -> Router<S> where S: Clone + Send + Sync + 'static
{
    let repo = Arc::new(PgGrowthRepo::new(pool));
    let service = Arc::new(GrowthService::new(repo));
    let state = GrowthAppState { service };

    Router::new()
        .route("/referrals/invite", post(handle_referral_invite))
        .route("/referrals/:id/complete", post(handle_referral_complete))
        .route("/business/:id/share-card", post(handle_create_share_card))
        .route("/social/schedule", post(handle_schedule_social_post))
        .route("/campaigns/launch", post(handle_launch_campaign))
        .route("/tier/:business_id/verify", get(handle_verify_free_tier))
        .route("/storefront/:id/ensure-viral", post(handle_ensure_viral_storefront))
        .route("/milestones/:id/check", post(handle_check_milestone))
        .layer(Extension(state))
}

#[derive(Deserialize)]
pub struct ReferralInviteReq {
    pub referrer_id: String,
    pub email: String,
}

async fn handle_referral_invite(
    Extension(state): Extension<GrowthAppState>,
    Json(req): Json<ReferralInviteReq>,
) -> impl IntoResponse {
    if !email_regex().is_match(&req.email) {
        return (StatusCode::BAD_REQUEST, "Invalid email".to_string()).into_response();
    }
    match state.service.process_referral_invite(&req.referrer_id, &req.email).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({"referral_id": id.to_string()}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

async fn handle_referral_complete(
    Extension(state): Extension<GrowthAppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let uuid_id = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UUID".to_string()).into_response(),
    };
    match state.service.complete_referral(&uuid_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

#[derive(Deserialize)]
pub struct ShareCardReq {
    pub title: String,
    pub tagline: String,
}

async fn handle_create_share_card(
    Extension(state): Extension<GrowthAppState>,
    Path(business_id): Path<String>,
    Json(req): Json<ShareCardReq>,
) -> impl IntoResponse {
    match state.service.generate_share_card(&business_id, &req.title, &req.tagline).await {
        Ok(card) => (StatusCode::CREATED, Json(serde_json::json!({"card_id": card.id.to_string()}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

#[derive(Deserialize)]
pub struct SocialPostReq {
    pub business_id: String,
    pub content: String,
    pub platform: String,
}

async fn handle_schedule_social_post(
    Extension(state): Extension<GrowthAppState>,
    Json(req): Json<SocialPostReq>,
) -> impl IntoResponse {
    match state.service.schedule_social_post(&req.business_id, &req.content, &req.platform).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({"post_id": id.to_string()}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

#[derive(Deserialize)]
pub struct CampaignReq {
    pub business_id: String,
    pub subject: String,
    pub html_body: String,
}

async fn handle_launch_campaign(
    Extension(state): Extension<GrowthAppState>,
    Json(req): Json<CampaignReq>,
) -> impl IntoResponse {
    match state.service.launch_email_campaign(&req.business_id, &req.subject, &req.html_body).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({"campaign_id": id.to_string()}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

async fn handle_verify_free_tier(
    Extension(state): Extension<GrowthAppState>,
    Path(business_id): Path<String>,
) -> impl IntoResponse {
    match state.service.verify_free_tier_limits(&business_id).await {
        Ok(can_proceed) => (StatusCode::OK, Json(serde_json::json!({"can_proceed": can_proceed}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

async fn handle_ensure_viral_storefront(
    Extension(state): Extension<GrowthAppState>,
    Path(business_id): Path<String>,
) -> impl IntoResponse {
    match state.service.initialize_viral_storefront(&business_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}

#[derive(Deserialize)]
pub struct MilestoneReq {
    pub metric: String,
    pub value: i32,
}

async fn handle_check_milestone(
    Extension(state): Extension<GrowthAppState>,
    Path(business_id): Path<String>,
    Json(req): Json<MilestoneReq>,
) -> impl IntoResponse {
    match state.service.check_and_unlock_milestone(&business_id, &req.metric, req.value).await {
        Ok(Some(milestone)) => (StatusCode::CREATED, Json(serde_json::json!({"milestone_id": milestone.id.to_string()}))).into_response(),
        Ok(None) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.message).into_response(),
    }
}
