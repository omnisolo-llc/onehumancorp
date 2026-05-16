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
}


#[cfg(test)]
#[path = "sip_tests.rs"]
mod sip_tests;
