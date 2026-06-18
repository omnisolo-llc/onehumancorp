use tonic::{Request, Response, Status};
use ::server_ohc::collective::{
    collective_service_server::CollectiveService,
    GetNearbyTenantsRequest, GetNearbyTenantsResponse,
    InviteTenantRequest, InviteTenantResponse,
    AcceptInviteRequest, AcceptInviteResponse,
    GetCollectivesRequest, GetCollectivesResponse,
    GetLoyaltyBalanceRequest, CollectiveLoyaltyBalance,
    EarnPointsRequest, RedeemPointsRequest, RedeemPointsResponse,
    Collective
};
use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::utils::geo;

pub struct MyCollectiveService {
    db: Arc<DB>,
}

impl MyCollectiveService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl CollectiveService for MyCollectiveService {
    async fn get_nearby_tenants(
        &self,
        request: Request<GetNearbyTenantsRequest>,
    ) -> Result<Response<GetNearbyTenantsResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let radius = req.radius_meters;

        // 1. Get tenant geohash
        let geohash: Option<String> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_scalar("SELECT geohash FROM tenants WHERE id = $1")
                    .bind(&tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
            DbStore::Sqlite(pool) => {
                sqlx::query_scalar("SELECT geohash FROM tenants WHERE id = ?")
                    .bind(&tenant_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
        };

        let Some(center_geohash) = geohash else {
            return Ok(Response::new(GetNearbyTenantsResponse { tenant_ids: vec![] }));
        };

        // 2. Fetch all tenants with geohashes (in production we'd use H3 radius queries or PostGIS)
        // For now, we perform a simple neighbor check if geohashes exist
        let all_tenants: Vec<(String, String)> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as("SELECT id, geohash FROM tenants WHERE geohash IS NOT NULL AND id != $1")
                    .bind(&tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
            DbStore::Sqlite(pool) => {
                sqlx::query_as("SELECT id, geohash FROM tenants WHERE geohash IS NOT NULL AND id != ?")
                    .bind(&tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
        };

        let mut nearby = Vec::new();
        for (tid, gh) in all_tenants {
            if geo::are_neighbors(&center_geohash, &gh) {
                nearby.push(tid);
            }
        }

        Ok(Response::new(GetNearbyTenantsResponse {
            tenant_ids: nearby,
        }))
    }

    async fn invite_tenant(
        &self,
        request: Request<InviteTenantRequest>,
    ) -> Result<Response<InviteTenantResponse>, Status> {
        let req = request.into_inner();
        let cid = req.collective_id;
        let target_id = req.target_tenant_id;

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES ($1, $2, 'PENDING') ON CONFLICT DO NOTHING")
                    .bind(&cid)
                    .bind(&target_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES (?, ?, 'PENDING') ON CONFLICT DO NOTHING")
                    .bind(&cid)
                    .bind(&target_id)
                    .execute(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
        }

        Ok(Response::new(InviteTenantResponse {
            success: true,
        }))
    }

    async fn accept_invite(
        &self,
        request: Request<AcceptInviteRequest>,
    ) -> Result<Response<AcceptInviteResponse>, Status> {
        let req = request.into_inner();
        let cid = req.collective_id;
        let tid = req.tenant_id;

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE ohc_collective_member SET status = 'ACTIVE' WHERE collective_id = $1 AND tenant_id = $2")
                    .bind(&cid)
                    .bind(&tid)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE ohc_collective_member SET status = 'ACTIVE' WHERE collective_id = ? AND tenant_id = ?")
                    .bind(&cid)
                    .bind(&tid)
                    .execute(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
        }

        Ok(Response::new(AcceptInviteResponse {
            success: true,
        }))
    }

    async fn get_collectives(
        &self,
        request: Request<GetCollectivesRequest>,
    ) -> Result<Response<GetCollectivesResponse>, Status> {
        let req = request.into_inner();
        let tid = req.tenant_id;

        let rows: Vec<(String, String, Option<String>, Option<f64>)> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as("SELECT c.id, c.name, c.location_center, c.radius_meters FROM ohc_collective c JOIN ohc_collective_member m ON c.id = m.collective_id WHERE m.tenant_id = $1 AND m.status = 'ACTIVE'")
                    .bind(&tid)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
            DbStore::Sqlite(pool) => {
                sqlx::query_as("SELECT c.id, c.name, c.location_center, c.radius_meters FROM ohc_collective c JOIN ohc_collective_member m ON c.id = m.collective_id WHERE m.tenant_id = ? AND m.status = 'ACTIVE'")
                    .bind(&tid)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
        };

        let collectives = rows.into_iter().map(|(id, name, loc, rad)| Collective {
            id,
            name,
            location_center: loc.unwrap_or_default(),
            radius_meters: rad.unwrap_or_default() as f32,
        }).collect();

        Ok(Response::new(GetCollectivesResponse { collectives }))
    }

    async fn get_loyalty_balance(
        &self,
        request: Request<GetLoyaltyBalanceRequest>,
    ) -> Result<Response<CollectiveLoyaltyBalance>, Status> {
        let req = request.into_inner();

        let balance: i32 = if req.tenant_id.is_empty() {
            // Get total balance for collective
            match &self.db.store {
                DbStore::Postgres => {
                    sqlx::query_scalar("SELECT COALESCE(SUM(balance), 0)::int4 FROM ohc_collective_loyalty_balance WHERE collective_id = $1 AND buyer_id = $2")
                        .bind(&req.collective_id)
                        .bind(&req.buyer_id)
                        .fetch_one(&self.db.pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?
                }
                DbStore::Sqlite(pool) => {
                    sqlx::query_scalar("SELECT COALESCE(SUM(balance), 0) FROM ohc_collective_loyalty_balance WHERE collective_id = ? AND buyer_id = ?")
                        .bind(&req.collective_id)
                        .bind(&req.buyer_id)
                        .fetch_one(pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?
                }
            }
        } else {
            // Get balance for specific tenant
            match &self.db.store {
                DbStore::Postgres => {
                    sqlx::query_scalar("SELECT balance FROM ohc_collective_loyalty_balance WHERE collective_id = $1 AND buyer_id = $2 AND tenant_id = $3")
                        .bind(&req.collective_id)
                        .bind(&req.buyer_id)
                        .bind(&req.tenant_id)
                        .fetch_optional(&self.db.pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?
                        .unwrap_or(0)
                }
                DbStore::Sqlite(pool) => {
                    sqlx::query_scalar("SELECT balance FROM ohc_collective_loyalty_balance WHERE collective_id = ? AND buyer_id = ? AND tenant_id = ?")
                        .bind(&req.collective_id)
                        .bind(&req.buyer_id)
                        .bind(&req.tenant_id)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?
                        .unwrap_or(0)
                }
            }
        };

        Ok(Response::new(CollectiveLoyaltyBalance {
            collective_id: req.collective_id,
            buyer_id: req.buyer_id,
            balance,
        }))
    }

    async fn earn_points(
        &self,
        request: Request<EarnPointsRequest>,
    ) -> Result<Response<CollectiveLoyaltyBalance>, Status> {
        let req = request.into_inner();

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("INSERT INTO ohc_collective_loyalty_balance (collective_id, buyer_id, tenant_id, balance) VALUES ($1, $2, $3, $4) ON CONFLICT(collective_id, buyer_id, tenant_id) DO UPDATE SET balance = ohc_collective_loyalty_balance.balance + $4, updated_at = CURRENT_TIMESTAMP")
                    .bind(&req.collective_id)
                    .bind(&req.buyer_id)
                    .bind(&req.tenant_id)
                    .bind(req.amount)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO ohc_collective_loyalty_balance (collective_id, buyer_id, tenant_id, balance) VALUES (?, ?, ?, ?) ON CONFLICT(collective_id, buyer_id, tenant_id) DO UPDATE SET balance = balance + ?, updated_at = CURRENT_TIMESTAMP")
                    .bind(&req.collective_id)
                    .bind(&req.buyer_id)
                    .bind(&req.tenant_id)
                    .bind(req.amount)
                    .bind(req.amount)
                    .execute(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
        }

        self.get_loyalty_balance(Request::new(GetLoyaltyBalanceRequest {
            collective_id: req.collective_id,
            buyer_id: req.buyer_id,
            tenant_id: req.tenant_id,
        })).await
    }

    async fn redeem_points(
        &self,
        request: Request<RedeemPointsRequest>,
    ) -> Result<Response<RedeemPointsResponse>, Status> {
        let req = request.into_inner();
        let cid = req.collective_id;
        let bid = req.buyer_id;
        let tid = req.tenant_id;
        let mut amount_to_redeem = req.amount;

        // 1. Check total balance
        let current_balance_res = self.get_loyalty_balance(Request::new(GetLoyaltyBalanceRequest {
            collective_id: cid.clone(),
            buyer_id: bid.clone(),
            tenant_id: "".to_string(),
        })).await?;

        let total_balance = current_balance_res.into_inner().balance;
        if total_balance < amount_to_redeem {
            return Ok(Response::new(RedeemPointsResponse {
                success: false,
                message: "Insufficient balance".to_string(),
                new_balance: None,
            }));
        }

        // 2. Fetch breakdown of points per originating tenant
        let mut balances: Vec<(String, i32)> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as("SELECT tenant_id, balance FROM ohc_collective_loyalty_balance WHERE collective_id = $1 AND buyer_id = $2 AND balance > 0 ORDER BY updated_at ASC")
                    .bind(&cid)
                    .bind(&bid)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
            DbStore::Sqlite(pool) => {
                sqlx::query_as("SELECT tenant_id, balance FROM ohc_collective_loyalty_balance WHERE collective_id = ? AND buyer_id = ? AND balance > 0 ORDER BY updated_at ASC")
                    .bind(&cid)
                    .bind(&bid)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?
            }
        };

        // 3. Deduct points (FIFO approach based on updated_at) and record in Shared Ledger
        let mut tx = match &self.db.store {
            DbStore::Postgres => Some(self.db.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?),
            DbStore::Sqlite(_) => None, // Sqlite transactions are harder to pass around in this pattern
        };

        for (originator_id, balance) in balances {
            if amount_to_redeem <= 0 { break; }

            let to_deduct = std::cmp::min(amount_to_redeem, balance);

            match &self.db.store {
                DbStore::Postgres => {
                    let tx_ref = tx.as_mut().unwrap();
                    sqlx::query("UPDATE ohc_collective_loyalty_balance SET balance = balance - $1, updated_at = CURRENT_TIMESTAMP WHERE collective_id = $2 AND buyer_id = $3 AND tenant_id = $4")
                        .bind(to_deduct)
                        .bind(&cid)
                        .bind(&bid)
                        .bind(&originator_id)
                        .execute(&mut **tx_ref)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;

                    let value_cents = (to_deduct as i64) * 10;
                    sqlx::query("INSERT INTO ohc_shared_loyalty_ledger (id, collective_id, originating_tenant_id, target_tenant_id, buyer_id, points_redeemed, value_cents) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(&cid)
                        .bind(&originator_id)
                        .bind(&tid)
                        .bind(&bid)
                        .bind(to_deduct)
                        .bind(value_cents)
                        .execute(&mut **tx_ref)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
                DbStore::Sqlite(pool) => {
                    sqlx::query("UPDATE ohc_collective_loyalty_balance SET balance = balance - ? WHERE collective_id = ? AND buyer_id = ? AND tenant_id = ?")
                        .bind(to_deduct)
                        .bind(&cid)
                        .bind(&bid)
                        .bind(&originator_id)
                        .execute(pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;

                    let value_cents = (to_deduct as i64) * 10;
                    sqlx::query("INSERT INTO ohc_shared_loyalty_ledger (id, collective_id, originating_tenant_id, target_tenant_id, buyer_id, points_redeemed, value_cents) VALUES (?, ?, ?, ?, ?, ?, ?)")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(&cid)
                        .bind(&originator_id)
                        .bind(&tid)
                        .bind(&bid)
                        .bind(to_deduct)
                        .bind(value_cents)
                        .execute(pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
            }

            amount_to_redeem -= to_deduct;
        }

        if let Some(t) = tx {
            t.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        }

        let new_balance_res = self.get_loyalty_balance(Request::new(GetLoyaltyBalanceRequest {
            collective_id: cid.clone(),
            buyer_id: bid.clone(),
            tenant_id: "".to_string(),
        })).await?;

        Ok(Response::new(RedeemPointsResponse {
            success: true,
            message: "Success".to_string(),
            new_balance: Some(new_balance_res.into_inner()),
        }))
    }
}
