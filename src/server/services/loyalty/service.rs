use crate::domain::loyalty::{LoyaltyProgram, CustomerLoyaltyAccount, LoyaltyTransaction};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct LoyaltyService {
    pool: Pool<Postgres>,
}

impl LoyaltyService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    async fn execute_with_tenant<'a, T>(
        &self,
        tenant_id: &str,
        mut f: impl for<'b> FnMut(&'b mut sqlx::Transaction<'a, Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, sqlx::Error>> + Send + 'b>>
    ) -> Result<T, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let q = format!("SET LOCAL app.current_tenant = '{}'", tenant_id);
        sqlx::query(&q).execute(&mut *tx).await?;

        let result = f(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn create_program(&self, tenant_id: &str, program: LoyaltyProgram) -> Result<LoyaltyProgram, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let config_json = serde_json::to_value(&program.config).unwrap_or(serde_json::json!({}));
        let tid = tenant_id.to_string();
        let name = program.name.clone();
        let prog_type = program.program_type.clone();
        let is_act = program.is_active;
        let id_clone = id.clone();

        self.execute_with_tenant(tenant_id, move |tx| {
            let id = id_clone.clone();
            let tid = tid.clone();
            let name = name.clone();
            let prog_type = prog_type.clone();
            let config_json = config_json.clone();
            Box::pin(async move {
                sqlx::query(
                    r#"
                    INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(id)
                .bind(tid)
                .bind(name)
                .bind(prog_type)
                .bind(config_json)
                .bind(is_act)
                .execute(&mut **tx)
                .await?;
                Ok(())
            })
        }).await?;

        self.get_program(tenant_id, &id).await?.ok_or_else(|| sqlx::Error::RowNotFound)
    }

    pub async fn get_program(&self, tenant_id: &str, id: &str) -> Result<Option<LoyaltyProgram>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let q = format!("SET LOCAL app.current_tenant = '{}'", tenant_id);
        sqlx::query(&q).execute(&mut *tx).await?;

        let record = sqlx::query(
            r#"
            SELECT id, tenant_id, name, program_type, config, is_active, created_at, updated_at
            FROM loyalty_programs
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(record.map(|r| LoyaltyProgram {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            name: r.get("name"),
            program_type: r.get("program_type"),
            config: r.get("config"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn list_programs(&self, tenant_id: &str) -> Result<Vec<LoyaltyProgram>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let q = format!("SET LOCAL app.current_tenant = '{}'", tenant_id);
        sqlx::query(&q).execute(&mut *tx).await?;

        let records = sqlx::query(
            r#"
            SELECT id, tenant_id, name, program_type, config, is_active, created_at, updated_at
            FROM loyalty_programs
            WHERE tenant_id = $1
            "#
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(records.into_iter().map(|r| LoyaltyProgram {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            name: r.get("name"),
            program_type: r.get("program_type"),
            config: r.get("config"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn get_or_create_account(&self, tenant_id: &str, program_id: &str, customer_id: &str) -> Result<CustomerLoyaltyAccount, sqlx::Error> {
        if let Some(account) = self.get_account(tenant_id, program_id, customer_id).await? {
            return Ok(account);
        }

        let id = Uuid::new_v4().to_string();
        let tid = tenant_id.to_string();
        let pid = program_id.to_string();
        let cid = customer_id.to_string();

        self.execute_with_tenant(tenant_id, move |tx| {
            let id = id.clone();
            let tid = tid.clone();
            let pid = pid.clone();
            let cid = cid.clone();
            Box::pin(async move {
                sqlx::query(
                    r#"
                    INSERT INTO customer_loyalty_accounts (id, tenant_id, program_id, customer_id)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (tenant_id, program_id, customer_id) DO NOTHING
                    "#
                )
                .bind(id)
                .bind(tid)
                .bind(pid)
                .bind(cid)
                .execute(&mut **tx)
                .await?;
                Ok(())
            })
        }).await?;

        self.get_account(tenant_id, program_id, customer_id).await?.ok_or_else(|| sqlx::Error::RowNotFound)
    }

    pub async fn get_account(&self, tenant_id: &str, program_id: &str, customer_id: &str) -> Result<Option<CustomerLoyaltyAccount>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let q = format!("SET LOCAL app.current_tenant = '{}'", tenant_id);
        sqlx::query(&q).execute(&mut *tx).await?;

        let record = sqlx::query(
            r#"
            SELECT id, tenant_id, program_id, customer_id, points_balance, punches, current_tier, created_at, updated_at
            FROM customer_loyalty_accounts
            WHERE tenant_id = $1 AND program_id = $2 AND customer_id = $3
            "#
        )
        .bind(tenant_id)
        .bind(program_id)
        .bind(customer_id)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(record.map(|r| CustomerLoyaltyAccount {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            program_id: r.get("program_id"),
            customer_id: r.get("customer_id"),
            points_balance: r.get("points_balance"),
            punches: r.get("punches"),
            current_tier: r.get("current_tier"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn earn_points(&self, tenant_id: &str, program_id: &str, customer_id: &str, amount: i32, reason: Option<String>, order_id: Option<String>) -> Result<LoyaltyTransaction, sqlx::Error> {
        let account = self.get_or_create_account(tenant_id, program_id, customer_id).await?;
        let program = self.get_program(tenant_id, program_id).await?.unwrap();

        let tx_id = Uuid::new_v4().to_string();
        let tid = tenant_id.to_string();
        let acc_id = account.id.clone();
        let tx_id_clone = tx_id.clone();

        self.execute_with_tenant(tenant_id, move |tx| {
            let tx_id = tx_id_clone.clone();
            let tid = tid.clone();
            let acc_id = acc_id.clone();
            let reason = reason.clone();
            let order_id = order_id.clone();
            let prog_type = program.program_type.clone();

            Box::pin(async move {
                sqlx::query(
                    r#"
                    INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, amount, reason, order_id)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(tx_id)
                .bind(tid)
                .bind(acc_id.clone())
                .bind("earn")
                .bind(amount)
                .bind(reason)
                .bind(order_id)
                .execute(&mut **tx)
                .await?;

                if prog_type == "punch_card" {
                     sqlx::query(
                        r#"
                        UPDATE customer_loyalty_accounts
                        SET punches = punches + $1, updated_at = CURRENT_TIMESTAMP
                        WHERE id = $2
                        "#
                    )
                    .bind(amount)
                    .bind(acc_id.clone())
                    .execute(&mut **tx)
                    .await?;
                } else {
                    sqlx::query(
                        r#"
                        UPDATE customer_loyalty_accounts
                        SET points_balance = points_balance + $1, updated_at = CURRENT_TIMESTAMP
                        WHERE id = $2
                        "#
                    )
                    .bind(amount)
                    .bind(acc_id.clone())
                    .execute(&mut **tx)
                    .await?;
                }

                Ok(())
            })
        }).await?;

        let mut tx = self.pool.begin().await?;
        let q = format!("SET LOCAL app.current_tenant = '{}'", tenant_id);
        sqlx::query(&q).execute(&mut *tx).await?;

        let record = sqlx::query(
            r#"
            SELECT id, tenant_id, account_id, transaction_type, amount, reason, order_id, created_at
            FROM loyalty_transactions
            WHERE id = $1
            "#
        )
        .bind(&tx_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(LoyaltyTransaction {
            id: record.get("id"),
            tenant_id: record.get("tenant_id"),
            account_id: record.get("account_id"),
            transaction_type: record.get("transaction_type"),
            amount: record.get("amount"),
            reason: record.get("reason"),
            order_id: record.get("order_id"),
            created_at: record.get("created_at"),
        })
    }

    pub async fn redeem_reward(&self, tenant_id: &str, program_id: &str, customer_id: &str, reward_id: &str, order_id: Option<String>) -> Result<LoyaltyTransaction, sqlx::Error> {
        let account = self.get_or_create_account(tenant_id, program_id, customer_id).await?;
        let program = self.get_program(tenant_id, program_id).await?.unwrap();

        let mut tx = self.pool.begin().await?;
        let q = format!("SET LOCAL app.current_tenant = '{}'", tenant_id);
        sqlx::query(&q).execute(&mut *tx).await?;

        let reward = sqlx::query(
            r#"
            SELECT cost_in_points, cost_in_punches
            FROM rewards
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(reward_id)
        .fetch_one(&mut *tx).await?;

        let cost_in_points: Option<i32> = reward.get("cost_in_points");
        let cost_in_punches: Option<i32> = reward.get("cost_in_punches");

        let amount_to_deduct = cost_in_points.unwrap_or(cost_in_punches.unwrap_or(0));
        let tx_id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, amount, reason, order_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&tx_id)
        .bind(tenant_id)
        .bind(&account.id)
        .bind("redeem")
        .bind(amount_to_deduct)
        .bind(format!("Redeemed reward {}", reward_id))
        .bind(order_id)
        .execute(&mut *tx)
        .await?;

        if program.program_type == "punch_card" {
             sqlx::query(
                r#"
                UPDATE customer_loyalty_accounts
                SET punches = punches - $1, updated_at = CURRENT_TIMESTAMP
                WHERE id = $2
                "#
            )
            .bind(amount_to_deduct)
            .bind(&account.id)
            .execute(&mut *tx)
            .await?;
        } else {
            sqlx::query(
                r#"
                UPDATE customer_loyalty_accounts
                SET points_balance = points_balance - $1, updated_at = CURRENT_TIMESTAMP
                WHERE id = $2
                "#
            )
            .bind(amount_to_deduct)
            .bind(&account.id)
            .execute(&mut *tx)
            .await?;
        }

        let record = sqlx::query(
            r#"
            SELECT id, tenant_id, account_id, transaction_type, amount, reason, order_id, created_at
            FROM loyalty_transactions
            WHERE id = $1
            "#
        )
        .bind(&tx_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(LoyaltyTransaction {
            id: record.get("id"),
            tenant_id: record.get("tenant_id"),
            account_id: record.get("account_id"),
            transaction_type: record.get("transaction_type"),
            amount: record.get("amount"),
            reason: record.get("reason"),
            order_id: record.get("order_id"),
            created_at: record.get("created_at"),
        })
    }
}
