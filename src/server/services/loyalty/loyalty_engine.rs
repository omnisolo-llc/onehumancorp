use sqlx::{PgPool, Error};
use crate::services::loyalty::data_models::LoyaltyState;

pub struct LoyaltyEngine {
    pool: PgPool,
}

impl LoyaltyEngine {
    pub fn new(pool: PgPool) -> Self {
        LoyaltyEngine { pool }
    }

    pub async fn process_checkout_event(&self, tenant_id: &str, customer_id: &str) -> Result<(), Error> {
        // Increment purchase frequency and update last purchase date
        let query = r#"
            INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, tier_name, last_updated)
            VALUES (gen_random_uuid()::text, $1, $2, 10, 'Bronze', CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE SET
                points_balance = loyalty_ledger.points_balance + 10,
                last_updated = CURRENT_TIMESTAMP
        "#;

        sqlx::query(query)
            .bind(tenant_id)
            .bind(customer_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn scan_at_risk_customers(&self, tenant_id: &str) -> Result<Vec<LoyaltyState>, Error> {
        let customers = sqlx::query_as!(
            LoyaltyState,
            r#"
            SELECT customer_id, tenant_id, tier_name as tier, points_balance as points,
                   0 as purchase_frequency,
                   EXTRACT(DAY FROM CURRENT_TIMESTAMP - last_updated)::int as days_since_last_purchase
            FROM loyalty_ledger
            WHERE tenant_id = $1 AND EXTRACT(DAY FROM CURRENT_TIMESTAMP - last_updated) > 30
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(customers)
    }

    pub async fn generate_proactive_offer(&self, _tenant_id: &str, _customer_id: &str) -> Result<String, Error> {
        Ok(format!("We miss you! Here is a 10% discount: COMEBACK10"))
    }
}
