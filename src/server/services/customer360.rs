use crate::domain::customer360::Customer360;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

pub struct Customer360Service {
    pool: Arc<PgPool>,
}

impl Customer360Service {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_or_create(
        &self,
        tenant_id: &str,
        customer_id: &str,
    ) -> Result<Customer360, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO customer360 (id, tenant_id, customer_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (tenant_id, customer_id) DO UPDATE SET updated_at = CURRENT_TIMESTAMP
            RETURNING id, tenant_id, customer_id, email, phone, mood, preferences, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4().to_string())
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(Customer360 {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            email: row.get("email"),
            phone: row.get("phone"),
            mood: row.get("mood"),
            preferences: row.get("preferences"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
