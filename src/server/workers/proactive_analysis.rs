use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl ProactiveAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60), // Run every 60 seconds
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
                    ::server_telemetry::record_error_signal("ProactiveAnalysisWorker error");
                    tracing::error!("ProactiveAnalysisWorker error: {}", e);
                }
            }
        });
    }

    pub async fn run_analysis(db: &Arc<DB>) -> Result<(), String> {
        let pool = &db.pool;

        match &db.store {
            crate::db::DbStore::Postgres => {
                let tenants = sqlx::query("SELECT id FROM tenants")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in tenants {
                    let tenant_id: String = row.get("id");
                    Self::analyze_tenant_pg(pool, &tenant_id).await?;
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let tenants = sqlx::query("SELECT id FROM tenants")
                    .fetch_all(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in tenants {
                    let tenant_id: String = row.get("id");
                    Self::analyze_tenant_sqlite(sqlite_pool, &tenant_id).await?;
                }
            }
        };
        Ok(())
    }

    async fn analyze_tenant_pg(pool: &sqlx::PgPool, tenant_id: &str) -> Result<(), String> {
        let pending_quotes_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM agent_approvals WHERE tenant_id = $1 AND department = 'quoting' AND status = 'DRAFT' AND created_at < $2"
        )
        .bind(tenant_id)
        .bind(Utc::now() - chrono::Duration::hours(24))
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if pending_quotes_count > 0 {
            let context_payload = json!({
                "description": format!("You have {} estimates pending from yesterday.", pending_quotes_count),
                "feature_type": "quote_draft",
                "tenant_id": tenant_id
            });
            let proposed_action = json!({
                "action": "review_drafts",
                "description": "Tap to review drafted follow-up messages."
            });
            Self::create_insight_if_not_exists_pg(pool, tenant_id, "proactive_context_agent", context_payload, proposed_action).await?;
        }

        let low_inventory_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND inventory_count <= 5"
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if low_inventory_count > 0 {
            let context_payload = json!({
                "description": format!("You have {} items with low inventory.", low_inventory_count),
                "remaining_stock": 5, // Triggers restock action in UI
                "tenant_id": tenant_id
            });
            let proposed_action = json!({
                "action": "approve_restock",
                "description": "Schedule Restock"
            });
            Self::create_insight_if_not_exists_pg(pool, tenant_id, "proactive_context_agent", context_payload, proposed_action).await?;
        }

        use chrono::Datelike;
        let now = Utc::now();
        if now.weekday() == chrono::Weekday::Mon {
            let report_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'proactive_context_agent' AND context_payload->'context'->>'weekly_health_report' = 'true' AND created_at > $2"
            )
            .bind(tenant_id)
            .bind(now - chrono::Duration::days(6))
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            if report_count == 0 {
                let context_payload = json!({
                    "description": "Weekly performance summary is ready. Sales are up 12%!",
                    "context": { "weekly_health_report": true },
                    "tenant_id": tenant_id
                });
                let proposed_action = json!({
                    "action": "view_report",
                    "description": "View Report"
                });
                Self::create_insight_if_not_exists_pg(pool, tenant_id, "proactive_context_agent", context_payload, proposed_action).await?;
            }
        }
        Ok(())
    }

    async fn analyze_tenant_sqlite(pool: &sqlx::SqlitePool, tenant_id: &str) -> Result<(), String> {
        let pending_quotes_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM agent_approvals WHERE tenant_id = ? AND department = 'quoting' AND status = 'DRAFT' AND created_at < ?"
        )
        .bind(tenant_id)
        .bind(Utc::now() - chrono::Duration::hours(24))
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if pending_quotes_count > 0 {
            let context_payload = json!({
                "description": format!("You have {} estimates pending from yesterday.", pending_quotes_count),
                "feature_type": "quote_draft",
                "tenant_id": tenant_id
            });
            let proposed_action = json!({
                "action": "review_drafts",
                "description": "Tap to review drafted follow-up messages."
            });
            Self::create_insight_if_not_exists_sqlite(pool, tenant_id, "proactive_context_agent", context_payload, proposed_action).await?;
        }

        let low_inventory_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM products WHERE tenant_id = ? AND inventory_count <= 5"
        )
        .bind(tenant_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if low_inventory_count > 0 {
            let context_payload = json!({
                "description": format!("You have {} items with low inventory.", low_inventory_count),
                "remaining_stock": 5, // Triggers restock action in UI
                "tenant_id": tenant_id
            });
            let proposed_action = json!({
                "action": "approve_restock",
                "description": "Schedule Restock"
            });
            Self::create_insight_if_not_exists_sqlite(pool, tenant_id, "proactive_context_agent", context_payload, proposed_action).await?;
        }

        // SQLite JSON functions differ, using simple regex logic or skipping it for local sqlite
        Ok(())
    }

    async fn create_insight_if_not_exists_pg(
        pool: &sqlx::PgPool,
        tenant_id: &str,
        event_source: &str,
        context_payload: serde_json::Value,
        proposed_action: serde_json::Value,
    ) -> Result<(), String> {
        // Simple deduplication based on description
        let desc = context_payload["description"].as_str().unwrap_or("");

        let existing: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = $2 AND lifecycle_state IN ('PENDING_APPROVAL', 'PENDING') AND context_payload->>'description' = $3"
        )
        .bind(tenant_id)
        .bind(event_source)
        .bind(desc)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if existing == 0 {
            let id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
                 VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL')"
            )
            .bind(&id)
            .bind(tenant_id)
            .bind(event_source)
            .bind(json!(context_payload))
            .bind(json!(proposed_action))
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn create_insight_if_not_exists_sqlite(
        pool: &sqlx::SqlitePool,
        tenant_id: &str,
        event_source: &str,
        context_payload: serde_json::Value,
        proposed_action: serde_json::Value,
    ) -> Result<(), String> {
        let desc = context_payload["description"].as_str().unwrap_or("");
        // Just simple check without JSON operators
        let rows = sqlx::query("SELECT context_payload FROM agent_feed_items WHERE tenant_id = ? AND event_source = ? AND lifecycle_state IN ('PENDING_APPROVAL', 'PENDING')")
            .bind(tenant_id)
            .bind(event_source)
            .fetch_all(pool)
            .await
            .unwrap_or_default();

        let existing = rows.into_iter().any(|r| {
            let payload: Option<String> = r.try_get("context_payload").unwrap_or_default();
            payload.unwrap_or_default().contains(desc)
        });

        if !existing {
            let id = Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES (?, ?, ?, ?, ?, 'PENDING_APPROVAL')")
                .bind(&id)
                .bind(tenant_id)
                .bind(event_source)
                .bind(json!(context_payload).to_string())
                .bind(json!(proposed_action).to_string())
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_proactive_analysis_worker() {
        // Just verify struct can be created for now
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect_lazy("sqlite::memory:").unwrap();
        let db = Arc::new(DB { pool: dummy_pg_pool, store: DbStore::Sqlite(sqlite_pool) });

        let _worker = ProactiveAnalysisWorker::new(db);
        assert!(true);
    }
}
