use crate::domain::interaction_timeline::InteractionTimeline;
use sqlx::{PgPool, Row};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct InteractionTimelineService {
    pool: Arc<PgPool>,
}

impl InteractionTimelineService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn add_interaction(
        &self,
        tenant_id: &str,
        customer_id: &str,
        source: &str,
        sentiment: &str,
    ) -> Result<InteractionTimeline, sqlx::Error> {
        let id = Uuid::new_v4();
        let occurred_at = Utc::now();

        let row = sqlx::query(
            r#"
            INSERT INTO interaction_timeline (id, tenant_id, customer_id, source, sentiment, occurred_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, customer_id, source, sentiment, occurred_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(source)
        .bind(sentiment)
        .bind(occurred_at)
        .fetch_one(&*self.pool)
        .await?;

        Ok(InteractionTimeline {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            source: row.get("source"),
            sentiment: row.get("sentiment"),
            occurred_at: row.get("occurred_at"),
        })
    }
}
