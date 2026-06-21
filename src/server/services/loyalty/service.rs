use async_trait::async_trait;
use loyalty_proto::ohc::loyalty::{
    loyalty_service_server::LoyaltyService, CreateProgramRequest, CreateProgramResponse,
    CustomerLoyaltyAccount, EarnPointsRequest, EarnPointsResponse, GetCustomerStatusRequest,
    GetCustomerStatusResponse, GetProgramRequest, GetProgramResponse, LoyaltyProgram,
    LoyaltyTransaction, RedeemRewardRequest, RedeemRewardResponse, UpdateProgramRequest,
    UpdateProgramResponse,
};
use sqlx::{PgPool, Row};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct LoyaltyServiceImpl {
    pool: PgPool,
    msgbus: std::sync::Arc<dyn server_lib::msgbus::Bus>,
}

impl LoyaltyServiceImpl {
    pub fn new(pool: PgPool, msgbus: std::sync::Arc<dyn server_lib::msgbus::Bus>) -> Self {
        Self { pool, msgbus }
    }
}

#[async_trait]
impl LoyaltyService for LoyaltyServiceImpl {
    async fn create_program(
        &self,
        request: Request<CreateProgramRequest>,
    ) -> Result<Response<CreateProgramResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::new_v4().to_string();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let created_at = chrono::Utc::now();

        sqlx::query(
            r#"
            INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5::jsonb, $6, $7, $8)
            "#,
        )
        .bind(&id)
        .bind(&req.tenant_id)
        .bind(&req.name)
        .bind(&req.program_type)
        .bind(&req.config)
        .bind(true)
        .bind(created_at)
        .bind(created_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert loyalty program: {}", e)))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(format!("Failed to commit tx: {}", e)))?;

        Ok(Response::new(CreateProgramResponse {
            program: Some(LoyaltyProgram {
                id,
                tenant_id: req.tenant_id,
                name: req.name,
                program_type: req.program_type,
                config: req.config,
                is_active: true,
                created_at: created_at.timestamp(),
                updated_at: created_at.timestamp(),
            }),
        }))
    }

    async fn update_program(
        &self,
        request: Request<UpdateProgramRequest>,
    ) -> Result<Response<UpdateProgramResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let updated_at = chrono::Utc::now();

        sqlx::query(
            r#"
            UPDATE loyalty_programs
            SET name = $1, program_type = $2, config = $3::jsonb, is_active = $4, updated_at = $5
            WHERE id = $6 AND tenant_id = $7
            "#,
        )
        .bind(&req.name)
        .bind(&req.program_type)
        .bind(&req.config)
        .bind(req.is_active)
        .bind(updated_at)
        .bind(&req.id)
        .bind(&req.tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update loyalty program: {}", e)))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(format!("Failed to commit tx: {}", e)))?;

        // Refetch or return expected state
        Ok(Response::new(UpdateProgramResponse {
            program: Some(LoyaltyProgram {
                id: req.id,
                tenant_id: req.tenant_id,
                name: req.name,
                program_type: req.program_type,
                config: req.config,
                is_active: req.is_active,
                created_at: 0, // In complete implementation, we'd fetch this from DB
                updated_at: updated_at.timestamp(),
            }),
        }))
    }

    async fn get_program(
        &self,
        request: Request<GetProgramRequest>,
    ) -> Result<Response<GetProgramResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, name, program_type, config, is_active, created_at, updated_at
            FROM loyalty_programs
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(&req.id)
        .bind(&req.tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::not_found(format!("Program not found: {}", e)))?;

        let program = LoyaltyProgram {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            program_type: row.get("program_type"),
            config: row.get::<serde_json::Value, _>("config").to_string(),
            is_active: row.get("is_active"),
            created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").timestamp(),
            updated_at: row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").timestamp(),
        };

        Ok(Response::new(GetProgramResponse {
            program: Some(program),
        }))
    }

    async fn earn_points(
        &self,
        request: Request<EarnPointsRequest>,
    ) -> Result<Response<EarnPointsResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let account_id = Uuid::new_v4().to_string();
        let updated_at = chrono::Utc::now();

        let row = sqlx::query(
            r#"
            INSERT INTO customer_loyalty_accounts (id, tenant_id, customer_id, program_id, points_balance, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, customer_id, program_id)
            DO UPDATE SET points_balance = customer_loyalty_accounts.points_balance + EXCLUDED.points_balance, updated_at = EXCLUDED.updated_at
            RETURNING id, points_balance, tier, created_at
            "#,
        )
        .bind(&account_id)
        .bind(&req.tenant_id)
        .bind(&req.customer_id)
        .bind(&req.program_id)
        .bind(req.points)
        .bind(updated_at)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to upsert customer loyalty account: {}", e)))?;

        let real_account_id: String = row.get("id");
        let new_balance: i32 = row.get("points_balance");
        let tier: Option<String> = row.get("tier");
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

        let tx_id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, description, order_id, created_at)
            VALUES ($1, $2, $3, 'EARN', $4, $5, $6, $7)
            "#,
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&real_account_id)
        .bind(req.points)
        .bind(&req.description)
        .bind(&req.order_id)
        .bind(updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert loyalty transaction: {}", e)))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(format!("Failed to commit tx: {}", e)))?;

        // Publish event to msgbus
        let event = server_lib::msgbus::Message {
            topic: "loyalty.points_awarded".to_string(),
            payload: serde_json::to_vec(&serde_json::json!({
                "tenant_id": req.tenant_id.clone(),
                "customer_id": req.customer_id,
                "program_id": req.program_id,
                "points": req.points,
                "new_balance": new_balance,
                "order_id": req.order_id,
                "timestamp": updated_at.timestamp()
            })).unwrap_or_default(),
        };
        let _ = self.msgbus.publish(event).await;

        Ok(Response::new(EarnPointsResponse {
            account: Some(CustomerLoyaltyAccount {
                id: real_account_id.clone(),
                tenant_id: req.tenant_id.clone(),
                customer_id: req.customer_id.clone(),
                program_id: req.program_id.clone(),
                points_balance: new_balance,
                tier: tier.unwrap_or_default(),
                created_at: created_at.timestamp(),
                updated_at: updated_at.timestamp(),
            }),
            transaction: Some(LoyaltyTransaction {
                id: tx_id,
                tenant_id: req.tenant_id,
                account_id: real_account_id,
                transaction_type: "EARN".to_string(),
                points: req.points,
                description: req.description,
                order_id: req.order_id,
                created_at: updated_at.timestamp(),
            }),
        }))
    }

    async fn redeem_reward(
        &self,
        request: Request<RedeemRewardRequest>,
    ) -> Result<Response<RedeemRewardResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let reward_row = sqlx::query(
            r#"
            SELECT points_cost, name
            FROM rewards
            WHERE id = $1 AND program_id = $2 AND tenant_id = $3
            "#
        )
        .bind(&req.reward_id)
        .bind(&req.program_id)
        .bind(&req.tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::not_found(format!("Reward not found: {}", e)))?;

        let cost: i32 = reward_row.get("points_cost");
        let reward_name: String = reward_row.get("name");

        let account_row = sqlx::query(
            r#"
            SELECT id, points_balance, tier, created_at
            FROM customer_loyalty_accounts
            WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3
            FOR UPDATE
            "#
        )
        .bind(&req.tenant_id)
        .bind(&req.customer_id)
        .bind(&req.program_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::not_found(format!("Customer account not found: {}", e)))?;

        let account_id: String = account_row.get("id");
        let current_balance: i32 = account_row.get("points_balance");
        let tier: Option<String> = account_row.get("tier");
        let created_at: chrono::DateTime<chrono::Utc> = account_row.get("created_at");

        if current_balance < cost {
            return Err(Status::failed_precondition("Insufficient points"));
        }

        let updated_at = chrono::Utc::now();

        sqlx::query(
            r#"
            UPDATE customer_loyalty_accounts
            SET points_balance = points_balance - $1, updated_at = $2
            WHERE id = $3
            "#
        )
        .bind(cost)
        .bind(updated_at)
        .bind(&account_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update balance: {}", e)))?;

        let tx_id = Uuid::new_v4().to_string();
        let description = format!("Redeemed reward: {}", reward_name);

        sqlx::query(
            r#"
            INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, points, description, created_at)
            VALUES ($1, $2, $3, 'REDEEM', $4, $5, $6)
            "#,
        )
        .bind(&tx_id)
        .bind(&req.tenant_id)
        .bind(&account_id)
        .bind(cost)
        .bind(&description)
        .bind(updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert loyalty transaction: {}", e)))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(format!("Failed to commit tx: {}", e)))?;

        Ok(Response::new(RedeemRewardResponse {
            account: Some(CustomerLoyaltyAccount {
                id: account_id.clone(),
                tenant_id: req.tenant_id.clone(),
                customer_id: req.customer_id.clone(),
                program_id: req.program_id.clone(),
                points_balance: current_balance - cost,
                tier: tier.unwrap_or_default(),
                created_at: created_at.timestamp(),
                updated_at: updated_at.timestamp(),
            }),
            transaction: Some(LoyaltyTransaction {
                id: tx_id,
                tenant_id: req.tenant_id,
                account_id,
                transaction_type: "REDEEM".to_string(),
                points: cost,
                description,
                order_id: "".to_string(),
                created_at: updated_at.timestamp(),
            }),
        }))
    }

    async fn get_customer_status(
        &self,
        request: Request<GetCustomerStatusRequest>,
    ) -> Result<Response<GetCustomerStatusResponse>, Status> {
        let req = request.into_inner();

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("Failed to begin tx: {}", e)))?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Failed to set tenant context: {}", e)))?;

        let row = sqlx::query(
            r#"
            SELECT id, points_balance, tier, created_at, updated_at
            FROM customer_loyalty_accounts
            WHERE tenant_id = $1 AND customer_id = $2 AND program_id = $3
            "#
        )
        .bind(&req.tenant_id)
        .bind(&req.customer_id)
        .bind(&req.program_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch customer account: {}", e)))?;

        match row {
            Some(r) => {
                let tier: Option<String> = r.get("tier");
                Ok(Response::new(GetCustomerStatusResponse {
                    account: Some(CustomerLoyaltyAccount {
                        id: r.get("id"),
                        tenant_id: req.tenant_id,
                        customer_id: req.customer_id,
                        program_id: req.program_id,
                        points_balance: r.get("points_balance"),
                        tier: tier.unwrap_or_default(),
                        created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").timestamp(),
                        updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").timestamp(),
                    }),
                }))
            }
            None => {
                // If not found, return empty account (0 points)
                Ok(Response::new(GetCustomerStatusResponse {
                    account: Some(CustomerLoyaltyAccount {
                        id: "".to_string(),
                        tenant_id: req.tenant_id,
                        customer_id: req.customer_id,
                        program_id: req.program_id,
                        points_balance: 0,
                        tier: "".to_string(),
                        created_at: 0,
                        updated_at: 0,
                    }),
                }))
            }
        }
    }
}
