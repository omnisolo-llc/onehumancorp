use sqlx::{Pool, Postgres, Sqlite};

pub enum TrackerPool {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

pub struct AsyncTaskTracker {
    pool: TrackerPool,
}

impl AsyncTaskTracker {
    pub fn new_postgres(pool: Pool<Postgres>) -> Self {
        Self { pool: TrackerPool::Postgres(pool) }
    }

    pub fn new_sqlite(pool: Pool<Sqlite>) -> Self {
        Self { pool: TrackerPool::Sqlite(pool) }
    }

    pub async fn create_task(
        &self,
        id: &str,
        tenant_id: &str,
        agent_id: &str,
        payload: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.pool {
            TrackerPool::Postgres(pool) => {
                let mut tx = pool.begin().await?;
                sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                    .bind(tenant_id)
                    .execute(&mut *tx)
                    .await?;
                sqlx::query(
                    "INSERT INTO mcp_async_tasks (id, tenant_id, agent_id, status, payload) VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(id)
                .bind(tenant_id)
                .bind(agent_id)
                .bind("PENDING")
                .bind(payload)
                .execute(&mut *tx)
                .await?;
                tx.commit().await?;
            }
            TrackerPool::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO mcp_async_tasks (id, tenant_id, agent_id, status, payload) VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(id)
                .bind(tenant_id)
                .bind(agent_id)
                .bind("PENDING")
                .bind(payload)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    pub async fn complete_task(
        &self,
        id: &str,
        payload: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.pool {
            TrackerPool::Postgres(pool) => {
                let mut tx = pool.begin().await?;
                sqlx::query("SELECT set_config('app.current_tenant', 'system', true)")
                    .execute(&mut *tx)
                    .await?;
                let result = sqlx::query(
                    "UPDATE mcp_async_tasks SET status = 'COMPLETED', payload = $1 WHERE id = $2"
                )
                .bind(payload)
                .bind(id)
                .execute(&mut *tx)
                .await?;
                if result.rows_affected() == 0 {
                    return Err(format!("Task {} not found", id).into());
                }
                tx.commit().await?;
            }
            TrackerPool::Sqlite(pool) => {
                let result = sqlx::query(
                    "UPDATE mcp_async_tasks SET status = 'COMPLETED', payload = $1 WHERE id = $2"
                )
                .bind(payload)
                .bind(id)
                .execute(pool)
                .await?;
                if result.rows_affected() == 0 {
                    return Err(format!("Task {} not found", id).into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_complete_task_pg() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(database_url)
            .await
            .unwrap();

        let tracker = AsyncTaskTracker::new_postgres(pool);

        let id = "test-task-pg-123";
        let tenant_id = "system";
        let agent_id = "agent-1";
        let payload = "{\"data\":\"test\"}";

        tracker.create_task(id, tenant_id, agent_id, payload).await.unwrap();

        let res_complete = tracker.complete_task(id, "{\"result\":\"ok\"}").await;
        assert!(res_complete.is_ok());

        let res_fail = tracker.complete_task("nonexistent-id", "{\"result\":\"ok\"}").await;
        assert!(res_fail.is_err());
    }

    #[tokio::test]
    async fn test_create_and_complete_task_sqlite() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS mcp_async_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                status TEXT NOT NULL,
                payload TEXT,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&pool).await.unwrap();

        let tracker = AsyncTaskTracker::new_sqlite(pool);

        let id = "test-task-sq";
        let tenant_id = "tenant-1";
        let agent_id = "agent-1";
        let payload = "{\"data\":\"test\"}";

        tracker.create_task(id, tenant_id, agent_id, payload).await.unwrap();

        let res_complete = tracker.complete_task(id, "{\"result\":\"ok\"}").await;
        assert!(res_complete.is_ok());

        let res_fail = tracker.complete_task("nonexistent-id", "{\"result\":\"ok\"}").await;
        assert!(res_fail.is_err());
    }
}
