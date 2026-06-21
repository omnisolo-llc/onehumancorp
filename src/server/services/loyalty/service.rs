use crate::db::get_pool;
use crate::ultraplan::generate_id;
use ohc_rust_protos::loyalty::{
    CreateLoyaltyProgramRequest, CreateRewardRequest, CustomerLoyaltyAccount,
    EarnPointsRequest, GetCustomerAccountRequest, GetLoyaltyProgramRequest,
    ListLoyaltyProgramsRequest, ListLoyaltyProgramsResponse, ListRewardsRequest,
    ListRewardsResponse, LoyaltyProgram, RedeemRewardRequest, RedeemRewardResponse,
    Reward, UpdateLoyaltyProgramRequest, UpdateRewardRequest,
};
use ohc_rust_protos::loyalty::loyalty_service_server::LoyaltyService;
use sqlx::{PgPool, Row};
use tonic::{Request, Response, Status};
use chrono::Utc;
use serde_json::json;

#[derive(Clone)]
pub struct LoyaltyServiceImpl {
    pub pool: PgPool,
}

impl LoyaltyServiceImpl {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }
}

#[tonic::async_trait]
impl LoyaltyService for LoyaltyServiceImpl {
    async fn create_loyalty_program(
        &self,
        request: Request<CreateLoyaltyProgramRequest>,
    ) -> Result<Response<LoyaltyProgram>, Status> {
        let req = request.into_inner();
        let id = generate_id("lp");
        let tenant_id = req.tenant_id;
        let config_json = if req.config_json.is_empty() { "{}".to_string() } else { req.config_json };

        let row = sqlx::query(
            "INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active)
             VALUES ($1, $2, $3, $4, $5::jsonb, $6)
             RETURNING id, tenant_id, name, program_type, config::text as config_json, is_active, created_at, updated_at",
        )
        .bind(&id)
        .bind(&tenant_id)
        .bind(&req.name)
        .bind(&req.program_type)
        .bind(&config_json)
        .bind(true)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to create program: {}", e)))?;

        Ok(Response::new(LoyaltyProgram {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            program_type: row.get("program_type"),
            config_json: row.get("config_json"),
            is_active: row.get("is_active"),
            created_at: row.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: row.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
        }))
    }

    async fn update_loyalty_program(
        &self,
        request: Request<UpdateLoyaltyProgramRequest>,
    ) -> Result<Response<LoyaltyProgram>, Status> {
        let req = request.into_inner();
        let config_json = if req.config_json.is_empty() { "{}".to_string() } else { req.config_json };

        let row = sqlx::query(
            "UPDATE loyalty_programs SET name = $1, config = $2::jsonb, is_active = $3, updated_at = CURRENT_TIMESTAMP
             WHERE id = $4 AND tenant_id = $5
             RETURNING id, tenant_id, name, program_type, config::text as config_json, is_active, created_at, updated_at",
        )
        .bind(&req.name)
        .bind(&config_json)
        .bind(req.is_active)
        .bind(&req.program_id)
        .bind(&req.tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to update program: {}", e)))?;

        match row {
            Some(r) => Ok(Response::new(LoyaltyProgram {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                name: r.get("name"),
                program_type: r.get("program_type"),
                config_json: r.get("config_json"),
                is_active: r.get("is_active"),
                created_at: r.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: r.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
            })),
            None => Err(Status::not_found("Loyalty program not found")),
        }
    }

    async fn get_loyalty_program(
        &self,
        request: Request<GetLoyaltyProgramRequest>,
    ) -> Result<Response<LoyaltyProgram>, Status> {
        let req = request.into_inner();

        let row = sqlx::query(
            "SELECT id, tenant_id, name, program_type, config::text as config_json, is_active, created_at, updated_at
             FROM loyalty_programs WHERE id = $1 AND tenant_id = $2",
        )
        .bind(&req.program_id)
        .bind(&req.tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch program: {}", e)))?;

        match row {
            Some(r) => Ok(Response::new(LoyaltyProgram {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                name: r.get("name"),
                program_type: r.get("program_type"),
                config_json: r.get("config_json"),
                is_active: r.get("is_active"),
                created_at: r.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: r.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
            })),
            None => Err(Status::not_found("Loyalty program not found")),
        }
    }

    async fn list_loyalty_programs(
        &self,
        request: Request<ListLoyaltyProgramsRequest>,
    ) -> Result<Response<ListLoyaltyProgramsResponse>, Status> {
        let req = request.into_inner();

        let rows = sqlx::query(
            "SELECT id, tenant_id, name, program_type, config::text as config_json, is_active, created_at, updated_at
             FROM loyalty_programs WHERE tenant_id = $1",
        )
        .bind(&req.tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to list programs: {}", e)))?;

        let programs = rows.into_iter().map(|r| LoyaltyProgram {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            name: r.get("name"),
            program_type: r.get("program_type"),
            config_json: r.get("config_json"),
            is_active: r.get("is_active"),
            created_at: r.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: r.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
        }).collect();

        Ok(Response::new(ListLoyaltyProgramsResponse { programs }))
    }

    async fn get_customer_account(
        &self,
        request: Request<GetCustomerAccountRequest>,
    ) -> Result<Response<CustomerLoyaltyAccount>, Status> {
        let req = request.into_inner();

        let row = sqlx::query(
            "SELECT id, tenant_id, program_id, customer_id, points_balance, punches_count, tier_id, created_at, updated_at
             FROM customer_loyalty_accounts WHERE tenant_id = $1 AND program_id = $2 AND customer_id = $3",
        )
        .bind(&req.tenant_id)
        .bind(&req.program_id)
        .bind(&req.customer_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch account: {}", e)))?;

        match row {
            Some(r) => Ok(Response::new(CustomerLoyaltyAccount {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                program_id: r.get("program_id"),
                customer_id: r.get("customer_id"),
                points_balance: r.get("points_balance"),
                punches_count: r.get("punches_count"),
                tier_id: r.get::<Option<String>, _>("tier_id").unwrap_or_default(),
                created_at: r.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: r.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
            })),
            None => {
                // Auto-create account if it doesn't exist
                let id = generate_id("la");
                let new_row = sqlx::query(
                    "INSERT INTO customer_loyalty_accounts (id, tenant_id, program_id, customer_id)
                     VALUES ($1, $2, $3, $4)
                     RETURNING id, tenant_id, program_id, customer_id, points_balance, punches_count, tier_id, created_at, updated_at",
                )
                .bind(&id)
                .bind(&req.tenant_id)
                .bind(&req.program_id)
                .bind(&req.customer_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| Status::internal(format!("Failed to create account: {}", e)))?;

                Ok(Response::new(CustomerLoyaltyAccount {
                    id: new_row.get("id"),
                    tenant_id: new_row.get("tenant_id"),
                    program_id: new_row.get("program_id"),
                    customer_id: new_row.get("customer_id"),
                    points_balance: new_row.get("points_balance"),
                    punches_count: new_row.get("punches_count"),
                    tier_id: new_row.get::<Option<String>, _>("tier_id").unwrap_or_default(),
                    created_at: new_row.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
                    updated_at: new_row.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
                }))
            }
        }
    }

    async fn earn_points(
        &self,
        request: Request<EarnPointsRequest>,
    ) -> Result<Response<CustomerLoyaltyAccount>, Status> {
        let req = request.into_inner();

        // Ensure account exists
        let account_req = GetCustomerAccountRequest {
            tenant_id: req.tenant_id.clone(),
            program_id: req.program_id.clone(),
            customer_id: req.customer_id.clone(),
        };
        let account_res = self.get_customer_account(Request::new(account_req)).await?.into_inner();
        let account_id = account_res.id;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Update account
        let row = sqlx::query(
            "UPDATE customer_loyalty_accounts
             SET points_balance = points_balance + $1, punches_count = punches_count + $2, updated_at = CURRENT_TIMESTAMP
             WHERE id = $3 AND tenant_id = $4
             RETURNING id, tenant_id, program_id, customer_id, points_balance, punches_count, tier_id, created_at, updated_at",
        )
        .bind(req.points)
        .bind(req.punches)
        .bind(&account_id)
        .bind(&req.tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update account balances: {}", e)))?;

        // Log transaction
        let tx_id = generate_id("lt");
        sqlx::query(
            "INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, punches, reason, order_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&account_id)
        .bind("earn")
        .bind(req.points)
        .bind(req.punches)
        .bind(&req.reason)
        .bind(&req.order_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to log transaction: {}", e)))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        // Trigger Event
        let _ = crate::msgbus::publish(
            &req.tenant_id,
            "system",
            "loyalty.points_awarded",
            &json!({
                "account_id": account_id,
                "customer_id": req.customer_id,
                "program_id": req.program_id,
                "points_earned": req.points,
                "punches_earned": req.punches,
                "order_id": req.order_id,
            }),
        ).await;

        Ok(Response::new(CustomerLoyaltyAccount {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            program_id: row.get("program_id"),
            customer_id: row.get("customer_id"),
            points_balance: row.get("points_balance"),
            punches_count: row.get("punches_count"),
            tier_id: row.get::<Option<String>, _>("tier_id").unwrap_or_default(),
            created_at: row.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: row.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
        }))
    }

    async fn create_reward(
        &self,
        request: Request<CreateRewardRequest>,
    ) -> Result<Response<Reward>, Status> {
        let req = request.into_inner();
        let id = generate_id("rw");
        let reward_value_json = if req.reward_value_json.is_empty() { "{}".to_string() } else { req.reward_value_json };

        let row = sqlx::query(
            "INSERT INTO rewards (id, tenant_id, program_id, name, description, points_cost, reward_type, reward_value)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb)
             RETURNING id, tenant_id, program_id, name, description, points_cost, reward_type, reward_value::text as reward_value_json, is_active, created_at, updated_at",
        )
        .bind(&id)
        .bind(&req.tenant_id)
        .bind(&req.program_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.points_cost)
        .bind(&req.reward_type)
        .bind(&reward_value_json)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to create reward: {}", e)))?;

        Ok(Response::new(Reward {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            program_id: row.get("program_id"),
            name: row.get("name"),
            description: row.get::<Option<String>, _>("description").unwrap_or_default(),
            points_cost: row.get("points_cost"),
            reward_type: row.get("reward_type"),
            reward_value_json: row.get("reward_value_json"),
            is_active: row.get("is_active"),
            created_at: row.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: row.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
        }))
    }

    async fn update_reward(
        &self,
        request: Request<UpdateRewardRequest>,
    ) -> Result<Response<Reward>, Status> {
        let req = request.into_inner();

        let row = sqlx::query(
            "UPDATE rewards SET name = $1, description = $2, points_cost = $3, is_active = $4, updated_at = CURRENT_TIMESTAMP
             WHERE id = $5 AND tenant_id = $6
             RETURNING id, tenant_id, program_id, name, description, points_cost, reward_type, reward_value::text as reward_value_json, is_active, created_at, updated_at",
        )
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.points_cost)
        .bind(req.is_active)
        .bind(&req.reward_id)
        .bind(&req.tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to update reward: {}", e)))?;

        match row {
            Some(r) => Ok(Response::new(Reward {
                id: r.get("id"),
                tenant_id: r.get("tenant_id"),
                program_id: r.get("program_id"),
                name: r.get("name"),
                description: r.get::<Option<String>, _>("description").unwrap_or_default(),
                points_cost: r.get("points_cost"),
                reward_type: r.get("reward_type"),
                reward_value_json: r.get("reward_value_json"),
                is_active: r.get("is_active"),
                created_at: r.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: r.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
            })),
            None => Err(Status::not_found("Reward not found")),
        }
    }

    async fn list_rewards(
        &self,
        request: Request<ListRewardsRequest>,
    ) -> Result<Response<ListRewardsResponse>, Status> {
        let req = request.into_inner();

        let query_str = if req.only_active {
            "SELECT id, tenant_id, program_id, name, description, points_cost, reward_type, reward_value::text as reward_value_json, is_active, created_at, updated_at
             FROM rewards WHERE tenant_id = $1 AND program_id = $2 AND is_active = TRUE"
        } else {
            "SELECT id, tenant_id, program_id, name, description, points_cost, reward_type, reward_value::text as reward_value_json, is_active, created_at, updated_at
             FROM rewards WHERE tenant_id = $1 AND program_id = $2"
        };

        let rows = sqlx::query(query_str)
            .bind(&req.tenant_id)
            .bind(&req.program_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Status::internal(format!("Failed to list rewards: {}", e)))?;

        let rewards = rows.into_iter().map(|r| Reward {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            program_id: r.get("program_id"),
            name: r.get("name"),
            description: r.get::<Option<String>, _>("description").unwrap_or_default(),
            points_cost: r.get("points_cost"),
            reward_type: r.get("reward_type"),
            reward_value_json: r.get("reward_value_json"),
            is_active: r.get("is_active"),
            created_at: r.try_get::<chrono::NaiveDateTime, _>("created_at").map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: r.try_get::<chrono::NaiveDateTime, _>("updated_at").map(|dt| dt.to_string()).unwrap_or_default(),
        }).collect();

        Ok(Response::new(ListRewardsResponse { rewards }))
    }

    async fn redeem_reward(
        &self,
        request: Request<RedeemRewardRequest>,
    ) -> Result<Response<RedeemRewardResponse>, Status> {
        let req = request.into_inner();
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Lock account for update
        let account = sqlx::query(
            "SELECT points_balance, punches_count FROM customer_loyalty_accounts
             WHERE id = $1 AND tenant_id = $2 FOR UPDATE",
        )
        .bind(&req.account_id)
        .bind(&req.tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch account: {}", e)))?;

        let account_row = match account {
            Some(a) => a,
            None => return Err(Status::not_found("Account not found")),
        };
        let current_points: i32 = account_row.get("points_balance");

        // Fetch reward cost
        let reward = sqlx::query(
            "SELECT points_cost, name FROM rewards WHERE id = $1 AND tenant_id = $2 AND is_active = TRUE",
        )
        .bind(&req.reward_id)
        .bind(&req.tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch reward: {}", e)))?;

        let reward_row = match reward {
            Some(r) => r,
            None => return Ok(Response::new(RedeemRewardResponse {
                success: false,
                transaction_id: "".to_string(),
                error_message: "Reward not found or inactive".to_string(),
            })),
        };
        let cost: i32 = reward_row.get("points_cost");
        let reward_name: String = reward_row.get("name");

        if current_points < cost {
            return Ok(Response::new(RedeemRewardResponse {
                success: false,
                transaction_id: "".to_string(),
                error_message: "Insufficient points".to_string(),
            }));
        }

        // Deduct points
        sqlx::query(
            "UPDATE customer_loyalty_accounts
             SET points_balance = points_balance - $1, updated_at = CURRENT_TIMESTAMP
             WHERE id = $2 AND tenant_id = $3",
        )
        .bind(cost)
        .bind(&req.account_id)
        .bind(&req.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to deduct points: {}", e)))?;

        // Log transaction
        let tx_id = generate_id("lt");
        sqlx::query(
            "INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, punches, reason)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&req.account_id)
        .bind("redeem")
        .bind(-cost)
        .bind(0)
        .bind(format!("Redeemed reward: {}", reward_name))
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to log transaction: {}", e)))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RedeemRewardResponse {
            success: true,
            transaction_id: tx_id,
            error_message: "".to_string(),
        }))
    }
}
