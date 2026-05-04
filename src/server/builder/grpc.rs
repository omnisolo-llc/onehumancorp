use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use sqlx::PgPool;

use crate::ohc::orchestration::sites_service_server::SitesService;
use crate::ohc::orchestration::{
    BlockProto, GenerateSiteRequest, GenerateSiteResponse, PageProto, PublishSiteRequest,
    PublishSiteResponse, SiteProto, UpdateBlockRequest, UpdateBlockResponse,
};

pub struct SitesServiceImpl {
    pool: PgPool,
}

impl SitesServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl SitesService for SitesServiceImpl {
    async fn generate_site(
        &self,
        request: Request<GenerateSiteRequest>,
    ) -> Result<Response<GenerateSiteResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        // 1. Create a draft site
        let site = super::db::create_site(&self.pool, tenant_id, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to create site: {}", e)))?;

        // 2. Schedule AI task to generate layout
        sqlx::query("INSERT INTO tasks (tenant_id, mission_type, payload, status) VALUES ($1, 'generate_site', $2, 'pending') ON CONFLICT DO NOTHING")
            .bind(tenant_id)
            .bind(serde_json::json!({
                "site_id": site.id,
                "business_category": req.business_category,
                "initial_prompt": req.initial_prompt
            }))
            .execute(&self.pool)
            .await
            .map_err(|e| Status::internal(format!("Failed to schedule AI task: {}", e)))?;

        Ok(Response::new(GenerateSiteResponse {
            site: Some(SiteProto {
                id: site.id.to_string(),
                domain: "".to_string(),
            }),
            pages: vec![],
            blocks: vec![],
        }))
    }

    async fn update_block(
        &self,
        request: Request<UpdateBlockRequest>,
    ) -> Result<Response<UpdateBlockResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let block_id = Uuid::parse_str(&req.block_id)
            .map_err(|_| Status::invalid_argument("Invalid block_id"))?;

        let content_json: serde_json::Value = serde_json::from_str(&req.content_json)
            .map_err(|_| Status::invalid_argument("Invalid content_json"))?;

        let block = super::db::update_block(&self.pool, tenant_id, block_id, content_json)
            .await
            .map_err(|e| Status::internal(format!("Failed to update block: {}", e)))?;

        Ok(Response::new(UpdateBlockResponse {
            block: Some(BlockProto {
                id: block.id.to_string(),
                block_type: block.block_type,
                content_json: block.content.to_string(),
                sort_order: block.sort_order,
            }),
        }))
    }

    async fn publish_site(
        &self,
        request: Request<PublishSiteRequest>,
    ) -> Result<Response<PublishSiteResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let site_id = Uuid::parse_str(&req.site_id)
            .map_err(|_| Status::invalid_argument("Invalid site_id"))?;

        super::jobs::enqueue_publish_site_job(&self.pool, tenant_id, site_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to publish site: {}", e)))?;

        Ok(Response::new(PublishSiteResponse {
            live_url: format!("https://{}.ohc.app", site_id),
        }))
    }
}
