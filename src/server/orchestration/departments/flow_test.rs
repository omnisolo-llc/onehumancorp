#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator};
    use crate::orchestration::departments::types::{DepartmentEvent};
    use crate::orchestration::departments::operations_agent::OperationsAgent;
    use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;
    use crate::orchestration::departments::sales_agent::SalesAgent;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
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
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let ops_agent = Arc::new(RwLock::new(OperationsAgent::new(orchestrator.clone())));
        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));
        let sales_agent = Arc::new(RwLock::new(SalesAgent::new(orchestrator.clone())));

        orchestrator.register_department(ops_agent).await;
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

        // Simulate new order event routing to Operations
        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.order.created".to_string(), // Operations agent subscribes to this
            payload: serde_json::json!({"order_id": "12345"}),
        };

        // Operations agent will automatically chain the Customer Success event
        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        // Poll to allow async event handling to complete instead of sleep
        let mut has_ops_auto = false;
        let mut has_cs_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Process Order & Update Inventory")) {
                has_ops_auto = true;
            }
            if pending.iter().any(|req| req.description.contains("Send personalized thank you")) {
                has_cs_draft = true;
            }
            if has_ops_auto && has_cs_draft {
                break;
            }
        }

        assert!(has_ops_auto, "Cross-department flow should result in an Operations task");
        assert!(has_cs_draft, "Cross-department flow should result in a pending Customer Success approval");
    }

    #[tokio::test]
    async fn test_customer_success_message_handling() {
        use crate::orchestration::departments::orchestrator::Department;
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));
        orchestrator.register_department(cs_agent.clone()).await;

        let tenant_id = "test-tenant-456".to_string();

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

        // Add a memory record
        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            agent_id: "customer_success_agent".to_string(),
            content: "We make excellent vegan cakes for special occasions.".to_string(),
            embedding: vec![0.5; 1536],
            source_type: "MANUAL".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 100,
            owner_override: false,
            metadata: None,
        };
        orchestrator.write_long_term_memory(record).await.unwrap();

        // 1. Test Draft Mode (auto_approve_limits = 0.0)
        {
            let mut agent = cs_agent.write().await;
            use crate::orchestration::departments::types::DepartmentConfig;
            agent.set_config(tenant_id.clone(), DepartmentConfig { tone_of_voice: "friendly".to_string(), auto_approve_limits: 0.0 });
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({"message": "Do you do vegan cakes?"}),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Draft email for review")) {
                has_draft = true;
                break;
            }
        }
        assert!(has_draft, "Should generate a draft for review");
    }

    #[tokio::test]
    async fn test_department_service_msgbus_integration() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        use crate::msgbus::{Bus, MemoryBus, Message};
        use crate::services::agent::department::service::DepartmentService;

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let ops_agent = Arc::new(RwLock::new(OperationsAgent::new(orchestrator.clone())));
        orchestrator.register_department(ops_agent).await;

        let tenant_id = "test-tenant-bus-123".to_string();

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

        let memory_bus = Arc::new(MemoryBus::new());
        let department_service = DepartmentService::new(memory_bus.clone(), orchestrator.clone());

        department_service.start().await.unwrap();

        let payload_json = serde_json::json!({
            "tenant_id": tenant_id
        });

        let msg = Message {
            topic: "system:order_received".to_string(),
            payload: payload_json.to_string().into_bytes(),
        };

        memory_bus.publish(msg).await.unwrap();

        let mut has_ops_auto = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Process Order & Update Inventory")) {
                has_ops_auto = true;
                break;
            }
        }

        assert!(has_ops_auto, "Msgbus integration should map system:order_received to an Operations task");
    }

    #[tokio::test]
    async fn test_get_department_status() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let tenant_id = "test-tenant-status-123".to_string();

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

        use crate::orchestration::departments::types::{ApprovalRequest, DepartmentType, ApprovalStatus, ActionRisk};

        let req1 = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            department: DepartmentType::Operations,
            description: "Task 1".to_string(),
            status: ApprovalStatus::PendingApproval,
            action_risk: ActionRisk::DraftForReview,
            payload: None,
        };
        orchestrator.add_approval_request(req1).await;

        let req2 = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            department: DepartmentType::Operations,
            description: "Task 2".to_string(),
            status: ApprovalStatus::Approved,
            action_risk: ActionRisk::AutoExecute,
            payload: None,
        };
        orchestrator.add_approval_request(req2).await;

        let req3 = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            department: DepartmentType::CustomerSuccess,
            description: "Task 3".to_string(),
            status: ApprovalStatus::PendingApproval,
            action_risk: ActionRisk::DraftForReview,
            payload: None,
        };
        orchestrator.add_approval_request(req3).await;

        let statuses = orchestrator.get_department_status(&tenant_id).await;

        let ops = statuses.iter().find(|s| s.department == DepartmentType::Operations).unwrap();
        assert_eq!(ops.pending_approvals, 1);
        assert_eq!(ops.completed_actions, 1);

        let cs = statuses.iter().find(|s| s.department == DepartmentType::CustomerSuccess).unwrap();
        assert_eq!(cs.pending_approvals, 1);
        assert_eq!(cs.completed_actions, 0);

        let sales = statuses.iter().find(|s| s.department == DepartmentType::Sales).unwrap();
        assert_eq!(sales.pending_approvals, 0);
        assert_eq!(sales.completed_actions, 0);
    }
}
