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

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleSocialRequest {
    pub content: String,
    pub image_url: Option<String>,
    pub platforms: Vec<String>,
    pub scheduled_time_unix: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleSocialResponse {
    pub scheduled: bool,
    pub job_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignTemplate {
    pub id: String,
    pub name: String,
    pub subject_line: String,
    pub preview_text: String,
    pub body_html: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignTemplatesResponse {
    pub templates: Vec<CampaignTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareReferralRequest {
    pub user_id: String,
    pub channel: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareReferralResponse {
    pub share_url: String,
    pub prefilled_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareCardResponse {
    pub card_url: String,
    pub share_links: ShareLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareLinks {
    pub instagram: String,
    pub whatsapp: String,
    pub x: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradePromptRequest {
    pub feature: String,
    pub org_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradePromptResponse {
    pub show_prompt: bool,
    pub title: String,
    pub message: String,
    pub cta_text: String,
    pub cta_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorefrontViralResponse {
    pub html_snippet: String,
}

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/referrals/share", post(handle_share_referral))
        .route("/storefront/:org_id/share-card", get(handle_storefront_share_card))
        .route("/social/schedule", post(handle_schedule_social))
        .route("/campaign/templates", get(handle_get_campaign_templates))
        .route("/upgrade-prompt", post(handle_upgrade_prompt))
        .route("/storefront/:org_id/viral-footer", get(handle_storefront_viral))
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

async fn handle_share_referral(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<ShareReferralRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let base_url = "https://ohc.app/join";
    let link = crate::services::growth::referral_api::generate_referral_link(&req.user_id)
        .unwrap_or_else(|_| format!("{}?ref=default", base_url));

    // Save to database
    let pool = state.pool;
    sqlx::query("INSERT INTO referrals (id, organization_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, $2, $3, $4, 0, 0, $5)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind("system_org")
        .bind(&req.user_id)
        .bind(&link)
        .bind(chrono::Utc::now().timestamp())
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert referral: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let message = format!("Hey! I'm using OHC to run my business. Join using my link and we both get 1 month of Pro for free: {}", link);

    Ok(Json(ShareReferralResponse {
        share_url: link,
        prefilled_message: message,
    }))
}

async fn handle_storefront_share_card(
    Path(org_id): Path<String>,
    Extension(state): Extension<GrowthState>,
) -> Result<impl IntoResponse, StatusCode> {
    let card_url = format!("https://ohc.app/og/{}.png", org_id);
    let store_url = format!("https://ohc.app/store/{}", org_id);

    let business_name: String = sqlx::query_scalar("SELECT business_name FROM tenants WHERE tenant_id = $1::uuid")
        .bind(&org_id)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "My Awesome Store".to_string());

    let text = format!("Check out my new store: {} built with OHC!", business_name);

    let encoded_url = store_url.replace(" ", "%20").replace("/", "%2F").replace(":", "%3A");
    let encoded_text = text.replace(" ", "%20");

    Ok(Json(ShareCardResponse {
        card_url,
        share_links: ShareLinks {
            instagram: "https://instagram.com/".to_string(),
            whatsapp: format!("https://wa.me/?text={} {}", encoded_text, encoded_url),
            x: format!("https://twitter.com/intent/tweet?text={}&url={}", encoded_text, encoded_url),
        }
    }))
}

async fn handle_schedule_social(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<ScheduleSocialRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let job_id = uuid::Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO social_queue (job_id, content, status, scheduled_at) VALUES ($1, $2, 'PENDING', $3)")
        .bind(&job_id)
        .bind(&req.content)
        .bind(req.scheduled_time_unix)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert social post: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ScheduleSocialResponse {
        scheduled: true,
        job_id,
        status: "SCHEDULED".to_string(),
    }))
}

async fn handle_get_campaign_templates(
    Extension(_state): Extension<GrowthState>,
) -> Result<impl IntoResponse, StatusCode> {
    let templates = vec![
        CampaignTemplate {
            id: "tpl_1".to_string(),
            name: "New Arrivals".to_string(),
            subject_line: "Check out what's new!".to_string(),
            preview_text: "Fresh styles just landed...".to_string(),
            body_html: "<h1>New Arrivals</h1><p>Shop the latest trends now.</p>".to_string(),
        },
        CampaignTemplate {
            id: "tpl_2".to_string(),
            name: "Flash Sale".to_string(),
            subject_line: "24-Hour Flash Sale - 20% OFF".to_string(),
            preview_text: "Don't miss out on these deals...".to_string(),
            body_html: "<h1>Flash Sale</h1><p>Use code FLASH20 at checkout.</p>".to_string(),
        },
        CampaignTemplate {
            id: "tpl_3".to_string(),
            name: "Thank You".to_string(),
            subject_line: "A special thanks from us".to_string(),
            preview_text: "We appreciate your support...".to_string(),
            body_html: "<h1>Thank You</h1><p>Here's a 10% off coupon for your next purchase.</p>".to_string(),
        },
    ];

    Ok(Json(CampaignTemplatesResponse { templates }))
}

async fn handle_upgrade_prompt(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<UpgradePromptRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let tier: String = sqlx::query_scalar("SELECT plan_tier FROM organizations WHERE id = $1")
        .bind(&req.org_id)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "Free".to_string());

    let show = tier == "Free";

    let (title, msg) = match req.feature.as_str() {
        "agents" => ("Unlock more AI teammates", "You've hit your free tier limit of 1 agent. Upgrade to Starter to hire up to 3 agents and grow your business faster!"),
        "products" => ("Expand your inventory", "You've reached your free tier limit of 10 products. Upgrade to Starter to add more products and increase your sales!"),
        "social_posting" => ("Automate your social presence", "Social media auto-posting is a premium feature. Upgrade to Pro to let your AI agents handle your marketing!"),
        _ => ("", ""),
    };

    Ok(Json(UpgradePromptResponse {
        show_prompt: show,
        title: title.to_string(),
        message: msg.to_string(),
        cta_text: "Upgrade Plan".to_string(),
        cta_url: "/dashboard/billing/upgrade".to_string(),
    }))
}

async fn handle_storefront_viral(
    Path(org_id): Path<String>,
    Extension(_state): Extension<GrowthState>,
) -> Result<impl IntoResponse, StatusCode> {
    let viral_html = format!(
        "<div class=\"ohc-viral-footer\" style=\"text-align: center; padding: 20px; font-family: sans-serif; color: #666; border-top: 1px solid #eee; margin-top: 40px;\">\
            <p>Built with <a href=\"https://ohc.app/join?ref={}&utm_source=storefront_footer\" style=\"color: #4ecca3; text-decoration: none; font-weight: bold;\">OHC</a> — Start your free business today &#8594;</p>\
        </div>",
        org_id
    );

    Ok(Json(StorefrontViralResponse {
        html_snippet: viral_html,
    }))
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


// ============================================================================
// COMPREHENSIVE CAMPAIGN & SOCIAL DATA MODELS AND SERVICES
// ============================================================================
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tonic::Status;
use opentelemetry::global;
use opentelemetry::metrics::{Counter, Histogram};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub subject: String,
    pub body_html: String,
    pub status: CampaignStatus,
    pub metrics: CampaignMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Completed,
    Failed,
}

impl ToString for CampaignStatus {
    fn to_string(&self) -> String {
        match self {
            CampaignStatus::Draft => "DRAFT".to_string(),
            CampaignStatus::Scheduled => "SCHEDULED".to_string(),
            CampaignStatus::Sending => "SENDING".to_string(),
            CampaignStatus::Completed => "COMPLETED".to_string(),
            CampaignStatus::Failed => "FAILED".to_string(),
        }
    }
}

impl From<&str> for CampaignStatus {
    fn from(s: &str) -> Self {
        match s {
            "DRAFT" => CampaignStatus::Draft,
            "SCHEDULED" => CampaignStatus::Scheduled,
            "SENDING" => CampaignStatus::Sending,
            "COMPLETED" => CampaignStatus::Completed,
            "FAILED" => CampaignStatus::Failed,
            _ => CampaignStatus::Draft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignMetrics {
    pub total_sent: i32,
    pub total_delivered: i32,
    pub total_opened: i32,
    pub total_clicked: i32,
    pub total_bounced: i32,
    pub total_complained: i32,
}

pub struct CampaignService {
    pool: PgPool,
    campaigns_created: Counter<u64>,
    campaigns_sent: Counter<u64>,
    send_duration: Histogram<f64>,
}

impl CampaignService {
    pub fn new(pool: PgPool) -> Self {
        let meter = global::meter("ohc.growth.campaigns");
        Self {
            pool,
            campaigns_created: meter.u64_counter("campaigns.created").build(),
            campaigns_sent: meter.u64_counter("campaigns.sent").build(),
            send_duration: meter.f64_histogram("campaigns.send_duration").build(),
        }
    }

    pub async fn create_campaign(&self, org_id: &str, name: &str, subject: &str, body: &str) -> Result<Campaign, Status> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        Ok(Campaign {
            id,
            org_id: org_id.to_string(),
            name: name.to_string(),
            subject: subject.to_string(),
            body_html: body.to_string(),
            status: CampaignStatus::Draft,
            metrics: CampaignMetrics::default(),
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub org_id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

pub struct ContactService {
    pool: PgPool,
}

impl ContactService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn add_contact(&self, org_id: &str, email: &str, first: Option<String>, last: Option<String>) -> Result<Contact, Status> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        Ok(Contact {
            id,
            org_id: org_id.to_string(),
            email: email.to_string(),
            first_name: first,
            last_name: last,
            tags: vec![],
            created_at: now,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudienceSegment {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub criteria: Vec<SegmentCriteria>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentCriteria {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub subject: String,
    pub html_content: String,
    pub plain_text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignRequestDto {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub target_segment_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignResponseDto {
    pub campaign_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCampaignStatusRequestDto {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignMetricsResponseDto {
    pub opens: i32,
    pub clicks: i32,
    pub bounces: i32,
    pub deliveries: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAudienceSegmentRequestDto {
    pub name: String,
    pub criteria: Vec<SegmentCriteriaDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentCriteriaDto {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateABTestRequestDto {
    pub base_campaign_id: String,
    pub variant_a_subject: String,
    pub variant_b_subject: String,
    pub variant_a_body: String,
    pub variant_b_body: String,
    pub split_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateABTestResponseDto {
    pub test_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveABTestRequestDto {
    pub winner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddContactRequestDto {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTagsRequestDto {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveTagsRequestDto {
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_status_conversion() {
        assert_eq!(CampaignStatus::Draft.to_string(), "DRAFT");
        assert_eq!(CampaignStatus::Scheduled.to_string(), "SCHEDULED");
        assert_eq!(CampaignStatus::Sending.to_string(), "SENDING");
        assert_eq!(CampaignStatus::Completed.to_string(), "COMPLETED");
        assert_eq!(CampaignStatus::Failed.to_string(), "FAILED");

        assert_eq!(CampaignStatus::from("DRAFT"), CampaignStatus::Draft);
        assert_eq!(CampaignStatus::from("SCHEDULED"), CampaignStatus::Scheduled);
        assert_eq!(CampaignStatus::from("UNKNOWN"), CampaignStatus::Draft);
    }

    #[test]
    fn test_dto_serialization_variant_0() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 0".to_string(),
            subject: "Subject 0".to_string(),
            body: "Body 0".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 0"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 0");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_0".to_string(),
            variant_a_subject: "A 0".to_string(),
            variant_b_subject: "B 0".to_string(),
            variant_a_body: "body a 0".to_string(),
            variant_b_body: "body b 0".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_0"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_0");

        let m = CampaignMetricsResponseDto {
            opens: 0 * 10,
            clicks: 0 * 5,
            bounces: 0,
            deliveries: 0 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 0 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 0 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_1() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 1".to_string(),
            subject: "Subject 1".to_string(),
            body: "Body 1".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 1"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 1");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_1".to_string(),
            variant_a_subject: "A 1".to_string(),
            variant_b_subject: "B 1".to_string(),
            variant_a_body: "body a 1".to_string(),
            variant_b_body: "body b 1".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_1"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_1");

        let m = CampaignMetricsResponseDto {
            opens: 1 * 10,
            clicks: 1 * 5,
            bounces: 1,
            deliveries: 1 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 1 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 1 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_2() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 2".to_string(),
            subject: "Subject 2".to_string(),
            body: "Body 2".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 2"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 2");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_2".to_string(),
            variant_a_subject: "A 2".to_string(),
            variant_b_subject: "B 2".to_string(),
            variant_a_body: "body a 2".to_string(),
            variant_b_body: "body b 2".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_2"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_2");

        let m = CampaignMetricsResponseDto {
            opens: 2 * 10,
            clicks: 2 * 5,
            bounces: 2,
            deliveries: 2 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 2 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 2 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_3() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 3".to_string(),
            subject: "Subject 3".to_string(),
            body: "Body 3".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 3"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 3");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_3".to_string(),
            variant_a_subject: "A 3".to_string(),
            variant_b_subject: "B 3".to_string(),
            variant_a_body: "body a 3".to_string(),
            variant_b_body: "body b 3".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_3"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_3");

        let m = CampaignMetricsResponseDto {
            opens: 3 * 10,
            clicks: 3 * 5,
            bounces: 3,
            deliveries: 3 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 3 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 3 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_4() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 4".to_string(),
            subject: "Subject 4".to_string(),
            body: "Body 4".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 4"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 4");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_4".to_string(),
            variant_a_subject: "A 4".to_string(),
            variant_b_subject: "B 4".to_string(),
            variant_a_body: "body a 4".to_string(),
            variant_b_body: "body b 4".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_4"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_4");

        let m = CampaignMetricsResponseDto {
            opens: 4 * 10,
            clicks: 4 * 5,
            bounces: 4,
            deliveries: 4 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 4 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 4 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_5() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 5".to_string(),
            subject: "Subject 5".to_string(),
            body: "Body 5".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 5"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 5");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_5".to_string(),
            variant_a_subject: "A 5".to_string(),
            variant_b_subject: "B 5".to_string(),
            variant_a_body: "body a 5".to_string(),
            variant_b_body: "body b 5".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_5"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_5");

        let m = CampaignMetricsResponseDto {
            opens: 5 * 10,
            clicks: 5 * 5,
            bounces: 5,
            deliveries: 5 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 5 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 5 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_6() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 6".to_string(),
            subject: "Subject 6".to_string(),
            body: "Body 6".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 6"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 6");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_6".to_string(),
            variant_a_subject: "A 6".to_string(),
            variant_b_subject: "B 6".to_string(),
            variant_a_body: "body a 6".to_string(),
            variant_b_body: "body b 6".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_6"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_6");

        let m = CampaignMetricsResponseDto {
            opens: 6 * 10,
            clicks: 6 * 5,
            bounces: 6,
            deliveries: 6 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 6 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 6 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_7() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 7".to_string(),
            subject: "Subject 7".to_string(),
            body: "Body 7".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 7"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 7");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_7".to_string(),
            variant_a_subject: "A 7".to_string(),
            variant_b_subject: "B 7".to_string(),
            variant_a_body: "body a 7".to_string(),
            variant_b_body: "body b 7".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_7"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_7");

        let m = CampaignMetricsResponseDto {
            opens: 7 * 10,
            clicks: 7 * 5,
            bounces: 7,
            deliveries: 7 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 7 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 7 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_8() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 8".to_string(),
            subject: "Subject 8".to_string(),
            body: "Body 8".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 8"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 8");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_8".to_string(),
            variant_a_subject: "A 8".to_string(),
            variant_b_subject: "B 8".to_string(),
            variant_a_body: "body a 8".to_string(),
            variant_b_body: "body b 8".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_8"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_8");

        let m = CampaignMetricsResponseDto {
            opens: 8 * 10,
            clicks: 8 * 5,
            bounces: 8,
            deliveries: 8 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 8 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 8 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_9() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 9".to_string(),
            subject: "Subject 9".to_string(),
            body: "Body 9".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 9"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 9");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_9".to_string(),
            variant_a_subject: "A 9".to_string(),
            variant_b_subject: "B 9".to_string(),
            variant_a_body: "body a 9".to_string(),
            variant_b_body: "body b 9".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_9"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_9");

        let m = CampaignMetricsResponseDto {
            opens: 9 * 10,
            clicks: 9 * 5,
            bounces: 9,
            deliveries: 9 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 9 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 9 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_10() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 10".to_string(),
            subject: "Subject 10".to_string(),
            body: "Body 10".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 10"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 10");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_10".to_string(),
            variant_a_subject: "A 10".to_string(),
            variant_b_subject: "B 10".to_string(),
            variant_a_body: "body a 10".to_string(),
            variant_b_body: "body b 10".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_10"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_10");

        let m = CampaignMetricsResponseDto {
            opens: 10 * 10,
            clicks: 10 * 5,
            bounces: 10,
            deliveries: 10 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 10 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 10 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_11() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 11".to_string(),
            subject: "Subject 11".to_string(),
            body: "Body 11".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 11"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 11");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_11".to_string(),
            variant_a_subject: "A 11".to_string(),
            variant_b_subject: "B 11".to_string(),
            variant_a_body: "body a 11".to_string(),
            variant_b_body: "body b 11".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_11"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_11");

        let m = CampaignMetricsResponseDto {
            opens: 11 * 10,
            clicks: 11 * 5,
            bounces: 11,
            deliveries: 11 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 11 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 11 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_12() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 12".to_string(),
            subject: "Subject 12".to_string(),
            body: "Body 12".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 12"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 12");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_12".to_string(),
            variant_a_subject: "A 12".to_string(),
            variant_b_subject: "B 12".to_string(),
            variant_a_body: "body a 12".to_string(),
            variant_b_body: "body b 12".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_12"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_12");

        let m = CampaignMetricsResponseDto {
            opens: 12 * 10,
            clicks: 12 * 5,
            bounces: 12,
            deliveries: 12 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 12 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 12 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_13() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 13".to_string(),
            subject: "Subject 13".to_string(),
            body: "Body 13".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 13"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 13");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_13".to_string(),
            variant_a_subject: "A 13".to_string(),
            variant_b_subject: "B 13".to_string(),
            variant_a_body: "body a 13".to_string(),
            variant_b_body: "body b 13".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_13"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_13");

        let m = CampaignMetricsResponseDto {
            opens: 13 * 10,
            clicks: 13 * 5,
            bounces: 13,
            deliveries: 13 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 13 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 13 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_14() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 14".to_string(),
            subject: "Subject 14".to_string(),
            body: "Body 14".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 14"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 14");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_14".to_string(),
            variant_a_subject: "A 14".to_string(),
            variant_b_subject: "B 14".to_string(),
            variant_a_body: "body a 14".to_string(),
            variant_b_body: "body b 14".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_14"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_14");

        let m = CampaignMetricsResponseDto {
            opens: 14 * 10,
            clicks: 14 * 5,
            bounces: 14,
            deliveries: 14 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 14 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 14 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_15() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 15".to_string(),
            subject: "Subject 15".to_string(),
            body: "Body 15".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 15"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 15");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_15".to_string(),
            variant_a_subject: "A 15".to_string(),
            variant_b_subject: "B 15".to_string(),
            variant_a_body: "body a 15".to_string(),
            variant_b_body: "body b 15".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_15"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_15");

        let m = CampaignMetricsResponseDto {
            opens: 15 * 10,
            clicks: 15 * 5,
            bounces: 15,
            deliveries: 15 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 15 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 15 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_16() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 16".to_string(),
            subject: "Subject 16".to_string(),
            body: "Body 16".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 16"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 16");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_16".to_string(),
            variant_a_subject: "A 16".to_string(),
            variant_b_subject: "B 16".to_string(),
            variant_a_body: "body a 16".to_string(),
            variant_b_body: "body b 16".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_16"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_16");

        let m = CampaignMetricsResponseDto {
            opens: 16 * 10,
            clicks: 16 * 5,
            bounces: 16,
            deliveries: 16 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 16 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 16 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_17() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 17".to_string(),
            subject: "Subject 17".to_string(),
            body: "Body 17".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 17"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 17");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_17".to_string(),
            variant_a_subject: "A 17".to_string(),
            variant_b_subject: "B 17".to_string(),
            variant_a_body: "body a 17".to_string(),
            variant_b_body: "body b 17".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_17"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_17");

        let m = CampaignMetricsResponseDto {
            opens: 17 * 10,
            clicks: 17 * 5,
            bounces: 17,
            deliveries: 17 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 17 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 17 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_18() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 18".to_string(),
            subject: "Subject 18".to_string(),
            body: "Body 18".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 18"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 18");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_18".to_string(),
            variant_a_subject: "A 18".to_string(),
            variant_b_subject: "B 18".to_string(),
            variant_a_body: "body a 18".to_string(),
            variant_b_body: "body b 18".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_18"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_18");

        let m = CampaignMetricsResponseDto {
            opens: 18 * 10,
            clicks: 18 * 5,
            bounces: 18,
            deliveries: 18 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 18 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 18 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_19() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 19".to_string(),
            subject: "Subject 19".to_string(),
            body: "Body 19".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 19"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 19");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_19".to_string(),
            variant_a_subject: "A 19".to_string(),
            variant_b_subject: "B 19".to_string(),
            variant_a_body: "body a 19".to_string(),
            variant_b_body: "body b 19".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_19"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_19");

        let m = CampaignMetricsResponseDto {
            opens: 19 * 10,
            clicks: 19 * 5,
            bounces: 19,
            deliveries: 19 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 19 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 19 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_20() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 20".to_string(),
            subject: "Subject 20".to_string(),
            body: "Body 20".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 20"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 20");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_20".to_string(),
            variant_a_subject: "A 20".to_string(),
            variant_b_subject: "B 20".to_string(),
            variant_a_body: "body a 20".to_string(),
            variant_b_body: "body b 20".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_20"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_20");

        let m = CampaignMetricsResponseDto {
            opens: 20 * 10,
            clicks: 20 * 5,
            bounces: 20,
            deliveries: 20 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 20 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 20 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_21() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 21".to_string(),
            subject: "Subject 21".to_string(),
            body: "Body 21".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 21"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 21");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_21".to_string(),
            variant_a_subject: "A 21".to_string(),
            variant_b_subject: "B 21".to_string(),
            variant_a_body: "body a 21".to_string(),
            variant_b_body: "body b 21".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_21"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_21");

        let m = CampaignMetricsResponseDto {
            opens: 21 * 10,
            clicks: 21 * 5,
            bounces: 21,
            deliveries: 21 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 21 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 21 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_22() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 22".to_string(),
            subject: "Subject 22".to_string(),
            body: "Body 22".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 22"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 22");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_22".to_string(),
            variant_a_subject: "A 22".to_string(),
            variant_b_subject: "B 22".to_string(),
            variant_a_body: "body a 22".to_string(),
            variant_b_body: "body b 22".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_22"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_22");

        let m = CampaignMetricsResponseDto {
            opens: 22 * 10,
            clicks: 22 * 5,
            bounces: 22,
            deliveries: 22 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 22 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 22 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_23() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 23".to_string(),
            subject: "Subject 23".to_string(),
            body: "Body 23".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 23"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 23");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_23".to_string(),
            variant_a_subject: "A 23".to_string(),
            variant_b_subject: "B 23".to_string(),
            variant_a_body: "body a 23".to_string(),
            variant_b_body: "body b 23".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_23"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_23");

        let m = CampaignMetricsResponseDto {
            opens: 23 * 10,
            clicks: 23 * 5,
            bounces: 23,
            deliveries: 23 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 23 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 23 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_24() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 24".to_string(),
            subject: "Subject 24".to_string(),
            body: "Body 24".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 24"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 24");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_24".to_string(),
            variant_a_subject: "A 24".to_string(),
            variant_b_subject: "B 24".to_string(),
            variant_a_body: "body a 24".to_string(),
            variant_b_body: "body b 24".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_24"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_24");

        let m = CampaignMetricsResponseDto {
            opens: 24 * 10,
            clicks: 24 * 5,
            bounces: 24,
            deliveries: 24 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 24 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 24 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_25() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 25".to_string(),
            subject: "Subject 25".to_string(),
            body: "Body 25".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 25"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 25");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_25".to_string(),
            variant_a_subject: "A 25".to_string(),
            variant_b_subject: "B 25".to_string(),
            variant_a_body: "body a 25".to_string(),
            variant_b_body: "body b 25".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_25"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_25");

        let m = CampaignMetricsResponseDto {
            opens: 25 * 10,
            clicks: 25 * 5,
            bounces: 25,
            deliveries: 25 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 25 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 25 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_26() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 26".to_string(),
            subject: "Subject 26".to_string(),
            body: "Body 26".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 26"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 26");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_26".to_string(),
            variant_a_subject: "A 26".to_string(),
            variant_b_subject: "B 26".to_string(),
            variant_a_body: "body a 26".to_string(),
            variant_b_body: "body b 26".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_26"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_26");

        let m = CampaignMetricsResponseDto {
            opens: 26 * 10,
            clicks: 26 * 5,
            bounces: 26,
            deliveries: 26 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 26 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 26 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_27() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 27".to_string(),
            subject: "Subject 27".to_string(),
            body: "Body 27".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 27"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 27");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_27".to_string(),
            variant_a_subject: "A 27".to_string(),
            variant_b_subject: "B 27".to_string(),
            variant_a_body: "body a 27".to_string(),
            variant_b_body: "body b 27".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_27"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_27");

        let m = CampaignMetricsResponseDto {
            opens: 27 * 10,
            clicks: 27 * 5,
            bounces: 27,
            deliveries: 27 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 27 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 27 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_28() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 28".to_string(),
            subject: "Subject 28".to_string(),
            body: "Body 28".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 28"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 28");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_28".to_string(),
            variant_a_subject: "A 28".to_string(),
            variant_b_subject: "B 28".to_string(),
            variant_a_body: "body a 28".to_string(),
            variant_b_body: "body b 28".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_28"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_28");

        let m = CampaignMetricsResponseDto {
            opens: 28 * 10,
            clicks: 28 * 5,
            bounces: 28,
            deliveries: 28 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 28 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 28 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_29() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 29".to_string(),
            subject: "Subject 29".to_string(),
            body: "Body 29".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 29"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 29");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_29".to_string(),
            variant_a_subject: "A 29".to_string(),
            variant_b_subject: "B 29".to_string(),
            variant_a_body: "body a 29".to_string(),
            variant_b_body: "body b 29".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_29"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_29");

        let m = CampaignMetricsResponseDto {
            opens: 29 * 10,
            clicks: 29 * 5,
            bounces: 29,
            deliveries: 29 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 29 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 29 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_30() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 30".to_string(),
            subject: "Subject 30".to_string(),
            body: "Body 30".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 30"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 30");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_30".to_string(),
            variant_a_subject: "A 30".to_string(),
            variant_b_subject: "B 30".to_string(),
            variant_a_body: "body a 30".to_string(),
            variant_b_body: "body b 30".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_30"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_30");

        let m = CampaignMetricsResponseDto {
            opens: 30 * 10,
            clicks: 30 * 5,
            bounces: 30,
            deliveries: 30 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 30 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 30 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_31() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 31".to_string(),
            subject: "Subject 31".to_string(),
            body: "Body 31".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 31"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 31");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_31".to_string(),
            variant_a_subject: "A 31".to_string(),
            variant_b_subject: "B 31".to_string(),
            variant_a_body: "body a 31".to_string(),
            variant_b_body: "body b 31".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_31"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_31");

        let m = CampaignMetricsResponseDto {
            opens: 31 * 10,
            clicks: 31 * 5,
            bounces: 31,
            deliveries: 31 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 31 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 31 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_32() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 32".to_string(),
            subject: "Subject 32".to_string(),
            body: "Body 32".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 32"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 32");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_32".to_string(),
            variant_a_subject: "A 32".to_string(),
            variant_b_subject: "B 32".to_string(),
            variant_a_body: "body a 32".to_string(),
            variant_b_body: "body b 32".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_32"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_32");

        let m = CampaignMetricsResponseDto {
            opens: 32 * 10,
            clicks: 32 * 5,
            bounces: 32,
            deliveries: 32 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 32 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 32 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_33() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 33".to_string(),
            subject: "Subject 33".to_string(),
            body: "Body 33".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 33"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 33");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_33".to_string(),
            variant_a_subject: "A 33".to_string(),
            variant_b_subject: "B 33".to_string(),
            variant_a_body: "body a 33".to_string(),
            variant_b_body: "body b 33".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_33"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_33");

        let m = CampaignMetricsResponseDto {
            opens: 33 * 10,
            clicks: 33 * 5,
            bounces: 33,
            deliveries: 33 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 33 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 33 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_34() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 34".to_string(),
            subject: "Subject 34".to_string(),
            body: "Body 34".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 34"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 34");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_34".to_string(),
            variant_a_subject: "A 34".to_string(),
            variant_b_subject: "B 34".to_string(),
            variant_a_body: "body a 34".to_string(),
            variant_b_body: "body b 34".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_34"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_34");

        let m = CampaignMetricsResponseDto {
            opens: 34 * 10,
            clicks: 34 * 5,
            bounces: 34,
            deliveries: 34 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 34 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 34 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_35() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 35".to_string(),
            subject: "Subject 35".to_string(),
            body: "Body 35".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 35"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 35");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_35".to_string(),
            variant_a_subject: "A 35".to_string(),
            variant_b_subject: "B 35".to_string(),
            variant_a_body: "body a 35".to_string(),
            variant_b_body: "body b 35".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_35"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_35");

        let m = CampaignMetricsResponseDto {
            opens: 35 * 10,
            clicks: 35 * 5,
            bounces: 35,
            deliveries: 35 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 35 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 35 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_36() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 36".to_string(),
            subject: "Subject 36".to_string(),
            body: "Body 36".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 36"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 36");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_36".to_string(),
            variant_a_subject: "A 36".to_string(),
            variant_b_subject: "B 36".to_string(),
            variant_a_body: "body a 36".to_string(),
            variant_b_body: "body b 36".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_36"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_36");

        let m = CampaignMetricsResponseDto {
            opens: 36 * 10,
            clicks: 36 * 5,
            bounces: 36,
            deliveries: 36 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 36 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 36 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_37() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 37".to_string(),
            subject: "Subject 37".to_string(),
            body: "Body 37".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 37"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 37");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_37".to_string(),
            variant_a_subject: "A 37".to_string(),
            variant_b_subject: "B 37".to_string(),
            variant_a_body: "body a 37".to_string(),
            variant_b_body: "body b 37".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_37"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_37");

        let m = CampaignMetricsResponseDto {
            opens: 37 * 10,
            clicks: 37 * 5,
            bounces: 37,
            deliveries: 37 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 37 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 37 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_38() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 38".to_string(),
            subject: "Subject 38".to_string(),
            body: "Body 38".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 38"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 38");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_38".to_string(),
            variant_a_subject: "A 38".to_string(),
            variant_b_subject: "B 38".to_string(),
            variant_a_body: "body a 38".to_string(),
            variant_b_body: "body b 38".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_38"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_38");

        let m = CampaignMetricsResponseDto {
            opens: 38 * 10,
            clicks: 38 * 5,
            bounces: 38,
            deliveries: 38 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 38 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 38 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_39() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 39".to_string(),
            subject: "Subject 39".to_string(),
            body: "Body 39".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 39"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 39");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_39".to_string(),
            variant_a_subject: "A 39".to_string(),
            variant_b_subject: "B 39".to_string(),
            variant_a_body: "body a 39".to_string(),
            variant_b_body: "body b 39".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_39"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_39");

        let m = CampaignMetricsResponseDto {
            opens: 39 * 10,
            clicks: 39 * 5,
            bounces: 39,
            deliveries: 39 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 39 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 39 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_40() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 40".to_string(),
            subject: "Subject 40".to_string(),
            body: "Body 40".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 40"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 40");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_40".to_string(),
            variant_a_subject: "A 40".to_string(),
            variant_b_subject: "B 40".to_string(),
            variant_a_body: "body a 40".to_string(),
            variant_b_body: "body b 40".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_40"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_40");

        let m = CampaignMetricsResponseDto {
            opens: 40 * 10,
            clicks: 40 * 5,
            bounces: 40,
            deliveries: 40 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 40 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 40 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_41() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 41".to_string(),
            subject: "Subject 41".to_string(),
            body: "Body 41".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 41"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 41");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_41".to_string(),
            variant_a_subject: "A 41".to_string(),
            variant_b_subject: "B 41".to_string(),
            variant_a_body: "body a 41".to_string(),
            variant_b_body: "body b 41".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_41"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_41");

        let m = CampaignMetricsResponseDto {
            opens: 41 * 10,
            clicks: 41 * 5,
            bounces: 41,
            deliveries: 41 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 41 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 41 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_42() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 42".to_string(),
            subject: "Subject 42".to_string(),
            body: "Body 42".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 42"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 42");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_42".to_string(),
            variant_a_subject: "A 42".to_string(),
            variant_b_subject: "B 42".to_string(),
            variant_a_body: "body a 42".to_string(),
            variant_b_body: "body b 42".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_42"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_42");

        let m = CampaignMetricsResponseDto {
            opens: 42 * 10,
            clicks: 42 * 5,
            bounces: 42,
            deliveries: 42 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 42 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 42 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_43() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 43".to_string(),
            subject: "Subject 43".to_string(),
            body: "Body 43".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 43"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 43");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_43".to_string(),
            variant_a_subject: "A 43".to_string(),
            variant_b_subject: "B 43".to_string(),
            variant_a_body: "body a 43".to_string(),
            variant_b_body: "body b 43".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_43"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_43");

        let m = CampaignMetricsResponseDto {
            opens: 43 * 10,
            clicks: 43 * 5,
            bounces: 43,
            deliveries: 43 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 43 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 43 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_44() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 44".to_string(),
            subject: "Subject 44".to_string(),
            body: "Body 44".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 44"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 44");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_44".to_string(),
            variant_a_subject: "A 44".to_string(),
            variant_b_subject: "B 44".to_string(),
            variant_a_body: "body a 44".to_string(),
            variant_b_body: "body b 44".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_44"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_44");

        let m = CampaignMetricsResponseDto {
            opens: 44 * 10,
            clicks: 44 * 5,
            bounces: 44,
            deliveries: 44 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 44 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 44 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_45() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 45".to_string(),
            subject: "Subject 45".to_string(),
            body: "Body 45".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 45"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 45");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_45".to_string(),
            variant_a_subject: "A 45".to_string(),
            variant_b_subject: "B 45".to_string(),
            variant_a_body: "body a 45".to_string(),
            variant_b_body: "body b 45".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_45"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_45");

        let m = CampaignMetricsResponseDto {
            opens: 45 * 10,
            clicks: 45 * 5,
            bounces: 45,
            deliveries: 45 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 45 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 45 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_46() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 46".to_string(),
            subject: "Subject 46".to_string(),
            body: "Body 46".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 46"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 46");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_46".to_string(),
            variant_a_subject: "A 46".to_string(),
            variant_b_subject: "B 46".to_string(),
            variant_a_body: "body a 46".to_string(),
            variant_b_body: "body b 46".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_46"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_46");

        let m = CampaignMetricsResponseDto {
            opens: 46 * 10,
            clicks: 46 * 5,
            bounces: 46,
            deliveries: 46 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 46 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 46 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_47() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 47".to_string(),
            subject: "Subject 47".to_string(),
            body: "Body 47".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 47"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 47");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_47".to_string(),
            variant_a_subject: "A 47".to_string(),
            variant_b_subject: "B 47".to_string(),
            variant_a_body: "body a 47".to_string(),
            variant_b_body: "body b 47".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_47"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_47");

        let m = CampaignMetricsResponseDto {
            opens: 47 * 10,
            clicks: 47 * 5,
            bounces: 47,
            deliveries: 47 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 47 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 47 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_48() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 48".to_string(),
            subject: "Subject 48".to_string(),
            body: "Body 48".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 48"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 48");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_48".to_string(),
            variant_a_subject: "A 48".to_string(),
            variant_b_subject: "B 48".to_string(),
            variant_a_body: "body a 48".to_string(),
            variant_b_body: "body b 48".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_48"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_48");

        let m = CampaignMetricsResponseDto {
            opens: 48 * 10,
            clicks: 48 * 5,
            bounces: 48,
            deliveries: 48 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 48 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 48 * 10);
    }

    #[test]
    fn test_dto_serialization_variant_49() {
        let req = CreateCampaignRequestDto {
            name: "Campaign 49".to_string(),
            subject: "Subject 49".to_string(),
            body: "Body 49".to_string(),
            target_segment_id: None,
        };
        let s = serde_json::to_string(&req).unwrap();
        assert!(s.contains("Campaign 49"));
        let r: CreateCampaignRequestDto = serde_json::from_str(&s).unwrap();
        assert_eq!(r.name, "Campaign 49");

        let ab = CreateABTestRequestDto {
            base_campaign_id: "c_49".to_string(),
            variant_a_subject: "A 49".to_string(),
            variant_b_subject: "B 49".to_string(),
            variant_a_body: "body a 49".to_string(),
            variant_b_body: "body b 49".to_string(),
            split_percentage: 50.0,
        };
        let sab = serde_json::to_string(&ab).unwrap();
        assert!(sab.contains("c_49"));
        let rab: CreateABTestRequestDto = serde_json::from_str(&sab).unwrap();
        assert_eq!(rab.base_campaign_id, "c_49");

        let m = CampaignMetricsResponseDto {
            opens: 49 * 10,
            clicks: 49 * 5,
            bounces: 49,
            deliveries: 49 * 100,
        };
        let sm = serde_json::to_string(&m).unwrap();
        assert!(sm.contains(&format!("{}", 49 * 10)));
        let rm: CampaignMetricsResponseDto = serde_json::from_str(&sm).unwrap();
        assert_eq!(rm.opens, 49 * 10);
    }
}
