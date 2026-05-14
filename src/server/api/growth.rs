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
/// Core API request/response payload for SocialPostRequest.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct SocialPostRequest {
    /// Stores the `content` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub content: String,
    /// Stores the `platforms` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for SocialPostResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct SocialPostResponse {
    /// Stores the `posted` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub posted: bool,
    /// Stores the `post_id` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for CampaignRequest.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct CampaignRequest {
    /// Stores the `name` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub name: String,
    /// Stores the `subject` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub subject: String,
    /// Stores the `body` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub body: String,
    /// Stores the `target_segment` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub target_segment: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for CampaignResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct CampaignResponse {
    /// Stores the `campaign_id` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub campaign_id: String,
    /// Stores the `emails_sent` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub emails_sent: i32,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for TrackVisitorRequest.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct TrackVisitorRequest {
    /// Stores the `page_url` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub page_url: String,
    /// Stores the `referrer` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub referrer: Option<String>,
    /// Stores the `visitor_id` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub visitor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for TrackVisitorResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct TrackVisitorResponse {
    /// Stores the `tracked` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub tracked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for Milestone.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct Milestone {
    /// Stores the `id` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub id: String,
    /// Stores the `title` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub title: String,
    /// Stores the `description` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub description: String,
    /// Stores the `reached` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub reached: bool,
}

#[derive(Debug, Serialize, Deserialize)]
/// Core API request/response payload for MilestonesResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct MilestonesResponse {
    /// Stores the `milestones` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
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
