#[cfg(test)]
mod tests {
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig};
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_multi_tenant_config_isolation() {
        if std::env::var("DATABASE_URL").is_err() {
            unsafe { std::env::set_var("DATABASE_URL", "sqlite::memory:"); }
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        db.run_migrations().await.unwrap();
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = DepartmentOrchestrator::new(db.clone(), mesh);

        let tenant_a = format!("tenant-a-{}", Uuid::new_v4());
        let tenant_b = format!("tenant-b-{}", Uuid::new_v4());

        // Set config for Tenant A
        let config_a = DepartmentConfig {
            tone_of_voice: "funny".to_string(),
            auto_approve_limits: 0.0,
            auto_execute_enabled: true,
        };
        orchestrator.update_department_config(&tenant_a, "operations", config_a.clone()).await.unwrap();

        // Load config for Tenant B (should be default)
        let config_b_loaded = orchestrator.load_department_config(&tenant_b, DepartmentType::Operations).await.unwrap();
        assert_eq!(config_b_loaded.auto_execute_enabled, false);
        assert_eq!(config_b_loaded.tone_of_voice, "professional");

        // Load config for Tenant A
        let config_a_loaded = orchestrator.load_department_config(&tenant_a, DepartmentType::Operations).await.unwrap();
        assert_eq!(config_a_loaded.auto_execute_enabled, true);
        assert_eq!(config_a_loaded.tone_of_voice, "funny");
    }

    #[tokio::test]
    async fn test_multi_tenant_memory_isolation() {
        if std::env::var("DATABASE_URL").is_err() {
            unsafe { std::env::set_var("DATABASE_URL", "sqlite::memory:"); }
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        db.run_migrations().await.unwrap();
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = DepartmentOrchestrator::new(db.clone(), mesh);

        let tenant_a = format!("tenant-a-{}", Uuid::new_v4());
        let tenant_b = format!("tenant-b-{}", Uuid::new_v4());

        let record_a = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_a.clone(),
            agent_id: "agent_a".to_string(),
            content: "Secret data for Tenant A".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "MANUAL".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 0,
            reliability_score: 100,
            owner_override: false,
            metadata: None,
        };
        orchestrator.write_long_term_memory(record_a).await.unwrap();

        // Search from Tenant B
        let query_embedding = vec![0.1; 1536];
        let results_b = orchestrator.query_long_term_memory(&tenant_b, &query_embedding, 10).await.unwrap();
        assert!(results_b.is_empty(), "Tenant B should not see Tenant A's memory");

        // Search from Tenant A
        let results_a = orchestrator.query_long_term_memory(&tenant_a, &query_embedding, 10).await.unwrap();
        assert_eq!(results_a.len(), 1);
        assert_eq!(results_a[0], "Secret data for Tenant A");
    }
}
