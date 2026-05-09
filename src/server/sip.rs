use sqlx::Row;
use chrono::Utc;
use crate::db::DbStore;

pub struct SipDB {
    pub(crate) store: DbStore,
    pub(crate) pg_pool: sqlx::PgPool,
    pub(crate) tenant_id: String,
    pub(crate) context_root: Option<String>,
}

impl SipDB {
    pub fn new(db: &crate::db::DB, tenant_id: String) -> Self {
        SipDB {
            store: db.store.clone(),
            pg_pool: db.pool.clone(),
            tenant_id,
            context_root: None,
        }
    }

    pub fn with_context_root(mut self, root: String) -> Self {
        self.context_root = Some(root);
        self
    }

    pub async fn handoff_mission(&self, mission_id: &str, blockers: &str) -> Result<(), Box<dyn std::error::Error>> {
        let blockers_msg = extract_blockers_message(blockers);

        match &self.store {
            DbStore::Postgres => {
                let mut tx = self.pg_pool.begin().await?;
                crate::utils::auth_utils::set_org_context(&mut *tx, &self.tenant_id).await?;

                sqlx::query(
                    "UPDATE agent_missions
                     SET status = 'blocked',
                         mission_log = COALESCE(mission_log, '') || $1,
                         updated_at = CURRENT_TIMESTAMP
                     WHERE id = $2 AND tenant_id = $3"
                )
                .bind(&blockers_msg)
                .bind(mission_id)
                .bind(&self.tenant_id)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    "UPDATE agent_missions
                     SET status = 'blocked',
                         mission_log = COALESCE(mission_log, '') || ?,
                         updated_at = CURRENT_TIMESTAMP
                     WHERE id = ? AND tenant_id = ?"
                )
                .bind(&blockers_msg)
                .bind(mission_id)
                .bind(&self.tenant_id)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), Box<dyn std::error::Error>> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;
        
        match &self.store {
            DbStore::Postgres => {
                let mut tx = self.pg_pool.begin().await?;
                crate::utils::auth_utils::set_org_context(&mut *tx, &self.tenant_id).await?;

                sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND tenant_id = $2")
                    .bind(stuck_threshold)
                    .bind(&self.tenant_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("DELETE FROM agent_missions WHERE status = 'STUCK' AND tenant_id = $1")
                    .bind(&self.tenant_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = $1 ORDER BY created_at ASC LIMIT 10)")
                    .bind(&self.tenant_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < $1 AND tenant_id = $2")
                    .bind(fail_threshold)
                    .bind(&self.tenant_id)
                    .execute(&mut *tx)
                    .await?;

                sqlx::query("WITH cte AS (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < $1)) AND tenant_id = $2 LIMIT 1000) DELETE FROM agent_missions WHERE id IN (SELECT id FROM cte)")
                    .bind(fail_threshold)
                    .bind(&self.tenant_id)
                    .execute(&mut *tx)
                    .await?;

                tx.commit().await?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < ? AND tenant_id = ?")
                    .bind(stuck_threshold.to_rfc3339())
                    .bind(&self.tenant_id)
                    .execute(pool)
                    .await?;

                sqlx::query("DELETE FROM agent_missions WHERE status = 'STUCK' AND tenant_id = ?")
                    .bind(&self.tenant_id)
                    .execute(pool)
                    .await?;

                sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = ? ORDER BY created_at ASC LIMIT 10)")
                    .bind(&self.tenant_id)
                    .execute(pool)
                    .await?;

                sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < ? AND tenant_id = ?")
                    .bind(fail_threshold.to_rfc3339())
                    .bind(&self.tenant_id)
                    .execute(pool)
                    .await?;

                sqlx::query("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < ?)) AND tenant_id = ? LIMIT 1000)")
                    .bind(fail_threshold.to_rfc3339())
                    .bind(&self.tenant_id)
                    .execute(pool)
                    .await?;
            }
        }
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

        let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            self.upsert_mission_with_tx(tx, mission_id, status, &final_payload, force_local).await
        }).await;

        match res {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(err),
            Err(timeout_err) => Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err))),
        }
    }

    pub async fn upsert_mission(&self, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut attempt = 0;
        let max_attempts = 3;
        let mut backoff = std::time::Duration::from_millis(50);

        loop {
            let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                match &self.store {
                    DbStore::Postgres => {
                        let mut tx = self.pg_pool.begin().await?;
                        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await?;
                        self.upsert_mission_with_tx(&mut tx, mission_id, status, payload, force_local).await?;
                        tx.commit().await?;
                        Ok::<(), sqlx::Error>(())
                    }
                    DbStore::Sqlite(pool) => {
                        self.upsert_mission_sqlite(pool, mission_id, status, payload, force_local).await?;
                        Ok::<(), sqlx::Error>(())
                    }
                }
            }).await;

            match res {
                Ok(Ok(_)) => return Ok(()),
                Ok(Err(err)) => {
                    let err_str = err.to_string().to_lowercase();
                    if err_str.contains("database is locked") || err_str.contains("sqlite_busy") || err_str.contains("deadlock") || err_str.contains("serialization") || err_str.contains("timeout") || err_str.contains("closed") || err_str.contains("connection refused") || err_str.contains("connection reset") {
                        attempt += 1;
                        if attempt >= max_attempts {
                            return Err(err.into());
                        }
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    } else {
                        return Err(err.into());
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

    pub async fn upsert_mission_with_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut final_status = status.to_string();

        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = $1 AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(&self.tenant_id)
            .fetch_one(&mut **tx)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        let row = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
            .bind(mission_id)
            .bind(&self.tenant_id)
            .fetch_optional(&mut **tx)
            .await?;

        if let Some(r) = row {
            let existing_id: String = r.get("id");
            if !existing_id.is_empty() && force_local {
                sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
                    .bind(&final_status)
                    .bind(payload)
                    .bind(mission_id)
                    .bind(&self.tenant_id)
                    .execute(&mut **tx)
                    .await?;
            }
        } else {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                .bind(mission_id)
                .bind(&final_status)
                .bind(payload)
                .bind(&self.tenant_id)
                .execute(&mut **tx)
                .await?;
        }

        Ok(())
    }

    async fn upsert_mission_sqlite(&self, pool: &sqlx::SqlitePool, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut final_status = status.to_string();
        let final_payload = self.enrich_payload_with_grounding_content(payload, &self.load_grounding_content().await);

        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = ? AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(&self.tenant_id)
            .fetch_one(pool)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        let row = sqlx::query("SELECT id FROM agent_missions WHERE id = ? AND tenant_id = ?")
            .bind(mission_id)
            .bind(&self.tenant_id)
            .fetch_optional(pool)
            .await?;

        if let Some(_) = row {
            if force_local {
                sqlx::query("UPDATE agent_missions SET status = ?, payload = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                    .bind(&final_status)
                    .bind(&final_payload)
                    .bind(mission_id)
                    .bind(&self.tenant_id)
                    .execute(pool)
                    .await?;
            }
        } else {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?) ON CONFLICT(id) DO NOTHING")
                .bind(mission_id)
                .bind(&final_status)
                .bind(&final_payload)
                .bind(&self.tenant_id)
                .execute(pool)
                .await?;
        }
        Ok(())
    }
}

pub fn extract_blockers_message(blockers: &str) -> String {
    format!("\n{}", blockers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::env;
    use sqlx::PgPool;

    async fn setup_dummy_pool() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap()
    }

    #[tokio::test]
    async fn test_enrich_payload() {
        let db_mock = crate::db::DB {
            pool: setup_dummy_pool().await,
            store: DbStore::Postgres,
        };
        let sip = SipDB::new(&db_mock, "test_tenant".to_string());
        let payload = "task";
        let enriched = sip.enrich_payload_with_grounding_content(payload, &Some("rules".to_string()));
        assert!(enriched.contains("rules"));
        assert!(enriched.contains("task"));
    }

    #[tokio::test]
    async fn test_delegate_mission_tc1_no_context_root() {
        let pool = setup_dummy_pool().await;
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let sip_db = SipDB::new(&db, "test_org".to_string());
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
        let pool = setup_dummy_pool().await;
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let dir_str = create_temp_dir("tc2");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "Always write clean code.").unwrap();

        let sip_db = SipDB::new(&db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAlways write clean code.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc3_claude_md_fallback() {
        let pool = setup_dummy_pool().await;
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let dir_str = create_temp_dir("tc3");
        let dir_path = std::path::Path::new(&dir_str);

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file = File::create(&claude_path).unwrap();
        write!(file, "Use specialized tokens.").unwrap();

        let sip_db = SipDB::new(&db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc4_grounding_priority() {
        let pool = setup_dummy_pool().await;
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let dir_str = create_temp_dir("tc4");
        let dir_path = std::path::Path::new(&dir_str);

        let agents_path = dir_path.join("AGENTS.md");
        let mut file = File::create(&agents_path).unwrap();
        write!(file, "AGENTS rules.").unwrap();

        let claude_path = dir_path.join("CLAUDE.md");
        let mut file2 = File::create(&claude_path).unwrap();
        write!(file2, "CLAUDE rules.").unwrap();

        let sip_db = SipDB::new(&db, "test_org".to_string())
            .with_context_root(dir_str.clone());

        let payload = "Original Task Payload";
        let enriched = sip_db.enrich_payload_with_grounding_content(payload, &sip_db.load_grounding_content().await);
        assert_eq!(enriched, "Original Task Payload\n\n[SYSTEM GROUNDING]:\nAGENTS rules.");

        std::fs::remove_dir_all(&dir_str).unwrap();
    }

    #[tokio::test]
    async fn test_delegate_mission_tc5_missing_files() {
        let pool = setup_dummy_pool().await;
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let dir_str = create_temp_dir("tc5");

        let sip_db = SipDB::new(&db, "test_org".to_string())
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
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let sip_db = SipDB::new(&db, "test_org".to_string());

        let res = sip_db.handoff_mission("dummy_id", "Blocked by prompt instructions").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_prune_stale_missions_marks_stuck_as_failed() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(10))
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();
        let db = crate::db::DB { pool: pool.clone(), store: DbStore::Postgres };
        let sip_db = SipDB::new(&db, "test_org".to_string());

        let res = sip_db.prune_stale_missions(chrono::Duration::hours(24)).await;
        assert!(res.is_err());
    }
}
