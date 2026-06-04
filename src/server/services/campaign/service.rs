use std::sync::Arc;
use tonic::{Request, Response, Status};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::repository::campaign_repo::CampaignRepository;
use crate::domain::repository::models::{Campaign, CampaignAsset};

use ::server_ohc::campaign::campaign_service_server::CampaignService;
use ::server_ohc::campaign::{
    AddAssetRequest, AddAssetResponse, CreateDraftRequest, CreateDraftResponse,
    LaunchCampaignRequest, LaunchCampaignResponse,
    Campaign as ProtoCampaign, CampaignAsset as ProtoCampaignAsset,
};

pub struct MyCampaignService {
    repo: Arc<CampaignRepository>,
}

impl MyCampaignService {
    pub fn new(repo: Arc<CampaignRepository>) -> Self {
        Self { repo }
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::models::Tenant;
    use crate::domain::repository::campaign_repo::CampaignRepository;
    use sqlx::PgPool;
    use tonic::Request;
    use std::sync::Arc;
    use uuid::Uuid;

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
    async fn test_create_draft_campaign() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool));
        let service = MyCampaignService::new(repo);

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
    async fn test_add_asset_to_campaign() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool));
        let service = MyCampaignService::new(repo);

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
    async fn test_launch_campaign_requires_asset() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool));
        let service = MyCampaignService::new(repo);

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
    async fn test_complete_campaign_flow() {
        let (pool, tenant_id) = setup_db().await;
        let repo = Arc::new(CampaignRepository::new(pool));
        let service = MyCampaignService::new(repo);

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
            r#type: "Copy".to_string(),
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
    }

    #[tokio::test]
    async fn test_tenant_isolation() {
        let (pool1, tenant_1) = setup_db().await;
        let (_, tenant_2) = setup_db().await; // Setup second tenant, using same DB structure

        let repo = Arc::new(CampaignRepository::new(pool1.clone()));
        let service = MyCampaignService::new(repo);

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
            .execute(&pool1)
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
