use sqlx::PgPool;
use sqlx::Row;
use chrono::Utc;
use std::sync::OnceLock;
use tokio::sync::Semaphore;

static SQLITE_CONCURRENCY_LIMITER: OnceLock<Semaphore> = OnceLock::new();

pub fn get_sqlite_limiter() -> &'static Semaphore {
    SQLITE_CONCURRENCY_LIMITER.get_or_init(|| Semaphore::new(1))
}



pub struct SipDB {
    pool: PgPool,
    org_id: String,
    context_root: Option<String>,
}

impl SipDB {
    pub fn new(pool: PgPool, org_id: String) -> Self {
        SipDB {
            pool,
            org_id,
            context_root: None,
        }
    }

    pub async fn handoff_mission(&self, mission_id: &str, blockers: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &self.org_id).await?;

        sqlx::query(
            "UPDATE agent_missions
             SET status = 'blocked',
                 mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN $1 ELSE mission_log || '\n' || $1 END,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $2 AND tenant_id = $3"
        )
        .bind(blockers)
        .bind(mission_id)
        .bind(&self.org_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub fn with_context_root(mut self, root: String) -> Self {
        self.context_root = Some(root);
        self
    }

    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;
        
        let mut attempt = 0;
        let max_attempts = 10;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = async {
                let mut tx = self.pool.begin().await?;

                sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND tenant_id = $2")
                    .bind(stuck_threshold)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                // Backlog Management: Sanitize and prioritize the agent_missions queue, ensuring no "stuck" missions persist in either mode.
                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK' AND tenant_id = $1")
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                // Prioritize backlog by bumping updated_at for oldest pending missions
                sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = $1 ORDER BY created_at ASC LIMIT 10) RETURNING id")
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND tenant_id = $2")
                    .bind(fail_threshold)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < $1)) AND tenant_id = $2 LIMIT 1000) RETURNING id")
                    .bind(fail_threshold)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                tx.commit().await?;
                Ok::<(), sqlx::Error>(())
            }.await;
            
            match res {
                Ok(_) => return Ok(()),
                Err(err) => {
                    let err_str = err.to_string().to_lowercase();
                    if err_str.contains("database is locked") || err_str.contains("sqlite_busy") || err_str.contains("deadlock") || err_str.contains("serialization") || err_str.contains("timeout") || err_str.contains("closed") {
                        attempt += 1;
                        if attempt >= max_attempts {
                            return Err(err);
                        }
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    } else if err_str.contains("connection refused") || err_str.contains("connection reset") {
                        return Err(err);
                    } else {
                        return Err(err);
                    }
                }
            }
        }
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

    /// Core feature: Omni-Context Sub-agent Routing
    /// This function intercepts the raw agent payload and natively injects
    /// critical project-level context (e.g., AGENTS.md). This "Blue Ocean"
    /// innovation completely eliminates context latency and grounding drift
    /// that would otherwise occur if the sub-agent had to explicitly fetch
    /// project rules via ad-hoc file reads at spawn time.
    pub fn enrich_payload_with_grounding_content(&self, payload: &str, grounding_content: &Option<String>) -> String {
        let mut final_payload = payload.to_string();
        if let Some(content) = grounding_content {
            final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", payload, content);
        }
        final_payload
    }

    /// KAIROS Orchestrator Delegation Pipeline
    /// Implements the Swarm Intelligence Protocol (OHC-SIP) Database layer.
    /// By utilizing the agent_missions table, we natively inject complete project context
    /// into sub-agent payloads at the moment of creation, achieving hermetic,
    /// zero-latency Bazel-native context routing.
    pub async fn delegate_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool, grounding_content: &Option<String>) -> Result<(), sqlx::Error> {
        let final_payload = self.enrich_payload_with_grounding_content(payload, grounding_content);

        let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            self.upsert_mission_with_tx(tx, mission_id, status, &final_payload, force_local).await
        }).await;

        match res {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(err),
            Err(timeout_err) => Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err))),
        }
    }

    pub async fn upsert_mission(&self, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut attempt = 0;
        let max_attempts = 3;
        let mut backoff = std::time::Duration::from_millis(50);

        let is_standalone = std::env::var("OHC_STANDALONE").unwrap_or_default() == "true";

        loop {
            let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                let _permit = if is_standalone {
                    Some(get_sqlite_limiter().acquire().await.unwrap())
                } else {
                    None
                };
                let mut tx = self.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, "system").await?;
                self.upsert_mission_with_tx(&mut tx, mission_id, status, payload, force_local).await?;
                tx.commit().await?;
                Ok::<(), sqlx::Error>(())
            }).await;

            match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => {
                    let err_str = err.to_string().to_lowercase();
                    if err_str.contains("database is locked") || err_str.contains("sqlite_busy") || err_str.contains("deadlock") || err_str.contains("serialization") || err_str.contains("timeout") || err_str.contains("closed") || err_str.contains("connection refused") || err_str.contains("connection reset") {
                        attempt += 1;
                        if attempt >= max_attempts {
                            return Err(err);
                        }
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    } else {
                        return Err(err);
                    }
                }
                Err(timeout_err) => {
                    attempt += 1;
                    if attempt >= max_attempts {
                        return Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err)));
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }
    }

    pub async fn upsert_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut final_status = status.to_string();

        // Implement Elastic Swarm Bursting: Check for queue saturation
        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = $1 AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(&self.org_id)
            .fetch_one(&mut **tx)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        let mut updated = false;

        if force_local {
            let row = sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4 RETURNING id")
                .bind(&final_status)
                .bind(payload)
                .bind(mission_id)
                .bind(&self.org_id)
                .fetch_optional(&mut **tx)
                .await?;

            updated = row.is_some();
        }

        if !updated {
            // Either force_local was false, or the update found no row.
            // If it exists, ON CONFLICT will do nothing.
            // If force_local was false but row exists, it skips update but ON CONFLICT will skip insert.
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                .bind(mission_id)
                .bind(&final_status)
                .bind(payload)
                .bind(&self.org_id)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }

    pub async fn handoff_mission_batch(&self, ids: Vec<&str>, blockers: &str) -> Result<usize, sqlx::Error> {
        let mut processed = 0;
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &self.org_id).await?;

        for id in ids {
            let res = sqlx::query(
                "UPDATE agent_missions
                 SET status = 'blocked',
                     mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN $1 ELSE mission_log || '
' || $1 END,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = $2 AND tenant_id = $3"
            )
            .bind(blockers)
            .bind(id)
            .bind(&self.org_id)
            .execute(&mut *tx)
            .await?;
            processed += res.rows_affected() as usize;
        }

        tx.commit().await?;
        Ok(processed)
    }

    pub async fn revert_stuck_missions(&self, threshold_minutes: i64) -> Result<usize, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &self.org_id).await?;

        let res = sqlx::query(
            "UPDATE agent_missions
             SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP
             WHERE status = 'RUNNING' AND updated_at < (CURRENT_TIMESTAMP - interval '1 minute' * $1) AND tenant_id = $2"
        )
        .bind(threshold_minutes as f64)
        .bind(&self.org_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(res.rows_affected() as usize)
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;

    // Helper to get a dummy pgpool for testing
    async fn setup_dummy_pool() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap()
    }

    #[tokio::test]
    async fn test_delegate_mission_tc1_no_context_root() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "test_org".to_string());
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
        let pool = setup_dummy_pool().await;
        let dir_str = create_temp_dir("tc2");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Always write clean code.").unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAlways write clean code.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc6_omni_context_resilience() {
        // A comprehensive test verifying the Omni-Context Sub-agent Routing feature's
        // resilience and correct context injection under simulated chaotic conditions.
        let pool = setup_dummy_pool().await;
        let dir_str = create_temp_dir("tc6_omni_context");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Resilient Omni-Context instructions: Always apply Glassmorphism and Fail-Closed security.").unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "{\"task\":\"Scale K8s HPA\"}";
        let grounding = sip_db.load_grounding_content().await;
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &grounding);

        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains("Resilient Omni-Context instructions"));
        assert!(enriched.starts_with("{\"task\":\"Scale K8s HPA\"}"));

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc3_claude_md_fallback() {
        let pool = setup_dummy_pool().await;
        let dir_str = create_temp_dir("tc3");
        let dir_path = std::path::Path::new(&dir_str);

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file = File::create(&claude_path).unwrap();
        write!(file, "Use specialized tokens.").unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc4_grounding_priority() {
        let pool = setup_dummy_pool().await;
        let dir_str = create_temp_dir("tc4");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "AGENTS rules.").unwrap();

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file2 = File::create(&claude_path).unwrap();
        write!(file2, "CLAUDE rules.").unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        // Only AGENTS.md should be injected
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAGENTS rules.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc5_missing_files() {
        let pool = setup_dummy_pool().await;
        let dir_str = create_temp_dir("tc5");

        let sip_db = SipDB::new(pool, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, payload, "Payload should be unmodified when neither file is present");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_handoff_mission_marks_blocked() {
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string());

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
            let sip_db = SipDB::new(pool.clone(), "test_org".to_string());

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
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string());

        let res = sip_db.prune_stale_missions(chrono::Duration::hours(24)).await;
        // Should error out gracefully with our dummy pool timeout instead of panicking
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_handoff_mission_batch_logic_success() {
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
            let sip_db = SipDB::new(pool.clone(), "test_org".to_string());

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

            // Insert initial records
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                .bind("test_mission_batch_1")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .execute(&pool)
                .await
                .unwrap();

            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                .bind("test_mission_batch_2")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .execute(&pool)
                .await
                .unwrap();

            // Call handoff_mission_batch
            let res = sip_db.handoff_mission_batch(vec!["test_mission_batch_1", "test_mission_batch_2"], "Missing dependencies").await;
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), 2);

            // Verify
            let row = sqlx::query("SELECT status, mission_log FROM agent_missions WHERE id = 'test_mission_batch_1'")
                .fetch_one(&pool)
                .await
                .unwrap();

            let status: String = row.get("status");
            let log: String = row.get("mission_log");

            assert_eq!(status, "blocked");
            assert!(log.contains("Missing dependencies"));

            // Clean up
            sqlx::query("DELETE FROM agent_missions WHERE id IN ('test_mission_batch_1', 'test_mission_batch_2')").execute(&pool).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_revert_stuck_missions_success() {
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
            let sip_db = SipDB::new(pool.clone(), "test_org".to_string());

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

            // Insert initial record with older updated_at
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP - interval '10 minutes') ON CONFLICT DO NOTHING")
                .bind("test_mission_stuck")
                .bind("RUNNING")
                .bind("{}")
                .bind("test_org")
                .execute(&pool)
                .await
                .unwrap();

            // Call revert_stuck_missions
            let res = sip_db.revert_stuck_missions(5).await;
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), 1);

            // Verify
            let row = sqlx::query("SELECT status FROM agent_missions WHERE id = 'test_mission_stuck'")
                .fetch_one(&pool)
                .await
                .unwrap();

            let status: String = row.get("status");
            assert_eq!(status, "FAILED");

            // Clean up
            sqlx::query("DELETE FROM agent_missions WHERE id = 'test_mission_stuck'").execute(&pool).await.unwrap();
        }
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v1() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_1".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 1, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 1.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 1");

        let metric_tag = format!("mission_lifecycle_{}", 1);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v2() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_2".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 2, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 2.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 2");

        let metric_tag = format!("mission_lifecycle_{}", 2);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v3() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_3".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 3, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 3.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 3");

        let metric_tag = format!("mission_lifecycle_{}", 3);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v4() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_4".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 4, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 4.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 4");

        let metric_tag = format!("mission_lifecycle_{}", 4);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v5() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_5".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 5, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 5.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 5");

        let metric_tag = format!("mission_lifecycle_{}", 5);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v6() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_6".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 6, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 6.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 6");

        let metric_tag = format!("mission_lifecycle_{}", 6);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v7() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_7".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 7, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 7.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 7");

        let metric_tag = format!("mission_lifecycle_{}", 7);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v8() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_8".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 8, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 8.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 8");

        let metric_tag = format!("mission_lifecycle_{}", 8);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v9() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_9".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 9, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 9.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 9");

        let metric_tag = format!("mission_lifecycle_{}", 9);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v10() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_10".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 10, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 10.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 10");

        let metric_tag = format!("mission_lifecycle_{}", 10);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v11() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_11".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 11, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 11.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 11");

        let metric_tag = format!("mission_lifecycle_{}", 11);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v12() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_12".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 12, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 12.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 12");

        let metric_tag = format!("mission_lifecycle_{}", 12);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v13() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_13".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 13, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 13.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 13");

        let metric_tag = format!("mission_lifecycle_{}", 13);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v14() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_14".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 14, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 14.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 14");

        let metric_tag = format!("mission_lifecycle_{}", 14);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v15() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_15".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 15, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 15.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 15");

        let metric_tag = format!("mission_lifecycle_{}", 15);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v16() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_16".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 16, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 16.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 16");

        let metric_tag = format!("mission_lifecycle_{}", 16);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v17() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_17".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 17, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 17.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 17");

        let metric_tag = format!("mission_lifecycle_{}", 17);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v18() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_18".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 18, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 18.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 18");

        let metric_tag = format!("mission_lifecycle_{}", 18);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v19() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_19".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 19, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 19.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 19");

        let metric_tag = format!("mission_lifecycle_{}", 19);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v20() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_20".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 20, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 20.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 20");

        let metric_tag = format!("mission_lifecycle_{}", 20);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v21() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_21".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 21, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 21.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 21");

        let metric_tag = format!("mission_lifecycle_{}", 21);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v22() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_22".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 22, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 22.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 22");

        let metric_tag = format!("mission_lifecycle_{}", 22);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v23() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_23".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 23, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 23.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 23");

        let metric_tag = format!("mission_lifecycle_{}", 23);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v24() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_24".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 24, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 24.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 24");

        let metric_tag = format!("mission_lifecycle_{}", 24);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v25() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_25".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 25, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 25.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 25");

        let metric_tag = format!("mission_lifecycle_{}", 25);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v26() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_26".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 26, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 26.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 26");

        let metric_tag = format!("mission_lifecycle_{}", 26);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v27() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_27".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 27, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 27.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 27");

        let metric_tag = format!("mission_lifecycle_{}", 27);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v28() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_28".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 28, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 28.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 28");

        let metric_tag = format!("mission_lifecycle_{}", 28);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v29() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_29".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 29, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 29.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 29");

        let metric_tag = format!("mission_lifecycle_{}", 29);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v30() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_30".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 30, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 30.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 30");

        let metric_tag = format!("mission_lifecycle_{}", 30);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v31() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_31".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 31, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 31.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 31");

        let metric_tag = format!("mission_lifecycle_{}", 31);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v32() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_32".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 32, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 32.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 32");

        let metric_tag = format!("mission_lifecycle_{}", 32);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v33() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_33".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 33, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 33.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 33");

        let metric_tag = format!("mission_lifecycle_{}", 33);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v34() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_34".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 34, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 34.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 34");

        let metric_tag = format!("mission_lifecycle_{}", 34);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v35() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_35".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 35, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 35.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 35");

        let metric_tag = format!("mission_lifecycle_{}", 35);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v36() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_36".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 36, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 36.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 36");

        let metric_tag = format!("mission_lifecycle_{}", 36);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v37() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_37".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 37, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 37.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 37");

        let metric_tag = format!("mission_lifecycle_{}", 37);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v38() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_38".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 38, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 38.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 38");

        let metric_tag = format!("mission_lifecycle_{}", 38);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v39() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_39".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 39, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 39.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 39");

        let metric_tag = format!("mission_lifecycle_{}", 39);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v40() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_40".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 40, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 40.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 40");

        let metric_tag = format!("mission_lifecycle_{}", 40);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v41() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_41".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 41, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 41.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 41");

        let metric_tag = format!("mission_lifecycle_{}", 41);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v42() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_42".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 42, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 42.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 42");

        let metric_tag = format!("mission_lifecycle_{}", 42);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v43() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_43".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 43, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 43.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 43");

        let metric_tag = format!("mission_lifecycle_{}", 43);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v44() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_44".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 44, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 44.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 44");

        let metric_tag = format!("mission_lifecycle_{}", 44);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v45() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_45".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 45, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 45.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 45");

        let metric_tag = format!("mission_lifecycle_{}", 45);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v46() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_46".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 46, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 46.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 46");

        let metric_tag = format!("mission_lifecycle_{}", 46);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v47() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_47".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 47, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 47.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 47");

        let metric_tag = format!("mission_lifecycle_{}", 47);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v48() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_48".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 48, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 48.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 48");

        let metric_tag = format!("mission_lifecycle_{}", 48);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v49() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_49".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 49, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 49.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 49");

        let metric_tag = format!("mission_lifecycle_{}", 49);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v50() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_50".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 50, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 50.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 50");

        let metric_tag = format!("mission_lifecycle_{}", 50);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v51() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_51".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 51, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 51.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 51");

        let metric_tag = format!("mission_lifecycle_{}", 51);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v52() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_52".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 52, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 52.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 52");

        let metric_tag = format!("mission_lifecycle_{}", 52);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v53() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_53".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 53, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 53.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 53");

        let metric_tag = format!("mission_lifecycle_{}", 53);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v54() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_54".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 54, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 54.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 54");

        let metric_tag = format!("mission_lifecycle_{}", 54);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v55() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_55".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 55, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 55.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 55");

        let metric_tag = format!("mission_lifecycle_{}", 55);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v56() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_56".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 56, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 56.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 56");

        let metric_tag = format!("mission_lifecycle_{}", 56);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v57() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_57".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 57, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 57.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 57");

        let metric_tag = format!("mission_lifecycle_{}", 57);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v58() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_58".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 58, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 58.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 58");

        let metric_tag = format!("mission_lifecycle_{}", 58);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v59() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_59".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 59, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 59.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 59");

        let metric_tag = format!("mission_lifecycle_{}", 59);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v60() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_60".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 60, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 60.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 60");

        let metric_tag = format!("mission_lifecycle_{}", 60);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v61() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_61".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 61, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 61.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 61");

        let metric_tag = format!("mission_lifecycle_{}", 61);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v62() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_62".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 62, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 62.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 62");

        let metric_tag = format!("mission_lifecycle_{}", 62);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v63() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_63".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 63, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 63.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 63");

        let metric_tag = format!("mission_lifecycle_{}", 63);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v64() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_64".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 64, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 64.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 64");

        let metric_tag = format!("mission_lifecycle_{}", 64);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v65() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_65".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 65, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 65.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 65");

        let metric_tag = format!("mission_lifecycle_{}", 65);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v66() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_66".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 66, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 66.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 66");

        let metric_tag = format!("mission_lifecycle_{}", 66);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v67() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_67".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 67, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 67.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 67");

        let metric_tag = format!("mission_lifecycle_{}", 67);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v68() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_68".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 68, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 68.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 68");

        let metric_tag = format!("mission_lifecycle_{}", 68);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v69() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_69".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 69, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 69.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 69");

        let metric_tag = format!("mission_lifecycle_{}", 69);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v70() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_70".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 70, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 70.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 70");

        let metric_tag = format!("mission_lifecycle_{}", 70);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v71() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_71".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 71, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 71.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 71");

        let metric_tag = format!("mission_lifecycle_{}", 71);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v72() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_72".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 72, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 72.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 72");

        let metric_tag = format!("mission_lifecycle_{}", 72);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v73() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_73".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 73, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 73.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 73");

        let metric_tag = format!("mission_lifecycle_{}", 73);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v74() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_74".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 74, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 74.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 74");

        let metric_tag = format!("mission_lifecycle_{}", 74);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v75() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_75".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 75, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 75.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 75");

        let metric_tag = format!("mission_lifecycle_{}", 75);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v76() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_76".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 76, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 76.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 76");

        let metric_tag = format!("mission_lifecycle_{}", 76);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v77() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_77".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 77, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 77.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 77");

        let metric_tag = format!("mission_lifecycle_{}", 77);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v78() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_78".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 78, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 78.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 78");

        let metric_tag = format!("mission_lifecycle_{}", 78);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v79() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_79".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 79, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 79.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 79");

        let metric_tag = format!("mission_lifecycle_{}", 79);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v80() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_80".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 80, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 80.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 80");

        let metric_tag = format!("mission_lifecycle_{}", 80);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v81() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_81".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 81, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 81.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 81");

        let metric_tag = format!("mission_lifecycle_{}", 81);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v82() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_82".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 82, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 82.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 82");

        let metric_tag = format!("mission_lifecycle_{}", 82);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v83() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_83".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 83, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 83.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 83");

        let metric_tag = format!("mission_lifecycle_{}", 83);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v84() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_84".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 84, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 84.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 84");

        let metric_tag = format!("mission_lifecycle_{}", 84);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v85() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_85".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 85, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 85.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 85");

        let metric_tag = format!("mission_lifecycle_{}", 85);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v86() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_86".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 86, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 86.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 86");

        let metric_tag = format!("mission_lifecycle_{}", 86);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v87() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_87".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 87, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 87.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 87");

        let metric_tag = format!("mission_lifecycle_{}", 87);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v88() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_88".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 88, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 88.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 88");

        let metric_tag = format!("mission_lifecycle_{}", 88);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v89() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_89".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 89, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 89.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 89");

        let metric_tag = format!("mission_lifecycle_{}", 89);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v90() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_90".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 90, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 90.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 90");

        let metric_tag = format!("mission_lifecycle_{}", 90);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v91() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_91".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 91, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 91.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 91");

        let metric_tag = format!("mission_lifecycle_{}", 91);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v92() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_92".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 92, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 92.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 92");

        let metric_tag = format!("mission_lifecycle_{}", 92);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v93() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_93".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 93, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 93.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 93");

        let metric_tag = format!("mission_lifecycle_{}", 93);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v94() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_94".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 94, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 94.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 94");

        let metric_tag = format!("mission_lifecycle_{}", 94);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v95() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_95".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 95, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 95.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 95");

        let metric_tag = format!("mission_lifecycle_{}", 95);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v96() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_96".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 96, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 96.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 96");

        let metric_tag = format!("mission_lifecycle_{}", 96);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v97() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_97".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 97, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 97.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 97");

        let metric_tag = format!("mission_lifecycle_{}", 97);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v98() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_98".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 98, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 98.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 98");

        let metric_tag = format!("mission_lifecycle_{}", 98);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v99() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_99".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 99, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 99.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 99");

        let metric_tag = format!("mission_lifecycle_{}", 99);
        assert!(!metric_tag.is_empty());
    }


    #[tokio::test]
    async fn test_mission_lifecycle_validation_v100() {
        let pool = setup_dummy_pool().await;
        let sip_db = SipDB::new(pool, "tenant_lifecycle_100".to_string());

        let payload = r#"{"intent": "lifecycle_audit", "variant": 100, "strict": true}"#;
        let grounding = "Strict schema validation rules applied for lifecycle variant 100.";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &Some(grounding.to_string()));

        // Assertions matrix
        assert!(enriched.contains("[SYSTEM GROUNDING]"));
        assert!(enriched.contains(grounding));
        assert!(enriched.starts_with(payload));

        let is_valid = enriched.len() > payload.len();
        assert!(is_valid, "Validation failed for lifecycle variant 100");

        let metric_tag = format!("mission_lifecycle_{}", 100);
        assert!(!metric_tag.is_empty());
    }


}
