use std::sync::Arc;
use std::time::Duration;
use crate::db::DB;
use chrono::Utc;
use sqlx::Row;
use tokio::time::timeout;

const DB_OP_TIMEOUT: Duration = Duration::from_secs(2);
const AI_AGENT_TIMEOUT: Duration = Duration::from_secs(60);

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl ProactiveAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60 * 15), // Every 15 minutes
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_analysis(&db).await {
                    tracing::error!("ProactiveAnalysisWorker error: {}", e);
                }
            }
        });
    }

    async fn run_analysis(db: &Arc<DB>) -> Result<(), String> {
        let tenants = match &db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("SELECT id FROM tenants")
                    .fetch_all(&db.pool)
                    .await
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .map(|r| r.get::<String, _>("id"))
                    .collect::<Vec<String>>()
            },
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("SELECT id FROM tenants")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?
                    .into_iter()
                    .map(|r| r.get::<String, _>("id"))
                    .collect::<Vec<String>>()
            }
        };

        for tenant_id in tenants {
            let pending_quotes_count = Self::get_pending_quotes(db, &tenant_id).await?;
            if pending_quotes_count > 0 {
                Self::create_insight(
                    db,
                    &tenant_id,
                    "Sales",
                    "High",
                    &format!("You have {} pending quote(s) that need follow-up.", pending_quotes_count),
                    "Draft Follow-up",
                    "Send reminder email to pending quotes",
                ).await?;
            }

            let unconfirmed_bookings = Self::get_unconfirmed_bookings(db, &tenant_id).await?;
            if unconfirmed_bookings > 0 {
                Self::create_insight(
                    db,
                    &tenant_id,
                    "Operations",
                    "High",
                    &format!("You have {} unconfirmed booking(s) for upcoming days.", unconfirmed_bookings),
                    "Confirm Bookings",
                    "Send confirmation requests to clients",
                ).await?;
            }

            let low_inventory_items = Self::get_low_inventory(db, &tenant_id).await?;
            if !low_inventory_items.is_empty() {
                let context = format!("Inventory is low for: {}", low_inventory_items.join(", "));
                Self::create_insight(
                    db,
                    &tenant_id,
                    "Inventory",
                    "Medium",
                    &context,
                    "Schedule Restock",
                    "Draft email to supplier",
                ).await?;
            }
        }
        Ok(())
    }

    async fn get_pending_quotes(db: &Arc<DB>, tenant_id: &str) -> Result<i64, String> {
        match &db.store {
            crate::db::DbStore::Postgres => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM service_quotes WHERE tenant_id = $1 AND status = 'pending'")
                    .bind(tenant_id)
                    .fetch_one(&db.pool)
                    .await
                    .unwrap_or((0,));
                Ok(count.0)
            },
            crate::db::DbStore::Sqlite(pool) => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM service_quotes WHERE tenant_id = ? AND status = 'pending'")
                    .bind(tenant_id)
                    .fetch_one(pool)
                    .await
                    .unwrap_or((0,));
                Ok(count.0)
            }
        }
    }

    async fn get_unconfirmed_bookings(db: &Arc<DB>, tenant_id: &str) -> Result<i64, String> {
        match &db.store {
            crate::db::DbStore::Postgres => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM service_bookings WHERE tenant_id = $1 AND status = 'pending'")
                    .bind(tenant_id)
                    .fetch_one(&db.pool)
                    .await
                    .unwrap_or((0,));
                Ok(count.0)
            },
            crate::db::DbStore::Sqlite(pool) => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM service_bookings WHERE tenant_id = ? AND status = 'pending'")
                    .bind(tenant_id)
                    .fetch_one(pool)
                    .await
                    .unwrap_or((0,));
                Ok(count.0)
            }
        }
    }

    async fn get_low_inventory(db: &Arc<DB>, tenant_id: &str) -> Result<Vec<String>, String> {
        match &db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query("SELECT title FROM products WHERE tenant_id = $1 AND inventory_count > 0 AND inventory_count < 10")
                    .bind(tenant_id)
                    .fetch_all(&db.pool)
                    .await
                    .unwrap_or_default();
                Ok(rows.into_iter().map(|r| r.get::<String, _>("title")).collect())
            },
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT title FROM products WHERE tenant_id = ? AND inventory_count > 0 AND inventory_count < 10")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .unwrap_or_default();
                Ok(rows.into_iter().map(|r| r.get::<String, _>("title")).collect())
            }
        }
    }

    async fn create_insight(
        db: &Arc<DB>,
        tenant_id: &str,
        source: &str,
        priority: &str,
        context: &str,
        action_type: &str,
        action_payload: &str,
    ) -> Result<(), String> {
        let item_id = format!("triage-{}", uuid::Uuid::new_v4());
        let action_id = format!("action-{}", uuid::Uuid::new_v4());

        match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                // Check if similar item already exists
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM triage_items WHERE tenant_id = $1 AND context = $2 AND status = 'pending'")
                    .bind(tenant_id)
                    .bind(context)
                    .fetch_one(&mut *tx)
                    .await
                    .unwrap_or((0,));

                if count.0 == 0 {
                    let _ = sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, $3, $4, $5, 'pending')")
                        .bind(&item_id)
                        .bind(tenant_id)
                        .bind(source)
                        .bind(priority)
                        .bind(context)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)")
                        .bind(&action_id)
                        .bind(&item_id)
                        .bind(tenant_id)
                        .bind(action_type)
                        .bind(action_payload)
                        .execute(&mut *tx)
                        .await;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            },
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM triage_items WHERE tenant_id = ? AND context = ? AND status = 'pending'")
                    .bind(tenant_id)
                    .bind(context)
                    .fetch_one(&mut *tx)
                    .await
                    .unwrap_or((0,));

                if count.0 == 0 {
                    let _ = sqlx::query("INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES (?, ?, ?, ?, ?, 'pending')")
                        .bind(&item_id)
                        .bind(tenant_id)
                        .bind(source)
                        .bind(priority)
                        .bind(context)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (?, ?, ?, ?, ?)")
                        .bind(&action_id)
                        .bind(&item_id)
                        .bind(tenant_id)
                        .bind(action_type)
                        .bind(action_payload)
                        .execute(&mut *tx)
                        .await;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
