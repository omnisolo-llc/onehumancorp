use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReferralStatus {
    Pending,
    Completed,
    Expired,
    Fraudulent,
}

impl fmt::Display for ReferralStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReferralStatus::Pending => write!(f, "pending"),
            ReferralStatus::Completed => write!(f, "completed"),
            ReferralStatus::Expired => write!(f, "expired"),
            ReferralStatus::Fraudulent => write!(f, "fraudulent"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Referral {
    pub id: Uuid,
    pub referrer_id: String,
    pub referred_email: String,
    pub status: ReferralStatus,
    pub credits_awarded: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareCard {
    pub id: Uuid,
    pub business_id: String,
    pub title: String,
    pub tagline: String,
    pub template: String,
    pub embed_html: String,
    pub primary_color: String,
    pub logo_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAgentConfig {
    pub business_id: String,
    pub enabled: bool,
    pub connected_platforms: Vec<String>,
    pub auto_approve: bool,
    pub generation_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub id: Uuid,
    pub business_id: String,
    pub platform: String,
    pub content: String,
    pub media_url: Option<String>,
    pub status: String,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub posted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub id: Uuid,
    pub business_id: String,
    pub subject: String,
    pub html_content: String,
    pub text_content: String,
    pub audience_filter: serde_json::Value,
    pub status: String,
    pub sent_count: i32,
    pub open_count: i32,
    pub click_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPlan {
    pub tier_type: String,
    pub limits: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessTier {
    pub business_id: String,
    pub current_tier: String,
    pub usage: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViralStorefront {
    pub business_id: String,
    pub public_url: String,
    pub show_ohc_branding: bool,
    pub conversion_clicks: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessMilestone {
    pub id: Uuid,
    pub business_id: String,
    pub metric_type: String,
    pub threshold: i32,
    pub notification_sent: bool,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthError {
    pub code: String,
    pub message: String,
}

impl GrowthError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}
