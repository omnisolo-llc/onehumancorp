use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;
use crate::db::DbStore;

pub struct DeliveryDispatchWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl DeliveryDispatchWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                loop {
                    match Self::poll(&db).await {
                        Ok(true) => continue,
                        Ok(false) => break,
                        Err(e) => {
                            tracing::error!("DeliveryDispatchWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let order = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                // Find an order that might need delivery but doesn't have a task yet
                let row = sqlx::query(
                    r#"
                    SELECT o.id, o.tenant_id
                    FROM orders o
                    LEFT JOIN delivery_tasks dt ON o.id = dt.order_id
                    WHERE dt.id IS NULL AND o.status = 'pending'
                    ORDER BY o.created_at ASC
                    LIMIT 1
                    FOR UPDATE SKIP LOCKED
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT o.id, o.tenant_id
                    FROM orders o
                    LEFT JOIN delivery_tasks dt ON o.id = dt.order_id
                    WHERE dt.id IS NULL AND o.status = 'pending'
                    ORDER BY o.created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        let Some((order_id, tenant_id)) = order else {
            return Ok(false);
        };

        // Create a delivery task
        let task_id = Uuid::new_v4();

        match &db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO delivery_tasks (id, organization_id, order_id, status)
                    VALUES ($1, $2, $3, 'PENDING')
                    "#
                )
                .bind(task_id)
                .bind(&tenant_id)
                .bind(&order_id)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string())?;
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                // Ensure table exists for sqlite fallback tests
                let _ = sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS delivery_tasks (
                        id TEXT PRIMARY KEY,
                        organization_id TEXT NOT NULL,
                        order_id TEXT NOT NULL,
                        driver_id TEXT,
                        route_plan_id TEXT,
                        status TEXT NOT NULL DEFAULT 'PENDING',
                        estimated_arrival TIMESTAMP,
                        delivery_location TEXT,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    "#
                ).execute(sqlite_pool).await;

                sqlx::query(
                    r#"
                    INSERT INTO delivery_tasks (id, organization_id, order_id, status)
                    VALUES (?, ?, ?, 'PENDING')
                    "#
                )
                .bind(task_id.to_string())
                .bind(&tenant_id)
                .bind(&order_id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        tracing::info!("Created delivery task {} for order {} in tenant {}", task_id, order_id, tenant_id);

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> Arc<DB> {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Setup dummy Postgres pool to satisfy struct initialization
        // We use the testing setup approach from department_workers.rs
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@127.0.0.1:5432/postgres")
            .unwrap();

        Arc::new(DB {
            pool: dummy_pg_pool,
            store: DbStore::Sqlite(sqlite_pool),
        })
    }

    #[tokio::test]
    async fn test_delivery_dispatch_worker_creates_task() {
        let db = setup_test_db().await;
        if let DbStore::Sqlite(pool) = &db.store {
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT PRIMARY KEY, tenant_id TEXT, status TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(pool).await;

            sqlx::query("INSERT INTO orders (id, tenant_id, status) VALUES ('order_delivery_1', 'tenant1', 'pending')")
                .execute(pool).await.unwrap();
        }

        let processed = DeliveryDispatchWorker::poll(&db).await.unwrap();
        assert!(processed);

        if let DbStore::Sqlite(pool) = &db.store {
            let row = sqlx::query("SELECT order_id, status FROM delivery_tasks WHERE organization_id = 'tenant1'")
                .fetch_optional(pool).await.unwrap();

            if let Some(row) = row {
                let order_id: String = row.get("order_id");
                let status: String = row.get("status");
                assert_eq!(order_id, "order_delivery_1");
                assert_eq!(status, "PENDING");
            } else {
                panic!("Delivery task was not created");
            }
        }
    }
}