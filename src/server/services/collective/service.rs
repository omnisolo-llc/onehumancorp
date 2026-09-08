use ::server_ohc::collective::{
    AcceptInviteRequest, AcceptInviteResponse, Collective, GetCollectivesRequest,
    GetCollectivesResponse, GetNearbyTenantsRequest, GetNearbyTenantsResponse, InviteTenantRequest,
    InviteTenantResponse, collective_service_server::CollectiveService,
};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

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
        let req = request.into_inner();
        let tenant_id = req.tenant_id.trim();
        if tenant_id.is_empty() || tenant_id.eq_ignore_ascii_case("system") {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let tenant_ids = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT m.tenant_id
             FROM ohc_collective c
             JOIN ohc_collective_member m ON m.collective_id = c.id
             WHERE c.tenant_id = $1
               AND m.tenant_id <> $1
               AND m.status IN ('ACTIVE', 'PENDING')
             LIMIT 50",
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetNearbyTenantsResponse { tenant_ids }))
    }

    async fn invite_tenant(
        &self,
        request: Request<InviteTenantRequest>,
    ) -> Result<Response<InviteTenantResponse>, Status> {
        let req = request.into_inner();

        if req.collective_id.trim().is_empty()
            || req.target_tenant_id.trim().is_empty()
            || req.target_tenant_id.eq_ignore_ascii_case("system")
        {
            return Err(Status::invalid_argument(
                "collective_id and target_tenant_id are required",
            ));
        }

        let tenant_id = req.target_tenant_id;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let target_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM tenants WHERE id = $1 AND id <> 'system')",
        )
        .bind(&tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        if !target_exists {
            return Err(Status::not_found("target tenant not found"));
        }
        let collective_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM ohc_collective WHERE id = $1)",
        )
        .bind(req.collective_id.trim())
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        if !collective_exists {
            return Err(Status::not_found("collective not found"));
        }

        sqlx::query(
            "INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES ($1, $2, 'PENDING') ON CONFLICT DO NOTHING"
        )
        .bind(req.collective_id.trim())
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(InviteTenantResponse { success: true }))
    }

    async fn accept_invite(
        &self,
        request: Request<AcceptInviteRequest>,
    ) -> Result<Response<AcceptInviteResponse>, Status> {
        let req = request.into_inner();

        if req.collective_id.trim().is_empty() || req.tenant_id.trim().is_empty() {
            return Err(Status::invalid_argument(
                "collective_id and tenant_id are required",
            ));
        }
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let result = sqlx::query(
            "UPDATE ohc_collective_member SET status = 'ACTIVE' WHERE collective_id = $1 AND tenant_id = $2"
        )
        .bind(req.collective_id.trim())
        .bind(req.tenant_id.trim())
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(Status::not_found("collective invitation not found"));
        }

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AcceptInviteResponse { success: true }))
    }

    async fn get_collectives(
        &self,
        request: Request<GetCollectivesRequest>,
    ) -> Result<Response<GetCollectivesResponse>, Status> {
        let req = request.into_inner();

        if req.tenant_id.trim().is_empty() || req.tenant_id.eq_ignore_ascii_case("system") {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        // sqlx::query as typed isn't trivial without macros, so use sqlx::query_as
        #[derive(sqlx::FromRow)]
        struct CollectiveRow {
            id: String,
            name: String,
            location_center: Option<String>,
            radius_meters: Option<f64>,
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let records = sqlx::query_as::<_, CollectiveRow>(
            "SELECT c.id, c.name, c.location_center, c.radius_meters
             FROM ohc_collective c
             JOIN ohc_collective_member m ON c.id = m.collective_id
             WHERE m.tenant_id = $1",
        )
        .bind(req.tenant_id.trim())
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let collectives = records
            .into_iter()
            .map(|rec| Collective {
                id: rec.id,
                name: rec.name,
                location_center: rec.location_center.unwrap_or_default(),
                radius_meters: rec.radius_meters.unwrap_or_default() as f32,
            })
            .collect();

        Ok(Response::new(GetCollectivesResponse { collectives }))
    }
}
