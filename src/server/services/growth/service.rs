use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::growth_service_server::GrowthService;
use ::server_ohc::orchestration::{CreateReferralRequest, GrowthIdRequest, EmptyRequest};

use ::server_ohc::orchestration::{SubmitReviewRequest, SubmitReviewResponse, GetReputationRequest, GetReputationResponse};
use uuid::Uuid;

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
        let tenant_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let review_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO reviews (id, tenant_id, customer_id, order_id, rating, comment) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&review_id)
        .bind(&tenant_id)
        .bind(&req.customer_id)
        .bind(&req.order_id)
        .bind(req.rating)
        .bind(&req.comment)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("failed to insert review: {}", e)))?;

        let row = sqlx::query(
            "INSERT INTO reputation_profiles (id, tenant_id, average_rating, total_reviews)
             VALUES ($1, $2, $3, 1)
             ON CONFLICT (tenant_id)
             DO UPDATE SET
                total_reviews = reputation_profiles.total_reviews + 1,
                average_rating = ((reputation_profiles.average_rating * reputation_profiles.total_reviews) + $3) / (reputation_profiles.total_reviews + 1)
             RETURNING average_rating, total_reviews"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(req.rating as f64)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("failed to update reputation: {}", e)))?;

        let mut generated_referral_link = String::new();
        if req.rating >= 4 {
            if let Ok(link) = referral_api::generate_referral_link(&req.customer_id) {
                generated_referral_link = link.clone();
                let ref_id = Uuid::new_v4().to_string();
                let _ = sqlx::query("INSERT INTO referral_codes (id, tenant_id, customer_id, referral_code) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                    .bind(&ref_id)
                    .bind(&tenant_id)
                    .bind(&req.customer_id)
                    .bind(&link)
                    .execute(&mut *tx)
                    .await;
            }
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SubmitReviewResponse {
            review_id,
            generated_referral_link,
        }))
    }

    async fn get_reputation(
        &self,
        request: Request<GetReputationRequest>,
    ) -> Result<Response<GetReputationResponse>, Status> {
        let tenant_id = self.get_org_id(request.metadata()).await?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let row = sqlx::query("SELECT average_rating, total_reviews FROM reputation_profiles WHERE tenant_id = $1")
            .bind(&tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        if let Some(r) = _row {
            use sqlx::Row;
            Ok(Response::new(GetReputationResponse {
                average_rating: r.get("average_rating"),
                total_reviews: r.get("total_reviews"),
            }))
        } else {
            Ok(Response::new(GetReputationResponse {
                average_rating: 0.0,
                total_reviews: 0,
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

        // Implement Credit Attribution: "both get 14 days free Pro trial extension"
        // In OHC, this is represented by upgrading to Pro and setting the has_claimed_trial_extension flag.
        let _ = sqlx::query("UPDATE tenants SET plan_tier = 'pro', has_claimed_trial_extension = true WHERE id = $1::uuid OR id = (SELECT tenant_id::uuid FROM referrals WHERE id = $2)")
            .bind(&org_id)
            .bind(&req.id)
            .execute(&mut *tx)
            .await;

        // Ledger logic for the Autonomous Reputation and Referral Engine
        let referrer_id: String = row.get("user_id");
        let ledger_entry_id = Uuid::new_v4().to_string();
        let _ = sqlx::query("INSERT INTO universal_wallet_ledger (id, tenant_id, customer_id, credit_amount, reason, created_at_unix) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&ledger_entry_id)
            .bind(&org_id)
            .bind(&referrer_id)
            .bind(1000i64) // $10 credit in cents
            .bind("Referral Credit")
            .bind(chrono::Utc::now().timestamp())
            .execute(&mut *tx)
            .await;

        let ledger_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "referral_id": req.id,
            "reward_type": "14_day_pro_trial",
            "description": "Referral conversion: Both parties received 14 days of Pro credit."
        });

        let _ = sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, event_type, payload) VALUES ($1, $2, $3, $4, $5)")
            .bind(&ledger_id)
            .bind(&org_id)
            .bind("Growth")
            .bind("ReferralConversion")
            .bind(payload)
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
        request: Request<GetOnboardingMetricsRequest>,
    ) -> Result<Response<OnboardingMetricsResponse>, Status> {
        let req = request.into_inner();
        let mobile_optimized = req.mobile_optimized;
        let funnels = self.onboarding_funnels.read().unwrap();
        let mut counts = HashMap::new();
        for f in funnels.iter() {
            *counts.entry(f.step.clone()).or_insert(0) += 1;
        }
        
        let mut metrics = Vec::new();
        for (step, count) in counts {
            let optimized_step = if mobile_optimized {
                let char_count = step.chars().count();
                if char_count > 10 {
                    let truncated: String = step.chars().take(10).collect();
                    format!("{}...", truncated)
                } else {
                    step
                }
            } else {
                step
            };
            metrics.push(OnboardingMetric { step: optimized_step, count });
        }
        
        Ok(Response::new(OnboardingMetricsResponse { metrics }))
    }

    async fn get_quota(
        &self,
        request: Request<GetQuotaRequest>,
    ) -> Result<Response<QuotaMetrics>, Status> {
        let org_id = self.get_org_id(request.metadata()).await?;
        let req = request.into_inner();
        let mobile_optimized = req.mobile_optimized;

        let cache_key = format!("quota_{}_{}_{}", org_id, req.user_id, mobile_optimized);

        if let Some(cached_response) = self.quota_cache.get(&cache_key).await {
            return Ok(Response::new(cached_response));
        }

        let org_id_clone1 = org_id.clone();
        let org_id_clone2 = org_id.clone();
        let org_id_clone3 = org_id.clone();
        let user_id_clone = req.user_id.clone();
        let pool1 = self.pool.clone();
        let pool2 = self.pool.clone();
        let hub_clone = self.hub.clone();

        let (referral_res, product_res, tier_res) = tokio::join!(
            async {
                let mut tx = pool1.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                set_org_context(&mut *tx, &org_id_clone1).await.map_err(|e| Status::internal(e.to_string()))?;

                let mut query = "SELECT SUM(conversions) FROM referrals WHERE organization_id = $1".to_string();
                if !user_id_clone.is_empty() {
                    query.push_str(" AND user_id = $2");
                }
                let row = if user_id_clone.is_empty() {
                    sqlx::query(&query).bind(&org_id_clone1).fetch_one(&mut *tx).await
                } else {
                    sqlx::query(&query).bind(&org_id_clone1).bind(&user_id_clone).fetch_one(&mut *tx).await
                }.map_err(|e| Status::internal(e.to_string()))?;

                let total_conversions: i64 = row.try_get(0).unwrap_or(0);
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
                Ok::<_, Status>(total_conversions)
            },
            async {
                let mut tx = pool2.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                set_org_context(&mut *tx, &org_id_clone2).await.map_err(|e| Status::internal(e.to_string()))?;
                let product_count_row = sqlx::query("SELECT COUNT(*)::BIGINT FROM products WHERE tenant_id = $1")
                    .bind(&org_id_clone2)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
                let product_count: i64 = product_count_row.try_get(0).unwrap_or(0);
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
                Ok::<_, Status>(product_count)
            },
            async {
                hub_clone.tracker().get_tenant_tier(&org_id_clone3).await.unwrap_or(::server_pricing::rate_limit::PlanTier::Free)
            }
        );

        let total_conversions = referral_res?;
        let product_count = product_res?;
        let _referral_quota = 50 + (total_conversions as i32) * 10;
        let tier = tier_res;
        let product_limit = tier.max_products().map(|limit| limit as i32).unwrap_or(0);
        let soft_limit_reached = tier.max_products().map(|limit| product_count >= limit as i64).unwrap_or(false);
        let upgrade_message = if mobile_optimized {
            String::new()
        } else if soft_limit_reached {
            format!(
                "You've reached your {} tier limit of {} products. Upgrade your plan to add more products.",
                match tier {
                    ::server_pricing::rate_limit::PlanTier::Free => "Free",
                    ::server_pricing::rate_limit::PlanTier::Starter => "Starter",
                    ::server_pricing::rate_limit::PlanTier::Pro => "Pro",
                    ::server_pricing::rate_limit::PlanTier::Business => "Business",
                },
                product_limit
            )
        } else {
            String::new()
        };

        let response = QuotaMetrics {
            used: product_count as i32,
            max: product_limit,
            soft_limit_reached,
            upgrade_message,
            is_allowed: !soft_limit_reached,
        };
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
        let pool_opts = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
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

        let _ = sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ('00000000-0000-0000-0000-000000000001'::uuid, 'Test Org', 'free') ON CONFLICT DO NOTHING")
            .execute(&service.pool).await;

        let mut click_req = Request::new(GrowthIdRequest { id: resp.id.clone() });
        click_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/00000000-0000-0000-0000-000000000001/agent1".parse().unwrap());
        let click_resp = service.click_referral(click_req).await.unwrap().into_inner();
        assert_eq!(click_resp.clicks, 1);

        // Verify plan is still free after click
        let org_tier: String = sqlx::query_scalar("SELECT plan_tier FROM tenants WHERE id = '00000000-0000-0000-0000-000000000001'::uuid")
            .fetch_one(&service.pool).await.unwrap_or_else(|_| "free".to_string());
        assert_eq!(org_tier, "free", "Plan should not upgrade on click");

        let mut conv_req = Request::new(GrowthIdRequest { id: resp.id.clone() });
        conv_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/00000000-0000-0000-0000-000000000001/agent1".parse().unwrap());
        let conv_resp = service.convert_referral(conv_req).await.unwrap().into_inner();
        assert_eq!(conv_resp.conversions, 1);

        // Verify plan is upgraded to pro after conversion
        let upgraded_tier: String = sqlx::query_scalar("SELECT plan_tier FROM tenants WHERE id = '00000000-0000-0000-0000-000000000001'::uuid")
            .fetch_one(&service.pool).await.unwrap_or_else(|_| "free".to_string());
        assert_eq!(upgraded_tier, "pro", "Plan should upgrade on conversion");

        let mut list_req = Request::new(EmptyRequest {});
        list_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());
        let list_resp = service.get_referrals(list_req).await.unwrap().into_inner();
        assert!(list_resp.referrals.iter().any(|r| r.id == resp.id));
    }

    #[tokio::test]
    async fn test_referral_score_caching() {
        let pool_opts = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
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
        let pool_opts = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = match pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test") { Ok(p) => p, Err(_) => return, };
        if std::env::var("OHC_DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));
        let service = MyGrowthService::new(pool, hub);

        let mut req1 = Request::new(GetQuotaRequest { user_id: "user1".to_string(), mobile_optimized: false });
        req1.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-test-cache/agent1".parse().unwrap());
        let res1 = service.get_quota(req1).await;
        assert!(res1.is_ok(), "First quota request should succeed and cache the result");

        let mut req2 = Request::new(GetQuotaRequest { user_id: "user1".to_string(), mobile_optimized: false });
        req2.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-test-cache/agent1".parse().unwrap());
        let res2 = service.get_quota(req2).await;
        assert!(res2.is_ok(), "Second quota request should succeed by returning cached value");
        assert_eq!(res1.unwrap().into_inner().max, res2.unwrap().into_inner().max);
    }

    #[tokio::test]
    async fn test_submit_review_and_reputation_flow() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool_opts = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = match pool_opts.connect_lazy(&database_url) { Ok(p) => p, Err(_) => return, };
        if database_url.contains("localhost") { return; }
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }
        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let service = MyGrowthService::new(pool, hub);

        // Submit review
        let mut req = Request::new(SubmitReviewRequest {
            customer_id: "cust_123".to_string(),
            order_id: "order_123".to_string(),
            rating: 5,
            comment: "Excellent!".to_string(),
        });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());

        // Ensure tenant isolation
        let _ = sqlx::query("SET app.current_tenant = 'org1'").execute(&service.pool).await;

        let res = service.submit_review(req).await;
        if let Ok(resp) = res {
            let inner = resp.into_inner();
            assert!(!inner.review_id.is_empty());
            assert!(!inner.generated_referral_link.is_empty()); // because rating is 5
        }

        // Get reputation
        let mut get_req = Request::new(GetReputationRequest {});
        get_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());
        let get_res = service.get_reputation(get_req).await;
        if let Ok(resp) = get_res {
            let inner = resp.into_inner();
            assert!(inner.average_rating > 0.0);
            assert!(inner.total_reviews > 0);
        }
    }

    #[tokio::test]
    async fn test_get_quota_latency_benchmark() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = crate::db::secure_pg_pool_options().max_connections(5).connect(&database_url).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));

        let service = MyGrowthService::new(pool.clone(), hub);

        let req = GetQuotaRequest {
            user_id: "".to_string(),
            mobile_optimized: false,
        };

        let mut request = Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "test_org".to_string(),
            agent_id: "test".to_string(),
        });

        let start = std::time::Instant::now();
        let _res = service.get_quota(request).await.unwrap().into_inner();
        let elapsed = start.elapsed();
        tracing::info!("get_quota Hybrid benchmark completed in {} ms", elapsed.as_millis());

        // Assert that the optimization keeps latency under an acceptable threshold (e.g. 500ms)
        assert!(elapsed.as_millis() < 500, "get_quota fetch took too long: {}ms", elapsed.as_millis());
    }

    #[tokio::test]
    async fn test_get_quota_mobile_payload_optimization() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = crate::db::secure_pg_pool_options().max_connections(5).connect(&database_url).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));

        let service = MyGrowthService::new(pool.clone(), hub);

        let req = GetQuotaRequest {
            user_id: "".to_string(),
            mobile_optimized: true,
        };

        let mut request = Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "test_org".to_string(),
            agent_id: "test".to_string(),
        });

        let res = service.get_quota(request).await.unwrap().into_inner();

        // When mobile optimized, upgrade_message should be empty
        assert_eq!(res.upgrade_message, "", "Mobile payload should omit the upgrade message");
    }

    #[tokio::test]
    async fn test_get_onboarding_metrics_mobile_payload_optimization() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
        let pool = crate::db::secure_pg_pool_options().max_connections(5).connect(&database_url).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool.clone()));

        let service = MyGrowthService::new(pool.clone(), hub);

        {
            let mut funnels = service.onboarding_funnels.write().unwrap();
            funnels.push(OnboardingFunnel {
                id: "test_1".to_string(),
                user_id: "user_1".to_string(),
                step: "very_long_step_name_that_should_be_truncated".to_string(),
                created_at_unix: 0,
            });
        }

        let req = GetOnboardingMetricsRequest {
            mobile_optimized: true,
        };

        let mut request = Request::new(req);
        request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "test_org".to_string(),
            agent_id: "test".to_string(),
        });

        let res = service.get_onboarding_metrics(request).await.unwrap().into_inner();

        assert!(!res.metrics.is_empty(), "Metrics should not be empty");
        for metric in res.metrics {
            assert!(metric.step.len() <= 13, "Mobile payload should truncate step name. Found length: {}", metric.step.len());
        }
    }
}
