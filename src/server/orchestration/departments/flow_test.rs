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
        if std::env::var("OHC_DATABASE_URL").is_err() {
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
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
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
        #[allow(unused_imports)]
        use crate::orchestration::departments::orchestrator::Department;
        if std::env::var("OHC_DATABASE_URL").is_err() {
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
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }


        // Add a memory record
        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: uuid::Uuid::new_v4().to_string(),
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

        // Let's seed the customer_timeline instead because customer_success reads from it now
        let timeline_event = crate::orchestration::departments::types::TimelineEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            customer_id: "unknown_customer".to_string(),
            event_type: "memory".to_string(),
            source: "system".to_string(),
            content: "We make excellent vegan cakes for special occasions.".to_string(),
            metadata: None,
            created_at: None,
        };
        orchestrator.append_to_timeline(timeline_event).await.unwrap();


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
        let mut draft_request_id = String::new();
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if let Some(req) = pending.iter().find(|req| req.description.contains("Draft email for review")) {
                has_draft = true;
                draft_request_id = req.id.clone();
                break;
            }
        }
        assert!(has_draft, "Should generate a draft for review");

        // 2. Approve the draft
        let decide_res = orchestrator.decide_approval(&draft_request_id, &tenant_id, true, None).await;
        assert!(decide_res.is_ok());

        // Wait a bit for the approved event to propagate and be processed by the CustomerSuccessAgent
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        // In a real scenario we'd assert on a mock sink, but for now we rely on the agent not panicking
        // and tracing out the EXECUTING APPROVED DRAFT line.

    }

    #[tokio::test]
    async fn test_department_service_msgbus_integration() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
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
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
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
    async fn test_marketing_job_completed_case_study() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        use crate::orchestration::departments::marketing_agent::MarketingAgent;

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let marketing_agent = Arc::new(RwLock::new(MarketingAgent::new(orchestrator.clone())));
        orchestrator.register_department(marketing_agent.clone()).await;

        let tenant_id = "test-tenant-marketing-1".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.job.completed".to_string(),
            payload: serde_json::json!({
                "service_name": "Cedar Fence Install",
                "media": ["https://example.com/finished-fence.jpg"]
            }),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_case_study_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Draft portfolio case study for Cedar Fence Install")) {
                has_case_study_draft = true;
                break;
            }
        }

        assert!(has_case_study_draft, "Marketing Agent should draft a case study when a job is completed with media.");
    }

    #[tokio::test]
    async fn test_marketing_product_created_social_post() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        use crate::orchestration::departments::marketing_agent::MarketingAgent;

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let marketing_agent = Arc::new(RwLock::new(MarketingAgent::new(orchestrator.clone())));
        orchestrator.register_department(marketing_agent.clone()).await;

        let tenant_id = "test-tenant-marketing-post".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.product.created".to_string(),
            payload: serde_json::json!({
                "name": "Vegan Chocolate Cake",
                "description": "Delicious vegan cake.",
                "images": ["https://example.com/cake.jpg"]
            }),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_social_post_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Draft Instagram post for Vegan Chocolate Cake")) {
                has_social_post_draft = true;
                break;
            }
        }

        assert!(has_social_post_draft, "Marketing Agent should draft a social post when a product is created.");
    }

    #[tokio::test]
    async fn test_sales_agent_quoting() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        use crate::orchestration::departments::sales_agent::SalesAgent;

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let sales_agent = Arc::new(RwLock::new(SalesAgent::new(orchestrator.clone())));
        orchestrator.register_department(sales_agent.clone()).await;

        let tenant_id = "test-tenant-sales-quote".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET plan_tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;

                let _ = sqlx::query("INSERT INTO services (id, tenant_id, name, description, duration_minutes, price) VALUES ($1, $2, 'Plumbing Fix', 'Fix plumbing issues', 60, 75.0) ON CONFLICT (id) DO NOTHING")
                    .bind(Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;

                // In SQLite schema this might be different but similar to postgres
                let _ = sqlx::query("INSERT INTO services (id, tenant_id, name, description, duration_minutes, price) VALUES (?, ?, 'Plumbing Fix', 'Fix plumbing issues', 60, 75.0) ON CONFLICT (id) DO NOTHING")
                    .bind(Uuid::new_v4().to_string())
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({
                "message": "My sink is leaking, can you fix it tomorrow?"
            }),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_quote_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Draft quote for Plumbing Fix")) {
                has_quote_draft = true;
                break;
            }
        }

        assert!(has_quote_draft, "Sales Agent should draft a quote based on the message intent.");
    }

    #[tokio::test]
    async fn test_legal_agent_compliance_check_generates_review_draft() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        use crate::orchestration::departments::legal_agent::LegalAgent;

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let legal_agent = Arc::new(RwLock::new(LegalAgent::new(orchestrator.clone())));
        orchestrator.register_department(legal_agent).await;

        let tenant_id = "test-tenant-legal-agent".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.compliance.check_needed".to_string(),
            payload: serde_json::json!({
                "region": "EU",
                "reason": "approaching_vat_threshold"
            }),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_legal_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Draft compliance terms and policy update")) {
                has_legal_draft = true;
                break;
            }
        }

        assert!(has_legal_draft, "LegalAgent should create a compliance approval draft.");
    }

    #[tokio::test]
    async fn test_business_advisory_agent_weekly_health_generates_review_draft() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        use crate::orchestration::departments::business_advisory_agent::BusinessAdvisoryAgent;

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let advisory_agent = Arc::new(RwLock::new(BusinessAdvisoryAgent::new(orchestrator.clone())));
        orchestrator.register_department(advisory_agent).await;

        let tenant_id = "test-tenant-advisory-agent".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.report.weekly_health".to_string(),
            payload: serde_json::json!({
                "gross_sales": 4200,
                "repeat_customer_rate": 0.37,
                "recommended_action": "promote weekend catering slots"
            }),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_advisory_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Draft weekly business health report")) {
                has_advisory_draft = true;
                break;
            }
        }

        assert!(has_advisory_draft, "BusinessAdvisoryAgent should create a weekly health approval draft.");
    }
}
    #[tokio::test]
    async fn test_predictive_restock_draft() {
        #[allow(unused_imports)]
        use crate::orchestration::departments::orchestrator::Department;
        use std::sync::Arc;
        use tokio::sync::RwLock;
        use ohc_builtin_agent::mesh::transport::InProcessTransport;
        use crate::orchestration::mesh::CentrifugeNode;
        use crate::orchestration::departments::DepartmentOrchestrator;
        use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;
        use crate::db::DbStore;
        use crate::orchestration::departments::DepartmentEvent;

        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let cs_agent = Arc::new(RwLock::new(CustomerSuccessAgent::new(orchestrator.clone())));
        orchestrator.register_department(cs_agent.clone()).await;

        let tenant_id = "test-tenant-restock".to_string();
        let customer_id = "cust-123".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO NOTHING")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;

                // insert past orders
                let _ = sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id TEXT, created_at TIMESTAMPTZ)").execute(&db.pool).await;

                let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, created_at) VALUES ('o1', $1, $2, NOW() - INTERVAL '30 days')")
                    .bind(&tenant_id).bind(&customer_id).execute(&db.pool).await;
                let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, created_at) VALUES ('o2', $1, $2, NOW() - INTERVAL '15 days')")
                    .bind(&tenant_id).bind(&customer_id).execute(&db.pool).await;
                let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, created_at) VALUES ('o3', $1, $2, NOW())")
                    .bind(&tenant_id).bind(&customer_id).execute(&db.pool).await;
            }
            _ => return, // SQLite not fully mocked in this test snippet context usually
        }

        let event = DepartmentEvent {
            id: "evt-restock-1".to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.subscription.check_predictive_restock".to_string(),
            payload: serde_json::json!({
                "customer_id": customer_id
            }),
        };

        let res = orchestrator.dispatch_event(event).await;
        assert!(res.is_ok());

        let mut has_restock_draft = false;
        for _ in 0..10 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;
            if pending.iter().any(|req| req.description.contains("Predictive Restock Draft")) {
                has_restock_draft = true;
                break;
            }
        }

        assert!(has_restock_draft, "Predictive restock check should result in a pending draft");
    }

    #[tokio::test]
    async fn test_redlock_prevents_double_booking_during_quote() {
        use std::sync::Arc;
        use uuid::Uuid;
        use crate::db::DbStore;
        use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
        use crate::orchestration::departments::types::{DepartmentEvent};
        use crate::orchestration::mesh::CentrifugeNode;
        use ohc_builtin_agent::mesh::transport::InProcessTransport;

        if std::env::var("OHC_DATABASE_URL").is_err() {
            return; // Skip if no DB config
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh.clone()));

        use crate::orchestration::departments::sales_agent::SalesAgent;

        let sales_agent = Arc::new(tokio::sync::RwLock::new(SalesAgent::new(orchestrator.clone())));
        orchestrator.register_department(sales_agent.clone()).await;

        let tenant_id = Uuid::new_v4().to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ($1, 'Redlock Test', 'starter') ON CONFLICT (id) DO UPDATE SET plan_tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;

                let _ = sqlx::query("INSERT INTO services (id, tenant_id, title, description, price_cents) VALUES ($1, $2, 'Handyman Fix', 'Fix things', 7500) ON CONFLICT (id) DO NOTHING")
                    .bind("service-handyman-1")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES (?, 'Redlock Test', 'starter') ON CONFLICT (id) DO UPDATE SET plan_tier = 'starter'")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;

                let _ = sqlx::query("INSERT INTO services (id, tenant_id, title, description, price_cents) VALUES (?, ?, 'Handyman Fix', 'Fix things', 7500) ON CONFLICT (id) DO NOTHING")
                    .bind("service-handyman-1")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event1 = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.work_intake.received".to_string(),
            payload: serde_json::json!({
                "message": "I need a Handyman Fix tomorrow at 2 PM."
            }),
        };

        let event2 = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.work_intake.received".to_string(),
            payload: serde_json::json!({
                "message": "Can I get a Handyman Fix tomorrow at 2 PM?"
            }),
        };

        let (res1, res2) = tokio::join!(
            orchestrator.dispatch_event(event1),
            orchestrator.dispatch_event(event2)
        );

        assert!(res1.is_ok());
        assert!(res2.is_ok());

        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        let pending = orchestrator.get_pending_approvals(&tenant_id, None, 100).await;

        let mut soft_locked_count = 0;
        let mut failed_to_lock_count = 0;

        for req in pending {
            if req.description.contains("Draft quote for Handyman Fix") {
                if let Some(payload) = req.payload {
                    if payload.get("proposed_slot_id").and_then(|v| v.as_str()).is_some() {
                        soft_locked_count += 1;
                    } else {
                        failed_to_lock_count += 1;
                    }
                }
            }
        }

        assert_eq!(soft_locked_count, 1, "Exactly one request should successfully acquire the soft lock.");
        assert_eq!(failed_to_lock_count, 1, "The second request should fail to acquire the lock and have None for proposed_slot_id.");
    }

#[cfg(test)]
mod finance_tests {
    use super::*;
    use crate::orchestration::departments::finance_agent::FinanceAgent;
    use crate::orchestration::departments::types::{DepartmentType, ApprovalStatus};

    #[tokio::test]
    async fn test_finance_agent_project_milestone_invoice_draft() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        use std::sync::Arc;
        use crate::db::{DB, DbStore};
        use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, Department};
        use crate::orchestration::departments::types::{DepartmentEvent};
        use crate::orchestration::mesh::CentrifugeNode;
        use ohc_builtin_agent::mesh::transport::InProcessTransport;
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh.clone()));

        let tenant_id = "test-tenant-finance-invoice".to_string();

        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test', 'starter') ON CONFLICT (id) DO NOTHING")
                    .bind(&tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test', 'starter') ON CONFLICT (tenant_id) DO NOTHING")
                    .bind(&tenant_id)
                    .execute(pool)
                    .await;
            }
        }

        let event = DepartmentEvent {
            id: "evt-proj-1".to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "project_milestone_completed".to_string(),
            payload: serde_json::json!({
                "project_id": "proj-1",
                "project_title": "Redesign Phase 1",
                "customer_id": "cust-1",
                "customer_name": "Nora Client",
                "amount": 1000.0
            }),
        };

        let finance_agent = FinanceAgent::new(orchestrator.clone());
        finance_agent.handle_event(&event).await.unwrap();

        let approvals = orchestrator.get_activity_feed(&tenant_id, None, 10).await;

        let mut has_draft = false;
        let mut req_id = String::new();
        for approval in approvals {
            if approval.department == DepartmentType::Finance && approval.status == ApprovalStatus::PendingApproval {
                if let Some(payload) = approval.payload {
                    if payload.get("feature_type").and_then(|v| v.as_str()) == Some("draft_invoice") {
                        has_draft = true;
                        req_id = approval.id.clone();
                        break;
                    }
                }
            }
        }

        assert!(has_draft, "Finance Agent should create a draft invoice approval");

        // Approve it
        let decide_res = orchestrator.decide_approval(&req_id, &tenant_id, true, None).await;
        assert!(decide_res.is_ok(), "Should approve invoice successfully");
    }
}
