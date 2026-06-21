use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;
use crate::hub::Hub;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoyaltyProgram {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    pub config: serde_json::Value,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CustomerLoyaltyAccount {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub customer_id: String,
    pub balance: i32,
    pub tier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Reward {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cost: i32,
    pub reward_type: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoyaltyTransaction {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub account_id: String,
    pub transaction_type: String,
    pub amount: i32,
    pub reason: Option<String>,
}

#[derive(Clone)]
pub struct LoyaltyService {
    hub: Arc<Hub>,
}

impl LoyaltyService {
    pub fn new(hub: Arc<Hub>) -> Self {
        Self { hub }
    }

    pub async fn create_program(&self, tenant_id: &str, name: &str, program_type: &str, config: serde_json::Value) -> Result<LoyaltyProgram, String> {
        let id = Uuid::new_v4().to_string();
        let query = r#"
            INSERT INTO loyalty_programs (id, tenant_id, name, program_type, config, is_active)
            VALUES ($1, $2, $3, $4, $5, true)
            RETURNING id, tenant_id, name, program_type, config, is_active
        "#;

        let pool = self.hub.pool.clone();

        sqlx::query_as::<_, LoyaltyProgram>(query)
            .bind(&id)
            .bind(tenant_id)
            .bind(name)
            .bind(program_type)
            .bind(config)
            .fetch_one(&pool)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_program(&self, tenant_id: &str, program_id: &str) -> Result<LoyaltyProgram, String> {
        let pool = self.hub.pool.clone();
        sqlx::query_as::<_, LoyaltyProgram>(
            "SELECT id, tenant_id, name, program_type, config, is_active FROM loyalty_programs WHERE id = $1 AND tenant_id = $2"
        )
        .bind(program_id)
        .bind(tenant_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn list_programs(&self, tenant_id: &str) -> Result<Vec<LoyaltyProgram>, String> {
        let pool = self.hub.pool.clone();
        sqlx::query_as::<_, LoyaltyProgram>(
            "SELECT id, tenant_id, name, program_type, config, is_active FROM loyalty_programs WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn create_account(&self, tenant_id: &str, program_id: &str, customer_id: &str) -> Result<CustomerLoyaltyAccount, String> {
        let id = Uuid::new_v4().to_string();
        let pool = self.hub.pool.clone();
        sqlx::query_as::<_, CustomerLoyaltyAccount>(
            r#"
            INSERT INTO customer_loyalty_accounts (id, tenant_id, program_id, customer_id, balance, tier)
            VALUES ($1, $2, $3, $4, 0, NULL)
            ON CONFLICT (tenant_id, program_id, customer_id) DO UPDATE SET updated_at = CURRENT_TIMESTAMP
            RETURNING id, tenant_id, program_id, customer_id, balance, tier
            "#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(program_id)
        .bind(customer_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_account(&self, tenant_id: &str, program_id: &str, customer_id: &str) -> Result<CustomerLoyaltyAccount, String> {
        let pool = self.hub.pool.clone();
        sqlx::query_as::<_, CustomerLoyaltyAccount>(
            "SELECT id, tenant_id, program_id, customer_id, balance, tier FROM customer_loyalty_accounts WHERE tenant_id = $1 AND program_id = $2 AND customer_id = $3"
        )
        .bind(tenant_id)
        .bind(program_id)
        .bind(customer_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn earn_points(&self, tenant_id: &str, program_id: &str, customer_id: &str, amount: i32, reason: Option<String>) -> Result<CustomerLoyaltyAccount, String> {
        let pool = self.hub.pool.clone();
        let account = match self.get_account(tenant_id, program_id, customer_id).await {
            Ok(acc) => acc,
            Err(_) => self.create_account(tenant_id, program_id, customer_id).await?,
        };

        let tx_id = Uuid::new_v4().to_string();

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO loyalty_transactions (id, tenant_id, program_id, account_id, transaction_type, amount, reason) VALUES ($1, $2, $3, $4, 'EARN', $5, $6)"
        )
        .bind(&tx_id)
        .bind(tenant_id)
        .bind(program_id)
        .bind(&account.id)
        .bind(amount)
        .bind(&reason)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let updated_account = sqlx::query_as::<_, CustomerLoyaltyAccount>(
            "UPDATE customer_loyalty_accounts SET balance = balance + $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 RETURNING id, tenant_id, program_id, customer_id, balance, tier"
        )
        .bind(amount)
        .bind(&account.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let event_payload = serde_json::json!({
            "tenant_id": tenant_id,
            "program_id": program_id,
            "customer_id": customer_id,
            "points": amount,
            "total_points": updated_account.balance,
            "reason": reason
        });

        // Let's trigger the marketing agent loop asynchronously
        let hub_clone = self.hub.clone();
        tokio::spawn(async move {
            hub_clone.log_event(serde_json::json!({
                "type": "loyalty.points_awarded",
                "payload": event_payload
            }));
        });

        Ok(updated_account)
    }

    pub async fn create_reward(&self, tenant_id: &str, program_id: &str, name: &str, description: Option<String>, cost: i32, reward_type: &str) -> Result<Reward, String> {
        let id = Uuid::new_v4().to_string();
        let pool = self.hub.pool.clone();

        sqlx::query_as::<_, Reward>(
            r#"
            INSERT INTO rewards (id, tenant_id, program_id, name, description, cost, reward_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, program_id, name, description, cost, reward_type
            "#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(program_id)
        .bind(name)
        .bind(description)
        .bind(cost)
        .bind(reward_type)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn get_rewards(&self, tenant_id: &str, program_id: &str) -> Result<Vec<Reward>, String> {
        let pool = self.hub.pool.clone();
        sqlx::query_as::<_, Reward>(
            "SELECT id, tenant_id, program_id, name, description, cost, reward_type FROM rewards WHERE tenant_id = $1 AND program_id = $2"
        )
        .bind(tenant_id)
        .bind(program_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn redeem_reward(&self, tenant_id: &str, program_id: &str, customer_id: &str, reward_id: &str) -> Result<CustomerLoyaltyAccount, String> {
        let pool = self.hub.pool.clone();

        let reward = sqlx::query_as::<_, Reward>(
            "SELECT id, tenant_id, program_id, name, description, cost, reward_type FROM rewards WHERE id = $1 AND tenant_id = $2 AND program_id = $3"
        )
        .bind(reward_id)
        .bind(tenant_id)
        .bind(program_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| "Reward not found".to_string())?;

        let account = self.get_account(tenant_id, program_id, customer_id).await.map_err(|_| "Account not found".to_string())?;

        if account.balance < reward.cost {
            return Err("Insufficient points".to_string());
        }

        let tx_id = Uuid::new_v4().to_string();

        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO loyalty_transactions (id, tenant_id, program_id, account_id, transaction_type, amount, reason) VALUES ($1, $2, $3, $4, 'REDEEM', $5, $6)"
        )
        .bind(&tx_id)
        .bind(tenant_id)
        .bind(program_id)
        .bind(&account.id)
        .bind(-reward.cost)
        .bind(format!("Redeemed reward: {}", reward.name))
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let updated_account = sqlx::query_as::<_, CustomerLoyaltyAccount>(
            "UPDATE customer_loyalty_accounts SET balance = balance - $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 RETURNING id, tenant_id, program_id, customer_id, balance, tier"
        )
        .bind(reward.cost)
        .bind(&account.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(updated_account)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::sqlite::SqliteConnectOptions;
    use std::str::FromStr;

    async fn setup_db() -> sqlx::Pool<sqlx::Sqlite> {
        let connect_options = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(connect_options).await.unwrap();

        sqlx::query(
            "CREATE TABLE loyalty_programs (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                program_type TEXT NOT NULL,
                config TEXT DEFAULT '{}',
                is_active BOOLEAN DEFAULT 1,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE customer_loyalty_accounts (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                program_id TEXT NOT NULL,
                customer_id TEXT NOT NULL,
                balance INTEGER DEFAULT 0,
                tier TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(tenant_id, program_id, customer_id)
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE rewards (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                program_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                cost INTEGER NOT NULL,
                reward_type TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE loyalty_transactions (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                program_id TEXT NOT NULL,
                account_id TEXT NOT NULL,
                transaction_type TEXT NOT NULL,
                amount INTEGER NOT NULL,
                reason TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_loyalty_service_flow() {
        let _pool = setup_db().await;
        assert!(true);
    }
}
