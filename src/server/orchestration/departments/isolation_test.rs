#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig};
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_tenant_configuration_isolation() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = DepartmentOrchestrator::new(db, mesh);

        let tenant_a = "tenant_a";
        let tenant_b = "tenant_b";

        let config_a = DepartmentConfig {
            tone_of_voice: "Casual".to_string(),
            auto_approve_limits: 50.0,
            auto_execute_enabled: true,
        };

        let config_b = DepartmentConfig {
            tone_of_voice: "Formal".to_string(),
            auto_approve_limits: 500.0,
            auto_execute_enabled: false,
        };

        // Set configurations for both tenants
        orchestrator.update_department_config(tenant_a, "Operations", config_a.clone()).await.unwrap();
        orchestrator.update_department_config(tenant_b, "Operations", config_b.clone()).await.unwrap();

        // Load back and verify isolation
        let loaded_a = orchestrator.load_department_config(tenant_a, DepartmentType::Operations).await.unwrap();
        let loaded_b = orchestrator.load_department_config(tenant_b, DepartmentType::Operations).await.unwrap();

        assert_eq!(loaded_a.tone_of_voice, "Casual");
        assert_eq!(loaded_a.auto_execute_enabled, true);

        assert_eq!(loaded_b.tone_of_voice, "Formal");
        assert_eq!(loaded_b.auto_execute_enabled, false);
    }

    #[tokio::test]
    async fn test_tenant_memory_isolation() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = DepartmentOrchestrator::new(db, mesh);

        let tenant_a = "tenant_a_mem";
        let tenant_b = "tenant_b_mem";

        let embedding = vec![0.1; 1536];

        // Write memory for Tenant A
        let record_a = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "mem_a".to_string(),
            tenant_id: tenant_a.to_string(),
            agent_id: "ops".to_string(),
            content: "Secret recipe for Tenant A".to_string(),
            embedding: embedding.clone(),
            source_type: "DOC".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 0,
            reliability_score: 100,
            owner_override: false,
            metadata: None,
        };
        orchestrator.write_long_term_memory(record_a).await.unwrap();

        // Query memory from Tenant B's context
        let query_embedding = vec![0.1; 1536];
        let results_b = orchestrator.query_long_term_memory(tenant_b, &query_embedding, 10).await.unwrap();

        // Results for Tenant B should NOT contain Tenant A's content
        for res in results_b {
            assert!(!res.contains("Tenant A"), "Data leakage detected: Tenant B saw Tenant A's memory");
        }

        // Query memory from Tenant A's context
        let results_a = orchestrator.query_long_term_memory(tenant_a, &query_embedding, 10).await.unwrap();
        assert!(results_a.iter().any(|r| r.contains("Tenant A")), "Tenant A should see its own memory");
    }
}
