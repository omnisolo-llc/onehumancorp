use super::models::*;
use super::repository::GrowthRepository;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct GrowthService {
    repo: Arc<dyn GrowthRepository>,
}

impl GrowthService {
    pub fn new(repo: Arc<dyn GrowthRepository>) -> Self {
        Self { repo }
    }

    pub async fn process_referral_invite(&self, referrer_id: &str, email: &str) -> Result<Uuid, GrowthError> {
        let referral = Referral {
            id: Uuid::new_v4(),
            referrer_id: referrer_id.to_string(),
            referred_email: email.to_string(),
            status: ReferralStatus::Pending,
            credits_awarded: 0,
            created_at: Utc::now(),
            completed_at: None,
        };
        self.repo.insert_referral(&referral).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        Ok(referral.id)
    }

    pub async fn complete_referral(&self, id: &Uuid) -> Result<(), GrowthError> {
        let r = self.repo.get_referral(id).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?
            .ok_or(GrowthError::new("NOT_FOUND", "Referral not found"))?;
        if r.status != ReferralStatus::Pending {
            return Err(GrowthError::new("INVALID_STATE", "Referral not pending"));
        }
        self.repo.update_referral_status(id, &ReferralStatus::Completed).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        Ok(())
    }

    pub async fn generate_share_card(&self, business_id: &str, title: &str, tagline: &str) -> Result<ShareCard, GrowthError> {
        let existing = self.repo.get_share_card(business_id).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        if let Some(card) = existing {
            return Ok(card);
        }

        let card = ShareCard {
            id: Uuid::new_v4(),
            business_id: business_id.to_string(),
            title: title.to_string(),
            tagline: tagline.to_string(),
            template: "glassmorphism".to_string(),
            embed_html: format!("<iframe src='https://ohc.app/embed/{}'></iframe>", business_id),
            primary_color: "#ffffff".to_string(),
            logo_url: None,
            created_at: Utc::now(),
        };
        self.repo.insert_share_card(&card).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        Ok(card)
    }

    pub async fn schedule_social_post(&self, business_id: &str, content: &str, platform: &str) -> Result<Uuid, GrowthError> {
        let config = self.repo.get_social_config(business_id).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;

        let status = if let Some(c) = config {
            if c.auto_approve { "scheduled".to_string() } else { "pending_approval".to_string() }
        } else {
            "draft".to_string()
        };

        let post = SocialPost {
            id: Uuid::new_v4(),
            business_id: business_id.to_string(),
            platform: platform.to_string(),
            content: content.to_string(),
            media_url: None,
            status,
            scheduled_time: Some(Utc::now() + chrono::Duration::hours(2)),
            posted_at: None,
        };
        self.repo.insert_social_post(&post).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        Ok(post.id)
    }

    pub async fn launch_email_campaign(&self, business_id: &str, subject: &str, html_body: &str) -> Result<Uuid, GrowthError> {
        let campaign = EmailCampaign {
            id: Uuid::new_v4(),
            business_id: business_id.to_string(),
            subject: subject.to_string(),
            html_content: html_body.to_string(),
            text_content: "View in browser".to_string(),
            audience_filter: serde_json::json!({"active": true}),
            status: "sending".to_string(),
            sent_count: 0,
            open_count: 0,
            click_count: 0,
        };
        self.repo.insert_campaign(&campaign).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        Ok(campaign.id)
    }

    pub async fn check_and_unlock_milestone(&self, business_id: &str, metric: &str, value: i32) -> Result<Option<SuccessMilestone>, GrowthError> {
        let existing = self.repo.get_milestones(business_id).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;

        let thresholds = vec![10, 100, 1000, 10000];
        for t in thresholds {
            if value >= t && !existing.iter().any(|m| m.metric_type == metric && m.threshold == t) {
                let milestone = SuccessMilestone {
                    id: Uuid::new_v4(),
                    business_id: business_id.to_string(),
                    metric_type: metric.to_string(),
                    threshold: t,
                    notification_sent: false,
                    unlocked_at: Utc::now(),
                };
                self.repo.insert_milestone(&milestone).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
                return Ok(Some(milestone));
            }
        }
        Ok(None)
    }

    pub async fn verify_free_tier_limits(&self, business_id: &str) -> Result<bool, GrowthError> {
        let tier = self.repo.get_business_tier(business_id).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        if let Some(t) = tier {
            if t.current_tier == "Free" {
                let usage = t.usage.get("products").and_then(|v| v.as_i64()).unwrap_or(0);
                if usage >= 10 {
                    return Ok(false); // Upgrade required
                }
            }
        }
        Ok(true)
    }

    pub async fn initialize_viral_storefront(&self, business_id: &str) -> Result<(), GrowthError> {
        let tier = self.repo.get_business_tier(business_id).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        let show_branding = tier.map(|t| t.current_tier == "Free").unwrap_or(true);

        let storefront = ViralStorefront {
            business_id: business_id.to_string(),
            public_url: format!("https://{}.ohc.app", business_id),
            show_ohc_branding: show_branding,
            conversion_clicks: 0,
        };
        self.repo.upsert_storefront(&storefront).await.map_err(|e| GrowthError::new("DB_ERROR", &e.to_string()))?;
        Ok(())
    }
}
