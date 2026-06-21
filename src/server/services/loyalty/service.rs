use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;

use loyalty_proto::ohc::loyalty::{
    loyalty_service_server::LoyaltyService,
    CreateLoyaltyProgramRequest, CreateLoyaltyProgramResponse,
    CustomerLoyaltyAccount, EarnPointsRequest, EarnPointsResponse,
    GetCustomerLoyaltyStatusRequest, GetCustomerLoyaltyStatusResponse, LoyaltyProgram,
    LoyaltyTransaction, RedeemRewardRequest, RedeemRewardResponse,
    UpdateLoyaltyProgramRequest, UpdateLoyaltyProgramResponse,
};

pub struct LoyaltyServiceImpl {
    pool: PgPool,
}

impl LoyaltyServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LoyaltyService for LoyaltyServiceImpl {
    async fn create_loyalty_program(
        &self,
        request: Request<CreateLoyaltyProgramRequest>,
    ) -> Result<Response<CreateLoyaltyProgramResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&id)
        .bind(&req.tenant_id)
        .bind(&req.name)
        .bind(&req.program_type)
        .bind(&req.config)
        .bind(true)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateLoyaltyProgramResponse {
            program: Some(LoyaltyProgram {
                id,
                tenant_id: req.tenant_id,
                name: req.name,
                program_type: req.program_type,
                config: req.config,
                is_active: true,
                created_at: Utc::now().timestamp_millis(),
                updated_at: Utc::now().timestamp_millis(),
            }),
        }))
    }

    async fn update_loyalty_program(
        &self,
        request: Request<UpdateLoyaltyProgramRequest>,
    ) -> Result<Response<UpdateLoyaltyProgramResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut update_query = "UPDATE loyalty_programs SET updated_at = CURRENT_TIMESTAMP".to_string();
        let mut bind_index = 3;

        if req.name.is_some() {
            update_query.push_str(&format!(", name = ${}", bind_index));
            bind_index += 1;
        }
        if req.config.is_some() {
            update_query.push_str(&format!(", config = ${}", bind_index));
            bind_index += 1;
        }
        if req.is_active.is_some() {
            update_query.push_str(&format!(", is_active = ${}", bind_index));
            bind_index += 1;
        }

        update_query.push_str(" WHERE id = $1 AND tenant_id = $2 RETURNING *");

        let mut query = sqlx::query(&update_query)
            .bind(&req.id)
            .bind(&req.tenant_id);

        if let Some(ref name) = req.name {
            query = query.bind(name);
        }
        if let Some(ref config) = req.config {
            query = query.bind(config);
        }
        if let Some(is_active) = req.is_active {
            query = query.bind(is_active);
        }

        let row = query.fetch_one(&mut *tx).await.map_err(|e| Status::not_found(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateLoyaltyProgramResponse {
            program: Some(LoyaltyProgram {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                name: row.get("name"),
                program_type: row.get("program_type"),
                config: row.get("config"),
                is_active: row.get("is_active"),
                created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").timestamp_millis(),
                updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").timestamp_millis(),
            }),
        }))
    }

    async fn earn_points(
        &self,
        request: Request<EarnPointsRequest>,
    ) -> Result<Response<EarnPointsResponse>, Status> {
        let req = request.into_inner();
        let tx_id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO loyalty_transactions (id, tenant_id, customer_id, program_id, transaction_type, points, punches, description) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&req.customer_id)
        .bind(&req.program_id)
        .bind("earn")
        .bind(req.points)
        .bind(req.punches)
        .bind(&req.description)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let account_id = Uuid::new_v4().to_string();
        let account_row = sqlx::query(
            r#"
            INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id, points_balance, punches)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, customer_id, program_id)
            DO UPDATE SET points_balance = customer_loyalty_accounts.points_balance + EXCLUDED.points_balance,
                          punches = customer_loyalty_accounts.punches + EXCLUDED.punches,
                          updated_at = CURRENT_TIMESTAMP
            RETURNING *
            "#
        )
        .bind(&account_id)
        .bind(&req.tenant_id)
        .bind(&req.customer_id)
        .bind(&req.program_id)
        .bind(req.points)
        .bind(req.punches)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EarnPointsResponse {
            transaction: Some(LoyaltyTransaction {
                id: tx_id,
                tenant_id: req.tenant_id.clone(),
                customer_id: req.customer_id.clone(),
                program_id: req.program_id.clone(),
                transaction_type: "earn".to_string(),
                points: req.points,
                punches: req.punches,
                description: req.description,
                created_at: Utc::now().timestamp_millis(),
            }),
            account: Some(CustomerLoyaltyAccount {
                id: account_row.get("id"),
                tenant_id: account_row.get("tenant_id"),
                customer_id: account_row.get("customer_id"),
                program_id: account_row.get("program_id"),
                points_balance: account_row.get("points_balance"),
                punches: account_row.get("punches"),
                tier_name: account_row.get("tier_name"),
                created_at: account_row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").timestamp_millis(),
                updated_at: account_row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").timestamp_millis(),
            }),
        }))
    }

    async fn redeem_reward(
        &self,
        request: Request<RedeemRewardRequest>,
    ) -> Result<Response<RedeemRewardResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // 1. Fetch the reward
        let reward_row = sqlx::query("SELECT * FROM rewards WHERE id = $1 AND tenant_id = $2 AND program_id = $3")
            .bind(&req.reward_id)
            .bind(&req.tenant_id)
            .bind(&req.program_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let reward_row = match reward_row {
            Some(row) => row,
            None => return Err(Status::not_found("Reward not found")),
        };

        let points_cost: i32 = reward_row.get("points_cost");
        let punches_cost: i32 = reward_row.get("punches_cost");

        // 2. Fetch current balance
        let account_row = sqlx::query("SELECT * FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3")
            .bind(&req.tenant_id)
            .bind(&req.customer_id)
            .bind(&req.program_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let account_row = match account_row {
            Some(row) => row,
            None => return Err(Status::not_found("Customer account not found")),
        };

        let current_points: i32 = account_row.get("points_balance");
        let current_punches: i32 = account_row.get("punches");

        if current_points < points_cost || current_punches < punches_cost {
            return Ok(Response::new(RedeemRewardResponse {
                transaction: None,
                account: None,
                success: false,
                message: "Insufficient balance".to_string(),
            }));
        }

        // 3. Deduct points/punches
        let updated_account_row = sqlx::query(
            "UPDATE customer_loyalty_accounts SET points_balance = points_balance - $1, punches = punches - $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 RETURNING *"
        )
        .bind(points_cost)
        .bind(punches_cost)
        .bind(account_row.get::<String, _>("id"))
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // 4. Record transaction
        let tx_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO loyalty_transactions (id, tenant_id, customer_id, program_id, transaction_type, points, punches, description) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&req.customer_id)
        .bind(&req.program_id)
        .bind("redeem")
        .bind(-points_cost)
        .bind(-punches_cost)
        .bind(format!("Redeemed reward: {}", reward_row.get::<String, _>("name")))
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RedeemRewardResponse {
            transaction: Some(LoyaltyTransaction {
                id: tx_id,
                tenant_id: req.tenant_id.clone(),
                customer_id: req.customer_id.clone(),
                program_id: req.program_id.clone(),
                transaction_type: "redeem".to_string(),
                points: -points_cost,
                punches: -punches_cost,
                description: format!("Redeemed reward: {}", reward_row.get::<String, _>("name")),
                created_at: Utc::now().timestamp_millis(),
            }),
            account: Some(CustomerLoyaltyAccount {
                id: updated_account_row.get("id"),
                tenant_id: updated_account_row.get("tenant_id"),
                customer_id: updated_account_row.get("customer_id"),
                program_id: updated_account_row.get("program_id"),
                points_balance: updated_account_row.get("points_balance"),
                punches: updated_account_row.get("punches"),
                tier_name: updated_account_row.get("tier_name"),
                created_at: updated_account_row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").timestamp_millis(),
                updated_at: updated_account_row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").timestamp_millis(),
            }),
            success: true,
            message: "Reward redeemed successfully".to_string(),
        }))
    }

    async fn get_customer_loyalty_status(
        &self,
        request: Request<GetCustomerLoyaltyStatusRequest>,
    ) -> Result<Response<GetCustomerLoyaltyStatusResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let account_row = sqlx::query("SELECT * FROM customer_loyalty_accounts WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3")
            .bind(&req.tenant_id)
            .bind(&req.customer_id)
            .bind(&req.program_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        match account_row {
            Some(row) => Ok(Response::new(GetCustomerLoyaltyStatusResponse {
                account: Some(CustomerLoyaltyAccount {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    customer_id: row.get("customer_id"),
                    program_id: row.get("program_id"),
                    points_balance: row.get("points_balance"),
                    punches: row.get("punches"),
                    tier_name: row.get("tier_name"),
                    created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").timestamp_millis(),
                    updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").timestamp_millis(),
                }),
            })),
            None => Err(Status::not_found("Customer account not found")),
        }
    }
}
