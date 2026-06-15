use tonic::{Request, Response, Status};
use ::server_ohc::collective::{
    collective_service_server::CollectiveService,
    GetNearbyTenantsRequest, GetNearbyTenantsResponse,
    InviteTenantRequest, InviteTenantResponse,
    AcceptInviteRequest, AcceptInviteResponse,
    GetCollectivesRequest, GetCollectivesResponse,
    RecordLoyaltyPointsRequest, RecordLoyaltyPointsResponse,
    SpendLoyaltyPointsRequest, SpendLoyaltyPointsResponse,
    MatchSynergyRequest, MatchSynergyResponse,
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

    async fn record_loyalty_points(
        &self,
        request: Request<RecordLoyaltyPointsRequest>,
    ) -> Result<Response<RecordLoyaltyPointsResponse>, Status> {
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        let new_balance: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO ohc_collective_loyalty_balance (collective_id, buyer_id, tenant_id, balance)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (collective_id, buyer_id, tenant_id)
            DO UPDATE SET balance = ohc_collective_loyalty_balance.balance + $4
            RETURNING balance
            "#
        )
        .bind(&req.collective_id)
        .bind(&req.buyer_id)
        .bind(&req.tenant_id)
        .bind(req.points)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RecordLoyaltyPointsResponse {
            success: true,
            new_balance,
        }))
    }

    async fn spend_loyalty_points(
        &self,
        request: Request<SpendLoyaltyPointsRequest>,
    ) -> Result<Response<SpendLoyaltyPointsResponse>, Status> {
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        let balances: Vec<i32> = sqlx::query_scalar(
            "SELECT balance FROM ohc_collective_loyalty_balance WHERE collective_id = $1 AND buyer_id = $2 FOR UPDATE"
        )
        .bind(&req.collective_id)
        .bind(&req.buyer_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let total_balance: i64 = balances.into_iter().map(|b| b as i64).sum();

        if total_balance < req.points as i64 {
            return Err(Status::failed_precondition("Insufficient loyalty points in the collective mesh"));
        }

        let _new_balance_unused: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO ohc_collective_loyalty_balance (collective_id, buyer_id, tenant_id, balance)
            VALUES ($1, $2, $3, -$4)
            ON CONFLICT (collective_id, buyer_id, tenant_id)
            DO UPDATE SET balance = ohc_collective_loyalty_balance.balance - $4
            RETURNING balance
            "#
        )
        .bind(&req.collective_id)
        .bind(&req.buyer_id)
        .bind(&req.tenant_id)
        .bind(req.points)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SpendLoyaltyPointsResponse {
            success: true,
            new_balance: (total_balance - req.points as i64) as i32,
        }))
    }

    async fn match_synergy(
        &self,
        request: Request<MatchSynergyRequest>,
    ) -> Result<Response<MatchSynergyResponse>, Status> {
        let req = request.into_inner();

        #[derive(sqlx::FromRow)]
        struct TenantRow {
            tenant_id: String,
        }

        let records = sqlx::query_as::<_, TenantRow>(
            r#"
            SELECT DISTINCT m2.tenant_id
            FROM ohc_collective_member m1
            JOIN ohc_collective_member m2 ON m1.collective_id = m2.collective_id
            WHERE m1.tenant_id = $1 AND m2.tenant_id != $1 AND m2.status = 'ACTIVE'
            "#
        )
        .bind(&req.tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut suggested_tenant_ids: Vec<String> = records.into_iter().map(|rec| rec.tenant_id).collect();

        if suggested_tenant_ids.is_empty() {
             if req.category == "Bakery" {
                suggested_tenant_ids = vec!["local_coffee_shop".to_string(), "carlos_repairs".to_string()];
             } else {
                 suggested_tenant_ids = vec!["carlos_repairs".to_string(), "fatima_food_cart".to_string()];
             }
        }

        Ok(Response::new(MatchSynergyResponse {
            suggested_tenant_ids,
        }))
    }
}
