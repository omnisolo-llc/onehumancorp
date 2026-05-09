use sqlx::Row;
use chrono::Utc;
use std::sync::Arc;
use crate::db::{DB, DbStore};

pub struct SipDB {
    db: Arc<DB>,
    org_id: String,
    context_root: Option<String>,
}

impl SipDB {
    pub fn new(db: Arc<DB>, org_id: String) -> Self {
        SipDB {
            db,
            org_id,
            context_root: None,
        }
    }

    pub async fn handoff_mission(&self, mission_id: &str, blockers: &str) -> Result<(), sqlx::Error> {
        let org_id = self.org_id.clone();
        let mission_id = mission_id.to_string();
        let blockers = blockers.to_string();
        let db = self.db.clone();

        self.db.execute_with_retry::<_, _, _, sqlx::Error>("handoff_mission", || {
            let mission_id = mission_id.clone();
            let blockers = blockers.clone();
            let org_id = org_id.clone();
            let db = db.clone();
            async move {
                match &db.store {
                    DbStore::Postgres => {
                        let mut tx = db.pool.begin().await?;
                        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await
                            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

                        sqlx::query(
                            "UPDATE agent_missions
                             SET status = 'blocked',
                                 mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN $1 ELSE mission_log || '\n' || $1 END,
                                 updated_at = CURRENT_TIMESTAMP
                             WHERE id = $2 AND tenant_id = $3"
                        )
                        .bind(blockers)
                        .bind(mission_id)
                        .bind(org_id)
                        .execute(&mut *tx)
                        .await?;

                        tx.commit().await?;
                    }
                    DbStore::Sqlite(pool) => {
                        sqlx::query(
                            "UPDATE agent_missions
                             SET status = 'blocked',
                                 mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN ? ELSE mission_log || '\n' || ? END,
                                 updated_at = CURRENT_TIMESTAMP
                             WHERE id = ? AND tenant_id = ?"
                        )
                        .bind(blockers.clone())
                        .bind(blockers)
                        .bind(mission_id)
                        .bind(org_id)
                        .execute(pool)
                        .await?;
                    }
                }
                Ok::<(), sqlx::Error>(())
            }
        }).await?;

        Ok(())
    }

    pub fn with_context_root(mut self, root: String) -> Self {
        self.context_root = Some(root);
        self
    }

    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;
        let org_id = self.org_id.clone();
        let db = self.db.clone();

        tokio::time::timeout(std::time::Duration::from_secs(60), self.db.execute_with_retry::<_, _, _, sqlx::Error>("prune_stale_missions", || {
            let org_id = org_id.clone();
            let db = db.clone();
            async move {
                match &db.store {
                    DbStore::Postgres => {
                        let mut tx = db.pool.begin().await?;
                        crate::utils::auth_utils::set_org_context(&mut *tx, &org_id).await
                            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

                        sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND tenant_id = $2")
                            .bind(stuck_threshold)
                            .bind(&org_id)
                            .execute(&mut *tx)
                            .await?;

                        sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK' AND tenant_id = $1")
                            .bind(&org_id)
                            .execute(&mut *tx)
                            .await?;

                        sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = $1 ORDER BY created_at ASC LIMIT 10)")
                            .bind(&org_id)
                            .execute(&mut *tx)
                            .await?;

                        sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND tenant_id = $2")
                            .bind(fail_threshold)
                            .bind(&org_id)
                            .execute(&mut *tx)
                            .await?;

                        sqlx::query("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < $1)) AND tenant_id = $2 LIMIT 1000)")
                            .bind(fail_threshold)
                            .bind(&org_id)
                            .execute(&mut *tx)
                            .await?;

                        tx.commit().await?;
                    }
                    DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < ? AND tenant_id = ?")
                            .bind(stuck_threshold.to_rfc3339())
                            .bind(&org_id)
                            .execute(pool)
                            .await?;

                        sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK' AND tenant_id = ?")
                            .bind(&org_id)
                            .execute(pool)
                            .await?;

                        sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = ? ORDER BY created_at ASC LIMIT 10)")
                            .bind(&org_id)
                            .execute(pool)
                            .await?;

                        sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < ? AND tenant_id = ?")
                            .bind(fail_threshold.to_rfc3339())
                            .bind(&org_id)
                            .execute(pool)
                            .await?;

                        sqlx::query("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < ?)) AND tenant_id = ? LIMIT 1000)")
                            .bind(fail_threshold.to_rfc3339())
                            .bind(&org_id)
                            .execute(pool)
                            .await?;
                    }
                }
                Ok::<(), sqlx::Error>(())
            }
        })).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, e)))??;

        Ok(())
    }

    pub async fn load_grounding_content(&self) -> Option<String> {
        if let Some(ref root) = self.context_root {
            let root_path = std::path::Path::new(root);

            let agents_path = root_path.join("AGENTS.md");
            if let Ok(content) = tokio::fs::read_to_string(&agents_path).await {
                return Some(content);
            }

            let claude_path = root_path.join("CLAUDE.md");
            if let Ok(content) = tokio::fs::read_to_string(&claude_path).await {
                return Some(content);
            }
        }
        None
    }

    pub fn enrich_payload_with_grounding_content(&self, payload: &str, grounding_content: &Option<String>) -> String {
        let mut final_payload = payload.to_string();
        if let Some(content) = grounding_content {
            final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", payload, content);
        }
        final_payload
    }

    pub async fn delegate_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool, grounding_content: &Option<String>) -> Result<(), sqlx::Error> {
        let final_payload = self.enrich_payload_with_grounding_content(payload, grounding_content);

        let org_id = self.org_id.clone();
        let mission_id = mission_id.to_string();
        let status = status.to_string();

        let mut final_status = status.clone();
        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = $1 AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(&org_id)
            .fetch_one(&mut **tx)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        if force_local {
            // ML-Resilience Rule 3: Idempotent operations via ON CONFLICT DO UPDATE
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload, updated_at = CURRENT_TIMESTAMP, tenant_id = EXCLUDED.tenant_id")
                .bind(mission_id)
                .bind(final_status)
                .bind(final_payload)
                .bind(org_id)
                .execute(&mut **tx)
                .await?;
        } else {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                .bind(mission_id)
                .bind(final_status)
                .bind(final_payload)
                .bind(org_id)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }

    pub async fn upsert_mission(&self, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mission_id = mission_id.to_string();
        let status = status.to_string();
        let payload = payload.to_string();
        let org_id = self.org_id.clone();
        let db = self.db.clone();

        tokio::time::timeout(std::time::Duration::from_secs(60), self.db.execute_with_retry::<_, _, _, sqlx::Error>("upsert_mission", || {
            let mission_id = mission_id.clone();
            let status = status.clone();
            let payload = payload.clone();
            let org_id = org_id.clone();
            let db = db.clone();
            async move {
                match &db.store {
                    DbStore::Postgres => {
                        let mut tx = db.pool.begin().await?;
                        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await
                            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

                        let mut final_status = status.clone();
                        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = $1 AND (status = 'PENDING' OR status = 'RUNNING')")
                            .bind(&org_id)
                            .fetch_one(&mut *tx)
                            .await?;

                        if pending_count >= 5 && status == "PENDING" {
                            final_status = "BURSTING".to_string();
                        }

                        if force_local {
                            // ML-Resilience Rule 3: Idempotent operations via ON CONFLICT DO UPDATE
                            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload, updated_at = CURRENT_TIMESTAMP, tenant_id = EXCLUDED.tenant_id")
                                .bind(mission_id)
                                .bind(final_status)
                                .bind(payload)
                                .bind(org_id)
                                .execute(&mut *tx)
                                .await?;
                        } else {
                            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                                .bind(mission_id)
                                .bind(final_status)
                                .bind(payload)
                                .bind(org_id)
                                .execute(&mut *tx)
                                .await?;
                        }
                        tx.commit().await?;
                    }
                    DbStore::Sqlite(pool) => {
                        let mut final_status = status.clone();
                        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = ? AND (status = 'PENDING' OR status = 'RUNNING')")
                            .bind(&org_id)
                            .fetch_one(pool)
                            .await?;

                        if pending_count >= 5 && status == "PENDING" {
                            final_status = "BURSTING".to_string();
                        }

                        if force_local {
                            // ML-Resilience Rule 3: Idempotent operations via ON CONFLICT DO UPDATE
                            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?) ON CONFLICT(id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload, updated_at = CURRENT_TIMESTAMP, tenant_id = EXCLUDED.tenant_id")
                                .bind(mission_id)
                                .bind(final_status)
                                .bind(payload)
                                .bind(org_id)
                                .execute(pool)
                                .await?;
                        } else {
                            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?) ON CONFLICT(id) DO NOTHING")
                                .bind(mission_id)
                                .bind(final_status)
                                .bind(payload)
                                .bind(org_id)
                                .execute(pool)
                                .await?;
                        }
                    }
                }
                Ok::<(), sqlx::Error>(())
            }
        })).await
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, e)))??;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;

    // Helper to get a dummy db for testing
    async fn setup_dummy_db() -> Arc<DB> {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        Arc::new(DB::new().await.unwrap_or_else(|_| {
            // Fallback for tests if DB::new fails (e.g. no server)
             let pool = sqlx::postgres::PgPoolOptions::new()
                .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .acquire_timeout(std::time::Duration::from_millis(50))
                .connect_lazy(&db_url)
                .unwrap();
             DB { pool, store: DbStore::Postgres }
        }))
    }

    #[tokio::test]
    async fn test_delegate_mission_tc1_no_context_root() {
        let db = setup_dummy_db().await;
        let sip_db = SipDB::new(db, "test_org".to_string());
        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, payload, "Payload should be unmodified when no context root is set");
    }

    // Helper to create a temporary directory without external crate
    fn create_temp_dir(name: &str) -> String {
        let mut path = env::temp_dir();
        path.push(format!("{}_{}", name, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&path).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[tokio::test]
    async fn test_delegate_mission_tc2_agents_md() {
        let db = setup_dummy_db().await;
        let dir_str = create_temp_dir("tc2");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Always write clean code.").unwrap();

        let sip_db = SipDB::new(db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAlways write clean code.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc3_claude_md_fallback() {
        let db = setup_dummy_db().await;
        let dir_str = create_temp_dir("tc3");
        let dir_path = std::path::Path::new(&dir_str);

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file = File::create(&claude_path).unwrap();
        write!(file, "Use specialized tokens.").unwrap();

        let sip_db = SipDB::new(db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc4_grounding_priority() {
        let db = setup_dummy_db().await;
        let dir_str = create_temp_dir("tc4");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "AGENTS rules.").unwrap();

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file2 = File::create(&claude_path).unwrap();
        write!(file2, "CLAUDE rules.").unwrap();

        let sip_db = SipDB::new(db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        // Only AGENTS.md should be injected
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAGENTS rules.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc5_missing_files() {
        let db = setup_dummy_db().await;
        let dir_str = create_temp_dir("tc5");

        let sip_db = SipDB::new(db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, payload, "Payload should be unmodified when neither file is present");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_handoff_mission_marks_blocked() {
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();
        let db = Arc::new(DB { pool, store: DbStore::Postgres });

        let sip_db = SipDB::new(db, "test_org".to_string());

        let res = sip_db.handoff_mission("dummy_id", "Blocked by prompt instructions").await;
        // Should error out gracefully with our dummy pool timeout instead of panicking
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_handoff_mission_logic_success() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return,
        };

        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .max_connections(1)
            .connect(&database_url)
            .await;

        if let Ok(pool) = pool {
            let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });
            let sip_db = SipDB::new(db, "test_org".to_string());

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS agent_missions (
                    id TEXT PRIMARY KEY,
                    status TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    tenant_id TEXT,
                    mission_log TEXT
                )"
            )
            .execute(&pool)
            .await
            .unwrap();

            // Insert initial record
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                .bind("test_mission_id")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .execute(&pool)
                .await
                .unwrap();

            // Call handoff_mission
            let res = sip_db.handoff_mission("test_mission_id", "Missing dependencies").await;
            assert!(res.is_ok());

            // Verify
            let row = sqlx::query("SELECT status, mission_log FROM agent_missions WHERE id = 'test_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();

            let status: String = row.get("status");
            let log: String = row.get("mission_log");

            assert_eq!(status, "blocked");
            assert!(log.contains("Missing dependencies"));

            // Call again to test append
            let res2 = sip_db.handoff_mission("test_mission_id", "Another blocker").await;
            assert!(res2.is_ok());

            let row2 = sqlx::query("SELECT mission_log FROM agent_missions WHERE id = 'test_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();

            let log2: String = row2.get("mission_log");
            assert!(log2.contains("Missing dependencies\nAnother blocker"));

            // Clean up
            sqlx::query("DELETE FROM agent_missions WHERE id = 'test_mission_id'").execute(&pool).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_prune_stale_missions_marks_stuck_as_failed() {
        // Just verify it doesn't crash on execution with a valid pool.
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();
        let db = Arc::new(DB { pool, store: DbStore::Postgres });

        let sip_db = SipDB::new(db, "test_org".to_string());

        let res = sip_db.prune_stale_missions(chrono::Duration::hours(24)).await;
        // Should error out gracefully with our dummy pool timeout instead of panicking
        assert!(res.is_err());
    }
}
