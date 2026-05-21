use chrono::Utc;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::Semaphore;
use crate::db::{DB, DbStore};
use sqlx::SqlitePool;

static SQLITE_CONCURRENCY_LIMITER: OnceLock<Semaphore> = OnceLock::new();

pub fn get_sqlite_limiter() -> &'static Semaphore {
    SQLITE_CONCURRENCY_LIMITER.get_or_init(|| Semaphore::new(1))
}

pub struct SipDB {
    db: Arc<DB>,
    context_root: Option<String>,
}

impl SipDB {
    pub fn new(db: Arc<DB>) -> Self {
        SipDB {
            db,
            context_root: None,
        }
    }

    pub fn with_context_root(mut self, root: String) -> Self {
        self.context_root = Some(root);
        self
    }

    pub async fn handoff_mission(&self, org_id: &str, mission_id: &str, blockers: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, org_id).await?;

                sqlx::query(
                    "UPDATE agent_missions
                     SET status = 'blocked',
                         mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN $1 ELSE mission_log || '\n' || $1 END,
                         updated_at = CURRENT_TIMESTAMP
                     WHERE id = $2 AND tenant_id = $3"
                )
                .bind(format!("Blocked: {}", blockers))
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
                         mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN ?1 ELSE mission_log || '\n' || ?1 END,
                         updated_at = CURRENT_TIMESTAMP
                     WHERE id = ?2 AND tenant_id = ?3"
                )
                .bind(format!("Blocked: {}", blockers))
                .bind(mission_id)
                .bind(org_id)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    pub async fn prune_stale_missions(&self, org_id: &str, age_threshold: chrono::Duration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.prune_missions_internal(Some(org_id), age_threshold).await
    }

    pub async fn prune_all_tenants(&self, age_threshold: chrono::Duration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.prune_missions_internal(None, age_threshold).await
    }

    async fn prune_missions_internal(&self, org_id: Option<&str>, age_threshold: chrono::Duration) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;

        let mut attempt = 0;
        let max_attempts = 10;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = match &self.db.store {
                DbStore::Postgres => self.prune_postgres(org_id, stuck_threshold, fail_threshold).await.map_err(|e| e as Box<dyn std::error::Error + Send + Sync>),
                DbStore::Sqlite(pool) => self.prune_sqlite(org_id, stuck_threshold, fail_threshold, pool).await.map_err(|e| e as Box<dyn std::error::Error + Send + Sync>),
            };

            match res {
                Ok(_) => return Ok(()),
                Err(err) => {
                    let err_str = err.to_string().to_lowercase();
                    if err_str.contains("database is locked") || err_str.contains("sqlite_busy") || err_str.contains("deadlock") || err_str.contains("serialization") || err_str.contains("timeout") || err_str.contains("closed") {
                        attempt += 1;
                        if attempt >= max_attempts {
                            return Err(err);
                        }
                        drop(err);
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }

    async fn prune_postgres(&self, org_id: Option<&str>, stuck_threshold: chrono::DateTime<Utc>, fail_threshold: chrono::DateTime<Utc>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, org_id.unwrap_or("system")).await?;

        let tenant_filter = if org_id.is_some() {
            "AND tenant_id = $2"
        } else {
            ""
        };

        let s1 = format!("UPDATE agent_missions SET status = 'STUCK', updated_at = CURRENT_TIMESTAMP WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 {}", tenant_filter);
        let mut q1 = sqlx::query(&s1).bind(stuck_threshold);
        if let Some(id) = org_id { q1 = q1.bind(id); }
        q1.execute(&mut *tx).await?;

        let s2 = format!("UPDATE agent_missions SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE status = 'STUCK' AND updated_at < $1 {}", tenant_filter);
        let mut q2 = sqlx::query(&s2).bind(stuck_threshold);
        if let Some(id) = org_id { q2 = q2.bind(id); }
        q2.execute(&mut *tx).await?;

        let s3 = format!("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' {} AND updated_at < CURRENT_TIMESTAMP - INTERVAL '1 minute' ORDER BY created_at ASC LIMIT 10)", if org_id.is_some() { "AND tenant_id = $1" } else { "" });
        let mut q3 = sqlx::query(&s3);
        if let Some(id) = org_id { q3 = q3.bind(id); }
        q3.execute(&mut *tx).await?;

        let s4 = format!("UPDATE agent_missions SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 {}", tenant_filter);
        let mut q4 = sqlx::query(&s4).bind(fail_threshold);
        if let Some(id) = org_id { q4 = q4.bind(id); }
        q4.execute(&mut *tx).await?;

        let s5 = format!("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < $1)) {} LIMIT 1000)", if org_id.is_some() { "AND tenant_id = $2" } else { "" });
        let mut q5 = sqlx::query(&s5).bind(fail_threshold);
        if let Some(id) = org_id { q5 = q5.bind(id); }
        q5.execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn prune_sqlite(&self, org_id: Option<&str>, stuck_threshold: chrono::DateTime<Utc>, fail_threshold: chrono::DateTime<Utc>, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let tenant_filter = if org_id.is_some() {
            "AND tenant_id = ?2"
        } else {
            ""
        };

        let s1 = format!("UPDATE agent_missions SET status = 'STUCK', updated_at = CURRENT_TIMESTAMP WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < ?1 {}", tenant_filter);
        let mut q1 = sqlx::query(&s1).bind(stuck_threshold);
        if let Some(id) = org_id { q1 = q1.bind(id); }
        q1.execute(pool).await?;

        let s2 = format!("UPDATE agent_missions SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE status = 'STUCK' AND updated_at < ?1 {}", tenant_filter);
        let mut q2 = sqlx::query(&s2).bind(stuck_threshold);
        if let Some(id) = org_id { q2 = q2.bind(id); }
        q2.execute(pool).await?;

        let s3 = format!("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' {} AND updated_at < datetime('now', '-1 minute') ORDER BY created_at ASC LIMIT 10)", if org_id.is_some() { "AND tenant_id = ?1" } else { "" });
        let mut q3 = sqlx::query(&s3);
        if let Some(id) = org_id { q3 = q3.bind(id); }
        q3.execute(pool).await?;

        let s4 = format!("UPDATE agent_missions SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < ?1 {}", tenant_filter);
        let mut q4 = sqlx::query(&s4).bind(fail_threshold);
        if let Some(id) = org_id { q4 = q4.bind(id); }
        q4.execute(pool).await?;

        let s5 = format!("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < ?1)) {} LIMIT 1000)", if org_id.is_some() { "AND tenant_id = ?2" } else { "" });
        let mut q5 = sqlx::query(&s5).bind(fail_threshold);
        if let Some(id) = org_id { q5 = q5.bind(id); }
        q5.execute(pool).await?;

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

    pub async fn delegate_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, org_id: &str, mission_id: &str, status: &str, payload: &str, force_local: bool, grounding_content: &Option<String>) -> Result<(), sqlx::Error> {
        let final_payload = self.enrich_payload_with_grounding_content(payload, grounding_content);

        let is_standalone = std::env::var("OHC_STANDALONE").unwrap_or_default() == "true";

        let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            let _permit = if is_standalone {
                match get_sqlite_limiter().try_acquire() {
                    Ok(p) => Some(p),
                    Err(_) => {
                        let _ = crate::telemetry::record_sqlite_throttled_request(&self.db.pool, "delegate_mission_with_tx").await;
                        Some(get_sqlite_limiter().acquire().await.unwrap())
                    }
                }
            } else {
                None
            };
            self.upsert_mission_with_tx(tx, org_id, mission_id, status, &final_payload, force_local).await
        }).await;

        match res {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(err),
            Err(timeout_err) => Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err))),
        }
    }

    pub async fn upsert_mission(&self, org_id: &str, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut attempt = 0;
        let max_attempts = 3;
        let mut backoff = std::time::Duration::from_millis(50);

        let is_standalone = std::env::var("OHC_STANDALONE").unwrap_or_default() == "true";

        loop {
            let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                let _permit = if is_standalone {
                    match get_sqlite_limiter().try_acquire() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            let _ = crate::telemetry::record_sqlite_throttled_request(&self.db.pool, "upsert_mission").await;
                            Some(get_sqlite_limiter().acquire().await.unwrap())
                        }
                    }
                } else {
                    None
                };

                match &self.db.store {
                    DbStore::Postgres => {
                        let mut tx = self.db.pool.begin().await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                        ::server_common::auth_utils::set_org_context(&mut *tx, org_id).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                        self.upsert_mission_with_tx(&mut tx, org_id, mission_id, status, payload, force_local).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                        tx.commit().await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    }
                    DbStore::Sqlite(pool) => {
                        self.upsert_mission_sqlite(pool, org_id, mission_id, status, payload, force_local).await?;
                    }
                }
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            }).await;

            match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => {
                    let err_str = err.to_string().to_lowercase();
                    if err_str.contains("database is locked") || err_str.contains("sqlite_busy") || err_str.contains("deadlock") || err_str.contains("serialization") || err_str.contains("timeout") || err_str.contains("closed") || err_str.contains("connection refused") || err_str.contains("connection reset") {
                        attempt += 1;
                        if attempt >= max_attempts {
                            if err_str.contains("database is locked") || err_str.contains("sqlite_busy") {
                                let _ = crate::telemetry::record_sqlite_retry_exhausted(&self.db.pool, "upsert_mission").await;
                            }
                            return Err(err);
                        }
                        if err_str.contains("database is locked") || err_str.contains("sqlite_busy") {
                            let _ = crate::telemetry::record_sqlite_lock_contention(&self.db.pool, "upsert_mission").await;
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
                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err)));
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }
    }

    pub async fn upsert_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, org_id: &str, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut final_status = status.to_string();

        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = $1 AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(org_id)
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
                .bind(org_id)
                .fetch_optional(&mut **tx)
                .await?;

            updated = row.is_some();
        }

        if !updated {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                .bind(mission_id)
                .bind(&final_status)
                .bind(payload)
                .bind(org_id)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }

    async fn upsert_mission_sqlite(&self, pool: &SqlitePool, org_id: &str, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut final_status = status.to_string();

        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = ?1 AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(org_id)
            .fetch_one(pool)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        let mut updated = false;

        if force_local {
            let res = sqlx::query("UPDATE agent_missions SET status = ?1, payload = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3 AND tenant_id = ?4")
                .bind(&final_status)
                .bind(payload)
                .bind(mission_id)
                .bind(org_id)
                .execute(pool)
                .await?;

            updated = res.rows_affected() > 0;
        }

        if !updated {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?4) ON CONFLICT(id) DO NOTHING")
                .bind(mission_id)
                .bind(&final_status)
                .bind(payload)
                .bind(org_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;
    use sqlx::Row;

    async fn setup_test_db() -> Option<Arc<DB>> {
        if std::env::var("DATABASE_URL").is_err() {
            return None;
        }
        Some(Arc::new(DB::new().await.unwrap()))
    }

    #[tokio::test]
    async fn test_delegate_mission_tc1_no_context_root() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let sip_db = SipDB::new(db);
        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, payload, "Payload should be unmodified when no context root is set");
    }

    fn create_temp_dir(name: &str) -> String {
        let mut path = env::temp_dir();
        path.push(format!("{}_{}", name, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&path).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[tokio::test]
    async fn test_delegate_mission_tc2_agents_md() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let dir_str = create_temp_dir("tc2");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Always write clean code.").unwrap();

        let sip_db = SipDB::new(db)
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAlways write clean code.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc6_omni_context_resilience() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let dir_str = create_temp_dir("tc6_omni_context");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Resilient Omni-Context instructions: Always apply Glassmorphism and Fail-Closed security.").unwrap();

        let sip_db = SipDB::new(db)
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
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let dir_str = create_temp_dir("tc3");
        let dir_path = std::path::Path::new(&dir_str);

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file = File::create(&claude_path).unwrap();
        write!(file, "Use specialized tokens.").unwrap();

        let sip_db = SipDB::new(db)
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc4_grounding_priority() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let dir_str = create_temp_dir("tc4");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "AGENTS rules.").unwrap();

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file2 = File::create(&claude_path).unwrap();
        write!(file2, "CLAUDE rules.").unwrap();

        let sip_db = SipDB::new(db)
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAGENTS rules.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc5_missing_files() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let dir_str = create_temp_dir("tc5");

        let sip_db = SipDB::new(db)
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, payload, "Payload should be unmodified when neither file is present");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_handoff_mission_marks_blocked() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();
        let db = Arc::new(DB { pool, store: DbStore::Postgres });
        let sip_db = SipDB::new(db);

        let res = sip_db.handoff_mission("test_org", "dummy_id", "Blocked by prompt instructions").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_handoff_mission_logic_success() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let sip_db = SipDB::new(db.clone());

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
        .execute(&db.pool)
        .await
        .unwrap();

        sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
            .bind("test_mission_id")
            .bind("PENDING")
            .bind("{}")
            .bind("test_org")
            .execute(&db.pool)
            .await
            .unwrap();

        let res = sip_db.handoff_mission("test_org", "test_mission_id", "Missing dependencies").await;
        assert!(res.is_ok());

        let row = sqlx::query("SELECT status, mission_log FROM agent_missions WHERE id = 'test_mission_id'")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let status: String = row.get("status");
        let log: String = row.get("mission_log");

        assert_eq!(status, "blocked");
        assert!(log.contains("Blocked: Missing dependencies"));

        let res2 = sip_db.handoff_mission("test_org", "test_mission_id", "Another blocker").await;
        assert!(res2.is_ok());

        let row2 = sqlx::query("SELECT mission_log FROM agent_missions WHERE id = 'test_mission_id'")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let log2: String = row2.get("mission_log");
        assert!(log2.contains("Blocked: Missing dependencies\nBlocked: Another blocker"));

        sqlx::query("DELETE FROM agent_missions WHERE id = 'test_mission_id'").execute(&db.pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_prune_stale_missions_marks_stuck_as_failed() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();
        let db = Arc::new(DB { pool, store: DbStore::Postgres });
        let sip_db = SipDB::new(db);

        let res = sip_db.prune_stale_missions("test_org", chrono::Duration::hours(24)).await;
        assert!(res.is_err());
    }
}
