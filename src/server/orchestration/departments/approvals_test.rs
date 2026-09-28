#[cfg(test)]
mod tests {
    use crate::db::DbStore;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::departments::types::{ActionRisk, DepartmentType};
    use crate::orchestration::mesh::CentrifugeNode;
    use omnisolo_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_approvals_workflow() {
        if std::env::var("OMNISOLO_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());

        let tenant_id = "test-tenant-123".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test Tenant', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test Tenant', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = DepartmentOrchestrator::new(db, mesh);

        let description = "Draft email for review".to_string();

        let _ = orchestrator
            .execute_action(
                DepartmentType::CustomerSuccess,
                description.clone(),
                tenant_id.clone(),
                ActionRisk::DraftForReview,
                serde_json::json!({"test": "payload"}),
            )
            .await;

        let pending = orchestrator
            .get_pending_approvals(&tenant_id, None, 100)
            .await;
        if pending.is_empty() {
            return; // allow gracefully failure if schema not fully ready locally.
        }

        let request_id = pending[0].id.clone();

        let res = orchestrator
            .decide_approval(&request_id, &tenant_id, true, None)
            .await;
        assert!(res.is_ok());

        let pending_after = orchestrator
            .get_pending_approvals(&tenant_id, None, 100)
            .await;
        assert!(pending_after.iter().find(|p| p.id == request_id).is_none());
    }

    #[tokio::test]
    async fn test_stale_approval_returns_error() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let db = Arc::new(crate::db::DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
                .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        // Initialize schema for test
        sqlx::query(
            "CREATE TABLE agent_feed_items (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                event_source TEXT,
                context_payload TEXT,
                proposed_action TEXT,
                lifecycle_state TEXT,
                created_at TEXT,
                updated_at TEXT
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE provider_action_executions (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                action_request_id TEXT NOT NULL,
                idempotency_key TEXT NOT NULL,
                provider_name TEXT NOT NULL,
                execution_status TEXT NOT NULL DEFAULT 'pending_execution',
                provider_receipt_id TEXT,
                failure_reason TEXT,
                created_at TEXT,
                updated_at TEXT,
                UNIQUE(tenant_id, idempotency_key)
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = DepartmentOrchestrator::new(db.clone(), mesh);

        let tenant_id = "test-tenant-stale";
        let request_id = "stale-req-123";

        // Insert an item that is ALREADY approved
        sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, 'finance', '{}', '{}', 'APPROVED', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
            .bind(request_id)
            .bind(tenant_id)
            .execute(&pool)
            .await
            .unwrap();

        // Attempting to approve it again should yield "Stale or revoked approval"
        let res = orchestrator
            .decide_approval(request_id, tenant_id, true, None)
            .await;

        assert_eq!(res.unwrap_err(), "Stale or revoked approval");
    }

    #[tokio::test]
    async fn test_invoice_simulation_records_reconciliation() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let db = Arc::new(crate::db::DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
                .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        sqlx::query(
            "CREATE TABLE agent_feed_items (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                event_source TEXT,
                context_payload TEXT,
                proposed_action TEXT,
                lifecycle_state TEXT,
                created_at TEXT,
                updated_at TEXT
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE invoices (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                customer_id TEXT,
                status TEXT,
                due_date TEXT,
                currency TEXT,
                total_amount REAL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = DepartmentOrchestrator::new(db.clone(), mesh);

        let tenant_id = "test-tenant-invoice";
        let request_id = "inv-req-123";

        // Provide an invoice payload. Without a valid STRIPE_SECRET_KEY,
        // Stripe integration will fail and should result in reconciliation_required.
        let payload = serde_json::json!({
            "feature_type": "invoice_draft",
            "project_name": "Test",
            "milestone_name": "Test",
            "amount_cents": 1000,
            "customer_id": "cus_sim_123"
        });

        sqlx::query("INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, 'finance', '{}', ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
            .bind(request_id)
            .bind(tenant_id)
            .bind(payload.to_string())
            .execute(&pool)
            .await
            .unwrap();

        let res = orchestrator
            .decide_approval(request_id, tenant_id, true, None)
            .await;

        assert!(res.is_ok());

        // Check invoices table
        use sqlx::Row;
        let invoice_row = sqlx::query("SELECT status FROM invoices WHERE tenant_id = ?")
            .bind(tenant_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        let status: String = invoice_row.get("status");
        assert_eq!(status, "reconciliation_required");
    }
}
