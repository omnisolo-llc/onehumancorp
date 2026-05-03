use crate::ohc::orchestration::SyncStateHandoff;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message as MeshMessage};
use crate::ohc::orchestration::TeammateMeshEvent;
use std::sync::Arc;
use prost::Message;
use crate::db::{DB, DbStore};

pub struct HandoffManager {
    transport: Arc<dyn MeshTransport>,
    db: Arc<DB>,
    is_cloud: bool,
}

impl HandoffManager {
    pub fn new(transport: Arc<dyn MeshTransport>, db: Arc<DB>, is_cloud: bool) -> Self {
        Self { transport, db, is_cloud }
    }

    pub async fn start_listener(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let db = self.db.clone();
        let is_cloud = self.is_cloud;
        let transport_clone = self.transport.clone();

        let handler = Box::new(move |msg: MeshMessage| {
            if let Ok(handoff) = SyncStateHandoff::decode(&msg.payload[..]) {
                // Prevent reflection (don't process messages we sent)
                let current_mode = if is_cloud { "cloud" } else { "standalone" };
                if handoff.mode_source == current_mode {
                    return;
                }

                let db_clone = db.clone();
                let transport = transport_clone.clone();

                tokio::spawn(async move {
                    let lock_key = format!("handoff:{}:{}", handoff.tenant_id, handoff.state_id);
                    if let Ok(true) = transport.acquire_lock(&lock_key, "handoff_manager", 60).await {
                        match &db_clone.store {
                            DbStore::Postgres => {
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content, updated_at) VALUES ($1, $2, $3, to_timestamp($4)) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content, updated_at = excluded.updated_at WHERE agent_memories.updated_at < excluded.updated_at")
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
                        let _ = transport.release_lock(&lock_key, "handoff_manager").await;
                    }
                });
            }
        });

        self.transport.subscribe("mesh:coordination:handoff", handler).await
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

        let msg = TeammateMeshEvent {
            agent_id: "handoff".to_string(),
            action: "mesh:coordination:handoff".to_string(),
            status: "ok".to_string(),
            payload: buf,
        };

        self.transport.publish("mesh:coordination:handoff", msg).await
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
        // For testing we will use a dummy Postgres pool, it doesn't need to connect if we don't await execution
        let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap(), store: DbStore::Postgres });

        let manager = HandoffManager::new(transport, db, false);

        let cancel = manager.start_listener().await.unwrap();

        let res = manager.initiate_handoff("tenant1", "state1", b"some_state".to_vec()).await;
        assert!(res.is_ok());

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

        sqlx::query("CREATE TABLE agent_memories (id TEXT PRIMARY KEY, organization_id TEXT, raw_content BLOB, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap(), store: DbStore::Sqlite(pool.clone()) });
        let transport = Arc::new(MemoryTransport::new());
        let manager = HandoffManager::new(transport.clone(), db.clone(), true);

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

        let msg = TeammateMeshEvent {
            agent_id: "handoff".to_string(),
            action: "mesh:coordination:handoff".to_string(),
            status: "ok".to_string(),
            payload: buf,
        };

        transport.publish("mesh:coordination:handoff", msg).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let content: Vec<u8> = row.get("raw_content");
        assert_eq!(content, b"hello_world".to_vec());

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

        let msg2 = TeammateMeshEvent {
            agent_id: "handoff".to_string(),
            action: "mesh:coordination:handoff".to_string(),
            status: "ok".to_string(),
            payload: buf2,
        };

        transport.publish("mesh:coordination:handoff", msg2).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row2 = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state_2'")
            .fetch_optional(&pool)
            .await
            .unwrap();

        assert!(row2.is_none());


        // Test idempotency: older timestamp should not overwrite
        let handoff3 = SyncStateHandoff {
            tenant_id: "test_tenant".to_string(),
            state_id: "test_state".to_string(),
            serialized_state: b"older_state_should_be_ignored".to_vec(),
            mode_source: "standalone".to_string(),
            timestamp: chrono::Utc::now().timestamp() - 10000, // Older timestamp
        };

        let mut buf3 = Vec::new();
        handoff3.encode(&mut buf3).unwrap();

        let msg3 = TeammateMeshEvent {
            agent_id: "handoff".to_string(),
            action: "mesh:coordination:handoff".to_string(),
            status: "ok".to_string(),
            payload: buf3,
        };

        transport.publish("mesh:coordination:handoff", msg3).await.unwrap();

        // Let listener process
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let row3 = sqlx::query("SELECT raw_content FROM agent_memories WHERE id = 'test_state'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let content3: Vec<u8> = row3.get("raw_content");
        assert_eq!(content3, b"hello_world".to_vec()); // Should still be hello_world

        cancel();
    }
}
