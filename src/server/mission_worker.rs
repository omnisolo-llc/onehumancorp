use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::sip::SipDB;
use std::time::Duration;
use sqlx::Row;

pub struct MissionWorker {
    db: Arc<DB>,
    poll_interval: Duration,
}

impl MissionWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(10),
        }
    }

    pub fn start(self: Arc<Self>) {
        let worker = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(worker.poll_interval);
            loop {
                interval.tick().await;
                if let Err(e) = worker.poll_and_execute().await {
                    tracing::error!("MissionWorker error during poll: {}", e);
                }
            }
        });
    }

    async fn poll_and_execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let missions = self.get_pending_missions().await?;

        for (id, tenant_id, payload) in missions {
            tracing::info!("MissionWorker: Processing mission {} for tenant {}", id, tenant_id);

            self.update_mission_status(&id, &tenant_id, "RUNNING").await?;

            let result = self.execute_mission(&payload).await;

            match result {
                Ok(_) => {
                    tracing::info!("MissionWorker: Mission {} completed successfully", id);
                    self.update_mission_status(&id, &tenant_id, "COMPLETED").await?;
                }
                Err(e) => {
                    tracing::error!("MissionWorker: Mission {} failed: {}", id, e);
                    let sip_db = SipDB::new(self.db.clone(), tenant_id.clone());
                    let _ = sip_db.handoff_mission(&id, &format!("Execution failed: {}", e)).await;
                }
            }
        }

        Ok(())
    }

    async fn get_pending_missions(&self) -> Result<Vec<(String, String, String)>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let rows = sqlx::query(
                    "SELECT id, tenant_id, payload FROM agent_missions
                     WHERE status = 'PENDING'
                     AND status NOT IN ('CLOUD_ESCALATION', 'BURSTING')
                     ORDER BY created_at ASC LIMIT 10"
                )
                .fetch_all(&self.db.pool)
                .await?;

                Ok(rows.into_iter().map(|r| (r.get(0), r.get(1), r.get(2))).collect())
            }
            DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT id, tenant_id, payload FROM agent_missions
                     WHERE status = 'PENDING'
                     ORDER BY created_at ASC LIMIT 10"
                )
                .fetch_all(pool)
                .await?;

                Ok(rows.into_iter().map(|r| (r.get(0), r.get(1), r.get(2))).collect())
            }
        }
    }

    async fn update_mission_status(&self, id: &str, tenant_id: &str, status: &str) -> Result<(), sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE agent_missions SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
                    .bind(status)
                    .bind(id)
                    .bind(tenant_id)
                    .execute(&self.db.pool)
                    .await?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE agent_missions SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                    .bind(status)
                    .bind(id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_mission(&self, _payload: &str) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;
    use chrono::Utc;

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
    async fn test_mission_worker_poll_and_execute() {
        let db = setup_test_db().await;
        let worker = Arc::new(MissionWorker::new(db.clone()));

        if let DbStore::Sqlite(pool) = &db.store {
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ('m1', 'PENDING', '{}', 't1')")
                .execute(pool).await.unwrap();
        }

        worker.poll_and_execute().await.unwrap();

        if let DbStore::Sqlite(pool) = &db.store {
            let status: String = sqlx::query_scalar("SELECT status FROM agent_missions WHERE id = 'm1'")
                .fetch_one(pool).await.unwrap();
            assert_eq!(status, "COMPLETED");
        }
    }
}
