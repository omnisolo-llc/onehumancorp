use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;

pub struct MaintenanceWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl MaintenanceWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(300), // 5 minutes
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let worker = MaintenanceWorker {
                    db: db.clone(),
                    poll_interval: interval_duration,
                };
                if let Err(e) = worker.cleanup_stale_missions().await {
                    eprintln!("MaintenanceWorker error: {}", e);
                }
            }
        });
    }

    pub async fn cleanup_stale_missions(&self) -> Result<(), String> {
        let timeout_threshold = Utc::now() - chrono::Duration::hours(1);

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    r#"
                    UPDATE agent_missions
                    SET status = 'STAGNANT', updated_at = CURRENT_TIMESTAMP
                    WHERE (status = 'PENDING' OR status = 'RUNNING')
                    AND updated_at < $1
                    "#
                )
                .bind(timeout_threshold)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    UPDATE agent_missions
                    SET status = 'STAGNANT', updated_at = CURRENT_TIMESTAMP
                    WHERE (status = 'PENDING' OR status = 'RUNNING')
                    AND updated_at < ?
                    "#
                )
                .bind(timeout_threshold.to_rfc3339())
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cleanup_stale_missions_sqlite() {
        let db_url = "sqlite::memory:";
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(db_url)
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE agent_missions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(),
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        let worker = MaintenanceWorker::new(db);

        let old_time = (Utc::now() - chrono::Duration::hours(2)).to_rfc3339();
        let recent_time = (Utc::now() - chrono::Duration::minutes(30)).to_rfc3339();

        sqlx::query("INSERT INTO agent_missions (id, status, payload, updated_at) VALUES (?, ?, ?, ?)")
            .bind("stale_pending")
            .bind("PENDING")
            .bind("{}")
            .bind(&old_time)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO agent_missions (id, status, payload, updated_at) VALUES (?, ?, ?, ?)")
            .bind("stale_running")
            .bind("RUNNING")
            .bind("{}")
            .bind(&old_time)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO agent_missions (id, status, payload, updated_at) VALUES (?, ?, ?, ?)")
            .bind("recent_running")
            .bind("RUNNING")
            .bind("{}")
            .bind(&recent_time)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO agent_missions (id, status, payload, updated_at) VALUES (?, ?, ?, ?)")
            .bind("completed_stale")
            .bind("COMPLETED")
            .bind("{}")
            .bind(&old_time)
            .execute(&pool)
            .await
            .unwrap();

        worker.cleanup_stale_missions().await.unwrap();

        let row1: (String,) = sqlx::query_as("SELECT status FROM agent_missions WHERE id = 'stale_pending'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row1.0, "STAGNANT");

        let row2: (String,) = sqlx::query_as("SELECT status FROM agent_missions WHERE id = 'stale_running'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row2.0, "STAGNANT");

        let row3: (String,) = sqlx::query_as("SELECT status FROM agent_missions WHERE id = 'recent_running'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row3.0, "RUNNING");

        let row4: (String,) = sqlx::query_as("SELECT status FROM agent_missions WHERE id = 'completed_stale'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row4.0, "COMPLETED");
    }
}
