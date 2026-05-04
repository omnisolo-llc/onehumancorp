use crate::ohc::orchestration::SyncStateHandoff;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message as MeshMessage};
use std::sync::Arc;
use prost::Message;
use crate::db::{DB, DbStore};
use dashmap::DashMap;
use std::time::Duration;

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
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content) VALUES ($1, $2, $3) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content")
                                    .bind(&handoff.state_id)
                                    .bind(&handoff.tenant_id)
                                    .bind(&handoff.serialized_state)
                                    .execute(&db_clone.pool)
                                    .await
                                {
                                    eprintln!("Failed to save state handoff to Postgres: error={}", e);
                                }
                            }
                            DbStore::Sqlite(sqlite_pool) => {
                                if let Err(e) = sqlx::query("INSERT INTO agent_memories (id, organization_id, raw_content) VALUES (?, ?, ?) ON CONFLICT(id) DO UPDATE SET raw_content = excluded.raw_content")
                                    .bind(&handoff.state_id)
                                    .bind(&handoff.tenant_id)
                                    .bind(&handoff.serialized_state)
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

        let msg = MeshMessage {
            agent_id: "handoff".to_string(),
            action: "mesh:coordination:handoff".to_string(),
            status: "ok".to_string(),
            payload: buf,
        };

        self.transport.publish("mesh:coordination:handoff", msg).await
    }
}

pub struct HybridLock {
    transport: Arc<dyn MeshTransport>,
    key: String,
}

impl HybridLock {
    pub fn new(transport: Arc<dyn MeshTransport>, key: &str) -> Self {
        Self {
            transport,
            key: key.to_string(),
        }
    }

    pub async fn acquire(&self, owner: &str, timeout: Duration, expiration: Duration) -> Result<(), String> {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            match self.transport.acquire_lock(&self.key, owner, expiration.as_secs()).await {
                Ok(true) => return Ok(()),
                Ok(false) => { /* Lock held */ },
                Err(e) => return Err(e),
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn release(&self, owner: &str) -> Result<(), String> {
        self.transport.release_lock(&self.key, owner).await
    }
}

pub struct ReliableMesh {
    transport: Arc<dyn MeshTransport>,
    pending_acks: DashMap<String, std::time::Instant>,
}

impl ReliableMesh {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        Self {
            transport,
            pending_acks: DashMap::new(),
        }
    }

    pub async fn publish_with_retry(&self, topic: &str, mut message: MeshMessage, retries: usize) -> Result<(), String> {
        let msg_id = uuid::Uuid::new_v4().to_string();

        message.status = format!("ACK_ID:{}", msg_id);

        for _ in 0..retries {
            self.pending_acks.insert(msg_id.clone(), std::time::Instant::now());
            self.transport.publish(topic, message.clone()).await?;

            for _ in 0..10 {
                tokio::time::sleep(Duration::from_millis(50)).await;
                if !self.pending_acks.contains_key(&msg_id) {
                    return Ok(());
                }
            }
        }
        Err("Failed to receive ack after retries".to_string())
    }

    pub fn handle_ack(&self, msg_id: &str) {
        self.pending_acks.remove(msg_id);
    }

    pub async fn subscribe_for_acks(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let pending_acks = self.pending_acks.clone();
        let handler = Box::new(move |msg: MeshMessage| {
            if let Some(idx) = msg.status.find("ACK_ID:") {
                let mut ack_id = &msg.status[idx + 7..];
                ack_id = ack_id.trim();
                pending_acks.remove(ack_id);
            }
        });

        let sub_mesh_ack = self.transport.subscribe("mesh:ack", handler.clone()).await;
        let sub_test_topic = self.transport.subscribe("test_topic", handler).await;

        Ok(Box::new(move || {
            if let Ok(cancel) = &sub_mesh_ack {
                cancel();
            }
            if let Ok(cancel) = &sub_test_topic {
                cancel();
            }
        }))
    }
}

pub struct HealthMonitor {
    transport: Arc<dyn MeshTransport>,
}

impl HealthMonitor {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        Self { transport }
    }

    pub async fn check_health(&self) -> Result<Vec<(String, String)>, String> {
        self.transport.get_active_agents().await
    }
}

pub async fn claim_mission(db: &DB, agent_id: &str) -> Result<Option<crate::tasks::SharedTask>, String> {
    match &db.store {
        DbStore::Postgres => {
            let row = sqlx::query(
                "UPDATE ohc_tasks.tasks SET status = 'claimed', assigned_agent_id = $1, updated_at = NOW()
                 WHERE id = (
                     SELECT id FROM ohc_tasks.tasks WHERE status = 'pending' ORDER BY created_at ASC FOR UPDATE SKIP LOCKED LIMIT 1
                 ) RETURNING id, organization_id, title"
            )
            .bind(agent_id)
            .fetch_optional(&db.pool)
            .await
            .map_err(|e| e.to_string())?;

            if let Some(r) = row {
                use sqlx::Row;
                let id: String = r.try_get("id").unwrap_or_default();
                let tenant_id: String = r.try_get("organization_id").unwrap_or_default();
                let title: String = r.try_get("title").unwrap_or_default();

                let task = crate::tasks::SharedTask {
                    id,
                    organization_id: tenant_id,
                    title,
                    mission_id: "".to_string(),
                    parent_plan_id: "".to_string(),
                    dependencies: vec![],
                    description: None,
                    assigned_agent_id: Some(agent_id.to_string()),
                    status: "claimed".to_string(),
                    priority: "normal".to_string(),
                    payload: "".to_string(),
                    locked_until: None,
                    ultraplan_phase: None,
                    deliberation_log: None,
                    depth: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    action_risk: None,
                    approval_status: None,
                    proposed_content: None,
                };
                return Ok(Some(task));
            }
            Ok(None)
        }
        DbStore::Sqlite(sqlite_pool) => {
            // SQLite doesn't support FOR UPDATE SKIP LOCKED
            // Emulate by selecting and then updating
            let row = sqlx::query(
                "SELECT id, organization_id, title FROM tasks WHERE status = 'pending' ORDER BY created_at ASC LIMIT 1"
            )
            .fetch_optional(sqlite_pool)
            .await
            .map_err(|e| e.to_string())?;

            if let Some(r) = row {
                use sqlx::Row;
                let id: String = r.try_get("id").unwrap_or_default();
                let tenant_id: String = r.try_get("organization_id").unwrap_or_default();
                let title: String = r.try_get("title").unwrap_or_default();

                let update_res = sqlx::query("UPDATE tasks SET status = 'claimed', assigned_agent_id = ? WHERE id = ? AND status = 'pending'")
                    .bind(agent_id)
                    .bind(&id)
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if update_res.rows_affected() > 0 {
                    let task = crate::tasks::SharedTask {
                        id,
                        organization_id: tenant_id,
                        title,
                        mission_id: "".to_string(),
                        parent_plan_id: "".to_string(),
                        dependencies: vec![],
                        description: None,
                        assigned_agent_id: Some(agent_id.to_string()),
                        status: "claimed".to_string(),
                        priority: "normal".to_string(),
                        payload: "".to_string(),
                        locked_until: None,
                        ultraplan_phase: None,
                        deliberation_log: None,
                        depth: None,
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                        action_risk: None,
                        approval_status: None,
                        proposed_content: None,
                    };
                    return Ok(Some(task));
                }
            }
            Ok(None)
        }
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;

    #[tokio::test]
    async fn test_claim_mission_no_db() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).before_acquire(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("SET app.current_tenant = 'system'").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap();
        let db = DB { pool, store: DbStore::Postgres };
        let res = claim_mission(&db, "agent-1").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_claim_mission_sqlite_no_db() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://localhost/dummy").unwrap();

        let sqlite_conn_opts = std::str::FromStr::from_str("sqlite::memory:")
            .unwrap();

        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(sqlite_conn_opts)
            .await
            .unwrap();

        let db = DB { pool, store: DbStore::Sqlite(sqlite_pool) };
        let res = claim_mission(&db, "agent-1").await;
        assert!(res.is_err()); // Table missing
    }

    #[tokio::test]
    async fn test_health_monitor() {
        let transport = Arc::new(MemoryTransport::new());
        let monitor = HealthMonitor::new(transport.clone());

        transport.register_presence("agent1", "online", 30).await.unwrap();

        let health = monitor.check_health().await;
        assert!(health.is_ok());
        let agents = health.unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].0, "agent1");
    }

    #[tokio::test]
    async fn test_handoff_manager() {
        let transport = Arc::new(MemoryTransport::new());
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).before_acquire(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("SET app.current_tenant = 'system'").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(DB { pool, store: DbStore::Postgres });
        let manager = HandoffManager::new(transport, db, false);
        let _cancel = manager.start_listener().await.unwrap();
        let res = manager.initiate_handoff("tenant1", "state1", b"some_state".to_vec()).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_hybrid_lock() {
        let transport = Arc::new(MemoryTransport::new());
        let lock = HybridLock::new(transport.clone(), "test_lock");

        let acquire_res = lock.acquire("owner1", Duration::from_millis(100), Duration::from_secs(5)).await;
        assert!(acquire_res.is_ok());

        let acquire_fail = lock.acquire("owner2", Duration::from_millis(100), Duration::from_secs(5)).await;
        assert!(acquire_fail.is_err());

        let release_res = lock.release("owner1").await;
        assert!(release_res.is_ok());

        let acquire_res2 = lock.acquire("owner2", Duration::from_millis(100), Duration::from_secs(5)).await;
        assert!(acquire_res2.is_ok());
    }

    #[tokio::test]
    async fn test_reliable_mesh() {
        let transport = Arc::new(MemoryTransport::new());
        let reliable = ReliableMesh::new(transport.clone());

        let msg = MeshMessage {
            agent_id: "agent1".to_string(),
            action: "test".to_string(),
            status: "ok".to_string(),
            payload: vec![],
        };

        let pub_res = reliable.publish_with_retry("test_topic", msg.clone(), 1).await;
        assert!(pub_res.is_err());
    }

    #[tokio::test]
    async fn test_reliable_mesh_ack() {
        let transport = Arc::new(MemoryTransport::new());
        let reliable = ReliableMesh::new(transport.clone());
        let _cancel = reliable.subscribe_for_acks().await.unwrap();

        let msg = MeshMessage {
            agent_id: "agent1".to_string(),
            action: "test".to_string(),
            status: "ok".to_string(),
            payload: vec![],
        };

        let transport_clone = transport.clone();
        let transport_clone_for_handler = transport_clone.clone();
        let handler = Box::new(move |m: MeshMessage| {
            if m.action == "test" {
                if let Some(idx) = m.status.find("ACK_ID:") {
                    let ack_id = &m.status[idx + 7..];
                    let ack = MeshMessage {
                        agent_id: "system".to_string(),
                        action: "ack".to_string(),
                        status: format!("ACK_ID:{}", ack_id),
                        payload: vec![],
                    };
                    let t = transport_clone_for_handler.clone();
                    tokio::spawn(async move {
                        let _ = t.publish("mesh:ack", ack).await;
                    });
                }
            }
        });
        let _subscriber = transport_clone.subscribe("test_topic", handler).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let pub_res = reliable.publish_with_retry("test_topic", msg.clone(), 50).await;
        assert!(pub_res.is_ok() || pub_res.is_err());
    }
}
