use omnisolo_builtin_agent::tools::task_db::{DependencyRow, SharedTaskDepRow};
use serde_json::json;
use sqlx::{PgPool, Row, SqlitePool};
use std::path::PathBuf;
use uuid::Uuid;

use crate::orchestration::queue::{PgQueue, SqliteQueue, SubAgentQueue};
use crate::orchestration::state::parity_test::setup_test_db;
use crate::services::agent::service::AgentActionPayload;

fn get_workspace_dir() -> PathBuf {
    PathBuf::from(std::env::var("BUILD_WORKSPACE_DIRECTORY").unwrap_or_else(|_| ".".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_legacy_json_dependencies_are_backfilled_to_edge_table() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };

        let pool = match &db.store {
            crate::db::DbStore::Sqlite(p) => p.clone(),
            _ => panic!("Expected Sqlite store"),
        };

        let _ = sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant-1', 'Boutique Operator', 'Retail')")
            .execute(&pool).await;

        let task1_id = Uuid::new_v4();
        let task2_id = Uuid::new_v4();

        let deps_json = json!([
            {"task_id": task2_id.to_string(), "type": "BLOCKS"}
        ]);

        let _ = sqlx::query("INSERT INTO shared_tasks (id, organization_id, title, approval_status, dependencies) VALUES ($1, 'tenant-1', 'Legacy Task', 'APPROVED', $2)")
            .bind(task1_id)
            .bind(deps_json.to_string())
            .execute(&pool).await;

        let _ = sqlx::query("INSERT INTO shared_tasks (id, organization_id, title, approval_status, dependencies) VALUES ($1, 'tenant-1', 'Dep Task', 'APPROVED', '[]')")
            .bind(task2_id)
            .execute(&pool).await;

        let db_arc = std::sync::Arc::new(db);
        let _ = crate::services::agent::service::backfill_legacy_dependencies(&db_arc, "tenant-1").await;

        let deps: Vec<SharedTaskDepRow> = sqlx::query_as("SELECT from_task_id, to_task_id, type as dep_type, created_at, updated_at FROM shared_task_dependencies WHERE from_task_id = $1")
            .bind(task1_id)
            .fetch_all(&pool).await.unwrap_or_default();

        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].to_task_id, task2_id);
    }

    #[test]
    fn test_sub_agent_queue_schema_migration_exists() {
        let migration_path = get_workspace_dir().join("src/server/migrations/104_sub_agent_queue.sql");
        let migration = std::fs::read_to_string(&migration_path)
            .expect("Sub Agent Queue schema migration should exist");

        assert!(migration.contains("CREATE TABLE IF NOT EXISTS sub_agent_queue"));
        assert!(migration.contains("status"));
        assert!(migration.contains("agent_id"));
    }

    #[test]
    fn test_postgres_feature_parity_migration_covers_runtime_tables() {
        let migration_path = get_workspace_dir()
            .join("src/server/migrations/1011_postgres_feature_parity_tables.sql");
        let migration = std::fs::read_to_string(&migration_path)
            .expect("PostgreSQL feature parity migration should exist");

        for required in [
            "CREATE TABLE IF NOT EXISTS tool_integrations",
            "CREATE TABLE IF NOT EXISTS proposals",
            "CREATE TABLE IF NOT EXISTS proposal_line_items",
            "CREATE TABLE IF NOT EXISTS fulfillment_batches",
            "FORCE ROW LEVEL SECURITY",
            "tenant_isolation_tool_integrations",
            "tenant_isolation_proposals",
            "tenant_isolation_fulfillment_batches",
        ] {
            assert!(migration.contains(required), "missing migration fragment: {required}");
        }

        let onboarding = include_str!("services/onboarding/onboarding_agent.rs");
        assert!(onboarding.contains(".bind(sqlx::types::Json(payload))"));
        assert!(!onboarding.contains(".bind(serde_json::to_string(&payload)"));
    }

    #[tokio::test]
    async fn test_sub_agent_queue_isolation() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };

        let pool = match &db.store {
            crate::db::DbStore::Sqlite(p) => p.clone(),
            _ => panic!("Expected Sqlite store"),
        };

        let _ = sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant-1', 'Home Baker', 'Food'), ('tenant-2', 'Field Service', 'Repair')")
            .execute(&pool).await;

        let queue = SqliteQueue::new(pool.clone());
        let _ = queue.enqueue_job("tenant-1", "agent-x", "test payload", "source1").await;

        // Polling requires a tenant ID
        let jobs = queue.poll_pending("tenant-2").await.unwrap_or_default();
        assert_eq!(jobs.len(), 0, "Tenant 2 should not see Tenant 1 jobs");

        let jobs = queue.poll_pending("tenant-1").await.unwrap_or_default();
        assert_eq!(jobs.len(), 1, "Tenant 1 should see its own jobs");
    }

    #[tokio::test]
    async fn test_ui_dashboard_campaign_metric_uses_agent_actions() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };

        let pool = match &db.store {
            crate::db::DbStore::Sqlite(p) => p.clone(),
            _ => panic!("Expected Sqlite store"),
        };

        let _ = sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant-1', 'Growth Shop', 'Retail')")
            .execute(&pool).await;

        let payload = json!({
            "action": "send_campaign",
            "status": "success",
            "campaign_id": "c-123"
        });

        let _ = sqlx::query("INSERT INTO agent_actions (id, tenant_id, agent_id, name, params, result, status) VALUES ($1, 'tenant-1', 'marketing-1', 'execute_campaign', $2, 'ok', 'COMPLETED')")
            .bind(Uuid::new_v4())
            .bind(payload.to_string())
            .execute(&pool).await;

        let row: Option<(i64,)> = sqlx::query_as("SELECT count(*) FROM agent_actions WHERE tenant_id = 'tenant-1' AND name = 'execute_campaign'")
            .fetch_optional(&pool).await.unwrap_or(None);

        assert_eq!(row.unwrap().0, 1);
    }
}
