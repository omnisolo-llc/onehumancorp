use crate::ohc::orchestration::SyncStateHandoff;
use ohc_builtin_agent::mesh::transport::Message as MeshMessage;
use crate::orchestration::mesh::TeammateMesh;
use std::sync::Arc;
use prost::Message;
use crate::db::{DB, DbStore};

pub struct HandoffManager {
    mesh: Arc<dyn TeammateMesh>,
    db: Arc<DB>,
    is_cloud: bool,
}

impl HandoffManager {
    pub fn new(mesh: Arc<dyn TeammateMesh>, db: Arc<DB>, is_cloud: bool) -> Self {
        Self { mesh, db, is_cloud }
    }

    pub async fn start_listener(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let db = self.db.clone();
        let is_cloud = self.is_cloud;
        let mesh_clone = self.mesh.clone();

        let handler = Box::new(move |msg: MeshMessage| {
            if let Ok(handoff) = SyncStateHandoff::decode(&msg.payload[..]) {
                // Prevent reflection (don't process messages we sent)
                let current_mode = if is_cloud { "cloud" } else { "standalone" };
                if handoff.mode_source == current_mode {
                    return;
                }

                let db_clone = db.clone();
                let mesh = mesh_clone.clone();
                let msg_id_for_ack = msg.msg_id.clone();

                tokio::spawn(async move {
                    let lock_key = format!("handoff:{}:{}", handoff.tenant_id, handoff.state_id);
                    if let Ok(true) = mesh.acquire_lock(&lock_key, "handoff_manager", 60).await {
                        match &db_clone.store {
                            DbStore::Postgres => {
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content, updated_at) VALUES ($1, $2, $3, to_timestamp($4::double precision)) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content, updated_at = excluded.updated_at WHERE agent_memories.updated_at < excluded.updated_at")
                                    .bind(&handoff.state_id)
                                    .bind(&handoff.tenant_id)
                                    .bind(&handoff.serialized_state)
                                    .bind(handoff.timestamp)
                                    .execute(&db_clone.pool)
                                    .await
                                {
                                    eprintln!("Failed to save state handoff to Postgres: error={}", e);
                                }
                            }
                            DbStore::Sqlite(sqlite_pool) => {
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content, updated_at) VALUES (?, ?, ?, datetime(?, 'unixepoch')) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content, updated_at = excluded.updated_at WHERE agent_memories.updated_at < excluded.updated_at")
                                    .bind(&handoff.state_id)
                                    .bind(&handoff.tenant_id)
                                    .bind(&handoff.serialized_state)
                                    .bind(handoff.timestamp)
                                    .execute(sqlite_pool)
                                    .await
                                {
                                    eprintln!("Failed to save state handoff to Sqlite: error={}", e);
                                }
                            }
                        }
                        let _ = mesh.release_lock(&lock_key, "handoff_manager").await;
                    }

                    if !msg_id_for_ack.is_empty() {
                        let ack_topic = format!("mesh:ack:{}", msg_id_for_ack);
                        let _ = mesh.publish(&ack_topic, vec![]).await;
                    }
                });
            }
        });

        self.mesh.subscribe("mesh:coordination:handoff", handler).await
    }

    pub async fn initiate_handoff(&self, tenant_id: &str, state_id: &str, state: Vec<u8>) -> Result<(), String> {
        let handoff = SyncStateHandoff {
            tenant_id: tenant_id.to_string(),
            state_id: state_id.to_string(),
            serialized_state: state,
            mode_source: if self.is_cloud { "cloud".to_string() } else { "standalone".to_string() },
            timestamp: chrono::Utc::now().timestamp(),
        };

        let mut buf = Vec::new();
        handoff.encode(&mut buf).map_err(|e| e.to_string())?;

        self.mesh.publish_with_ack("mesh:coordination:handoff", buf).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
    use std::str::FromStr;
    use sqlx::Row;

    #[tokio::test]
    async fn test_handoff_manager() {
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport.clone()));

        // Use SQLite memory db for the test to avoid mock postgres failure and handle ack
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE agent_memories (id TEXT PRIMARY KEY, organization_id TEXT, raw_content BLOB, updated_at TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(), store: DbStore::Sqlite(pool.clone()) });

        let manager = HandoffManager::new(mesh, db, false);
        let manager_arc = Arc::new(manager);

        let cancel = manager_arc.start_listener().await.unwrap();

        let _ = tokio::spawn(async move {
            let _ = manager_arc.initiate_handoff("tenant1", "state1", b"some_state".to_vec()).await;
        });

        // Let listener process loop
        let mut found = false;
        for _ in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let row = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'state1'")
                .fetch_optional(&pool)
                .await
                .unwrap();
            if let Some(r) = row {
                let content: Vec<u8> = r.get("raw_content");
                assert_eq!(content, b"some_state".to_vec());
                found = true;
                break;
            }
        }
        assert!(found, "Handoff state was not durably stored by the listener");

        cancel();
    }

    #[tokio::test]
    async fn test_handoff_listener() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE agent_memories (id TEXT PRIMARY KEY, organization_id TEXT, raw_content BLOB, updated_at TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap(), store: DbStore::Sqlite(pool.clone()) });
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport.clone()));
        let manager = HandoffManager::new(mesh.clone(), db.clone(), true);

        let cancel = manager.start_listener().await.unwrap();

        let handoff = SyncStateHandoff {
            tenant_id: "test_tenant".to_string(),
            state_id: "test_state".to_string(),
            serialized_state: b"hello_world".to_vec(),
            mode_source: "standalone".to_string(), // Source is different than current mode, so it should process it
            timestamp: chrono::Utc::now().timestamp(),
        };

        let mut buf = Vec::new();
        handoff.encode(&mut buf).unwrap();

        mesh.publish("mesh:coordination:handoff", buf).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let content: Vec<u8> = row.get("raw_content");
        assert_eq!(content, b"hello_world".to_vec());

        // Test older message is ignored (LWW)
        let older_handoff = SyncStateHandoff {
            tenant_id: "test_tenant".to_string(),
            state_id: "test_state".to_string(), // Same ID
            serialized_state: b"older_content".to_vec(),
            mode_source: "standalone".to_string(),
            timestamp: chrono::Utc::now().timestamp() - 100, // Older timestamp
        };
        let mut buf_older = Vec::new();
        older_handoff.encode(&mut buf_older).unwrap();
        mesh.publish("mesh:coordination:handoff", buf_older).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row_after_older = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let content_after_older: Vec<u8> = row_after_older.get("raw_content");
        assert_eq!(content_after_older, b"hello_world".to_vec()); // Should not have changed

        // Test newer message updates (LWW)
        let newer_handoff = SyncStateHandoff {
            tenant_id: "test_tenant".to_string(),
            state_id: "test_state".to_string(), // Same ID
            serialized_state: b"newer_content".to_vec(),
            mode_source: "standalone".to_string(),
            timestamp: chrono::Utc::now().timestamp() + 100, // Newer timestamp
        };
        let mut buf_newer = Vec::new();
        newer_handoff.encode(&mut buf_newer).unwrap();
        mesh.publish("mesh:coordination:handoff", buf_newer).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row_after_newer = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let content_after_newer: Vec<u8> = row_after_newer.get("raw_content");
        assert_eq!(content_after_newer, b"newer_content".to_vec()); // Should have changed

        // Test reflection prevention (same mode source)
        let handoff2 = SyncStateHandoff {
            tenant_id: "test_tenant".to_string(),
            state_id: "test_state_2".to_string(),
            serialized_state: b"should_not_save".to_vec(),
            mode_source: "cloud".to_string(), // Same as is_cloud=true
            timestamp: chrono::Utc::now().timestamp(),
        };

        let mut buf2 = Vec::new();
        handoff2.encode(&mut buf2).unwrap();

        mesh.publish("mesh:coordination:handoff", buf2).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row2 = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state_2'")
            .fetch_optional(&pool)
            .await
            .unwrap();

        assert!(row2.is_none());

        cancel();
    }
}
