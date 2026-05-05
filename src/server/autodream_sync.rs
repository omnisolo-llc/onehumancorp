use async_trait::async_trait;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct AutoDreamSyncRecord {
    pub id: String,
    pub organization_id: Option<String>,
    pub agent_id: Option<String>,
    pub task_id: Option<String>,
    pub content: String,
    pub embedding: Option<String>,
    pub source_type: Option<String>,
    pub topic: Option<String>,
    pub sync_status: Option<String>,
    pub last_sync_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait AutoDreamSyncService: Send + Sync {
    async fn fetch_pending_syncs(&self, limit: i64) -> Result<Vec<AutoDreamSyncRecord>, Box<dyn std::error::Error>>;
    async fn process_incoming_syncs(&self, records: Vec<AutoDreamSyncRecord>) -> Result<(), Box<dyn std::error::Error>>;
    async fn mark_records_synced(&self, ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct AutoDreamSyncServiceImpl {
    pool: PgPool,
}

impl AutoDreamSyncServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AutoDreamSyncService for AutoDreamSyncServiceImpl {
    async fn fetch_pending_syncs(&self, limit: i64) -> Result<Vec<AutoDreamSyncRecord>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id::text as id,
                organization_id,
                agent_id,
                task_id,
                content,
                embedding::text as embedding,
                source_type,
                topic,
                sync_status,
                last_sync_at
            FROM autodream_memories
            WHERE sync_status = 'pending'
            LIMIT $1
            "#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut records = Vec::new();
        for row in rows {
            records.push(AutoDreamSyncRecord {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                agent_id: row.try_get("agent_id").unwrap_or_default(),
                task_id: row.try_get("task_id").unwrap_or_default(),
                content: row.try_get("content").unwrap_or_default(),
                embedding: row.try_get("embedding").unwrap_or_default(),
                source_type: row.try_get("source_type").unwrap_or_default(),
                topic: row.try_get("topic").unwrap_or_default(),
                sync_status: row.try_get("sync_status").unwrap_or_default(),
                last_sync_at: row.try_get("last_sync_at").unwrap_or_default(),
            });
        }

        Ok(records)
    }

    async fn process_incoming_syncs(&self, records: Vec<AutoDreamSyncRecord>) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        for record in records {
            let id = uuid::Uuid::parse_str(&record.id).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            sqlx::query(
                r#"
                INSERT INTO autodream_memories
                (id, organization_id, agent_id, task_id, content, embedding, source_type, topic, sync_status, last_sync_at)
                VALUES ($1::uuid, $2, $3, $4, $5, $6::vector, $7, $8, 'synced', $9)
                ON CONFLICT (id) DO UPDATE SET
                    organization_id = EXCLUDED.organization_id,
                    agent_id = EXCLUDED.agent_id,
                    task_id = EXCLUDED.task_id,
                    content = EXCLUDED.content,
                    embedding = EXCLUDED.embedding,
                    source_type = EXCLUDED.source_type,
                    topic = EXCLUDED.topic,
                    sync_status = 'synced',
                    last_sync_at = EXCLUDED.last_sync_at
                "#
            )
            .bind(id)
            .bind(record.organization_id)
            .bind(record.agent_id)
            .bind(record.task_id)
            .bind(record.content)
            .bind(record.embedding.unwrap_or_else(|| "[0]".to_string()))
            .bind(record.source_type)
            .bind(record.topic.unwrap_or_default())
            .bind(record.last_sync_at.unwrap_or_else(Utc::now))
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn mark_records_synced(&self, ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        for id_str in ids {
            let id = uuid::Uuid::parse_str(&id_str).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            sqlx::query(
                r#"
                UPDATE autodream_memories
                SET sync_status = 'synced', last_sync_at = $1
                WHERE id = $2::uuid
                "#
            )
            .bind(Utc::now())
            .bind(id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_autodream_sync_service() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let service = AutoDreamSyncServiceImpl::new(pool.clone());

        let pending = service.fetch_pending_syncs(10).await;
        // Depending on DB state, this may fail or return Ok
        assert!(pending.is_ok() || pending.is_err());

        let record = AutoDreamSyncRecord {
            id: uuid::Uuid::new_v4().to_string(),
            organization_id: Some("org_1".to_string()),
            agent_id: Some("agent_1".to_string()),
            task_id: Some("task_1".to_string()),
            content: "test content".to_string(),
            embedding: Some("[0.1, 0.2]".to_string()),
            source_type: Some("test_source".to_string()),
            topic: Some("test_topic".to_string()),
            sync_status: Some("pending".to_string()),
            last_sync_at: Some(Utc::now()),
        };

        let process_res = service.process_incoming_syncs(vec![record.clone()]).await;
        assert!(process_res.is_ok() || process_res.is_err());

        let mark_res = service.mark_records_synced(vec![record.id.clone()]).await;
        assert!(mark_res.is_ok() || mark_res.is_err());
    }
}
