use chrono::Utc;
use std::sync::OnceLock;
use tokio::sync::Semaphore;

static SQLITE_CONCURRENCY_LIMITER: OnceLock<Semaphore> = OnceLock::new();

pub fn get_sqlite_limiter() -> &'static Semaphore {
    SQLITE_CONCURRENCY_LIMITER.get_or_init(|| Semaphore::new(1))
}

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
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &self.org_id).await
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
                .bind(&self.org_id)
                .execute(&mut *tx)
                .await?;

                tx.commit().await?;
            },
            DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await?;
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
            }
        }
        Ok(())
    }

    pub fn with_context_root(mut self, root: String) -> Self {
        self.context_root = Some(root);
        self
    }

    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;

        self.db.execute_with_retry("prune_stale_missions", || async {
            match &self.db.store {
                DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &self.org_id).await
                        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

                    sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND tenant_id = $2")
                        .bind(stuck_threshold)
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK' AND tenant_id = $1")
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

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
                },
                DbStore::Sqlite(pool) => {
                    let mut tx = pool.begin().await?;

                    sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < ? AND tenant_id = ?")
                        .bind(stuck_threshold.format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK' AND tenant_id = ?")
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("UPDATE agent_missions SET updated_at = CURRENT_TIMESTAMP WHERE id IN (SELECT id FROM agent_missions WHERE status = 'PENDING' AND tenant_id = ? ORDER BY created_at ASC LIMIT 10)")
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'BURSTING') AND created_at < ? AND tenant_id = ?")
                        .bind(fail_threshold.format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'BURSTING') AND created_at < ?)) AND tenant_id = ? LIMIT 1000)")
                        .bind(fail_threshold.format("%Y-%m-%d %H:%M:%S").to_string())
                        .bind(&self.org_id)
                        .execute(&mut *tx)
                        .await?;

                    tx.commit().await?;
                }
            }
            Ok::<(), sqlx::Error>(())
        }).await?;
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

    pub async fn delegate_mission_pg_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool, grounding_content: &Option<String>) -> Result<(), sqlx::Error> {
        let final_payload = self.enrich_payload_with_grounding_content(payload, grounding_content);

        let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            self.upsert_mission_pg_tx(tx, mission_id, status, &final_payload, force_local).await
        }).await;

        match res {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(err),
            Err(timeout_err) => Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err))),
        }
    }

    pub async fn delegate_mission_sqlite_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, mission_id: &str, status: &str, payload: &str, force_local: bool, grounding_content: &Option<String>) -> Result<(), sqlx::Error> {
        let final_payload = self.enrich_payload_with_grounding_content(payload, grounding_content);

        let res = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            self.upsert_mission_sqlite_tx(tx, mission_id, status, &final_payload, force_local).await
        }).await;

        match res {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(err)) => Err(err),
            Err(timeout_err) => Err(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, timeout_err))),
        }
    }

    pub async fn upsert_mission(&self, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        self.db.execute_with_retry("upsert_mission", || async {
            match &self.db.store {
                DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, "system").await
                        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
                    self.upsert_mission_pg_tx(&mut tx, mission_id, status, payload, force_local).await?;
                    tx.commit().await?;
                },
                DbStore::Sqlite(pool) => {
                    let mut tx = pool.begin().await?;
                    self.upsert_mission_sqlite_tx(&mut tx, mission_id, status, payload, force_local).await?;
                    tx.commit().await?;
                }
            }
            Ok::<(), sqlx::Error>(())
        }).await
    }

    pub async fn upsert_mission_pg_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut final_status = status.to_string();

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

    pub async fn upsert_mission_sqlite_tx(&self, tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut final_status = status.to_string();

        let pending_count: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE tenant_id = ? AND (status = 'PENDING' OR status = 'RUNNING')")
            .bind(&self.org_id)
            .fetch_one(&mut **tx)
            .await?;

        if pending_count >= 5 && status == "PENDING" {
            final_status = "BURSTING".to_string();
        }

        let mut updated = false;

        if force_local {
            let row = sqlx::query("UPDATE agent_missions SET status = ?, payload = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ? RETURNING id")
                .bind(&final_status)
                .bind(payload)
                .bind(mission_id)
                .bind(&self.org_id)
                .fetch_optional(&mut **tx)
                .await?;

            updated = row.is_some();
        }

        if !updated {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?) ON CONFLICT(id) DO NOTHING")
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

    async fn setup_test_db() -> Arc<DB> {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        let schema = r#"
            CREATE TABLE IF NOT EXISTS agent_missions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                tenant_id TEXT NOT NULL DEFAULT 'system',
                mission_log TEXT
            );
        "#;
        sqlx::query(schema).execute(&sqlite_pool).await.unwrap();

        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB { pool: dummy_pg_pool, store: DbStore::Sqlite(sqlite_pool) })
    }

    #[tokio::test]
    async fn test_delegate_mission_tc1_no_context_root() {
        let db = setup_test_db().await;
        let sip_db = SipDB::new(db, "test_org".to_string());
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
        let db = setup_test_db().await;
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
    async fn test_handoff_mission_logic_success() {
        let db = setup_test_db().await;
        let sip_db = SipDB::new(db.clone(), "test_org".to_string());

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4)")
                .bind("test_mission_id")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org")
                .execute(pool)
                .await
                .unwrap();

            let res = sip_db.handoff_mission("test_mission_id", "Missing dependencies").await;
            assert!(res.is_ok());

            let row = sqlx::query("SELECT status, mission_log FROM agent_missions WHERE id = 'test_mission_id'")
                .fetch_one(pool)
                .await
                .unwrap();

            let status: String = row.get("status");
            let log: String = row.get("mission_log");

            assert_eq!(status, "blocked");
            assert!(log.contains("Missing dependencies"));
        }
    }

    #[tokio::test]
    async fn test_prune_stale_missions_logic() {
        let db = setup_test_db().await;
        let sip_db = SipDB::new(db.clone(), "test_org".to_string());

        if let DbStore::Sqlite(pool) = &db.store {
            let stale_time = (Utc::now() - chrono::Duration::hours(2)).format("%Y-%m-%d %H:%M:%S").to_string();

            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at, created_at) VALUES ('m1', 'PENDING', '{}', 'test_org', ?, ?)")
                .bind(&stale_time)
                .bind(&stale_time)
                .execute(pool)
                .await
                .unwrap();

            let res = sip_db.prune_stale_missions(chrono::Duration::hours(1)).await;
            assert!(res.is_ok());

            let status: String = sqlx::query_scalar("SELECT status FROM agent_missions WHERE id = 'm1'")
                .fetch_one(pool)
                .await
                .unwrap();

            assert_eq!(status, "FAILED");
        }
    }
}
