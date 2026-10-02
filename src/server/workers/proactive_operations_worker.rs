use super::proactive_operations_polling::{bound_postgres_transaction, scan_tenants};
use super::proactive_operations_storage::{ScanCounts, scan_postgres, scan_sqlite};
use crate::db::{DB, DbStore};
use std::sync::Arc;
use std::time::Duration;

const TENANTS_PER_POLL: i64 = 100;
const POLL_INTERVAL: Duration = Duration::from_secs(60);

pub struct ProactiveOperationsWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}
impl ProactiveOperationsWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: POLL_INTERVAL,
        }
    }

    async fn scan_tenant(&self, tenant: &str) -> Result<ScanCounts, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                bound_postgres_transaction(&mut tx).await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
                let result = scan_postgres(&mut tx, tenant).await?;
                tx.commit().await?;
                Ok(result)
            }
            DbStore::Sqlite(pool) => scan_sqlite(pool, tenant).await,
        }
    }

    async fn poll_batch(&self, after: &mut String) -> Result<(), sqlx::Error> {
        let tenants: Vec<String> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_scalar("SELECT id FROM tenants WHERE id>$1 ORDER BY id LIMIT $2")
                    .bind(after.as_str())
                    .bind(TENANTS_PER_POLL)
                    .fetch_all(&self.db.pool)
                    .await?
            }
            DbStore::Sqlite(pool) => {
                sqlx::query_scalar("SELECT id FROM tenants WHERE id>?1 ORDER BY id LIMIT ?2")
                    .bind(after.as_str())
                    .bind(TENANTS_PER_POLL)
                    .fetch_all(pool)
                    .await?
            }
        };
        let results = scan_tenants(
            after,
            &tenants,
            TENANTS_PER_POLL as usize,
            |tenant| async move {
                let changes = self.scan_tenant(&tenant).await?;
                if changes.created > 0 || changes.retired > 0 {
                    crate::api::agent_feed::get_agent_feed_cache()
                        .invalidate_by_tag(&format!("agent_feed_tenant:{tenant}"))
                        .await;
                }
                Ok(())
            },
        )
        .await;
        for (tenant, result) in results {
            if let Err(error) = result {
                tracing::warn!(event="operations.tenant_alert_scan_failed", tenant_id=%tenant, %error, "Operational scan did not finish; committed observations, if any, remain recorded");
            }
        }
        Ok(())
    }

    pub fn start(&self) {
        let worker = Self {
            db: self.db.clone(),
            poll_interval: self.poll_interval.max(POLL_INTERVAL),
        };
        tracing::info!(
            event = "operations.alert_capabilities",
            supported = "configured inventory thresholds",
            unavailable = "supplier deadlines, staffing coverage requirements and scheduled checklist configuration are not stored; no such alerts are generated"
        );
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(worker.poll_interval);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            let mut after = String::new();
            loop {
                interval.tick().await;
                match tokio::time::timeout(Duration::from_secs(30), worker.poll_batch(&mut after))
                    .await
                {
                    Ok(Ok(())) => {}
                    Ok(Err(error)) => {
                        tracing::warn!(event="operations.alert_scan_failed", %error, "Operational facts were not verified")
                    }
                    Err(_) => tracing::warn!(
                        event = "operations.alert_scan_timed_out",
                        "Operational fact scan exceeded its bounded polling budget"
                    ),
                }
            }
        });
    }
}
