use crate::db::{DbStore, DB};
use crate::orchestration::mesh::TeammateMesh;
use crate::interop::protocol::{InteropProtocol, proto as interop_proto};
use std::sync::Arc;

pub struct HandoffManager {
    protocol: Arc<InteropProtocol>,
    db: Arc<DB>,
}

impl HandoffManager {
    pub fn new(mesh: Arc<dyn TeammateMesh>, db: Arc<DB>, is_cloud: bool) -> Self {
        use crate::interop::protocol::proto::DeploymentMode;
        let mode = if is_cloud { DeploymentMode::ModeCloud } else { DeploymentMode::ModeStandalone };

        // We need a Bus and DistributedLock implementation for InteropProtocol.
        // HandoffManager currently receives TeammateMesh which should be adapted.
        // For now, let's assume we can wrap mesh into what InteropProtocol needs or
        // refactor run_server to provide the right types.
        // To keep it simple and compatible with existing orchestration code:
        let protocol = Arc::new(InteropProtocol::new(
            Arc::new(MeshBusAdapter { mesh: mesh.clone() }),
            Arc::new(MeshLockAdapter { mesh: mesh.clone() }),
            format!("handoff-manager-{}", uuid::Uuid::new_v4()),
            mode,
        ));

        Self { protocol, db }
    }

    pub async fn start_listener(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let db = self.db.clone();
        let protocol = self.protocol.clone();

        protocol.listen_for_state_handoff(Box::new(move |handoff| {
            let db_clone = db.clone();

            // Note: InteropProtocol handles reflection prevention and idempotency via locks internally now.
            // But we should still be careful here.

            tokio::spawn(async move {
                // Determine entity type from mission_id prefix or metadata if needed.
                // Here we'll use a simple convention or assume the payload has it.
                // For backward compatibility with the existing schema:
                let entity_type = if handoff.mission_id.starts_with("task:") { "shared_tasks" } else { "agent_memories" };
                let state_id = handoff.mission_id.strip_prefix("task:").unwrap_or(&handoff.mission_id);

                match entity_type {
                    "agent_memories" => {
                        match &db_clone.store {
                            DbStore::Postgres => {
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content, updated_at) VALUES ($1, $2, $3, to_timestamp($4::double precision / 1000.0)) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content, updated_at = excluded.updated_at WHERE agent_memories.updated_at < excluded.updated_at")
                                    .bind(state_id)
                                    .bind(&handoff.tenant_id)
                                    .bind(&handoff.state_snapshot)
                                    .bind(handoff.timestamp_ms)
                                    .execute(&db_clone.pool)
                                    .await
                                {
                                    tracing::error!("Failed to save state handoff (agent_memories) to Postgres: error={}", e);
                                }
                            }
                            DbStore::Sqlite(sqlite_pool) => {
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content, updated_at) VALUES (?, ?, ?, datetime(?, 'unixepoch')) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content, updated_at = excluded.updated_at WHERE agent_memories.updated_at < excluded.updated_at")
                                    .bind(state_id)
                                    .bind(&handoff.tenant_id)
                                    .bind(&handoff.state_snapshot)
                                    .bind(handoff.timestamp_ms / 1000)
                                    .execute(sqlite_pool)
                                    .await
                                {
                                    tracing::error!("Failed to save state handoff (agent_memories) to Sqlite: error={}", e);
                                }
                            }
                        }
                    },
                    "shared_tasks" => {
                        let payload_str = String::from_utf8_lossy(&handoff.state_snapshot).to_string();
                        match &db_clone.store {
                            DbStore::Postgres => {
                                let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({}));
                                if let Err(e) = sqlx::query("UPDATE shared_tasks_decomposition SET payload = $1, updated_at = to_timestamp($2::double precision / 1000.0) WHERE id = $3 AND updated_at < to_timestamp($2::double precision / 1000.0)")
                                    .bind(&payload_json)
                                    .bind(handoff.timestamp_ms)
                                    .bind(state_id)
                                    .execute(&db_clone.pool)
                                    .await
                                {
                                    tracing::error!("Failed to save state handoff (shared_tasks) to Postgres: error={}", e);
                                }
                            }
                            DbStore::Sqlite(sqlite_pool) => {
                                if let Err(e) = sqlx::query("UPDATE shared_tasks_decomposition SET payload = ?, updated_at = datetime(?, 'unixepoch') WHERE id = ? AND updated_at < datetime(?, 'unixepoch')")
                                    .bind(&payload_str)
                                    .bind(handoff.timestamp_ms / 1000)
                                    .bind(state_id)
                                    .bind(handoff.timestamp_ms / 1000)
                                    .execute(sqlite_pool)
                                    .await
                                {
                                    tracing::error!("Failed to save state handoff (shared_tasks) to Sqlite: error={}", e);
                                }
                            }
                        }
                    },
                    _ => {
                        tracing::warn!("Received handoff for unknown entity type: {}", entity_type);
                    }
                }
            });
        })).await
    }

    pub async fn initiate_handoff(
        &self,
        tenant_id: &str,
        state_id: &str,
        state: Vec<u8>,
        entity_type: &str,
    ) -> Result<(), String> {
        let mission_id = if entity_type == "shared_tasks" {
            format!("task:{}", state_id)
        } else {
            state_id.to_string()
        };

        self.protocol.handoff(&mission_id, tenant_id, state).await
    }
}

struct MeshBusAdapter {
    mesh: Arc<dyn TeammateMesh>,
}

#[async_trait::async_trait]
impl crate::msgbus::Bus for MeshBusAdapter {
    async fn publish(&self, msg: crate::msgbus::Message) -> Result<(), String> {
        self.mesh.publish(&msg.topic, msg.payload).await
    }
    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(crate::msgbus::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let topic_clone = topic.clone();
        self.mesh.subscribe(&topic, Box::new(move |mesh_msg| {
            handler(crate::msgbus::Message {
                topic: topic_clone.clone(),
                payload: mesh_msg.payload,
            });
        })).await
    }
}

struct MeshLockAdapter {
    mesh: Arc<dyn TeammateMesh>,
}

#[async_trait::async_trait]
impl crate::msgbus::DistributedLock for MeshLockAdapter {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.mesh.acquire_lock(resource, owner, ttl_seconds).await
    }
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.mesh.release_lock(resource, owner).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use sqlx::Row;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_handoff_manager() {
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(
            transport.clone(),
        ));

        // Use SQLite memory db for the test to avoid mock postgres failure and handle ack
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE agent_memories (id TEXT PRIMARY KEY, organization_id TEXT, raw_content BLOB, updated_at TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .after_release(|conn, _meta| {
                    Box::pin(async move {
                        use sqlx::Executor;
                        conn.execute("DISCARD ALL").await?;
                        Ok(true)
                    })
                })
                .connect_lazy("postgres://localhost/dummy")
                .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let manager = HandoffManager::new(mesh, db, false);
        let manager_arc = Arc::new(manager);

        let _cancel = manager_arc.start_listener().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let m_arc = manager_arc.clone();
        tokio::spawn(async move {
            let _ = m_arc
                .initiate_handoff(
                    "tenant1",
                    "state1",
                    b"some_state".to_vec(),
                    "agent_memories",
                )
                .await;
        });

        // Let listener process loop
        let mut found = false;
        for _ in 0..30 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let row = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'state1'")
                .fetch_optional(&pool)
                .await
                .unwrap();
            if let Some(r) = row {
                let content: Vec<u8> = r.get("raw_content");
                if content == b"some_state".to_vec() {
                    found = true;
                    break;
                }
            }
        }
        assert!(found);
    }

    #[tokio::test]
    async fn test_handoff_listener() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE agent_memories (id TEXT PRIMARY KEY, organization_id TEXT, raw_content BLOB, updated_at TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .after_release(|conn, _meta| {
                    Box::pin(async move {
                        use sqlx::Executor;
                        conn.execute("DISCARD ALL").await?;
                        Ok(true)
                    })
                })
                .connect_lazy("postgres://localhost/dummy")
                .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(
            transport.clone(),
        ));
        let manager = HandoffManager::new(mesh.clone(), db.clone(), true);

        let _cancel = manager.start_listener().await.unwrap();

        let protocol = InteropProtocol::new(
            Arc::new(MeshBusAdapter { mesh: mesh.clone() }),
            Arc::new(MeshLockAdapter { mesh: mesh.clone() }),
            "test-node".to_string(),
            interop_proto::DeploymentMode::ModeStandalone,
        );

        protocol.handoff("test_state", "test_tenant", b"hello_world".to_vec()).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let content: Vec<u8> = row.get("raw_content");
        assert_eq!(content, b"hello_world".to_vec());
    }

    #[tokio::test]
    async fn test_handoff_listener_shared_tasks() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE shared_tasks_decomposition (id TEXT PRIMARY KEY, payload TEXT, updated_at TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        // Insert a dummy task with an older timestamp to satisfy the UPDATE statement
        sqlx::query("INSERT INTO shared_tasks_decomposition (id, payload, updated_at) VALUES ('task_123', 'old_payload', datetime('now', '-1 day'))")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .after_release(|conn, _meta| {
                    Box::pin(async move {
                        use sqlx::Executor;
                        conn.execute("DISCARD ALL").await?;
                        Ok(true)
                    })
                })
                .connect_lazy("postgres://localhost/dummy")
                .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(
            transport.clone(),
        ));
        let manager = HandoffManager::new(mesh.clone(), db.clone(), true);

        let _cancel = manager.start_listener().await.unwrap();

        let protocol = InteropProtocol::new(
            Arc::new(MeshBusAdapter { mesh: mesh.clone() }),
            Arc::new(MeshLockAdapter { mesh: mesh.clone() }),
            "test-node".to_string(),
            interop_proto::DeploymentMode::ModeStandalone,
        );

        protocol.handoff("task:task_123", "test_tenant", br#"{"key": "value"}"#.to_vec()).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        let row =
            sqlx::query("SELECT payload FROM shared_tasks_decomposition WHERE id = 'task_123'")
                .fetch_one(&pool)
                .await
                .unwrap();

        let content: String = row.get("payload");
        assert_eq!(content, r#"{"key": "value"}"#);
    }
}
