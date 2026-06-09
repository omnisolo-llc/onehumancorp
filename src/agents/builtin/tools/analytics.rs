use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};
use super::{Tool, ToolExecutor};

static PG_POOL: OnceLock<sqlx::PgPool> = OnceLock::new();
static SQLITE_POOL: OnceLock<sqlx::SqlitePool> = OnceLock::new();

pub struct AnalyticsQueryExecutor;

#[async_trait::async_trait]
impl ToolExecutor for AnalyticsQueryExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().unwrap_or("default");
        let db_url = std::env::var("OHC_DATABASE_URL").or_else(|_| std::env::var("DATABASE_URL")).unwrap_or_default();

        if db_url.is_empty() {
            return Err(ToolError::Fatal("No database connection string provided.".to_string()));
        }

        let is_sqlite = db_url.starts_with("sqlite");

        let (active_customers, pending_orders, total_sales, total_campaigns_sent) = if is_sqlite {
            let pool = if let Some(p) = SQLITE_POOL.get() {
                p.clone()
            } else {
                let p = sqlx::SqlitePool::connect(&db_url).await.map_err(|e| ToolError::Fatal(format!("Failed to connect to sqlite: {}", e)))?;
                SQLITE_POOL.set(p.clone()).map_err(|_| ToolError::Fatal("Failed to set SQLITE_POOL".to_string()))?;
                p
            };

            sqlx::query_as::<_, (i64, i64, f64, i64)>(
                "SELECT \
                    (SELECT COUNT(*) FROM customers WHERE tenant_id = ?) AS active_customers, \
                    (SELECT COUNT(*) FROM orders WHERE tenant_id = ? AND status = 'pending') AS pending_orders, \
                    (SELECT COALESCE(SUM(total_amount), 0.0) FROM orders WHERE tenant_id = ?) AS total_sales, \
                    (SELECT COUNT(*) FROM agent_actions WHERE tenant_id = ? AND action_type = 'growth.campaign_sent') AS total_campaigns_sent"
            )
            .bind(tenant_id)
            .bind(tenant_id)
            .bind(tenant_id)
            .bind(tenant_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| ToolError::Fatal(format!("Failed to fetch analytics from sqlite: {}", e)))?
        } else {
            let pool = if let Some(p) = PG_POOL.get() {
                p.clone()
            } else {
                let p = sqlx::PgPool::connect(&db_url).await.map_err(|e| ToolError::Fatal(format!("Failed to connect to postgres: {}", e)))?;
                PG_POOL.set(p.clone()).map_err(|_| ToolError::Fatal("Failed to set PG_POOL".to_string()))?;
                p
            };

            sqlx::query_as::<_, (i64, i64, f64, i64)>(
                "SELECT \
                    (SELECT COUNT(*) FROM customers WHERE tenant_id = $1) AS active_customers, \
                    (SELECT COUNT(*) FROM orders WHERE tenant_id = $1 AND status = 'pending') AS pending_orders, \
                    (SELECT COALESCE(SUM(total_amount), 0.0)::DOUBLE PRECISION FROM orders WHERE tenant_id = $1) AS total_sales, \
                    (SELECT COUNT(*) FROM agent_actions WHERE tenant_id = $1 AND action_type = 'growth.campaign_sent') AS total_campaigns_sent"
            )
            .bind(tenant_id)
            .fetch_one(&pool)
            .await
            .map_err(|e| ToolError::Fatal(format!("Failed to fetch analytics from postgres: {}", e)))?
        };

        Ok(json!({
            "status": "success",
            "active_customers": active_customers,
            "pending_orders": pending_orders,
            "total_sales": total_sales,
            "total_campaigns_sent": total_campaigns_sent,
        }).to_string())
    }
}

pub fn analytics_tool() -> Tool {
    Tool {
        name: "get_dashboard_metrics".to_string(),
        description: "Fetch real-time aggregated business metrics including active customers, pending orders, total sales, and total campaigns sent.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": {
                    "type": "string",
                    "description": "The tenant ID to fetch metrics for."
                }
            },
            "required": ["tenant_id"]
        }),
        execute: Arc::new(AnalyticsQueryExecutor),
    }
}
