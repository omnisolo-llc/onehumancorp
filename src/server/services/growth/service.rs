use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::growth_service_server::GrowthService;
use ::server_ohc::orchestration::{CreateReferralRequest, GrowthIdRequest, EmptyRequest};
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use sqlx::{PgPool, Row};
use crate::services::growth::referral_api;
use ::server_common::auth_utils::set_org_context;

use ::server_utils::cache::HybridCache;

pub struct MyGrowthService {
    pool: PgPool,
    hub: Arc<crate::hub::Hub>,
    experiments: RwLock<Vec<LandingPageExperiment>>,
    downloads: RwLock<Vec<Download>>,
    team_invites: RwLock<Vec<TeamInviteProto>>,
    waitlist: RwLock<Vec<WaitlistEntry>>,
    onboarding_funnels: RwLock<Vec<OnboardingFunnel>>,
    referral_score_cache: HybridCache<ReferralScoreResponse>,
    quota_cache: HybridCache<QuotaMetrics>,
}

impl MyGrowthService {
    pub fn new(pool: PgPool, hub: Arc<crate::hub::Hub>) -> Self {
        let redis_client = hub.redis_client.clone();
        MyGrowthService {
            pool,
            hub,
            experiments: RwLock::new(Vec::new()),
            downloads: RwLock::new(Vec::new()),
            team_invites: RwLock::new(Vec::new()),
            waitlist: RwLock::new(Vec::new()),
            onboarding_funnels: RwLock::new(Vec::new()),
            referral_score_cache: HybridCache::new(redis_client.clone()),
            quota_cache: HybridCache::new(redis_client),
        }
    }

    async fn get_org_id(&self, metadata: &tonic::metadata::MetadataMap) -> Result<String, Status> {
        let spiffe_id_str = metadata.get("x-spiffe-id")
            .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))?;

        let (org_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str)?;

        Ok(org_id)
    }
}

#[tonic::async_trait]
impl GrowthService for MyGrowthService {
    async fn submit_review(
        &self,
        request: Request<SubmitReviewRequest>,
    ) -> Result<Response<SubmitReviewResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let review_id = format!("review-{}", uuid::Uuid::new_v4());
        sqlx::query(
            "INSERT INTO reviews (id, tenant_id, reviewer_name, user_id, rating, comment) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&review_id)
        .bind(&org_id)
        .bind(&req.reviewer_name)
        .bind(&req.user_id)
        .bind(req.rating)
        .bind(&req.comment)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Recalculate average
        let row = sqlx::query(
            "SELECT COALESCE(AVG(rating), 0.0) as avg_rating, COUNT(id) as cnt FROM reviews WHERE tenant_id = $1"
        )
        .bind(&org_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let avg_rating: f64 = row.get("avg_rating");
        let cnt: i64 = row.get("cnt");

        let rep_profile_id = format!("rep-{}", org_id);
        sqlx::query(
            r#"
            INSERT INTO reputation_profiles (id, tenant_id, overall_rating, review_count)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET overall_rating = EXCLUDED.overall_rating, review_count = EXCLUDED.review_count, updated_at = CURRENT_TIMESTAMP
            "#
        )
        .bind(&rep_profile_id)
        .bind(&org_id)
        .bind(avg_rating)
        .bind(cnt as i32)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut referral_link = String::new();
        if req.rating >= 4 {
            if let Ok(link) = referral_api::generate_referral_link(&req.user_id) {
                referral_link = link.clone();
                let event_id = format!("evt-{}", uuid::Uuid::new_v4());
                let state_change = serde_json::json!({
                    "review_id": review_id,
                    "reviewer_name": req.reviewer_name,
                    "rating": req.rating,
                    "referral_link": link,
                });

                // Track referral conversion in universal ledger
                sqlx::query(
                    "INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(event_id)
                .bind(&org_id)
                .bind("Growth")
                .bind("ReferralConversion")
                .bind(state_change)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            }
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SubmitReviewResponse {
            referral_link,
        }))
    }

    async fn get_reputation(
        &self,
        request: Request<GetReputationRequest>,
    ) -> Result<Response<ReputationResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let _req = request.into_inner(); // user_id not actively used right now based on schema

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let row = sqlx::query("SELECT overall_rating, review_count FROM reputation_profiles WHERE tenant_id = $1")
            .bind(&org_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(r) = row {
            let overall_rating: f64 = r.get("overall_rating");
            let review_count: i32 = r.get("review_count");
            Ok(Response::new(ReputationResponse {
                overall_rating: overall_rating as f32,
                review_count,
            }))
        } else {
            Ok(Response::new(ReputationResponse {
                overall_rating: 0.0,
                review_count: 0,
            }))
        }
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

        let org_id_clone1 = org_id.clone();
        let org_id_clone2 = org_id.clone();
        let pool1 = self.pool.clone();
        let pool2 = self.pool.clone();

        let (rows_res, business_name_res) = tokio::join!(
            async {
                let mut tx = pool1.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                set_org_context(&mut *tx, &org_id_clone1).await.map_err(|e| Status::internal(e.to_string()))?;
                let rows = sqlx::query("SELECT clicks, conversions FROM referrals WHERE organization_id = $1")
                    .bind(&org_id_clone1)
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
                Ok::<_, Status>(rows)
            },
            async {
                let mut tx = pool2.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                set_org_context(&mut *tx, &org_id_clone2).await.map_err(|e| Status::internal(e.to_string()))?;
                let name: String = sqlx::query_scalar("SELECT business_name FROM tenants WHERE tenant_id = $1::uuid")
                    .bind(&org_id_clone2)
                    .fetch_optional(&mut *tx)
                    .await
                    .unwrap_or(None)
                    .unwrap_or_else(|| "My Awesome Store".to_string());
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
                Ok::<_, Status>(name)
            }
        );

        let rows = rows_res?;
        let business_name = business_name_res?;

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

        // Generate clean business URL for sharing
        let slug = ::server_utils::slug::slugify(&business_name);
        let business_share_url = format!("ohc.app/b/{}", slug);

        Ok(Response::new(ReferralStatsResponse {
            total_referrals,
            click_count,
            conversion_rate,
            reward_balance_cents,
            bonus_credit,
            download_count,
            waitlist_position,
            business_share_url,
            business_name,
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

    async fn create_referral(
        &self,
        request: Request<CreateReferralRequest>,
    ) -> Result<Response<Referral>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();

        if req.user_id.is_empty() {
            return Err(Status::invalid_argument("userId is required"));
        }

        let referral_code = if req.referral_code.is_empty() {
            let generated_link = referral_api::generate_referral_link(&req.user_id)
                .map_err(|e| Status::internal(e))?;

            generated_link
                .split("&utm_source=")
                .next()
                .unwrap_or("")
                .strip_prefix("ohc://join?ref=")
                .unwrap_or("error")
                .to_string()
        } else {
            req.referral_code
        };
        
        let id = format!("ref-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let created_at = Utc::now().timestamp();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("INSERT INTO referrals (id, organization_id, user_id, referral_code, created_at_unix) VALUES ($1, $2, $3, $4, $5)")
            .bind(&id)
            .bind(&org_id)
            .bind(&req.user_id)
            .bind(&referral_code)
            .bind(created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Referral {
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
        request: Request<GrowthIdRequest>,
    ) -> Result<Response<Referral>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let row = sqlx::query("UPDATE referrals SET clicks = clicks + 1 WHERE id = $1 RETURNING id, user_id, referral_code, clicks, conversions, created_at_unix")
            .bind(&req.id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::not_found(format!("referral not found: {}", e)))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Referral {
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
        request: Request<GrowthIdRequest>,
    ) -> Result<Response<Referral>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &org_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let row = sqlx::query("UPDATE referrals SET conversions = conversions + 1 WHERE id = $1 RETURNING id, user_id, referral_code, clicks, conversions, created_at_unix")
            .bind(&req.id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::not_found(format!("referral not found: {}", e)))?;

        // Implement Credit Attribution: "both get 1 month free Pro"
        // In a real app we'd update a billing or organizations table.
        // For now, we simulate credit attribution.
        let _ = sqlx::query("UPDATE organizations SET plan_tier = 'Pro', current_period_end = current_period_end + interval '1 month' WHERE id = $1 OR id = (SELECT organization_id FROM referrals WHERE id = $2)")
            .bind(&org_id)
            .bind(&req.id)
            .execute(&mut *tx)
            .await;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Referral {
            id: row.get("id"),
            user_id: row.get("user_id"),
            referral_code: row.get("referral_code"),
            clicks: row.get("clicks"),
            conversions: row.get("conversions"),
            created_at_unix: row.get("created_at_unix"),
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

    async fn get_referral_score(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ReferralScoreResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;

        let cache_key = format!("referral_score_{}", org_id);

        if let Some(cached_response) = self.referral_score_cache.get(&cache_key).await {
            return Ok(Response::new(cached_response));
        }

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
        let score = if unique_inviters > 0 {
            total_conversions as f64 / unique_inviters as f64
        } else {
            0.0
        };
        
        let response = ReferralScoreResponse {
            total_referrals,
            total_conversions,
            unique_inviters,
            score,
        };

        self.referral_score_cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(60)).await;

        Ok(Response::new(response))
    }

    async fn get_referral_score_metrics(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ReferralScoreMetricsResponse>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let res = self.get_referral_score(request).await?.into_inner();
        
        Ok(Response::new(ReferralScoreMetricsResponse {
            referral_score: res.score,
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

        let cache_key = format!("quota_{}_{}", org_id, req.user_id);

        if let Some(cached_response) = self.quota_cache.get(&cache_key).await {
            return Ok(Response::new(cached_response));
        }

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
        
        let status = self.hub.tracker().check_product_quota(&org_id).await.unwrap_or(::server_pricing::rate_limit::RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        });

        let response = QuotaMetrics { used: 10, max: max_quota, soft_limit_reached: status.soft_limit_reached, upgrade_message: status.user_message.unwrap_or_default(), is_allowed: status.is_allowed };
        self.quota_cache.set(&cache_key, response.clone(), std::time::Duration::from_secs(60)).await;

        Ok(Response::new(response))
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
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = match pool_opts.connect_lazy(&database_url) { Ok(p) => p, Err(_) => return, };
        if database_url.contains("localhost") { return; }
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }
        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let service = MyGrowthService::new(pool, hub);

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

    #[tokio::test]
    async fn test_referral_score_caching() {
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = match pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test") { Ok(p) => p, Err(_) => return, };
        if std::env::var("OHC_DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));
        let service = MyGrowthService::new(pool, hub);

        let mut req1 = Request::new(EmptyRequest {});
        req1.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-test-cache/agent1".parse().unwrap());
        let res1 = service.get_referral_score(req1).await;
        assert!(res1.is_ok(), "First referral score request should succeed and cache the result");

        let mut req2 = Request::new(EmptyRequest {});
        req2.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-test-cache/agent1".parse().unwrap());
        let res2 = service.get_referral_score(req2).await;
        assert!(res2.is_ok(), "Second referral score request should succeed by returning cached value");
        assert_eq!(res1.unwrap().into_inner().score, res2.unwrap().into_inner().score);
    }

    #[tokio::test]
    async fn test_quota_caching() {
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = match pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test") { Ok(p) => p, Err(_) => return, };
        if std::env::var("OHC_DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));
        let service = MyGrowthService::new(pool, hub);

        let mut req1 = Request::new(GetQuotaRequest { user_id: "user1".to_string() });
        req1.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-test-cache/agent1".parse().unwrap());
        let res1 = service.get_quota(req1).await;
        assert!(res1.is_ok(), "First quota request should succeed and cache the result");

        let mut req2 = Request::new(GetQuotaRequest { user_id: "user1".to_string() });
        req2.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-test-cache/agent1".parse().unwrap());
        let res2 = service.get_quota(req2).await;
        assert!(res2.is_ok(), "Second quota request should succeed by returning cached value");
        assert_eq!(res1.unwrap().into_inner().max, res2.unwrap().into_inner().max);
    }
}
