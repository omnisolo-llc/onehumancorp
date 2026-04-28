use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::growth_service_server::GrowthService;
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::RwLock;
use std::collections::HashMap;
use chrono::Utc;

pub struct MyGrowthService {
    experiments: RwLock<Vec<LandingPageExperiment>>,
    referrals: RwLock<Vec<Referral>>,
    downloads: RwLock<Vec<Download>>,
    team_invites: RwLock<Vec<TeamInviteProto>>,
    onboarding_funnels: RwLock<Vec<OnboardingFunnel>>,
    waitlist: RwLock<Vec<WaitlistEntry>>,
    pool: Option<PgPool>,
}

impl MyGrowthService {
    pub fn new() -> Self {
        MyGrowthService {
            experiments: RwLock::new(Vec::new()),
            referrals: RwLock::new(Vec::new()),
            downloads: RwLock::new(Vec::new()),
            team_invites: RwLock::new(Vec::new()),
            onboarding_funnels: RwLock::new(Vec::new()),
            waitlist: RwLock::new(Vec::new()),
            pool: None,
        }
    }

    pub fn with_pool(pool: PgPool) -> Self {
        MyGrowthService {
            experiments: RwLock::new(Vec::new()),
            referrals: RwLock::new(Vec::new()),
            downloads: RwLock::new(Vec::new()),
            team_invites: RwLock::new(Vec::new()),
            onboarding_funnels: RwLock::new(Vec::new()),
            waitlist: RwLock::new(Vec::new()),
            pool: Some(pool),
        }
    }

}

#[tonic::async_trait]
impl GrowthService for MyGrowthService {
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

    async fn get_referrals(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ReferralsResponse>, Status> {
        let mut refs = Vec::new();
        if let Some(pool) = &self.pool {
            let mut tx = pool.begin().await.unwrap();
            let _ = sqlx::query("SET LOCAL app.current_tenant = 'default_tenant'")
                .execute(&mut *tx)
                .await;

            let db_refs = sqlx::query!(
                r#"
                SELECT user_id, referral_code, invites_sent FROM user_referrals
                "#
            )
            .fetch_all(&mut *tx)
            .await;

            let _ = tx.commit().await;

            if let Ok(db_refs) = db_refs {
                for r in db_refs {
                    refs.push(Referral {
                        id: "".to_string(),
                        user_id: r.user_id,
                        referral_code: r.referral_code,
                        clicks: 0,
                        conversions: 0,
                        invites_sent: r.invites_sent.unwrap_or(0),
                        created_at_unix: 0,
                    });
                }
            }
        }

        Ok(Response::new(ReferralsResponse {
            referrals: refs,
        }))
    }

    async fn create_referral(
        &self,
        request: Request<CreateReferralRequest>,
    ) -> Result<Response<Referral>, Status> {
        let req = request.into_inner();
        if req.user_id.is_empty() || req.referral_code.is_empty() {
            return Err(Status::invalid_argument("userId and referralCode are required"));
        }
        
        let ref_obj = Referral {
            id: format!("ref-{}", Utc::now().timestamp()),
            user_id: req.user_id.clone(),
            referral_code: req.referral_code.clone(),
            clicks: 0,
            conversions: 0,
            created_at_unix: Utc::now().timestamp(),
        };

        if let Some(pool) = &self.pool {
            let mut tx = pool.begin().await.unwrap();
            let _ = sqlx::query("SET LOCAL app.current_tenant = 'default_tenant'")
                .execute(&mut *tx)
                .await;

            let res = sqlx::query(
                r#"
                INSERT INTO user_referrals (tenant_id, user_id, referral_code, invites_sent)
                VALUES ($1, $2, $3, 1)
                ON CONFLICT (referral_code) DO UPDATE
                SET invites_sent = user_referrals.invites_sent + 1
                RETURNING id
                "#
            )
            .bind("default_tenant")
            .bind(&req.user_id)
            .bind(&req.referral_code)
            .fetch_one(&mut *tx)
            .await;

            let _ = tx.commit().await;

            if let Err(e) = res {
                println!("Failed to insert referral into DB: {:?}", e);
            }
        }
        
        let mut refs = self.referrals.write().unwrap();
        refs.push(ref_obj.clone());
        
        Ok(Response::new(ref_obj))
    }

    async fn click_referral(
        &self,
        request: Request<GrowthIdRequest>,
    ) -> Result<Response<Referral>, Status> {
        let req = request.into_inner();
        let mut refs = self.referrals.write().unwrap();
        
        if let Some(r) = refs.iter_mut().find(|r| r.id == req.id) {
            r.clicks += 1;
            return Ok(Response::new(r.clone()));
        }
        
        Err(Status::not_found("referral not found"))
    }

    async fn convert_referral(
        &self,
        request: Request<GrowthIdRequest>,
    ) -> Result<Response<Referral>, Status> {
        let req = request.into_inner();
        let mut refs = self.referrals.write().unwrap();
        
        if let Some(r) = refs.iter_mut().find(|r| r.id == req.id) {
            r.conversions += 1;
            return Ok(Response::new(r.clone()));
        }
        
        Err(Status::not_found("referral not found"))
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
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ViralCoefficientResponse>, Status> {
        let refs = self.referrals.read().unwrap();
        let total_referrals = refs.len() as i32;
        let mut total_conversions = 0;
        let mut inviters = HashMap::new();
        
        for r in refs.iter() {
            total_conversions += r.conversions;
            inviters.insert(r.user_id.clone(), true);
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
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ViralCoefficientMetricsResponse>, Status> {
        let refs = self.referrals.read().unwrap();
        let mut total_conversions = 0;
        let mut inviters = HashMap::new();
        
        for r in refs.iter() {
            total_conversions += r.conversions;
            inviters.insert(r.user_id.clone(), true);
        }
        
        let unique_inviters = inviters.len() as i32;
        let k_factor = if unique_inviters > 0 {
            total_conversions as f64 / unique_inviters as f64
        } else {
            0.0
        };
        
        Ok(Response::new(ViralCoefficientMetricsResponse {
            viral_coefficient: k_factor,
            organization_id: "default".to_string(),
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
        let req = request.into_inner();
        let refs = self.referrals.read().unwrap();
        
        let mut total_conversions = 0;
        for r in refs.iter() {
            if req.user_id.is_empty() || r.user_id == req.user_id {
                total_conversions += r.conversions;
            }
        }
        
        let max_quota = 50 + total_conversions * 10;
        
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
