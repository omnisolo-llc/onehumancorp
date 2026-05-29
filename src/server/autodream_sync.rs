use async_trait::async_trait;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

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
    pub synced_to_cloud: Option<bool>,
}

#[async_trait]
pub trait AutoDreamSyncService: Send + Sync {
    async fn fetch_pending_syncs(&self, limit: i64) -> Result<Vec<AutoDreamSyncRecord>, Box<dyn std::error::Error>>;
    async fn process_incoming_syncs(&self, records: Vec<AutoDreamSyncRecord>) -> Result<(), Box<dyn std::error::Error>>;
    async fn mark_records_synced(&self, ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn process_forecast_tick(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct AutoDreamSyncServiceImpl {
    db: Arc<DB>,
}

impl AutoDreamSyncServiceImpl {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AutoDreamSyncService for AutoDreamSyncServiceImpl {
    async fn fetch_pending_syncs(&self, limit: i64) -> Result<Vec<AutoDreamSyncRecord>, Box<dyn std::error::Error>> {
        let (query, is_sqlite) = match &self.db.store {
            crate::db::DbStore::Sqlite(_) => {
                (r#"
                SELECT
                    id,
                    tenant_id as organization_id,
                    agent_id,
                    task_id,
                    content,
                    embedding,
                    source_type,
                    topic,
                    _sync_status as sync_status,
                    last_sync_at,
                    synced_to_cloud
                FROM autodream_memories
                WHERE (_sync_status = 'pending' OR synced_to_cloud = 0)
                LIMIT $1
                "#, true)
            },
            crate::db::DbStore::Postgres => {
                (r#"
                SELECT
                    id::text as id,
                    tenant_id as organization_id,
                    agent_id,
                    task_id,
                    content,
                    embedding::text as embedding,
                    source_type,
                    topic,
                    _sync_status as sync_status,
                    last_sync_at,
                    synced_to_cloud
                FROM autodream_memories
                WHERE (_sync_status = 'pending' OR synced_to_cloud = false)
                LIMIT $1
                "#, false)
            }
        };

        let rows = match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(query).bind(limit).fetch_all(pool).await?
            },
            crate::db::DbStore::Postgres => {
                sqlx::query(query).bind(limit).fetch_all(&self.db.pool).await?
            }
        };

        let mut records = Vec::new();
        for row in rows {
            let embedding: Option<String> = if is_sqlite {
                row.try_get::<Vec<u8>, _>("embedding").ok().map(|b| String::from_utf8_lossy(&b).to_string())
            } else {
                row.try_get("embedding").unwrap_or_default()
            };

            records.push(AutoDreamSyncRecord {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                agent_id: row.try_get("agent_id").unwrap_or_default(),
                task_id: row.try_get("task_id").unwrap_or_default(),
                content: row.try_get("content").unwrap_or_default(),
                embedding,
                source_type: row.try_get("source_type").unwrap_or_default(),
                topic: row.try_get("topic").unwrap_or_default(),
                sync_status: row.try_get("sync_status").unwrap_or_default(),
                last_sync_at: row.try_get("last_sync_at").unwrap_or_default(),
                synced_to_cloud: row.try_get("synced_to_cloud").unwrap_or_default(),
            });
        }

        Ok(records)
    }

    async fn process_incoming_syncs(&self, records: Vec<AutoDreamSyncRecord>) -> Result<(), Box<dyn std::error::Error>> {
        match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await?;

                for record in records {
                    sqlx::query(
                        r#"
                        INSERT INTO autodream_memories
                        (id, tenant_id, agent_id, task_id, content, embedding, source_type, topic, _sync_status, last_sync_at, synced_to_cloud)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'synced', ?, 1)
                        ON CONFLICT (id) DO UPDATE SET
                            tenant_id = EXCLUDED.tenant_id,
                            agent_id = EXCLUDED.agent_id,
                            task_id = EXCLUDED.task_id,
                            content = EXCLUDED.content,
                            embedding = EXCLUDED.embedding,
                            source_type = EXCLUDED.source_type,
                            topic = EXCLUDED.topic,
                            _sync_status = 'synced',
                            last_sync_at = EXCLUDED.last_sync_at,
                            synced_to_cloud = 1
                        "#
                    )
                    .bind(&record.id)
                    .bind(&record.organization_id)
                    .bind(&record.agent_id)
                    .bind(&record.task_id)
                    .bind(&record.content)
                    .bind(record.embedding.clone().unwrap_or_else(|| "[0]".to_string()).as_bytes())
                    .bind(&record.source_type)
                    .bind(record.topic.clone().unwrap_or_default())
                    .bind(record.last_sync_at.unwrap_or_else(Utc::now))
                    .execute(&mut *tx)
                    .await?;
                }
                tx.commit().await?;
            },
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

                for record in records {
                    let id = uuid::Uuid::parse_str(&record.id).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                    sqlx::query(
                        r#"
                        INSERT INTO autodream_memories
                        (id, organization_id, agent_id, task_id, content, embedding, source_type, topic, sync_status, last_sync_at, synced_to_cloud)
                        VALUES ($1::uuid, $2, $3, $4, $5, $6::vector, $7, $8, 'synced', $9, true)
                        ON CONFLICT (id) DO UPDATE SET
                            organization_id = EXCLUDED.organization_id,
                            agent_id = EXCLUDED.agent_id,
                            task_id = EXCLUDED.task_id,
                            content = EXCLUDED.content,
                            embedding = EXCLUDED.embedding,
                            source_type = EXCLUDED.source_type,
                            topic = EXCLUDED.topic,
                            sync_status = 'synced',
                            last_sync_at = EXCLUDED.last_sync_at,
                            synced_to_cloud = true
                        "#
                    )
                    .bind(id.to_string())
                    .bind(&record.organization_id)
                    .bind(&record.agent_id)
                    .bind(&record.task_id)
                    .bind(&record.content)
                    .bind(record.embedding.clone().unwrap_or_else(|| "[0]".to_string()))
                    .bind(&record.source_type)
                    .bind(record.topic.clone().unwrap_or_default())
                    .bind(record.last_sync_at.unwrap_or_else(Utc::now))
                    .execute(&mut *tx)
                    .await?;
                }
                tx.commit().await?;
            }
        }
        Ok(())
    }

    async fn mark_records_synced(&self, ids: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await?;
                for id_str in ids {
                    sqlx::query(
                        r#"
                        UPDATE autodream_memories
                        SET _sync_status = 'synced', last_sync_at = ?, synced_to_cloud = 1
                        WHERE id = ?
                        "#
                    )
                    .bind(Utc::now())
                    .bind(id_str)
                    .execute(&mut *tx)
                    .await?;
                }
                tx.commit().await?;
            },
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

                for id_str in ids {
                    let id = uuid::Uuid::parse_str(&id_str).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
                    sqlx::query(
                        r#"
                        UPDATE autodream_memories
                        SET sync_status = 'synced', last_sync_at = $1, synced_to_cloud = true
                        WHERE id = $2::uuid
                        "#
                    )
                    .bind(Utc::now())
                    .bind(id)
                    .execute(&mut *tx)
                    .await?;
                }

                tx.commit().await?;
            }
        }
        Ok(())
    }

    async fn process_forecast_tick(&self) -> Result<(), Box<dyn std::error::Error>> {
        let is_sqlite = self.db.is_sqlite();
        if is_sqlite {
            match self.fetch_pending_syncs(50).await {
                Ok(records) => {
                    let record_count = records.len() as f32;
                    if record_count > 0.0 {
                        // Dispatch to cloud via HTTP or queue...
                        // Here we simulate the sync success and mark them synced in the local DB.
                        match self.process_incoming_syncs(records.clone()).await {
                            Ok(_) => {
                                let ids: Vec<String> = records.into_iter().map(|r| r.id).collect();
                                match self.mark_records_synced(ids).await {
                                    Ok(_) => {
                                        let _ = crate::telemetry::record_autodream_sync(&self.db.pool, record_count).await;
                                    },
                                    Err(e) => {
                                        let _ = crate::telemetry::record_autodream_sync_error(&self.db.pool, record_count, &e.to_string()).await;
                                    }
                                }
                            },
                            Err(e) => {
                                let _ = crate::telemetry::record_autodream_sync_error(&self.db.pool, record_count, &e.to_string()).await;
                            }
                        }
                    }
                },
                Err(e) => {
                    let _ = crate::telemetry::record_autodream_sync_error(&self.db.pool, 1.0, &e.to_string()).await;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_autodream_sync_service_sqlite() {
        // Create an in-memory SQLite DB
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Create the autodream_memories table schema for the test
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS autodream_memories (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                task_id TEXT NOT NULL,
                content TEXT NOT NULL,
                embedding BLOB,
                source_type TEXT NOT NULL,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1,
                topic TEXT DEFAULT '',
                synced_to_cloud BOOLEAN DEFAULT 0,
                last_sync_at TIMESTAMP
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(crate::db::DB {
            pool: sqlx::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(), // unused for sqlite tests
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        let service = AutoDreamSyncServiceImpl::new(db);

        // Insert a dummy record using the service
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
            synced_to_cloud: Some(false),
        };

        let process_res = service.process_incoming_syncs(vec![record.clone()]).await;
        assert!(process_res.is_ok());

        // Fetch pending syncs
        let pending = service.fetch_pending_syncs(10).await;
        assert!(pending.is_ok());
        let pending_records = pending.unwrap();
        assert_eq!(pending_records.len(), 0); // process_incoming_syncs sets status to 'synced'

        // Manually insert an unsynced record to test process_forecast_tick implicitly and fetch_pending_syncs
        let unsynced_id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO autodream_memories (id, tenant_id, agent_id, task_id, content, source_type, _sync_status, synced_to_cloud) VALUES (?, 'org_2', 'agent_2', 'task_2', 'unsynced content', 'source', 'pending', 0)")
            .bind(&unsynced_id)
            .execute(&pool)
            .await
            .unwrap();

        let pending_after = service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(pending_after.len(), 1);

        let tick_res = service.process_forecast_tick().await;
        assert!(tick_res.is_ok());

        let pending_final = service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(pending_final.len(), 0);
    }
}
