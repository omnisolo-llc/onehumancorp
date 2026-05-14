use async_trait::async_trait;
use sqlx::{PgPool, Error};
use sqlx::Row;
use super::models::*;
use uuid::Uuid;
use std::str::FromStr;

#[async_trait]
pub trait GrowthRepository: Send + Sync {
    async fn insert_referral(&self, referral: &Referral) -> Result<(), Error>;
    async fn get_referral(&self, id: &Uuid) -> Result<Option<Referral>, Error>;
    async fn update_referral_status(&self, id: &Uuid, status: &ReferralStatus) -> Result<(), Error>;
    async fn get_referrals_by_referrer(&self, referrer_id: &str) -> Result<Vec<Referral>, Error>;
    async fn list_all_referrals(&self, limit: i64, offset: i64) -> Result<Vec<Referral>, Error>;
    async fn delete_referral(&self, id: &Uuid) -> Result<(), Error>;

    async fn insert_share_card(&self, card: &ShareCard) -> Result<(), Error>;
    async fn get_share_card(&self, business_id: &str) -> Result<Option<ShareCard>, Error>;
    async fn delete_share_card(&self, business_id: &str) -> Result<(), Error>;

    async fn upsert_social_config(&self, config: &SocialAgentConfig) -> Result<(), Error>;
    async fn get_social_config(&self, business_id: &str) -> Result<Option<SocialAgentConfig>, Error>;
    async fn delete_social_config(&self, business_id: &str) -> Result<(), Error>;

    async fn insert_social_post(&self, post: &SocialPost) -> Result<(), Error>;
    async fn update_social_post(&self, post: &SocialPost) -> Result<(), Error>;
    async fn get_social_post(&self, id: &Uuid) -> Result<Option<SocialPost>, Error>;
    async fn list_social_posts_by_business(&self, business_id: &str, limit: i64) -> Result<Vec<SocialPost>, Error>;
    async fn delete_social_post(&self, id: &Uuid) -> Result<(), Error>;

    async fn insert_campaign(&self, campaign: &EmailCampaign) -> Result<(), Error>;
    async fn update_campaign_metrics(&self, id: &Uuid, sent: i32, open: i32, click: i32) -> Result<(), Error>;
    async fn get_campaigns_by_business(&self, business_id: &str) -> Result<Vec<EmailCampaign>, Error>;
    async fn delete_campaign(&self, id: &Uuid) -> Result<(), Error>;

    async fn get_business_tier(&self, business_id: &str) -> Result<Option<BusinessTier>, Error>;
    async fn update_business_tier(&self, tier: &BusinessTier) -> Result<(), Error>;
    async fn delete_business_tier(&self, business_id: &str) -> Result<(), Error>;

    async fn upsert_storefront(&self, storefront: &ViralStorefront) -> Result<(), Error>;
    async fn get_storefront(&self, business_id: &str) -> Result<Option<ViralStorefront>, Error>;
    async fn delete_storefront(&self, business_id: &str) -> Result<(), Error>;

    async fn insert_milestone(&self, milestone: &SuccessMilestone) -> Result<(), Error>;
    async fn get_milestones(&self, business_id: &str) -> Result<Vec<SuccessMilestone>, Error>;
    async fn mark_milestone_notified(&self, id: &Uuid) -> Result<(), Error>;
    async fn delete_milestone(&self, id: &Uuid) -> Result<(), Error>;
}

pub struct PgGrowthRepo {
    pool: PgPool,
}

impl PgGrowthRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GrowthRepository for PgGrowthRepo {
    async fn insert_referral(&self, r: &Referral) -> Result<(), Error> {
        let status = match r.status {
            ReferralStatus::Completed => "completed",
            ReferralStatus::Expired => "expired",
            ReferralStatus::Fraudulent => "fraudulent",
            _ => "pending",
        };
        sqlx::query("INSERT INTO referrals (id, referrer_id, referred_email, status, credits_awarded, created_at, completed_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&r.id)
            .bind(&r.referrer_id)
            .bind(&r.referred_email)
            .bind(status)
            .bind(r.credits_awarded)
            .bind(r.created_at)
            .bind(r.completed_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_referral(&self, id: &Uuid) -> Result<Option<Referral>, Error> {
        let row = sqlx::query("SELECT id, referrer_id, referred_email, status, credits_awarded, created_at, completed_at FROM referrals WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            let status_str: String = r.try_get("status")?;
            let status: ReferralStatus = match status_str.as_str() { "completed" => ReferralStatus::Completed, "expired" => ReferralStatus::Expired, "fraudulent" => ReferralStatus::Fraudulent, _ => ReferralStatus::Pending };
            return Ok(Some(Referral {
                id: r.try_get("id")?,
                referrer_id: r.try_get("referrer_id")?,
                referred_email: r.try_get("referred_email")?,
                status,
                credits_awarded: r.try_get("credits_awarded")?,
                created_at: r.try_get("created_at")?,
                completed_at: r.try_get("completed_at").ok(),
            }));
        }
        Ok(None)
    }

    async fn update_referral_status(&self, id: &Uuid, status: &ReferralStatus) -> Result<(), Error> {
        let st = match status {
            ReferralStatus::Completed => "completed",
            ReferralStatus::Expired => "expired",
            ReferralStatus::Fraudulent => "fraudulent",
            _ => "pending",
        };
        sqlx::query("UPDATE referrals SET status = $1 WHERE id = $2").bind(st).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_referrals_by_referrer(&self, referrer_id: &str) -> Result<Vec<Referral>, Error> {
        let rows = sqlx::query("SELECT id, referrer_id, referred_email, status, credits_awarded, created_at, completed_at FROM referrals WHERE referrer_id = $1")
            .bind(referrer_id)
            .fetch_all(&self.pool).await?;
        let mut refs = Vec::new();
        for row in rows {
            let status_str: String = row.try_get("status")?;
            let status: ReferralStatus = match status_str.as_str() { "completed" => ReferralStatus::Completed, "expired" => ReferralStatus::Expired, "fraudulent" => ReferralStatus::Fraudulent, _ => ReferralStatus::Pending };
            refs.push(Referral {
                id: row.try_get("id")?,
                referrer_id: row.try_get("referrer_id")?,
                referred_email: row.try_get("referred_email")?,
                status,
                credits_awarded: row.try_get("credits_awarded")?,
                created_at: row.try_get("created_at")?,
                completed_at: row.try_get("completed_at").ok(),
            });
        }
        Ok(refs)
    }

    async fn list_all_referrals(&self, limit: i64, offset: i64) -> Result<Vec<Referral>, Error> {
        let rows = sqlx::query("SELECT id, referrer_id, referred_email, status, credits_awarded, created_at, completed_at FROM referrals ORDER BY created_at DESC LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool).await?;
        let mut refs = Vec::new();
        for row in rows {
            let status_str: String = row.try_get("status")?;
            let status: ReferralStatus = match status_str.as_str() { "completed" => ReferralStatus::Completed, "expired" => ReferralStatus::Expired, "fraudulent" => ReferralStatus::Fraudulent, _ => ReferralStatus::Pending };
            refs.push(Referral {
                id: row.try_get("id")?,
                referrer_id: row.try_get("referrer_id")?,
                referred_email: row.try_get("referred_email")?,
                status,
                credits_awarded: row.try_get("credits_awarded")?,
                created_at: row.try_get("created_at")?,
                completed_at: row.try_get("completed_at").ok(),
            });
        }
        Ok(refs)
    }

    async fn delete_referral(&self, id: &Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM referrals WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn insert_share_card(&self, card: &ShareCard) -> Result<(), Error> {
        sqlx::query("INSERT INTO share_cards (id, business_id, title, tagline, template, embed_html, primary_color, logo_url, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(&card.id)
            .bind(&card.business_id)
            .bind(&card.title)
            .bind(&card.tagline)
            .bind(&card.template)
            .bind(&card.embed_html)
            .bind(&card.primary_color)
            .bind(&card.logo_url)
            .bind(card.created_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_share_card(&self, business_id: &str) -> Result<Option<ShareCard>, Error> {
        let row = sqlx::query("SELECT id, business_id, title, tagline, template, embed_html, primary_color, logo_url, created_at FROM share_cards WHERE business_id = $1 LIMIT 1")
            .bind(business_id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            return Ok(Some(ShareCard {
                id: r.try_get("id")?,
                business_id: r.try_get("business_id")?,
                title: r.try_get("title")?,
                tagline: r.try_get("tagline")?,
                template: r.try_get("template")?,
                embed_html: r.try_get("embed_html")?,
                primary_color: r.try_get("primary_color")?,
                logo_url: r.try_get("logo_url").ok(),
                created_at: r.try_get("created_at")?,
            }));
        }
        Ok(None)
    }

    async fn delete_share_card(&self, business_id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM share_cards WHERE business_id = $1").bind(business_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn upsert_social_config(&self, config: &SocialAgentConfig) -> Result<(), Error> {
        let platforms = serde_json::to_string(&config.connected_platforms).unwrap_or_default();
        sqlx::query("INSERT INTO social_agent_configs (business_id, enabled, connected_platforms, auto_approve, generation_prompt) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (business_id) DO UPDATE SET enabled = EXCLUDED.enabled, connected_platforms = EXCLUDED.connected_platforms, auto_approve = EXCLUDED.auto_approve, generation_prompt = EXCLUDED.generation_prompt")
            .bind(&config.business_id)
            .bind(config.enabled)
            .bind(platforms)
            .bind(config.auto_approve)
            .bind(&config.generation_prompt)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_social_config(&self, business_id: &str) -> Result<Option<SocialAgentConfig>, Error> {
        let row = sqlx::query("SELECT business_id, enabled, connected_platforms, auto_approve, generation_prompt FROM social_agent_configs WHERE business_id = $1 LIMIT 1")
            .bind(business_id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            let platforms_str: String = r.try_get("connected_platforms")?;
            let connected_platforms: Vec<String> = serde_json::from_str(&platforms_str).unwrap_or_default();
            return Ok(Some(SocialAgentConfig {
                business_id: r.try_get("business_id")?,
                enabled: r.try_get("enabled")?,
                connected_platforms,
                auto_approve: r.try_get("auto_approve")?,
                generation_prompt: r.try_get("generation_prompt")?,
            }));
        }
        Ok(None)
    }

    async fn delete_social_config(&self, business_id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM social_agent_configs WHERE business_id = $1").bind(business_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn insert_social_post(&self, post: &SocialPost) -> Result<(), Error> {
        let status = post.status.to_string();
        sqlx::query("INSERT INTO social_posts (id, business_id, platform, content, media_url, status, scheduled_time, posted_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&post.id)
            .bind(&post.business_id)
            .bind(&post.platform)
            .bind(&post.content)
            .bind(&post.media_url)
            .bind(&status)
            .bind(post.scheduled_time)
            .bind(post.posted_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn update_social_post(&self, post: &SocialPost) -> Result<(), Error> {
        let status = post.status.to_string();
        sqlx::query("UPDATE social_posts SET status = $1, posted_at = $2 WHERE id = $3")
            .bind(&status).bind(post.posted_at).bind(&post.id).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_social_post(&self, id: &Uuid) -> Result<Option<SocialPost>, Error> {
        let row = sqlx::query("SELECT id, business_id, platform, content, media_url, status, scheduled_time, posted_at FROM social_posts WHERE id = $1 LIMIT 1")
            .bind(id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            let status_str: String = r.try_get("status")?;
            return Ok(Some(SocialPost {
                id: r.try_get("id")?,
                business_id: r.try_get("business_id")?,
                platform: r.try_get("platform")?,
                content: r.try_get("content")?,
                media_url: r.try_get("media_url").ok(),
                status: status_str,
                scheduled_time: r.try_get("scheduled_time").ok(),
                posted_at: r.try_get("posted_at").ok(),
            }));
        }
        Ok(None)
    }

    async fn list_social_posts_by_business(&self, business_id: &str, limit: i64) -> Result<Vec<SocialPost>, Error> {
        let rows = sqlx::query("SELECT id, business_id, platform, content, media_url, status, scheduled_time, posted_at FROM social_posts WHERE business_id = $1 ORDER BY scheduled_time DESC LIMIT $2")
            .bind(business_id).bind(limit).fetch_all(&self.pool).await?;
        let mut posts = Vec::new();
        for r in rows {
            let status_str: String = r.try_get("status")?;
            posts.push(SocialPost {
                id: r.try_get("id")?,
                business_id: r.try_get("business_id")?,
                platform: r.try_get("platform")?,
                content: r.try_get("content")?,
                media_url: r.try_get("media_url").ok(),
                status: status_str,
                scheduled_time: r.try_get("scheduled_time").ok(),
                posted_at: r.try_get("posted_at").ok(),
            });
        }
        Ok(posts)
    }

    async fn delete_social_post(&self, id: &Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM social_posts WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn insert_campaign(&self, campaign: &EmailCampaign) -> Result<(), Error> {
        let audience = serde_json::to_string(&campaign.audience_filter).unwrap_or_default();
        sqlx::query("INSERT INTO email_campaigns (id, business_id, subject, html_content, text_content, audience_filter, status, sent_count, open_count, click_count) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
            .bind(&campaign.id)
            .bind(&campaign.business_id)
            .bind(&campaign.subject)
            .bind(&campaign.html_content)
            .bind(&campaign.text_content)
            .bind(audience)
            .bind(&campaign.status)
            .bind(campaign.sent_count)
            .bind(campaign.open_count)
            .bind(campaign.click_count)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn update_campaign_metrics(&self, id: &Uuid, sent: i32, open: i32, click: i32) -> Result<(), Error> {
        sqlx::query("UPDATE email_campaigns SET sent_count = $1, open_count = $2, click_count = $3 WHERE id = $4")
            .bind(sent).bind(open).bind(click).bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_campaigns_by_business(&self, business_id: &str) -> Result<Vec<EmailCampaign>, Error> {
        let rows = sqlx::query("SELECT id, business_id, subject, html_content, text_content, audience_filter, status, sent_count, open_count, click_count FROM email_campaigns WHERE business_id = $1")
            .bind(business_id).fetch_all(&self.pool).await?;
        let mut camps = Vec::new();
        for r in rows {
            let aud_str: String = r.try_get("audience_filter")?;
            camps.push(EmailCampaign {
                id: r.try_get("id")?,
                business_id: r.try_get("business_id")?,
                subject: r.try_get("subject")?,
                html_content: r.try_get("html_content")?,
                text_content: r.try_get("text_content")?,
                audience_filter: serde_json::from_str(&aud_str).unwrap_or_default(),
                status: r.try_get("status")?,
                sent_count: r.try_get("sent_count")?,
                open_count: r.try_get("open_count")?,
                click_count: r.try_get("click_count")?,
            });
        }
        Ok(camps)
    }

    async fn delete_campaign(&self, id: &Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM email_campaigns WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_business_tier(&self, business_id: &str) -> Result<Option<BusinessTier>, Error> {
        let row = sqlx::query("SELECT business_id, current_tier, usage FROM business_tiers WHERE business_id = $1 LIMIT 1")
            .bind(business_id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            let usage_str: String = r.try_get("usage")?;
            let usage = serde_json::from_str(&usage_str).unwrap_or(serde_json::json!({}));
            return Ok(Some(BusinessTier {
                business_id: r.try_get("business_id")?,
                current_tier: r.try_get("current_tier")?,
                usage,
            }));
        }
        Ok(None)
    }

    async fn update_business_tier(&self, tier: &BusinessTier) -> Result<(), Error> {
        let usage = serde_json::to_string(&tier.usage).unwrap_or_default();
        sqlx::query("INSERT INTO business_tiers (business_id, current_tier, usage) VALUES ($1, $2, $3) ON CONFLICT (business_id) DO UPDATE SET current_tier = EXCLUDED.current_tier, usage = EXCLUDED.usage")
            .bind(&tier.business_id).bind(&tier.current_tier).bind(usage).execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_business_tier(&self, business_id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM business_tiers WHERE business_id = $1").bind(business_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn upsert_storefront(&self, storefront: &ViralStorefront) -> Result<(), Error> {
        sqlx::query("INSERT INTO viral_storefronts (business_id, public_url, show_ohc_branding, conversion_clicks) VALUES ($1, $2, $3, $4) ON CONFLICT (business_id) DO UPDATE SET public_url = EXCLUDED.public_url, show_ohc_branding = EXCLUDED.show_ohc_branding, conversion_clicks = EXCLUDED.conversion_clicks")
            .bind(&storefront.business_id).bind(&storefront.public_url).bind(storefront.show_ohc_branding).bind(storefront.conversion_clicks).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_storefront(&self, business_id: &str) -> Result<Option<ViralStorefront>, Error> {
        let row = sqlx::query("SELECT business_id, public_url, show_ohc_branding, conversion_clicks FROM viral_storefronts WHERE business_id = $1 LIMIT 1")
            .bind(business_id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            return Ok(Some(ViralStorefront {
                business_id: r.try_get("business_id")?,
                public_url: r.try_get("public_url")?,
                show_ohc_branding: r.try_get("show_ohc_branding")?,
                conversion_clicks: r.try_get("conversion_clicks")?,
            }));
        }
        Ok(None)
    }

    async fn delete_storefront(&self, business_id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM viral_storefronts WHERE business_id = $1").bind(business_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn insert_milestone(&self, milestone: &SuccessMilestone) -> Result<(), Error> {
        sqlx::query("INSERT INTO success_milestones (id, business_id, metric_type, threshold, notification_sent, unlocked_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING")
            .bind(&milestone.id).bind(&milestone.business_id).bind(&milestone.metric_type).bind(milestone.threshold).bind(milestone.notification_sent).bind(milestone.unlocked_at).execute(&self.pool).await?;
        Ok(())
    }

    async fn get_milestones(&self, business_id: &str) -> Result<Vec<SuccessMilestone>, Error> {
        let rows = sqlx::query("SELECT id, business_id, metric_type, threshold, notification_sent, unlocked_at FROM success_milestones WHERE business_id = $1")
            .bind(business_id)
            .fetch_all(&self.pool).await?;
        let mut miles = Vec::new();
        for row in rows {
            miles.push(SuccessMilestone {
                id: row.try_get("id")?,
                business_id: row.try_get("business_id")?,
                metric_type: row.try_get("metric_type")?,
                threshold: row.try_get("threshold")?,
                notification_sent: row.try_get("notification_sent")?,
                unlocked_at: row.try_get("unlocked_at")?,
            });
        }
        Ok(miles)
    }

    async fn mark_milestone_notified(&self, id: &Uuid) -> Result<(), Error> {
        sqlx::query("UPDATE success_milestones SET notification_sent = true WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_milestone(&self, id: &Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM success_milestones WHERE id = $1").bind(id).execute(&self.pool).await?;
        Ok(())
    }
}
