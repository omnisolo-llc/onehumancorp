use tonic::{Request, Response, Status};
use ::server_ohc::collective::{
    collective_service_server::CollectiveService,
    GetNearbyTenantsRequest, GetNearbyTenantsResponse,
    InviteTenantRequest, InviteTenantResponse,
    AcceptInviteRequest, AcceptInviteResponse,
    GetCollectivesRequest, GetCollectivesResponse,
    Collective
};
use sqlx::PgPool;

pub struct MyCollectiveService {
    pool: PgPool,
}

impl MyCollectiveService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl CollectiveService for MyCollectiveService {
    async fn get_nearby_tenants(
        &self,
        request: Request<GetNearbyTenantsRequest>,
    ) -> Result<Response<GetNearbyTenantsResponse>, Status> {
        let _req = request.into_inner();

        // Mock implementation for discovery - returns mock neighbors
        let tenant_ids = vec![
            "carlos_repairs".to_string(),
            "fatima_food_cart".to_string(),
        ];

        Ok(Response::new(GetNearbyTenantsResponse {
            tenant_ids,
        }))
    }

    async fn invite_tenant(
        &self,
        request: Request<InviteTenantRequest>,
    ) -> Result<Response<InviteTenantResponse>, Status> {
        let req = request.into_inner();

        let tenant_id = req.target_tenant_id;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES ($1, $2, 'PENDING') ON CONFLICT DO NOTHING"
        )
        .bind(req.collective_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(InviteTenantResponse {
            success: true,
        }))
    }

    async fn accept_invite(
        &self,
        request: Request<AcceptInviteRequest>,
    ) -> Result<Response<AcceptInviteResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "UPDATE ohc_collective_member SET status = 'ACTIVE' WHERE collective_id = $1 AND tenant_id = $2"
        )
        .bind(req.collective_id)
        .bind(req.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AcceptInviteResponse {
            success: true,
        }))
    }

    async fn get_collectives(
        &self,
        request: Request<GetCollectivesRequest>,
    ) -> Result<Response<GetCollectivesResponse>, Status> {
        let req = request.into_inner();

        // sqlx::query as typed isn't trivial without macros, so use sqlx::query_as
        #[derive(sqlx::FromRow)]
        struct CollectiveRow {
            id: String,
            name: String,
            location_center: Option<String>,
            radius_meters: Option<f64>,
        }

        let records = sqlx::query_as::<_, CollectiveRow>(
            "SELECT c.id, c.name, c.location_center, c.radius_meters
             FROM ohc_collective c
             JOIN ohc_collective_member m ON c.id = m.collective_id
             WHERE m.tenant_id = $1"
        )
        .bind(req.tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let collectives = records.into_iter().map(|rec| Collective {
            id: rec.id,
            name: rec.name,
            location_center: rec.location_center.unwrap_or_default(),
            radius_meters: rec.radius_meters.unwrap_or_default() as f32,
        }).collect();

        Ok(Response::new(GetCollectivesResponse {
            collectives,
        }))
    }
}
