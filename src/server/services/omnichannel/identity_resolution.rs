use std::sync::Arc;
use sqlx::{Row, Pool, Postgres};

pub struct IdentityResolutionEngine {
    pub pool: Arc<Pool<Postgres>>,
}

impl IdentityResolutionEngine {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn resolve_identity(&self, tenant_id: &str, sender_id: &str, source: &str) -> Result<Option<String>, sqlx::Error> {
        let query = if source == "email" || source == "email_inquiry" {
            "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
        } else if source == "whatsapp" || source == "sms" {
            "SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2 LIMIT 1"
        } else {
            // Check customer360 for social handles or other preferences if needed
            "SELECT customer_id as id FROM customer360 WHERE tenant_id = $1 AND (email = $2 OR phone = $2 OR preferences->>'handle' = $2) LIMIT 1"
        };

        let result = sqlx::query(query)
            .bind(tenant_id)
            .bind(sender_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(result.map(|row| row.get("id")))
    }
}
