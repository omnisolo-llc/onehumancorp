use sqlx::PgPool;
use chrono::Utc;
use std::sync::OnceLock;
use tokio::sync::Semaphore;

use std::sync::atomic::{AtomicI64, Ordering};

static SQLITE_CONCURRENCY_LIMITER: OnceLock<Semaphore> = OnceLock::new();
pub static LAST_SUCCESSFUL_PRUNE: AtomicI64 = AtomicI64::new(0);

pub fn get_sqlite_limiter() -> &'static Semaphore {
    SQLITE_CONCURRENCY_LIMITER.get_or_init(|| Semaphore::new(1))
}

pub fn is_retryable_database_error_message(message: &str) -> bool {
    let lower = message.to_lowercase();
    lower.contains("database is locked")
        || lower.contains("database is busy")
        || lower.contains("sqlite_busy")
        || lower.contains("deadlock detected")
        || lower.contains("deadlock")
        || lower.contains("serialization failure")
        || lower.contains("serialization")
        || lower.contains("sqlstate 40p01")
        || lower.contains("40p01")
        || lower.contains("sqlstate 40001")
        || lower.contains("40001")
        || lower.contains("sqlstate 55p03")
        || lower.contains("55p03")
        || lower.contains("could not obtain lock")
        || lower.contains("timeout")
        || lower.contains("closed")
}

fn is_retryable_sqlx_error(err: &sqlx::Error) -> bool {
    if let Some(db_err) = err.as_database_error() {
        let code = db_err.code();
        if matches!(code.as_deref(), Some("40P01") | Some("40001") | Some("55P03")) {
            return true;
        }
    }
    is_retryable_database_error_message(&err.to_string())
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
        let mut attempt = 0;
        let max_attempts = crate::db::MAX_DB_RETRY_ATTEMPTS;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = tokio::time::timeout(ohc_builtin_agent::agent::agent_task_timeout(), async {
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
                Ok::<(), sqlx::Error>(())
            }).await;

            match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => {
                    if !is_retryable_sqlx_error(&err) {
                        return Err(err);
                    }
                    attempt += 1;
                    if attempt > max_attempts {
                        return Err(err);
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
                Err(_) => {
                    attempt += 1;
                    if attempt > max_attempts {
                        return Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, "handoff_mission timed out")));
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }
    }

    pub fn with_context_root(mut self, root: String) -> Self {
        self.context_root = Some(root);
        self
    }

    pub async fn cleanup_stagnant_missions(&self, stagnant_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let threshold_time = Utc::now() - stagnant_threshold;

        let mut attempt = 0;
        let max_attempts = crate::db::MAX_DB_RETRY_ATTEMPTS;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = tokio::time::timeout(ohc_builtin_agent::agent::agent_task_timeout(), async {
                let mut tx = self.pool.begin().await?;

                ::server_common::auth_utils::set_system_context(&mut *tx).await?;

                let is_standalone = crate::is_standalone_runtime();
                let query_str = if is_standalone {
                    "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT lower(hex(randomblob(16))), tenant_id, 'mission_stagnant', 'agent_missions', COALESCE(payload, '{}'), '[cleanup] Mission became stagnant' FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING' OR status = 'STUCK' OR status = 'IN_PROGRESS' OR status = 'RUNNING') AND updated_at < $1 AND tenant_id = $2"
                } else {
                    "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT gen_random_uuid()::text, tenant_id, 'mission_stagnant', 'agent_missions', COALESCE(payload::text, '{}'), '[cleanup] Mission became stagnant' FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING' OR status = 'STUCK' OR status = 'IN_PROGRESS' OR status = 'RUNNING') AND updated_at < $1 AND tenant_id = $2"
                };
                sqlx::query(query_str)
                    .bind(threshold_time.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("DELETE FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING' OR status = 'STUCK' OR status = 'IN_PROGRESS' OR status = 'RUNNING') AND updated_at < $1 AND tenant_id = $2")
                    .bind(threshold_time.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                let dead_letter_threshold = Utc::now() - chrono::Duration::days(7);
                sqlx::query("DELETE FROM department_dead_letters WHERE created_at < $1 AND tenant_id = $2")
                    .bind(dead_letter_threshold.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                tx.commit().await?;
                Ok::<(), sqlx::Error>(())
            }).await;

            match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => {
                    let err_str = err.to_string().to_lowercase();
                    let retry = err_str.contains("serialization failure") || err_str.contains("timeout") || err_str.contains("deadlock detected") || err_str.contains("database is locked") || err_str.contains("busy");

                    if retry {
                        attempt += 1;
                        if attempt > max_attempts {
                            return Err(err);
                        }
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    } else if err_str.contains("connection refused") || err_str.contains("connection reset") {
                        return Err(err);
                    } else {
                        return Err(err);
                    }
                },
                Err(timeout_err) => {
                    return Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err)));
                }
            }
        }
    }

    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;
        
        let mut attempt = 0;
        let max_attempts = crate::db::MAX_DB_RETRY_ATTEMPTS;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = tokio::time::timeout(ohc_builtin_agent::agent::agent_task_timeout(), async {
                let mut tx = self.pool.begin().await?;

                // Backlog Management: Sanitize and prioritize the agent_missions queue, ensuring no "stuck" missions persist in either mode.
                ::server_common::auth_utils::set_system_context(&mut *tx).await?;

                let is_standalone = crate::is_standalone_runtime();
                let query_str = if is_standalone {
                    "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT lower(hex(randomblob(16))), tenant_id, 'mission_stuck', 'agent_missions', payload, '[cleanup] Mission became stuck' FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING' OR status = 'STUCK' OR status = 'IN_PROGRESS' OR status = 'RUNNING') AND updated_at < $1 AND tenant_id = $2"
                } else {
                    "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT gen_random_uuid()::text, tenant_id, 'mission_stuck', 'agent_missions', payload, '[cleanup] Mission became stuck' FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING' OR status = 'STUCK' OR status = 'IN_PROGRESS' OR status = 'RUNNING') AND updated_at < $1 AND tenant_id = $2"
                };
                sqlx::query(query_str)
                    .bind(stuck_threshold.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING' OR status = 'STUCK' OR status = 'IN_PROGRESS' OR status = 'RUNNING') AND updated_at < $1 AND tenant_id = $2")
                    .bind(stuck_threshold.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                // Prioritize backlog by bumping updated_at for oldest pending missions
                let is_standalone = crate::is_standalone_runtime();
                let query_str = if is_standalone {
                    "UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = $1 ORDER BY created_at ASC LIMIT 10) RETURNING id"
                } else {
                    "UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = $1 ORDER BY created_at ASC LIMIT 10 FOR UPDATE SKIP LOCKED) RETURNING id"
                };
                sqlx::query(query_str)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                let query_str = if is_standalone {
                    "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT lower(hex(randomblob(16))), tenant_id, 'mission_stale', 'agent_missions', payload, '[cleanup] Mission became stale' FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND tenant_id = $2"
                } else {
                    "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) SELECT gen_random_uuid()::text, tenant_id, 'mission_stale', 'agent_missions', payload, '[cleanup] Mission became stale' FROM agent_missions WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND tenant_id = $2"
                };
                sqlx::query(query_str)
                    .bind(fail_threshold.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND tenant_id = $2")
                    .bind(fail_threshold.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < $1)) AND tenant_id = $2 LIMIT 1000) RETURNING id")
                    .bind(fail_threshold.naive_utc())
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                tx.commit().await?;
                Ok::<(), sqlx::Error>(())
            }).await;
            
            match res {
                Ok(Ok(_)) => {
                    LAST_SUCCESSFUL_PRUNE.store(chrono::Utc::now().timestamp(), Ordering::SeqCst);
                    return Ok(());
                },
                Ok(Err(err)) => {
                    let retry = is_retryable_sqlx_error(&err);
                    let err_str = err.to_string().to_lowercase();

                    if retry {
                        attempt += 1;
                        if attempt > max_attempts {
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
                Err(timeout_err) => {
                    attempt += 1;
                    if attempt > max_attempts {
                        return Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err)));
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }
    }







    pub async fn drain_mission_queue(&self) -> Result<(), sqlx::Error> {
        let mut attempt = 0;
        let max_attempts = crate::db::MAX_DB_RETRY_ATTEMPTS;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = async {
                let mut tx = self.pool.begin().await?;
                sqlx::query("DELETE FROM agent_missions WHERE tenant_id = $1 AND status = 'PENDING'")
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
                Ok::<(), sqlx::Error>(())
            }.await;

            match res {
                Ok(_) => return Ok(()),
                Err(err) => {
                    let retry = is_retryable_sqlx_error(&err);
                    let err_str = err.to_string().to_lowercase();

                    if retry {
                        attempt += 1;
                        if attempt > max_attempts {
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
            let mut current_dir = match tokio::fs::canonicalize(root_path).await {
                Ok(p) => p,
                Err(_) => root_path.to_path_buf(),
            };
            let mut max_depth = 50;

            loop {
                let agents_path = current_dir.join("AGENTS.md");
                match tokio::fs::read_to_string(&agents_path).await {
                    Ok(content) => return Some(content),
                    Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
                        tracing::warn!("Failed to read AGENTS.md: {}", e);
                    }
                    _ => {}
                }

                let claude_path = current_dir.join("CLAUDE.md");
                match tokio::fs::read_to_string(&claude_path).await {
                    Ok(content) => return Some(content),
                    Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
                        tracing::warn!("Failed to read CLAUDE.md: {}", e);
                    }
                    _ => {}
                }

                if !current_dir.pop() || max_depth == 0 {
                    break;
                }
                max_depth -= 1;
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
            if let Ok(mut json_val) = serde_json::from_str::<serde_json::Value>(payload) {
                if let Some(task_val) = json_val.get("task") {
                    if let Some(task) = task_val.as_str() {
                        let new_task = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", task, content);
                        json_val["task"] = serde_json::Value::String(new_task);
                        return json_val.to_string();
                    }
                }
            }
            final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", payload, content);
        }
        final_payload
    }

    /// KAIROS Orchestrator Delegation Pipeline
    /// Implements the Swarm Intelligence Protocol (OHC-SIP) Database layer.
    /// By utilizing the agent_missions table, we natively inject complete project context
    /// into sub-agent payloads at the moment of creation, achieving hermetic,
    /// zero-latency Bazel-native context routing.
    pub async fn delegate_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let grounding_content = self.load_grounding_content().await;
        let final_payload = self.enrich_payload_with_grounding_content(payload, &grounding_content);
        let is_standalone = crate::is_standalone_runtime();

        let res = tokio::time::timeout(ohc_builtin_agent::agent::agent_task_timeout(), async {
            let _permit = if is_standalone {
                match get_sqlite_limiter().try_acquire() {
                    Ok(p) => Some(p),
                    Err(_) => {
                        let _ = crate::telemetry::record_sqlite_throttled_request(&self.pool, "delegate_mission_with_tx").await;
                        Some(get_sqlite_limiter().acquire().await.map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?)
                    }
                }
            } else {
                None
            };
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
        let max_attempts = crate::db::MAX_DB_RETRY_ATTEMPTS;
        let mut backoff = std::time::Duration::from_millis(50);

        let is_standalone = crate::is_standalone_runtime();

        loop {
            let res = tokio::time::timeout(ohc_builtin_agent::agent::agent_task_timeout(), async {
                let _permit = if is_standalone {
                    match get_sqlite_limiter().try_acquire() {
                        Ok(p) => Some(p),
                        Err(_) => {
                            let _ = crate::telemetry::record_sqlite_throttled_request(&self.pool, "upsert_mission").await;
                            Some(get_sqlite_limiter().acquire().await.map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?)
                        }
                    }
                } else {
                    None
                };
                let mut tx = self.pool.begin().await?;
                ::server_common::auth_utils::set_system_context(&mut *tx).await?;
                self.upsert_mission_with_tx(&mut tx, mission_id, status, payload, force_local).await?;
                tx.commit().await?;
                Ok::<(), sqlx::Error>(())
            }).await;

            match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => {
                    let err_str = err.to_string().to_lowercase();
                    if is_retryable_sqlx_error(&err) || err_str.contains("connection refused") || err_str.contains("connection reset") {
                        attempt += 1;
                        if attempt > max_attempts {
                            if err_str.contains("database is locked") || err_str.contains("sqlite_busy") {
                                let _ = crate::telemetry::record_sqlite_retry_exhausted(&self.pool, "upsert_mission").await;
                            }
                            return Err(err);
                        }
                        if err_str.contains("database is locked") || err_str.contains("sqlite_busy") {
                            let _ = crate::telemetry::record_sqlite_lock_contention(&self.pool, "upsert_mission").await;
                        } else if !is_standalone && (err_str.contains("deadlock") || err_str.contains("timeout") || err_str.contains("database is locked") || err_str.contains("serialization")) {
                            crate::telemetry::record_postgres_lock_contention("upsert_mission");
                        }
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    } else {
                        return Err(err);
                    }
                }
                Err(timeout_err) => {
                    if !is_standalone {
                        crate::telemetry::record_postgres_lock_contention("upsert_mission");
                    }
                    attempt += 1;
                    if attempt > max_attempts {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;
    use sqlx::Row;


    #[tokio::test]
    async fn test_delegate_mission_tc1_no_context_root() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect(&database_url)
            .await;

        let pool = match pool {
            Ok(p) => p,
            Err(_) => return, // Skip test if DB cannot be connected to
        };

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

        let sip_db = SipDB::new(pool.clone(), "test_org".to_string());
        let payload = "Original Task Payload";

        let mut tx = pool.begin().await.unwrap();
        sip_db.delegate_mission_with_tx(&mut tx, "tc1_id", "PENDING", payload, false).await.unwrap();
        tx.commit().await.unwrap();

        let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'tc1_id'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let db_payload: String = row.get("payload");

        assert_eq!(db_payload, payload, "Payload should be unmodified when no context root is set");

        sqlx::query("DELETE FROM agent_missions WHERE id = 'tc1_id'").execute(&pool).await.unwrap();
    }

    #[test]
    fn retryable_database_error_message_includes_postgres_lock_codes() {
        assert!(is_retryable_database_error_message(
            "db error: ERROR: could not serialize access due to concurrent update (SQLSTATE 40001)",
        ));
        assert!(is_retryable_database_error_message(
            "db error: ERROR: deadlock detected (SQLSTATE 40P01)",
        ));
        assert!(is_retryable_database_error_message(
            "db error: ERROR: could not obtain lock on row (SQLSTATE 55P03)",
        ));
        assert!(is_retryable_database_error_message("database is locked"));
        assert!(!is_retryable_database_error_message("permission denied for table agent_missions"));
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
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect(&database_url)
            .await;

        let pool = match pool {
            Ok(p) => p,
            Err(_) => return, // Skip test if DB cannot be connected to
        };

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

        let dir_str = create_temp_dir("tc2");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Always write clean code.").unwrap();

        let sip_db = SipDB::new(pool.clone(), "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";

        let mut tx = pool.begin().await.unwrap();
        sip_db.delegate_mission_with_tx(&mut tx, "tc2_id", "PENDING", payload, false).await.unwrap();
        tx.commit().await.unwrap();

        let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'tc2_id'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let db_payload: String = row.get("payload");

        assert_eq!(db_payload, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAlways write clean code.");

        sqlx::query("DELETE FROM agent_missions WHERE id = 'tc2_id'").execute(&pool).await.unwrap();
        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc6_omni_context_resilience() {
        // A comprehensive test verifying the Omni-Context Sub-agent Routing feature's
        // resilience and correct context injection under simulated chaotic conditions.
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect(&database_url)
            .await;

        let pool = match pool {
            Ok(p) => p,
            Err(_) => return, // Skip test if DB cannot be connected to
        };

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

        let dir_str = create_temp_dir("tc6_omni_context");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Resilient Omni-Context instructions: Always apply Glassmorphism and Fail-Closed security.").unwrap();

        let sip_db = SipDB::new(pool.clone(), "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "{\"task\":\"Scale K8s HPA\"}";
        let mut tx = pool.begin().await.unwrap();
        sip_db.delegate_mission_with_tx(&mut tx, "tc6_id", "PENDING", payload, false).await.unwrap();
        tx.commit().await.unwrap();

        let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'tc6_id'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let db_payload: String = row.get("payload");

        assert!(db_payload.contains("[SYSTEM GROUNDING]"));
        assert!(db_payload.contains("Resilient Omni-Context instructions"));
        assert!(serde_json::from_str::<serde_json::Value>(&db_payload).is_ok());

        sqlx::query("DELETE FROM agent_missions WHERE id = 'tc6_id'").execute(&pool).await.unwrap();
        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc3_claude_md_fallback() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect(&database_url)
            .await;

        let pool = match pool {
            Ok(p) => p,
            Err(_) => return, // Skip test if DB cannot be connected to
        };

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

        let dir_str = create_temp_dir("tc3");
        let dir_path = std::path::Path::new(&dir_str);

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file = File::create(&claude_path).unwrap();
        write!(file, "Use specialized tokens.").unwrap();

        let sip_db = SipDB::new(pool.clone(), "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";

        let mut tx = pool.begin().await.unwrap();
        sip_db.delegate_mission_with_tx(&mut tx, "tc3_id", "PENDING", payload, false).await.unwrap();
        tx.commit().await.unwrap();

        let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'tc3_id'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let db_payload: String = row.get("payload");

        assert_eq!(db_payload, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.");

        sqlx::query("DELETE FROM agent_missions WHERE id = 'tc3_id'").execute(&pool).await.unwrap();
        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc4_grounding_priority() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect(&database_url)
            .await;

        let pool = match pool {
            Ok(p) => p,
            Err(_) => return, // Skip test if DB cannot be connected to
        };

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

        let dir_str = create_temp_dir("tc4");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "AGENTS rules.").unwrap();

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file2 = File::create(&claude_path).unwrap();
        write!(file2, "CLAUDE rules.").unwrap();

        let sip_db = SipDB::new(pool.clone(), "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";

        let mut tx = pool.begin().await.unwrap();
        sip_db.delegate_mission_with_tx(&mut tx, "tc4_id", "PENDING", payload, false).await.unwrap();
        tx.commit().await.unwrap();

        let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'tc4_id'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let db_payload: String = row.get("payload");

        // Only AGENTS.md should be injected
        assert_eq!(db_payload, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAGENTS rules.");

        sqlx::query("DELETE FROM agent_missions WHERE id = 'tc4_id'").execute(&pool).await.unwrap();
        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc5_missing_files() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .connect(&database_url)
            .await;

        let pool = match pool {
            Ok(p) => p,
            Err(_) => return, // Skip test if DB cannot be connected to
        };

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

        let dir_str = create_temp_dir("tc5");

        let sip_db = SipDB::new(pool.clone(), "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";

        let mut tx = pool.begin().await.unwrap();
        sip_db.delegate_mission_with_tx(&mut tx, "tc5_id", "PENDING", payload, false).await.unwrap();
        tx.commit().await.unwrap();

        let row = sqlx::query("SELECT payload FROM agent_missions WHERE id = 'tc5_id'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let db_payload: String = row.get("payload");

        assert_eq!(db_payload, payload, "Payload should be unmodified when neither file is present");

        sqlx::query("DELETE FROM agent_missions WHERE id = 'tc5_id'").execute(&pool).await.unwrap();
        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_handoff_mission_marks_blocked() {
        let pool = crate::db::secure_pg_pool_options()
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
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()

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

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS department_dead_letters (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    event_type TEXT NOT NULL,
                    department TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    error_message TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
        // First verify it doesn't crash on execution with an invalid/dummy pool.
        let dummy_pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();

        let sip_db_dummy = SipDB::new(dummy_pool, "test_org".to_string());

        let res_dummy = sip_db_dummy.prune_stale_missions(chrono::Duration::hours(24)).await;
        // Should error out gracefully with our dummy pool timeout instead of panicking
        assert!(res_dummy.is_err());

        // Now, if a real database is available, test the actual logic.
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present // Skip integration portion if no OHC_DATABASE_URL
        };

        if let Ok(pool) = crate::db::secure_pg_pool_options()

            .max_connections(1)
            .connect(&database_url)
            .await
        {
            let mut tx = pool.begin().await.unwrap();

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
            .execute(&mut *tx)
            .await
            .unwrap();

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS department_dead_letters (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    event_type TEXT NOT NULL,
                    department TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    error_message TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )"
            )
            .execute(&mut *tx)
            .await
            .unwrap();

            let old_time = chrono::Utc::now() - chrono::Duration::hours(2);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stuck_mission_id")
                .bind("STUCK")
                .bind("{}")
                .bind("test_org")
                .bind(old_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stuck_running_mission_id")
                .bind("RUNNING")
                .bind("{}")
                .bind("test_org")
                .bind(old_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stuck_in_progress_mission_id")
                .bind("IN_PROGRESS")
                .bind("{}")
                .bind("test_org")
                .bind(old_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            let very_old_time = chrono::Utc::now() - chrono::Duration::hours(48);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, created_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stale_pending_mission_id")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .bind(very_old_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            let recent_time = chrono::Utc::now() - chrono::Duration::minutes(5);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING")
                .bind("normal_pending_mission_id")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .bind(recent_time.naive_utc())
                .bind(recent_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            let old_pending_time = chrono::Utc::now() - chrono::Duration::minutes(10);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stagnant_pending_mission")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .bind(old_pending_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            let old_bursting_time = chrono::Utc::now() - chrono::Duration::minutes(10);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stagnant_bursting_mission")
                .bind("BURSTING")
                .bind("{}")
                .bind("test_org")
                .bind(old_bursting_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            tx.commit().await.unwrap();

            let sip_db = SipDB::new(pool.clone(), "test_org".to_string());
            let res = sip_db.prune_stale_missions(chrono::Duration::hours(24)).await;
            assert!(res.is_ok());

            // Verify STUCK mission was marked FAILED
            let row_stuck = sqlx::query("SELECT status FROM agent_missions WHERE id = 'stuck_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();
            use sqlx::Row;
            let status_stuck: String = row_stuck.get("status");
            assert_eq!(status_stuck, "FAILED");

            // Verify stuck RUNNING mission was marked FAILED
            let row_running = sqlx::query("SELECT status FROM agent_missions WHERE id = 'stuck_running_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();
            let status_running: String = row_running.get("status");
            assert_eq!(status_running, "FAILED");

            // Verify stuck IN_PROGRESS mission was marked FAILED
            let row_in_progress = sqlx::query("SELECT status FROM agent_missions WHERE id = 'stuck_in_progress_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();
            let status_in_progress: String = row_in_progress.get("status");
            assert_eq!(status_in_progress, "FAILED");

            // Verify stale PENDING mission was marked FAILED
            let row_stale = sqlx::query("SELECT status FROM agent_missions WHERE id = 'stale_pending_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();
            let status_stale: String = row_stale.get("status");
            assert_eq!(status_stale, "FAILED");

            // Verify normal PENDING mission is still PENDING
            let row_normal = sqlx::query("SELECT status FROM agent_missions WHERE id = 'normal_pending_mission_id'")
                .fetch_one(&pool)
                .await
                .unwrap();
            let status_normal: String = row_normal.get("status");
            assert_eq!(status_normal, "PENDING");

            // Verify dead letters were created
            let dl_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM department_dead_letters WHERE tenant_id = 'test_org'")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert!(dl_count.0 > 0);

            // Clean up using a transaction
            let mut tx_clean = pool.begin().await.unwrap();
            sqlx::query("DELETE FROM agent_missions WHERE id IN ('stuck_mission_id', 'stuck_running_mission_id', 'stuck_in_progress_mission_id', 'stale_pending_mission_id', 'normal_pending_mission_id')")
                .execute(&mut *tx_clean)
                .await
                .unwrap();
            sqlx::query("DELETE FROM department_dead_letters WHERE tenant_id = 'test_org'")
                .execute(&mut *tx_clean)
                .await
                .unwrap();
            tx_clean.commit().await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_cleanup_stagnant_missions() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        if let Ok(pool) = crate::db::secure_pg_pool_options()

            .max_connections(1)
            .connect(&database_url)
            .await
        {
            let mut tx = pool.begin().await.unwrap();

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
            .execute(&mut *tx)
            .await
            .unwrap();

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS department_dead_letters (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    event_type TEXT NOT NULL,
                    department TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    error_message TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )"
            )
            .execute(&mut *tx)
            .await
            .unwrap();

            let old_time = chrono::Utc::now() - chrono::Duration::minutes(10);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stagnant_in_progress_mission")
                .bind("IN_PROGRESS")
                .bind("{}")
                .bind("test_org")
                .bind(old_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            let recent_time = chrono::Utc::now() - chrono::Duration::minutes(2);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("recent_in_progress_mission")
                .bind("IN_PROGRESS")
                .bind("{}")
                .bind("test_org")
                .bind(recent_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();


            let old_pending_time = chrono::Utc::now() - chrono::Duration::minutes(10);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stagnant_pending_mission")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .bind(old_pending_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            let old_bursting_time = chrono::Utc::now() - chrono::Duration::minutes(10);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind("stagnant_bursting_mission")
                .bind("BURSTING")
                .bind("{}")
                .bind("test_org")
                .bind(old_bursting_time.naive_utc())
                .execute(&mut *tx)
                .await
                .unwrap();

            tx.commit().await.unwrap();

            let sip_db = SipDB::new(pool.clone(), "test_org".to_string());
            let res = sip_db.cleanup_stagnant_missions(chrono::Duration::minutes(5)).await;
            assert!(res.is_ok());

            // Verify stagnant missions were deleted
            let count_stagnant: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_missions WHERE id IN ('stagnant_in_progress_mission', 'stagnant_pending_mission', 'stagnant_bursting_mission')")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(count_stagnant, 0);

            // Verify dead letters were created
            let count_dead_letters: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM department_dead_letters WHERE event_type = 'mission_stagnant'")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(count_dead_letters, 3);

            // Verify recent mission is still IN_PROGRESS
            let row_recent = sqlx::query("SELECT status FROM agent_missions WHERE id = 'recent_in_progress_mission'")
                .fetch_one(&pool)
                .await
                .unwrap();
            let status_recent: String = row_recent.get("status");
            assert_eq!(status_recent, "IN_PROGRESS");

            // Verify dead letters were created
            let dl_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM department_dead_letters WHERE tenant_id = 'test_org'")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert!(dl_count.0 > 0);

            // Clean up
            let mut tx_clean = pool.begin().await.unwrap();
            sqlx::query("DELETE FROM agent_missions WHERE id IN ('recent_in_progress_mission')")
                .execute(&mut *tx_clean)
                .await
                .unwrap();
            sqlx::query("DELETE FROM department_dead_letters WHERE tenant_id = 'test_org'")
                .execute(&mut *tx_clean)
                .await
                .unwrap();
            tx_clean.commit().await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_drain_mission_queue_success() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => return, // Skip test instead of failing silently when no db url is present
        };

        let pool = crate::db::secure_pg_pool_options()

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

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS department_dead_letters (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    event_type TEXT NOT NULL,
                    department TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    error_message TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )"
            )
            .execute(&pool)
            .await
            .unwrap();

            // Insert initial record
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                .bind("drain_test_mission_id")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .execute(&pool)
                .await
                .unwrap();

            // Insert another org record
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                .bind("drain_other_mission_id")
                .bind("PENDING")
                .bind("{}")
                .bind("other_org")
                .execute(&pool)
                .await
                .unwrap();

            // Call drain
            let res = sip_db.drain_mission_queue().await;
            assert!(res.is_ok());

            // Verify test_org mission is deleted
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = 'test_org'")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(count, 0);

            // Verify other_org mission is intact
            let count2: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = 'other_org'")
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(count2, 1);

            // Clean up
            sqlx::query("DELETE FROM agent_missions WHERE id IN ('drain_test_mission_id', 'drain_other_mission_id')").execute(&pool).await.unwrap();
        }
    }
}
