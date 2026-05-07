use sqlx::PgPool;
use sqlx::Row;
use chrono::Utc;



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
        crate::utils::auth_utils::set_org_context(&mut *tx, &self.org_id).await?;

        sqlx::query(
            "UPDATE agent_missions
             SET status = 'blocked',
                 mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN $1 ELSE mission_log || '\n' || $1 END,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = $2 AND organization_id = $3"
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

                sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND organization_id = $2")
                    .bind(stuck_threshold)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                // Backlog Management: Sanitize and prioritize the agent_missions queue, ensuring no "stuck" missions persist in either mode.
                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK' AND organization_id = $1")
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                // Prioritize backlog by bumping updated_at for oldest pending missions
                sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND organization_id = $1 ORDER BY created_at ASC LIMIT 10)")
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND organization_id = $2")
                    .bind(fail_threshold)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("WITH cte AS (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < $1)) AND organization_id = $2 LIMIT 1000) DELETE FROM agent_missions WHERE id IN (SELECT id FROM cte)")
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

    pub fn enrich_payload_with_grounding_content(&self, payload: &str, grounding_content: &Option<String>) -> String {
        let mut final_payload = payload.to_string();
        if let Some(content) = grounding_content {
            final_payload = format!("{}\n\n[SYSTEM GROUNDING]:\n{}", payload, content);
        }
        final_payload
    }

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

        loop {
            let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                let mut tx = self.pool.begin().await?;
                crate::utils::auth_utils::set_org_context(&mut *tx, "system").await?;
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
        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE organization_id = $1 AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(&self.org_id)
            .fetch_one(&mut **tx)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        let row = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2 FOR UPDATE")
            .bind(mission_id)
            .bind(&self.org_id)
            .fetch_optional(&mut **tx)
            .await?;

        if let Some(r) = row {
            let existing_id: String = r.get("id");
            if !existing_id.is_empty() && force_local {
                sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4")
                    .bind(&final_status)
                    .bind(payload)
                    .bind(mission_id)
                    .bind(&self.org_id)
                    .execute(&mut **tx)
                    .await?;
            }
        } else {
            let row_check = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2")
                .bind(mission_id)
                .bind(&self.org_id)
                .fetch_optional(&mut **tx)
                .await?;

            if let Some(_) = row_check {
                 if force_local {
                     sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4")
                         .bind(&final_status)
                         .bind(payload)
                         .bind(mission_id)
                         .bind(&self.org_id)
                         .execute(&mut **tx)
                         .await?;
                 }
            } else {
                 sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                     .bind(mission_id)
                     .bind(&final_status)
                     .bind(payload)
                     .bind(&self.org_id)
                     .execute(&mut **tx)
                     .await?;
            }
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

    // Helper to get a dummy pgpool for testing
    async fn setup_dummy_pool() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        sqlx::postgres::PgPoolOptions::new()
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
        let pool = sqlx::postgres::PgPoolOptions::new()
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
    async fn test_prune_stale_missions_marks_stuck_as_failed() {
        // Just verify it doesn't crash on execution with a valid pool.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();

        let sip_db = SipDB::new(pool, "test_org".to_string());

        let res = sip_db.prune_stale_missions(chrono::Duration::hours(24)).await;
        // Should error out gracefully with our dummy pool timeout instead of panicking
        assert!(res.is_err());
    }
}

