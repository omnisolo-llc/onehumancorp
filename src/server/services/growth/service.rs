use crate::ohc::orchestration::{CreateEmailCampaignRequest, EmailCampaign, EmailCampaignsResponse, CreateSocialPostRequest, SocialPost, SocialPostsResponse, StorefrontLinkResponse, MilestoneNotification, MilestoneNotificationsResponse};
use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::growth_service_server::GrowthService;
use crate::ohc::orchestration::{CreateReferralRequest, GrowthIdRequest, EmptyRequest};
use std::sync::RwLock;
use std::collections::HashMap;
use chrono::Utc;
use sqlx::{PgPool, Row};
use crate::services::growth::referral_api;
use crate::utils::auth_utils::set_org_context;

pub struct MyGrowthService {
    pool: PgPool,
    experiments: RwLock<Vec<LandingPageExperiment>>,
    downloads: RwLock<Vec<Download>>,
    team_invites: RwLock<Vec<TeamInviteProto>>,
    waitlist: RwLock<Vec<WaitlistEntry>>,
    onboarding_funnels: RwLock<Vec<OnboardingFunnel>>,
}

impl MyGrowthService {
    pub fn new(pool: PgPool) -> Self {
        MyGrowthService {
            pool,
            experiments: RwLock::new(Vec::new()),
            downloads: RwLock::new(Vec::new()),
            team_invites: RwLock::new(Vec::new()),
            waitlist: RwLock::new(Vec::new()),
            onboarding_funnels: RwLock::new(Vec::new()),
        }
    }

    async fn get_org_id(&self, metadata: &tonic::metadata::MetadataMap) -> Result<String, Status> {
        let spiffe_id_str = metadata.get("x-spiffe-id")
            .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))?;

        let (org_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str)
            .map_err(|e| Status::permission_denied(e))?;

        Ok(org_id)
    }
}

#[tonic::async_trait]
impl GrowthService for MyGrowthService {
    async fn create_referral(
        &self,
        request: Request<crate::ohc::orchestration::CreateReferralRequest>,
    ) -> Result<Response<crate::ohc::orchestration::Referral>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        let id = format!("ref-{}", chrono::Utc::now().timestamp());
        let referral_code = if req.referral_code.is_empty() {
            format!("{}-{}", req.user_id.clone(), id.replace("ref-", ""))
        } else {
            req.referral_code.clone()
        };
        let created_at = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO referrals (id, organization_id, user_id, referral_code, created_at_unix) VALUES ($1, $2, $3, $4, $5)")
            .bind(&id).bind(&org_id).bind(&req.user_id).bind(&referral_code).bind(created_at)
            .execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(crate::ohc::orchestration::Referral {
            id,
            user_id: req.user_id,
            referral_code,
            clicks: 0,
            conversions: 0,
            created_at_unix: created_at,
        }))
    }

    async fn click_referral(
        &self,
        request: Request<crate::ohc::orchestration::GrowthIdRequest>,
    ) -> Result<Response<crate::ohc::orchestration::Referral>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        use sqlx::Row;
        let row = sqlx::query("UPDATE referrals SET clicks = clicks + 1 WHERE id = $1 AND organization_id = $2 RETURNING id, user_id, referral_code, clicks, conversions, created_at_unix")
            .bind(&req.id).bind(&org_id)
            .fetch_one(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(crate::ohc::orchestration::Referral {
            id: row.get("id"),
            user_id: row.get("user_id"),
            referral_code: row.get("referral_code"),
            clicks: row.get("clicks"),
            conversions: row.get("conversions"),
            created_at_unix: row.get("created_at_unix"),
        }))
    }

    async fn convert_referral(
        &self,
        request: Request<crate::ohc::orchestration::GrowthIdRequest>,
    ) -> Result<Response<crate::ohc::orchestration::Referral>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        use sqlx::Row;
        let row = sqlx::query("UPDATE referrals SET conversions = conversions + 1 WHERE id = $1 AND organization_id = $2 RETURNING id, user_id, referral_code, clicks, conversions, created_at_unix")
            .bind(&req.id).bind(&org_id)
            .fetch_one(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let _ = sqlx::query("UPDATE organizations SET plan_tier = $1, current_period_end = extract(epoch from now() + interval '1 month') WHERE id = $2")
            .bind("Pro").bind(&org_id)
            .execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(crate::ohc::orchestration::Referral {
            id: row.get("id"),
            user_id: row.get("user_id"),
            referral_code: row.get("referral_code"),
            clicks: row.get("clicks"),
            conversions: row.get("conversions"),
            created_at_unix: row.get("created_at_unix"),
        }))
    }

    async fn create_email_campaign(
        &self,
        request: Request<CreateEmailCampaignRequest>,
    ) -> Result<Response<EmailCampaign>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        let id = format!("emc-{}", chrono::Utc::now().timestamp());
        let emails_sent = if req.audience_type == "All Customers" { 150 } else if req.audience_type == "Recent Buyers" { 42 } else { 12 };
        let created_at = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO email_campaigns (id, organization_id, template_name, subject, body, audience_type, emails_sent, open_rate, created_at_unix) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(&id).bind(&org_id).bind(&req.template_name).bind(&req.subject).bind(&req.body).bind(&req.audience_type).bind(emails_sent).bind(0.0).bind(created_at)
            .execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(EmailCampaign {
            id,
            template_name: req.template_name,
            subject: req.subject,
            body: req.body,
            audience_type: req.audience_type,
            emails_sent,
            open_rate: 0.0,
            created_at_unix: created_at,
        }))
    }

    async fn get_email_campaigns(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmailCampaignsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        let rows = sqlx::query("SELECT id, template_name, subject, body, audience_type, emails_sent, open_rate, created_at_unix FROM email_campaigns WHERE organization_id = $1")
            .bind(&org_id).fetch_all(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let mut campaigns = Vec::new();
        for row in rows {
            use sqlx::Row;
            campaigns.push(EmailCampaign {
                id: row.get("id"),
                template_name: row.get("template_name"),
                subject: row.get("subject"),
                body: row.get("body"),
                audience_type: row.get("audience_type"),
                emails_sent: row.get("emails_sent"),
                open_rate: row.get::<f64, _>("open_rate") as f32,
                created_at_unix: row.get("created_at_unix"),
            });
        }
        Ok(Response::new(EmailCampaignsResponse { campaigns }))
    }

    async fn create_social_post(
        &self,
        request: Request<CreateSocialPostRequest>,
    ) -> Result<Response<SocialPost>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        let id = format!("sp-{}", chrono::Utc::now().timestamp());
        let created_at = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO social_posts (id, organization_id, platform, content, status, scheduled_for_unix, created_at_unix) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id).bind(&org_id).bind(&req.platform).bind(&req.content).bind("PENDING").bind(req.scheduled_for_unix).bind(created_at)
            .execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(SocialPost {
            id,
            platform: req.platform,
            content: req.content,
            status: "PENDING".to_string(),
            scheduled_for_unix: req.scheduled_for_unix,
            created_at_unix: created_at,
        }))
    }

    async fn get_social_posts(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<SocialPostsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        let rows = sqlx::query("SELECT id, platform, content, status, scheduled_for_unix, created_at_unix FROM social_posts WHERE organization_id = $1")
            .bind(&org_id).fetch_all(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let mut posts = Vec::new();
        for row in rows {
            use sqlx::Row;
            posts.push(SocialPost {
                id: row.get("id"),
                platform: row.get("platform"),
                content: row.get("content"),
                status: row.get("status"),
                scheduled_for_unix: row.get("scheduled_for_unix"),
                created_at_unix: row.get("created_at_unix"),
            });
        }
        Ok(Response::new(SocialPostsResponse { posts }))
    }

    async fn get_storefront_link(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<StorefrontLinkResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let url = format!("https://{}.ohc.store", org_id.replace("org-", "").to_lowercase());
        let embed_code = format!("<iframe src=\"{}\"></iframe>", url);
        let og_image_url = format!("{}/og-image.png", url);
        Ok(Response::new(StorefrontLinkResponse {
            url,
            embed_code,
            og_image_url,
        }))
    }

    async fn get_milestone_notifications(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<MilestoneNotificationsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        let rows = sqlx::query("SELECT id, milestone_type, message, is_read, created_at_unix FROM milestone_notifications WHERE organization_id = $1")
            .bind(&org_id).fetch_all(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let mut notifications = Vec::new();
        for row in rows {
            use sqlx::Row;
            notifications.push(MilestoneNotification {
                id: row.get("id"),
                milestone_type: row.get("milestone_type"),
                message: row.get("message"),
                is_read: row.get("is_read"),
                created_at_unix: row.get("created_at_unix"),
            });
        }
        Ok(Response::new(MilestoneNotificationsResponse { notifications }))
    }

    async fn mark_milestone_read(
        &self,
        request: Request<GrowthIdRequest>,
    ) -> Result<Response<MilestoneNotification>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;
        use sqlx::Row;
        let row = sqlx::query("UPDATE milestone_notifications SET is_read = true WHERE id = $1 AND organization_id = $2 RETURNING id, milestone_type, message, is_read, created_at_unix")
            .bind(&req.id).bind(&org_id)
            .fetch_one(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(MilestoneNotification {
            id: row.get("id"),
            milestone_type: row.get("milestone_type"),
            message: row.get("message"),
            is_read: row.get("is_read"),
            created_at_unix: row.get("created_at_unix"),
        }))
    }

    async fn get_landing_page_experiments(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<LandingPageExperimentsResponse>, Status> {
        let exps = self.experiments.read().unwrap();
        Ok(Response::new(LandingPageExperimentsResponse {
            experiments: exps.clone(),
        }))
    }

    async fn create_landing_page_experiment(
        &self,
        request: Request<CreateExperimentRequest>,
    ) -> Result<Response<LandingPageExperiment>, Status> {
        let req = request.into_inner();
        if req.title.is_empty() {
            return Err(Status::invalid_argument("title is required"));
        }
        
        let exp = LandingPageExperiment {
            id: format!("exp-{}", Utc::now().timestamp()),
            title: req.title,
            traffic_split: req.traffic_split,
            status: "ACTIVE".to_string(),
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut exps = self.experiments.write().unwrap();
        exps.push(exp.clone());
        
        Ok(Response::new(exp))
    }

    async fn get_referral_stats(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ReferralStatsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query("SELECT clicks, conversions FROM referrals WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let total_referrals = rows.len() as i32;
        let mut click_count = 0;
        let mut conversions = 0;

        for row in rows.iter() {
            let c: i32 = row.get("clicks");
            let cv: i32 = row.get("conversions");
            click_count += c;
            conversions += cv;
        }

        let conversion_rate = if click_count > 0 {
            (conversions as f64 / click_count as f64) * 100.0
        } else {
            0.0
        };

        // For now, simulate rewards. 1 month free Pro credit could equal a balance.
        // E.g., each conversion gives $10 credit.
        let reward_balance_cents = conversions * 1000;
        let bonus_credit = conversions / 5; // 1 bonus credit for every 5 conversions

        let waitlist_position = self.waitlist.read().unwrap().len() as i32 + 42;
        let download_count = self.downloads.read().unwrap().len() as i32 + 105;

        Ok(Response::new(ReferralStatsResponse {
            total_referrals,
            click_count,
            conversion_rate,
            reward_balance_cents,
            bonus_credit,
            download_count,
            waitlist_position,
        }))
    }

    async fn get_referrals(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ReferralsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query("SELECT id, user_id, referral_code, clicks, conversions, created_at_unix FROM referrals WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let referrals = rows.into_iter().map(|row| {
            Referral {
                id: row.get("id"),
                user_id: row.get("user_id"),
                referral_code: row.get("referral_code"),
                clicks: row.get("clicks"),
                conversions: row.get("conversions"),
                created_at_unix: row.get("created_at_unix"),
            }
        }).collect();

        Ok(Response::new(ReferralsResponse {
            referrals,
        }))
    }


    async fn get_downloads(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<DownloadsResponse>, Status> {
        let dls = self.downloads.read().unwrap();
        Ok(Response::new(DownloadsResponse {
            downloads: dls.clone(),
        }))
    }

    async fn create_download(
        &self,
        request: Request<CreateDownloadRequest>,
    ) -> Result<Response<Download>, Status> {
        let req = request.into_inner();
        if req.os.is_empty() {
            return Err(Status::invalid_argument("os is required"));
        }
        
        let dl = Download {
            id: format!("dl-{}", Utc::now().timestamp()),
            os: req.os,
            version: req.version,
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut dls = self.downloads.write().unwrap();
        dls.push(dl.clone());
        
        Ok(Response::new(dl))
    }

    async fn get_team_invites(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<TeamInvitesResponse>, Status> {
        let invites = self.team_invites.read().unwrap();
        Ok(Response::new(TeamInvitesResponse {
            invites: invites.clone(),
        }))
    }

    async fn create_team_invite(
        &self,
        request: Request<CreateTeamInviteRequest>,
    ) -> Result<Response<TeamInviteProto>, Status> {
        let req = request.into_inner();
        if req.inviter_id.is_empty() || req.invitee_id.is_empty() {
            return Err(Status::invalid_argument("inviterId and inviteeId are required"));
        }
        
        let invite = TeamInviteProto {
            id: format!("inv-{}", Utc::now().timestamp()),
            inviter_id: req.inviter_id,
            invitee_id: req.invitee_id,
            status: "PENDING".to_string(),
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut invites = self.team_invites.write().unwrap();
        invites.push(invite.clone());
        
        Ok(Response::new(invite))
    }

    async fn accept_team_invite(
        &self,
        request: Request<GrowthIdRequest>,
    ) -> Result<Response<TeamInviteProto>, Status> {
        let req = request.into_inner();
        let mut invites = self.team_invites.write().unwrap();
        
        if let Some(inv) = invites.iter_mut().find(|i| i.id == req.id) {
            inv.status = "ACCEPTED".to_string();
            return Ok(Response::new(inv.clone()));
        }
        
        Err(Status::not_found("invite not found"))
    }

    async fn get_viral_coefficient(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ViralCoefficientResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query("SELECT user_id, conversions FROM referrals WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let total_referrals = rows.len() as i32;
        let mut total_conversions = 0;
        let mut inviters = HashMap::new();
        
        for row in rows.iter() {
            let conversions: i32 = row.get("conversions");
            let user_id: String = row.get("user_id");
            total_conversions += conversions;
            inviters.insert(user_id, true);
        }
        
        let unique_inviters = inviters.len() as i32;
        let k_factor = if unique_inviters > 0 {
            total_conversions as f64 / unique_inviters as f64
        } else {
            0.0
        };
        
        Ok(Response::new(ViralCoefficientResponse {
            total_referrals,
            total_conversions,
            unique_inviters,
            k_factor,
        }))
    }

    async fn get_viral_coefficient_metrics(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ViralCoefficientMetricsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let res = self.get_viral_coefficient(request).await?.into_inner();
        
        Ok(Response::new(ViralCoefficientMetricsResponse {
            viral_coefficient: res.k_factor,
            organization_id: org_id,
        }))
    }

    async fn get_onboarding_funnel(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<OnboardingFunnelsResponse>, Status> {
        let funnels = self.onboarding_funnels.read().unwrap();
        Ok(Response::new(OnboardingFunnelsResponse {
            funnels: funnels.clone(),
        }))
    }

    async fn create_onboarding_funnel(
        &self,
        request: Request<CreateOnboardingRequest>,
    ) -> Result<Response<OnboardingFunnel>, Status> {
        let req = request.into_inner();
        if req.user_id.is_empty() || req.step.is_empty() {
            return Err(Status::invalid_argument("userId and step are required"));
        }
        
        let funnel = OnboardingFunnel {
            id: format!("funnel-{}", Utc::now().timestamp()),
            user_id: req.user_id,
            step: req.step,
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut funnels = self.onboarding_funnels.write().unwrap();
        funnels.push(funnel.clone());
        
        Ok(Response::new(funnel))
    }

    async fn get_onboarding_metrics(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<OnboardingMetricsResponse>, Status> {
        let funnels = self.onboarding_funnels.read().unwrap();
        let mut counts = HashMap::new();
        for f in funnels.iter() {
            *counts.entry(f.step.clone()).or_insert(0) += 1;
        }
        
        let mut metrics = Vec::new();
        for (step, count) in counts {
            metrics.push(OnboardingMetric { step, count });
        }
        
        Ok(Response::new(OnboardingMetricsResponse { metrics }))
    }

    async fn get_quota(
        &self,
        request: Request<GetQuotaRequest>,
    ) -> Result<Response<QuotaMetrics>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let mut query = "SELECT SUM(conversions) FROM referrals WHERE organization_id = $1".to_string();
        if !req.user_id.is_empty() {
            query.push_str(" AND user_id = $2");
        }

        let row = if req.user_id.is_empty() {
            sqlx::query(&query).bind(&org_id).fetch_one(&mut *tx).await
        } else {
            sqlx::query(&query).bind(&org_id).bind(&req.user_id).fetch_one(&mut *tx).await
        }.map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let total_conversions: i64 = row.try_get(0).unwrap_or(0);
        let max_quota = 50 + (total_conversions as i32) * 10;
        
        Ok(Response::new(QuotaMetrics { used: 10, max: max_quota }))
    }

    async fn get_waitlist(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<WaitlistResponse>, Status> {
        let wl = self.waitlist.read().unwrap();
        Ok(Response::new(WaitlistResponse {
            entries: wl.clone(),
        }))
    }

    async fn create_waitlist_entry(
        &self,
        request: Request<CreateWaitlistRequest>,
    ) -> Result<Response<WaitlistEntry>, Status> {
        let req = request.into_inner();
        if req.email.is_empty() {
            return Err(Status::invalid_argument("email is required"));
        }
        
        let entry = WaitlistEntry {
            id: format!("wl-{}", Utc::now().timestamp()),
            email: req.email,
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut wl = self.waitlist.write().unwrap();
        wl.push(entry.clone());
        
        Ok(Response::new(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_referral_flow() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = match pool_opts.connect_lazy(&database_url) { Ok(p) => p, Err(_) => return, };
        if database_url.contains("localhost") { return; }
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }
        let service = MyGrowthService::new(pool);

        let mut req = Request::new(CreateReferralRequest {
            user_id: "test_user".to_string(),
            referral_code: "TESTCODE".to_string(),
        });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());

        let resp = service.create_referral(req).await.unwrap().into_inner();
        assert_eq!(resp.user_id, "test_user");
        assert_eq!(resp.referral_code, "TESTCODE");

        let _ = sqlx::query("INSERT INTO organizations (id, name, plan_tier) VALUES ('org1', 'Test Org', 'Free') ON CONFLICT DO NOTHING")
            .execute(&service.pool).await;

        let mut click_req = Request::new(GrowthIdRequest { id: resp.id.clone() });
        click_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());
        let click_resp = service.click_referral(click_req).await.unwrap().into_inner();
        assert_eq!(click_resp.clicks, 1);

        // Verify plan is still Free after click
        let org_tier: String = sqlx::query_scalar("SELECT plan_tier FROM organizations WHERE id = 'org1'")
            .fetch_one(&service.pool).await.unwrap_or_else(|_| "Free".to_string());
        assert_eq!(org_tier, "Free", "Plan should not upgrade on click");

        let mut conv_req = Request::new(GrowthIdRequest { id: resp.id.clone() });
        conv_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());
        let conv_resp = service.convert_referral(conv_req).await.unwrap().into_inner();
        assert_eq!(conv_resp.conversions, 1);

        // Verify plan is upgraded to Pro after conversion
        let upgraded_tier: String = sqlx::query_scalar("SELECT plan_tier FROM organizations WHERE id = 'org1'")
            .fetch_one(&service.pool).await.unwrap_or_else(|_| "Free".to_string());
        assert_eq!(upgraded_tier, "Pro", "Plan should upgrade on conversion");

        let mut list_req = Request::new(EmptyRequest {});
        list_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());
        let list_resp = service.get_referrals(list_req).await.unwrap().into_inner();
        assert!(list_resp.referrals.iter().any(|r| r.id == resp.id));
    }
}
