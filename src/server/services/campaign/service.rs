use std::sync::Arc;
use tonic::{Request, Response, Status};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::repository::campaign_repo::CampaignRepository;
use crate::domain::repository::models::{Campaign, CampaignAsset};
use crate::integrations::registry::IntegrationsRegistry;
use super::activation_routing::CampaignChannel;

use ::server_ohc::campaign::campaign_service_server::CampaignService;
use ::server_ohc::campaign::{
    AddAssetRequest, AddAssetResponse, CreateDraftRequest, CreateDraftResponse,
    LaunchCampaignRequest, LaunchCampaignResponse,
    ListSocialPostProposalsRequest, ListSocialPostProposalsResponse,
    UpdateSocialPostProposalRequest, UpdateSocialPostProposalResponse,
    Campaign as ProtoCampaign, CampaignAsset as ProtoCampaignAsset,
    SocialPostProposal as ProtoSocialPostProposal,
};

use crate::domain::repository::SocialPostProposalRepository;
use crate::domain::repository::models::SocialPostProposal;

pub struct MyCampaignService {
    repo: Arc<CampaignRepository>,
    social_repo: Arc<SocialPostProposalRepository>,
    activation_dispatcher: Arc<dyn CampaignActivationDispatcher>,
}

impl MyCampaignService {
    pub fn new(repo: Arc<CampaignRepository>, social_repo: Arc<SocialPostProposalRepository>) -> Self {
        Self::with_integrations_registry(repo, social_repo, Arc::new(IntegrationsRegistry::new()))
    }

    pub fn with_integrations_registry(
        repo: Arc<CampaignRepository>,
        social_repo: Arc<SocialPostProposalRepository>,
        registry: Arc<IntegrationsRegistry>,
    ) -> Self {
        Self {
            repo,
            social_repo,
            activation_dispatcher: Arc::new(RegistryCampaignActivationDispatcher::new(registry)),
        }
    }

    pub fn with_activation_dispatcher(
        repo: Arc<CampaignRepository>,
        social_repo: Arc<SocialPostProposalRepository>,
        activation_dispatcher: Arc<dyn CampaignActivationDispatcher>,
    ) -> Self {
        Self {
            repo,
            social_repo,
            activation_dispatcher,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CampaignActivationDispatch {
    pub channel: String,
    pub integration_id: String,
    pub metrics_sent: i32,
}

#[tonic::async_trait]
pub trait CampaignActivationDispatcher: Send + Sync {
    async fn dispatch_active_campaign(
        &self,
        campaign: Campaign,
        assets: Vec<CampaignAsset>,
    ) -> Result<Vec<CampaignActivationDispatch>, String>;
}

struct RegistryCampaignActivationDispatcher {
    registry: Arc<IntegrationsRegistry>,
    config: CampaignActivationConfig,
}

impl RegistryCampaignActivationDispatcher {
    fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        Self {
            registry,
            config: CampaignActivationConfig::from_env(),
        }
    }
}

#[derive(Debug, Clone)]
struct CampaignActivationConfig {
    sendgrid_integration_id: String,
    sendgrid_to_email: Option<String>,
    twilio_integration_id: String,
    twilio_to_phone: Option<String>,
    twilio_from_phone: Option<String>,
    meta_integration_id: String,
    meta_platform: String,
    meta_recipient_id: Option<String>,
}

impl CampaignActivationConfig {
    fn from_env() -> Self {
        Self {
            sendgrid_integration_id: env_or("CAMPAIGN_SENDGRID_INTEGRATION_ID", "sendgrid"),
            sendgrid_to_email: non_empty_env("CAMPAIGN_SENDGRID_TO_EMAIL"),
            twilio_integration_id: env_or("CAMPAIGN_TWILIO_INTEGRATION_ID", "twilio"),
            twilio_to_phone: non_empty_env("CAMPAIGN_TWILIO_TO_PHONE"),
            twilio_from_phone: non_empty_env("CAMPAIGN_TWILIO_FROM_PHONE")
                .or_else(|| non_empty_env("TWILIO_FROM_PHONE")),
            meta_integration_id: env_or("CAMPAIGN_META_INTEGRATION_ID", "meta"),
            meta_platform: env_or("CAMPAIGN_META_PLATFORM", "facebook"),
            meta_recipient_id: non_empty_env("CAMPAIGN_META_RECIPIENT_ID"),
        }
    }
}

#[tonic::async_trait]
impl CampaignActivationDispatcher for RegistryCampaignActivationDispatcher {
    async fn dispatch_active_campaign(
        &self,
        campaign: Campaign,
        assets: Vec<CampaignAsset>,
    ) -> Result<Vec<CampaignActivationDispatch>, String> {
        let mut dispatches = Vec::new();

        for asset in assets.iter() {
            let Some(channel) = CampaignChannel::from_asset_type(&asset.r#type) else {
                continue;
            };

            let body = asset.content_url.trim();
            if body.is_empty() {
                return Err(format!(
                    "Campaign activation requires non-empty {} campaign content",
                    asset.r#type
                ));
            }

            match channel {
                CampaignChannel::SendGrid => {
                    let to = self
                        .config
                        .sendgrid_to_email
                        .as_deref()
                        .ok_or_else(|| {
                            "Campaign activation requires CAMPAIGN_SENDGRID_TO_EMAIL for SendGrid dispatch".to_string()
                        })?;
                    self.registry
                        .send_email(
                            &self.config.sendgrid_integration_id,
                            to,
                            &campaign.goal,
                            body,
                        )
                        .await
                        .map_err(|e| {
                            format!(
                                "Campaign activation requires configured SendGrid integration '{}': {}",
                                self.config.sendgrid_integration_id, e
                            )
                        })?;
                    dispatches.push(CampaignActivationDispatch {
                        channel: "sendgrid".to_string(),
                        integration_id: self.config.sendgrid_integration_id.clone(),
                        metrics_sent: 1,
                    });
                }
                CampaignChannel::Twilio => {
                    let to = self
                        .config
                        .twilio_to_phone
                        .as_deref()
                        .ok_or_else(|| {
                            "Campaign activation requires CAMPAIGN_TWILIO_TO_PHONE for Twilio dispatch".to_string()
                        })?;
                    let from = self
                        .config
                        .twilio_from_phone
                        .as_deref()
                        .ok_or_else(|| {
                            "Campaign activation requires CAMPAIGN_TWILIO_FROM_PHONE for Twilio dispatch".to_string()
                        })?;
                    self.registry
                        .send_sms(&self.config.twilio_integration_id, to, from, body)
                        .await
                        .map_err(|e| {
                            format!(
                                "Campaign activation requires configured Twilio integration '{}': {}",
                                self.config.twilio_integration_id, e
                            )
                        })?;
                    dispatches.push(CampaignActivationDispatch {
                        channel: "twilio".to_string(),
                        integration_id: self.config.twilio_integration_id.clone(),
                        metrics_sent: 1,
                    });
                }
                CampaignChannel::Meta => {
                    let recipient = self
                        .config
                        .meta_recipient_id
                        .as_deref()
                        .ok_or_else(|| {
                            "Campaign activation requires CAMPAIGN_META_RECIPIENT_ID for Meta dispatch".to_string()
                        })?;
                    self.registry
                        .send_message(
                            &self.config.meta_integration_id,
                            &self.config.meta_platform,
                            recipient,
                            body,
                        )
                        .await
                        .map_err(|e| {
                            format!(
                                "Campaign activation requires configured Meta integration '{}': {}",
                                self.config.meta_integration_id, e
                            )
                        })?;
                    dispatches.push(CampaignActivationDispatch {
                        channel: "meta".to_string(),
                        integration_id: self.config.meta_integration_id.clone(),
                        metrics_sent: 1,
                    });
                }
            }
        }

        if dispatches.is_empty() {
            return Err(
                "Campaign activation requires at least one Email, SMS, or Meta/Social campaign asset"
                    .to_string(),
            );
        }

        Ok(dispatches)
    }
}

fn env_or(key: &str, fallback: &str) -> String {
    non_empty_env(key).unwrap_or_else(|| fallback.to_string())
}

fn non_empty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[tonic::async_trait]
impl CampaignService for MyCampaignService {
    async fn create_draft(
        &self,
        request: Request<CreateDraftRequest>,
    ) -> Result<Response<CreateDraftResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        let goal = req.goal;

        if goal.is_empty() {
            return Err(Status::invalid_argument("goal is required"));
        }

        let now = Utc::now();
        let campaign = Campaign {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            goal: goal.clone(),
            status: "Draft".to_string(),
            start_time: None,
            end_time: None,
            created_at: Some(now),
            updated_at: Some(now),
        };

        self.repo
            .create_campaign(&campaign)
            .await
            .map_err(|e| Status::internal(format!("Failed to create campaign: {}", e)))?;

        let proto_campaign = ProtoCampaign {
            id: campaign.id,
            tenant_id: campaign.tenant_id,
            goal: campaign.goal,
            status: campaign.status,
            start_time_unix: 0,
            end_time_unix: 0,
        };

        Ok(Response::new(CreateDraftResponse {
            campaign: Some(proto_campaign),
        }))
    }

    async fn add_asset(
        &self,
        request: Request<AddAssetRequest>,
    ) -> Result<Response<AddAssetResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        if req.campaign_id.is_empty() || req.r#type.is_empty() || req.content_url.is_empty() {
            return Err(Status::invalid_argument("Missing required fields for asset"));
        }

        let asset = CampaignAsset {
            id: Uuid::new_v4().to_string(),
            tenant_id: req.tenant_id.clone(),
            campaign_id: req.campaign_id.clone(),
            r#type: req.r#type.clone(),
            content_url: req.content_url.clone(),
            created_at: Some(Utc::now()),
        };

        self.repo
            .add_asset(&asset)
            .await
            .map_err(|e| Status::internal(format!("Failed to add asset: {}", e)))?;

        let proto_asset = ProtoCampaignAsset {
            id: asset.id,
            tenant_id: asset.tenant_id,
            campaign_id: asset.campaign_id,
            r#type: asset.r#type,
            content_url: asset.content_url,
        };

        Ok(Response::new(AddAssetResponse {
            asset: Some(proto_asset),
        }))
    }

    async fn launch_campaign(
        &self,
        request: Request<LaunchCampaignRequest>,
    ) -> Result<Response<LaunchCampaignResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        let campaign = self.repo
            .get_campaign(&req.tenant_id, &req.campaign_id)
            .await
            .map_err(|e| Status::not_found(format!("Campaign not found: {}", e)))?;

        let assets = self.repo
            .get_assets(&req.tenant_id, &req.campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch assets: {}", e)))?;

        if assets.is_empty() {
            return Err(Status::failed_precondition("Cannot launch campaign without assets"));
        }

        let dispatches = self.activation_dispatcher
            .dispatch_active_campaign(campaign, assets)
            .await
            .map_err(Status::failed_precondition)?;

        for dispatch in dispatches {
            self.repo
                .record_channel_execution(
                    &req.tenant_id,
                    &req.campaign_id,
                    &dispatch.channel,
                    dispatch.metrics_sent,
                )
                .await
                .map_err(|e| Status::internal(format!("Failed to record campaign channel execution: {}", e)))?;
        }

        self.repo
            .update_campaign_status(&req.tenant_id, &req.campaign_id, "Active")
            .await
            .map_err(|e| Status::internal(format!("Failed to update campaign status: {}", e)))?;

        let updated_campaign = self.repo
            .get_campaign(&req.tenant_id, &req.campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to refetch campaign: {}", e)))?;

        let proto_campaign = ProtoCampaign {
            id: updated_campaign.id,
            tenant_id: updated_campaign.tenant_id,
            goal: updated_campaign.goal,
            status: updated_campaign.status,
            start_time_unix: updated_campaign.start_time.map_or(0, |t| t.timestamp()),
            end_time_unix: updated_campaign.end_time.map_or(0, |t| t.timestamp()),
        };

        Ok(Response::new(LaunchCampaignResponse {
            campaign: Some(proto_campaign),
        }))
    }
    async fn list_social_post_proposals(
        &self,
        request: Request<ListSocialPostProposalsRequest>,
    ) -> Result<Response<ListSocialPostProposalsResponse>, Status> {
        let req = request.into_inner();
        let proposals = self.social_repo
            .list_proposals(&req.tenant_id, &req.status)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let proto_proposals = proposals.into_iter().map(|p| ProtoSocialPostProposal {
            id: p.id,
            tenant_id: p.tenant_id,
            product_id: p.product_id,
            content: p.content,
            image_url: p.image_url.unwrap_or_default(),
            seo_alt_text: p.seo_alt_text.unwrap_or_default(),
            seo_meta_description: p.seo_meta_description.unwrap_or_default(),
            status: p.status,
            created_at_unix: p.created_at_unix,
            updated_at_unix: p.updated_at_unix,
        }).collect();

        Ok(Response::new(ListSocialPostProposalsResponse {
            proposals: proto_proposals,
        }))
    }

    async fn update_social_post_proposal(
        &self,
        request: Request<UpdateSocialPostProposalRequest>,
    ) -> Result<Response<UpdateSocialPostProposalResponse>, Status> {
        let req = request.into_inner();
        let updated_at_unix = Utc::now().timestamp();
        self.social_repo
            .update_status(&req.tenant_id, &req.proposal_id, &req.status, updated_at_unix)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let proposal = self.social_repo
            .get_proposal(&req.tenant_id, &req.proposal_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Proposal not found"))?;

        Ok(Response::new(UpdateSocialPostProposalResponse {
            proposal: Some(ProtoSocialPostProposal {
                id: proposal.id,
                tenant_id: proposal.tenant_id,
                product_id: proposal.product_id,
                content: proposal.content,
                image_url: proposal.image_url.unwrap_or_default(),
                seo_alt_text: proposal.seo_alt_text.unwrap_or_default(),
                seo_meta_description: proposal.seo_meta_description.unwrap_or_default(),
                status: proposal.status,
                created_at_unix: proposal.created_at_unix,
                updated_at_unix: proposal.updated_at_unix,
            }),
        }))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::models::Tenant;
    use crate::domain::repository::campaign_repo::CampaignRepository;
    use sqlx::PgPool;
    use tonic::Request;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    #[derive(Clone)]
    struct RecordingActivationDispatcher {
        calls: Arc<Mutex<Vec<(String, Vec<String>)>>>,
        result: Result<Vec<CampaignActivationDispatch>, String>,
    }

    #[tonic::async_trait]
    impl CampaignActivationDispatcher for RecordingActivationDispatcher {
        async fn dispatch_active_campaign(
            &self,
            campaign: Campaign,
            assets: Vec<CampaignAsset>,
        ) -> Result<Vec<CampaignActivationDispatch>, String> {
            let asset_types = assets.into_iter().map(|asset| asset.r#type).collect();
            self.calls
                .lock()
                .unwrap()
                .push((campaign.id, asset_types));
            self.result.clone()
        }
    }

    fn test_campaign(id: &str) -> Campaign {
        Campaign {
            id: id.to_string(),
            tenant_id: "tenant-activation".to_string(),
            goal: "Spring launch".to_string(),
            status: "Draft".to_string(),
            start_time: None,
            end_time: None,
            created_at: None,
            updated_at: None,
        }
    }

    fn test_asset(asset_type: &str, content: &str) -> CampaignAsset {
        CampaignAsset {
            id: format!("asset-{}", asset_type),
            tenant_id: "tenant-activation".to_string(),
            campaign_id: "campaign-activation".to_string(),
            r#type: asset_type.to_string(),
            content_url: content.to_string(),
            created_at: None,
        }
    }

    #[test]
    fn test_campaign_activation_routes_third_party_asset_types() {
        assert_eq!(CampaignChannel::from_asset_type("Email"), Some(CampaignChannel::SendGrid));
        assert_eq!(CampaignChannel::from_asset_type("sendgrid"), Some(CampaignChannel::SendGrid));
        assert_eq!(CampaignChannel::from_asset_type("SMS"), Some(CampaignChannel::Twilio));
        assert_eq!(CampaignChannel::from_asset_type("twilio"), Some(CampaignChannel::Twilio));
        assert_eq!(CampaignChannel::from_asset_type("Social"), Some(CampaignChannel::Meta));
        assert_eq!(CampaignChannel::from_asset_type("instagram"), Some(CampaignChannel::Meta));
        assert_eq!(CampaignChannel::from_asset_type("Image"), None);
    }

    #[tokio::test]
    async fn test_registry_activation_dispatcher_fails_closed_without_sendgrid_config() {
        let dispatcher = RegistryCampaignActivationDispatcher {
            registry: Arc::new(IntegrationsRegistry::new()),
            config: CampaignActivationConfig {
                sendgrid_integration_id: "sendgrid".to_string(),
                sendgrid_to_email: None,
                twilio_integration_id: "twilio".to_string(),
                twilio_to_phone: None,
                twilio_from_phone: None,
                meta_integration_id: "meta".to_string(),
                meta_platform: "facebook".to_string(),
                meta_recipient_id: None,
            },
        };

        let err = dispatcher
            .dispatch_active_campaign(
                test_campaign("campaign-activation"),
                vec![test_asset("Email", "Real launch body")],
            )
            .await
            .unwrap_err();

        assert!(err.contains("CAMPAIGN_SENDGRID_TO_EMAIL"));
    }

    #[tokio::test]
    async fn test_recording_activation_dispatcher_records_campaign_and_asset_types() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let dispatcher = RecordingActivationDispatcher {
            calls: calls.clone(),
            result: Ok(vec![CampaignActivationDispatch {
                channel: "meta".to_string(),
                integration_id: "meta".to_string(),
                metrics_sent: 1,
            }]),
        };

        let dispatches = dispatcher
            .dispatch_active_campaign(
                test_campaign("campaign-recorded"),
                vec![test_asset("Social", "Post this")],
            )
            .await
            .unwrap();

        assert_eq!(dispatches[0].channel, "meta");
        assert_eq!(
            calls.lock().unwrap().as_slice(),
            &[("campaign-recorded".to_string(), vec!["Social".to_string()])]
        );
    }

    // Helper to setup an isolated database for tests
    async fn setup_db() -> (PgPool, String) {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        let pool = PgPool::connect(&db_url).await.unwrap();

        let tenant_id = Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO tenants (id, name, type) VALUES ($1, $2, 'test') ON CONFLICT DO NOTHING")
            .bind(&tenant_id)
            .bind(format!("Test Tenant {}", tenant_id))
            .execute(&pool)
            .await
            .unwrap();

        // Set the current tenant for RLS to pass in testing
        sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_id))
            .execute(&pool)
            .await
            .unwrap();

        (pool, tenant_id)
    }

    #[tokio::test]
    #[ignore = "requires local Postgres"]
    async fn test_create_draft_campaign() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool.clone()));
        let service = MyCampaignService::new(repo, Arc::new(crate::domain::repository::SocialPostProposalRepository::new(pool.clone())));

        let mut req = Request::new(CreateDraftRequest {
            tenant_id: tenant_id.clone(),
            goal: "Flash Sale".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let res = service.create_draft(req).await.unwrap().into_inner();
        let campaign = res.campaign.unwrap();

        assert_eq!(campaign.goal, "Flash Sale");
        assert_eq!(campaign.status, "Draft");
        assert_eq!(campaign.tenant_id, tenant_id);
        assert!(!campaign.id.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires local Postgres"]
    async fn test_add_asset_to_campaign() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool.clone()));
        let service = MyCampaignService::new(repo, Arc::new(crate::domain::repository::SocialPostProposalRepository::new(pool.clone())));

        let mut req = Request::new(CreateDraftRequest {
            tenant_id: tenant_id.clone(),
            goal: "Summer Promo".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let res = service.create_draft(req).await.unwrap().into_inner();
        let campaign_id = res.campaign.unwrap().id;

        let mut asset_req = Request::new(AddAssetRequest {
            tenant_id: tenant_id.clone(),
            campaign_id: campaign_id.clone(),
            r#type: "Image".to_string(),
            content_url: "https://example.com/asset.jpg".to_string(),
        });
        asset_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let asset_res = service.add_asset(asset_req).await.unwrap().into_inner();
        let asset = asset_res.asset.unwrap();

        assert_eq!(asset.campaign_id, campaign_id);
        assert_eq!(asset.content_url, "https://example.com/asset.jpg");
        assert_eq!(asset.r#type, "Image");
    }

    #[tokio::test]
    #[ignore = "requires local Postgres"]
    async fn test_launch_campaign_requires_asset() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool.clone()));
        let service = MyCampaignService::new(repo, Arc::new(crate::domain::repository::SocialPostProposalRepository::new(pool.clone())));

        let mut req = Request::new(CreateDraftRequest {
            tenant_id: tenant_id.clone(),
            goal: "Empty Campaign".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let res = service.create_draft(req).await.unwrap().into_inner();
        let campaign_id = res.campaign.unwrap().id;

        let mut launch_req = Request::new(LaunchCampaignRequest {
            tenant_id: tenant_id.clone(),
            campaign_id: campaign_id.clone(),
        });
        launch_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let err = service.launch_campaign(launch_req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::FailedPrecondition);
        assert_eq!(err.message(), "Cannot launch campaign without assets");
    }

    #[tokio::test]
    #[ignore = "requires local Postgres"]
    async fn test_complete_campaign_flow() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool.clone()));
        let dispatch_calls = Arc::new(Mutex::new(Vec::new()));
        let dispatcher = RecordingActivationDispatcher {
            calls: dispatch_calls.clone(),
            result: Ok(vec![CampaignActivationDispatch {
                channel: "sendgrid".to_string(),
                integration_id: "sendgrid".to_string(),
                metrics_sent: 1,
            }]),
        };
        let service = MyCampaignService::with_activation_dispatcher(repo, Arc::new(crate::domain::repository::SocialPostProposalRepository::new(pool.clone())), Arc::new(dispatcher));

        // 1. Create Draft
        let mut req = Request::new(CreateDraftRequest {
            tenant_id: tenant_id.clone(),
            goal: "Complete Flow".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });
        let res = service.create_draft(req).await.unwrap().into_inner();
        let campaign_id = res.campaign.unwrap().id;

        // 2. Add Asset
        let mut asset_req = Request::new(AddAssetRequest {
            tenant_id: tenant_id.clone(),
            campaign_id: campaign_id.clone(),
            r#type: "Email".to_string(),
            content_url: "Test Copy Content".to_string(),
        });
        asset_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });
        service.add_asset(asset_req).await.unwrap();

        // 3. Launch
        let mut launch_req = Request::new(LaunchCampaignRequest {
            tenant_id: tenant_id.clone(),
            campaign_id: campaign_id.clone(),
        });
        launch_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let launch_res = service.launch_campaign(launch_req).await.unwrap().into_inner();
        let final_campaign = launch_res.campaign.unwrap();

        assert_eq!(final_campaign.status, "Active");
        assert_eq!(
            dispatch_calls.lock().unwrap().as_slice(),
            &[(campaign_id.clone(), vec!["Email".to_string()])]
        );

        let channel: String = sqlx::query_scalar("SELECT channel FROM channel_executions WHERE tenant_id = $1 AND campaign_id = $2")
            .bind(&tenant_id)
            .bind(&campaign_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(channel, "sendgrid");
    }

    #[tokio::test]
    #[ignore = "requires local Postgres"]
    async fn test_launch_campaign_requires_third_party_activation_dispatch() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool.clone()));
        let service = MyCampaignService::new(repo, Arc::new(crate::domain::repository::SocialPostProposalRepository::new(pool.clone())));

        let mut req = Request::new(CreateDraftRequest {
            tenant_id: tenant_id.clone(),
            goal: "Launch through SendGrid".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });
        let res = service.create_draft(req).await.unwrap().into_inner();
        let campaign_id = res.campaign.unwrap().id;

        let mut asset_req = Request::new(AddAssetRequest {
            tenant_id: tenant_id.clone(),
            campaign_id: campaign_id.clone(),
            r#type: "Email".to_string(),
            content_url: "Real launch body".to_string(),
        });
        asset_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });
        service.add_asset(asset_req).await.unwrap();

        let mut launch_req = Request::new(LaunchCampaignRequest {
            tenant_id: tenant_id.clone(),
            campaign_id: campaign_id.clone(),
        });
        launch_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_id.clone(),
            agent_id: "test".to_string(),
        });

        let err = service.launch_campaign(launch_req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::FailedPrecondition);
        assert!(err.message().contains("Campaign activation requires"));

        let status: String = sqlx::query_scalar("SELECT status FROM campaigns WHERE tenant_id = $1 AND id = $2")
            .bind(&tenant_id)
            .bind(&campaign_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(status, "Draft");
    }

    #[tokio::test]
    #[ignore = "requires local Postgres"]
    async fn test_tenant_isolation() {
        let (pool, tenant_1) = setup_db().await;
        let (_, tenant_2) = setup_db().await; // Setup second tenant, using same DB structure

        let repo = Arc::new(CampaignRepository::new(pool.clone()));
        let service = MyCampaignService::new(repo, Arc::new(crate::domain::repository::SocialPostProposalRepository::new(pool.clone())));

        let mut req = Request::new(CreateDraftRequest {
            tenant_id: tenant_1.clone(),
            goal: "Tenant 1 Promo".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_1.clone(),
            agent_id: "test".to_string(),
        });
        let res = service.create_draft(req).await.unwrap().into_inner();
        let campaign_id = res.campaign.unwrap().id;

        // Reset context to tenant 2 for simulation
        sqlx::query(&format!("SET app.current_tenant = '{}'", tenant_2))
            .execute(&pool)
            .await
            .unwrap();

        let mut launch_req = Request::new(LaunchCampaignRequest {
            tenant_id: tenant_2.clone(), // tenant 2 trying to launch tenant 1's campaign
            campaign_id: campaign_id.clone(),
        });
        launch_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: tenant_2.clone(),
            agent_id: "test".to_string(),
        });

        // The query should not find the campaign due to RLS
        let err = service.launch_campaign(launch_req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::NotFound);
    }
}
