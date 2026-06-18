use crate::domain::loyalty::LoyaltyLedger;
use sqlx::{PgPool, Row};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct LoyaltyService {
    pool: Arc<PgPool>,
}

impl LoyaltyService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn add_points(
        &self,
        tenant_id: &str,
        customer_id: &str,
        points: i32,
    ) -> Result<LoyaltyLedger, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // This simulates adding points or creating a new ledger if one doesn't exist
        let row = sqlx::query(
            r#"
            INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, last_updated)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, customer_id)
            DO UPDATE SET points_balance = loyalty_ledger.points_balance + EXCLUDED.points_balance,
                          last_updated = EXCLUDED.last_updated
            RETURNING id, tenant_id, customer_id, points_balance, tier_name, last_updated
            "#
        )
        .bind(&id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(points)
        .bind(now)
        .fetch_one(&*self.pool)
        .await?;

        Ok(LoyaltyLedger {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            points_balance: row.get("points_balance"),
            tier_name: row.get("tier_name"),
            last_updated: row.get("last_updated"),
        })
    }
}
