use crate::domain::loyalty_ledger::{LoyaltyProgram, CustomerLoyaltyAccount, LoyaltyTransaction, LoyaltyReward};
use sqlx::{PgPool, Error, Row};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct LoyaltyRepo {
    pool: PgPool,
}

impl LoyaltyRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_program(&self, program: &LoyaltyProgram) -> Result<LoyaltyProgram, Error> {
        sqlx::query_as::<_, LoyaltyProgram>(
            r#"
            INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, name, program_type, config, is_active, created_at, updated_at
            "#)
            .bind(&program.id)
            .bind(&program.tenant_id)
            .bind(&program.name)
            .bind(&program.program_type)
            .bind(&program.config)
            .bind(program.is_active)
            .bind(program.created_at)
            .bind(program.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_program_by_tenant(&self, tenant_id: &str) -> Result<Option<LoyaltyProgram>, Error> {
         sqlx::query_as::<_, LoyaltyProgram>(
            r#"
            SELECT id, tenant_id, name, program_type, config, is_active, created_at, updated_at
            FROM loyalty_programs
            WHERE tenant_id = $1 AND is_active = true
            LIMIT 1
            "#)
            .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_account(&self, tenant_id: &str, program_id: &str, customer_id: &str) -> Result<Option<CustomerLoyaltyAccount>, Error> {
         sqlx::query_as::<_, CustomerLoyaltyAccount>(
            r#"
            SELECT id, tenant_id, program_id, customer_id, points_balance, punches, tier_name, created_at, updated_at
            FROM customer_loyalty_accounts
            WHERE tenant_id = $1 AND program_id = $2 AND customer_id = $3
            "#)
            .bind(tenant_id)
            .bind(program_id)
            .bind(customer_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_account(&self, account: &CustomerLoyaltyAccount) -> Result<CustomerLoyaltyAccount, Error> {
        sqlx::query_as::<_, CustomerLoyaltyAccount>(
            r#"
            INSERT INTO customer_loyalty_accounts (id, tenant_id, program_id, customer_id, points_balance, punches, tier_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, tenant_id, program_id, customer_id, points_balance, punches, tier_name, created_at, updated_at
            "#)
            .bind(&account.id)
            .bind(&account.tenant_id)
            .bind(&account.program_id)
            .bind(&account.customer_id)
            .bind(account.points_balance)
            .bind(account.punches)
            .bind(&account.tier_name)
            .bind(account.created_at)
            .bind(account.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn add_transaction(&self, tx: &LoyaltyTransaction) -> Result<LoyaltyTransaction, Error> {
        // Start a transaction to ensure both the ledger and the account balance update atomically
        let mut sqlx_tx = self.pool.begin().await?;

        let new_tx = sqlx::query_as::<_, LoyaltyTransaction>(
            r#"
            INSERT INTO loyalty_transactions (id, tenant_id, account_id, transaction_type, amount, reason, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, account_id, transaction_type, amount, reason, created_at
            "#)
            .bind(&tx.id)
            .bind(&tx.tenant_id)
            .bind(&tx.account_id)
            .bind(&tx.transaction_type)
            .bind(tx.amount)
            .bind(&tx.reason)
            .bind(tx.created_at)
        .fetch_one(&mut *sqlx_tx)
        .await?;

        // Update the account balance based on the transaction
        let amount_modifier = match tx.transaction_type.as_str() {
            "earn" => tx.amount,
            "redeem" => -tx.amount,
            "adjust" => tx.amount,
            _ => tx.amount,
        };

        sqlx::query(
            r#"
            UPDATE customer_loyalty_accounts
            SET points_balance = points_balance + $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2 AND tenant_id = $3
            "#)
            .bind(amount_modifier)
            .bind(&tx.account_id)
            .bind(&tx.tenant_id)
        .execute(&mut *sqlx_tx)
        .await?;

        sqlx_tx.commit().await?;

        Ok(new_tx)
    }

    pub async fn get_accounts_eligible_for_reward(&self, program_id: &str, points_threshold: i32) -> Result<Vec<CustomerLoyaltyAccount>, Error> {
         sqlx::query_as::<_, CustomerLoyaltyAccount>(
            r#"
            SELECT id, tenant_id, program_id, customer_id, points_balance, punches, tier_name, created_at, updated_at
            FROM customer_loyalty_accounts
            WHERE program_id = $1 AND points_balance >= $2
            "#)
            .bind(program_id)
            .bind(points_threshold)
        .fetch_all(&self.pool)
        .await
    }
}
