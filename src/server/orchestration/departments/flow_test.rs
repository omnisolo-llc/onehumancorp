#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::{DepartmentEvent};
    use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;
    use crate::orchestration::departments::sales_agent::SalesAgent;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;
    use crate::db::DbStore;

    #[tokio::test]
    async fn test_cross_department_flow() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));
        let sales_agent = Arc::new(RwLock::new(SalesAgent::new(orchestrator.clone())));

        orchestrator.register_department(cs_agent).await;
        orchestrator.register_department(sales_agent).await;

        let tenant_id = "test-tenant-123".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES ($1, 100) ON CONFLICT (tenant_id) DO UPDATE SET ai_budget = 100")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES (?, 100) ON CONFLICT (tenant_id) DO UPDATE SET ai_budget = 100")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({"message": "Do you make vegan cakes? How much?"}),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        // Poll to allow async event handling to complete instead of sleep
        let mut has_quote = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id).await;
            if pending.iter().any(|req| req.description.contains("Quote generated for review")) {
                has_quote = true;
                break;
            }
        }

        assert!(has_quote, "Cross-department flow should result in a pending quote approval");
    }

    #[tokio::test]
    async fn test_order_placed_flow() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let ops_agent = Arc::new(RwLock::new(crate::orchestration::departments::operations_agent::OperationsAgent::new(orchestrator.clone())));
        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));

        orchestrator.register_department(ops_agent).await;
        orchestrator.register_department(cs_agent).await;

        let tenant_id = "test-tenant-456".to_string();
        let product_id = "test-product-1".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES ($1, 100) ON CONFLICT (tenant_id) DO UPDATE SET ai_budget = 100")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;

                let _ = sqlx::query("INSERT INTO products (id, organization_id, name, inventory_count) VALUES ($1, $2, 'Cake', 10) ON CONFLICT (id) DO UPDATE SET inventory_count = 10")
                    .bind(&product_id)
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, ai_budget) VALUES (?, 100) ON CONFLICT (tenant_id) DO UPDATE SET ai_budget = 100")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;

                let _ = sqlx::query("INSERT INTO products (id, organization_id, name, inventory_count) VALUES (?, ?, 'Cake', 10) ON CONFLICT (id) DO UPDATE SET inventory_count = 10")
                    .bind(&product_id)
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "OrderPlaced".to_string(),
            payload: serde_json::json!({"items": [{"product_id": product_id.clone(), "quantity": 2}]}),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        // Poll to allow async event handling to complete instead of sleep
        let mut inventory_updated = false;
        let mut has_draft = false;

        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;

            // Check inventory
            let inventory_count: i32 = match &db.store {
                DbStore::Postgres => {
                    let r = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND organization_id = $2")
                        .bind(&product_id)
                        .bind(&tenant_id)
                        .fetch_one(&db.pool)
                        .await;
                    r.map(|row| sqlx::Row::try_get(&row, "inventory_count").unwrap_or(10)).unwrap_or(10)
                }
                DbStore::Sqlite(pool) => {
                    let r = sqlx::query("SELECT inventory_count FROM products WHERE id = ? AND organization_id = ?")
                        .bind(&product_id)
                        .bind(&tenant_id)
                        .fetch_one(pool)
                        .await;
                    r.map(|row| sqlx::Row::try_get(&row, "inventory_count").unwrap_or(10)).unwrap_or(10)
                }
            };

            if inventory_count == 8 {
                inventory_updated = true;
            }

            let pending = orchestrator.get_pending_approvals(&tenant_id).await;
            // The action risk is AutoExecute, so status is Approved. We actually should check action_counter or check if it was processed.
            // The method `get_pending_approvals` might not return it if it's already approved.
            // Wait let's just check the DB directly for agent_approvals where description contains 'Draft personalized order confirmation'

            let count: i64 = match &db.store {
                DbStore::Postgres => {
                    let r = sqlx::query("SELECT count(*) FROM agent_approvals WHERE tenant_id = $1 AND description LIKE '%Draft personalized order confirmation%'")
                        .bind(&tenant_id)
                        .fetch_one(&db.pool)
                        .await;
                    r.map(|row| sqlx::Row::try_get(&row, "count").unwrap_or(0)).unwrap_or(0)
                }
                DbStore::Sqlite(pool) => {
                    let r = sqlx::query("SELECT count(*) FROM agent_approvals WHERE tenant_id = ? AND description LIKE '%Draft personalized order confirmation%'")
                        .bind(&tenant_id)
                        .fetch_one(pool)
                        .await;
                    r.map(|row| sqlx::Row::try_get(&row, "count").unwrap_or(0)).unwrap_or(0)
                }
            };
            if count > 0 {
                has_draft = true;
            }
            if inventory_updated && has_draft {
                break;
            }
        }

        assert!(inventory_updated, "Inventory should be decremented");
        assert!(has_draft, "Customer success agent should generate auto-executed confirmation");
    }
}
