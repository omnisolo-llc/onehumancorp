use sqlx::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonedCart {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub total_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryMessage {
    pub id: String,
    pub tenant_id: String,
    pub cart_id: String,
    pub customer_id: String,
    pub drafted_message: String,
    pub discount_amount: i64,
    pub status: String,
}

pub struct AbandonedCartService;

impl AbandonedCartService {
    pub async fn detect_abandoned_carts(tenant_id: &str, timeout_minutes: i64) -> Result<Vec<AbandonedCart>, String> {
        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let timeout_time = Utc::now() - chrono::Duration::minutes(timeout_minutes);

        let rows = sqlx::query(
            "SELECT id, tenant_id, customer_id, created_at, status, total_amount as total_value
             FROM orders
             WHERE status = 'pending' AND updated_at < $1"
        )
        .bind(timeout_time)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let carts = rows.into_iter().map(|row| AbandonedCart {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            created_at: row.get("created_at"),
            status: row.get("status"),
            total_value: row.try_get("total_value").unwrap_or(0),
        }).collect();

        Ok(carts)
    }

    pub async fn generate_recovery_message(cart: &AbandonedCart) -> Result<RecoveryMessage, String> {
        // AI Promoter agent logic
        let discount = cart.total_value / 10;
        let drafted_message = format!(
            "Hi there! We noticed you left some items in your cart. Here is a 10% discount ({}) to complete your purchase!",
            discount
        );

        let message = RecoveryMessage {
            id: Uuid::new_v4().to_string(),
            tenant_id: cart.tenant_id.clone(),
            cart_id: cart.id.clone(),
            customer_id: cart.customer_id.clone(),
            drafted_message,
            discount_amount: discount,
            status: "drafted".to_string(),
        };

        Ok(message)
    }

    pub async fn queue_recovery_message(message: RecoveryMessage) -> Result<(), String> {
        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &message.tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO agent_actions (id, tenant_id, agent_id, interaction_id, action_type, payload)
             VALUES ($1, $2, 'Promoter', NULL, 'draft_recovery', $3)"
        )
        .bind(&message.id)
        .bind(&message.tenant_id)
        .bind(serde_json::to_value(&message).unwrap())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub async fn setup_marketing_agent_actions_table(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS agent_actions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            interaction_id TEXT,
            action_type TEXT NOT NULL,
            payload JSONB NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_recovery_message() {
        let cart = AbandonedCart {
            id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
            customer_id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            status: "pending".to_string(),
            total_value: 5000,
        };

        let msg = AbandonedCartService::generate_recovery_message(&cart).await.unwrap();
        assert_eq!(msg.discount_amount, 500);
        assert!(msg.drafted_message.contains("10% discount"));
    }
}
