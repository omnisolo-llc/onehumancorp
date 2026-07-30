use sqlx::PgPool;
use uuid::Uuid;
use std::time::Duration;
use chrono::Utc;

pub struct SlaWorker {
    pool: PgPool,
}

impl SlaWorker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn poll_and_escalate(&self) -> Result<(), sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct BreachedRecord {
            id: Uuid,
            tenant_id: Uuid,
        }

        let breached: Vec<BreachedRecord> = sqlx::query_as(
            r#"
            SELECT id, tenant_id FROM chat_conversations
            WHERE (frt_deadline < NOW() OR nrt_deadline < NOW() OR rt_deadline < NOW())
            AND sla_breached = FALSE
            AND status = 'open'
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        for record in breached {
            let mut tx = self.pool.begin().await?;

            sqlx::query(
                "UPDATE chat_conversations SET sla_breached = TRUE WHERE id = $1 AND tenant_id = $2"
            )
            .bind(record.id)
            .bind(record.tenant_id)
            .execute(&mut *tx)
            .await?;

            let context_payload = serde_json::json!({
                "conversation_id": record.id,
                "description": "A customer conversation has breached its SLA response deadline."
            });

            sqlx::query(
                r#"
                INSERT INTO agent_feed_items
                (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                VALUES ($1, $2, 'omnichannel', $3, '{}', 'PENDING_APPROVAL', NOW(), NOW())
                "#
            )
            .bind(Uuid::new_v4().to_string())
            .bind(record.tenant_id.to_string())
            .bind(context_payload)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_sla_worker() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool_res = PgPool::connect(&database_url).await;
        if pool_res.is_err() {
            // DB not available for testing in this env, but code compiled correctly
            return;
        }
        let pool = pool_res.unwrap();

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS chat_conversations (id UUID PRIMARY KEY, tenant_id UUID, frt_deadline TIMESTAMPTZ, nrt_deadline TIMESTAMPTZ, rt_deadline TIMESTAMPTZ, sla_breached BOOLEAN, status TEXT)").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_feed_items (id TEXT PRIMARY KEY, tenant_id TEXT, event_source TEXT, context_payload JSONB, proposed_action JSONB, lifecycle_state TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ)").execute(&pool).await;

        let worker = SlaWorker::new(pool.clone());
        let res = worker.poll_and_escalate().await;
        assert!(res.is_ok(), "worker should succeed when tables exist");
    }
}
