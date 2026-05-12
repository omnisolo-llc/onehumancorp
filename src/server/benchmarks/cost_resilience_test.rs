#[cfg(test)]
mod tests {
    use crate::storage::local_provider::LocalProvider;
    use crate::storage::provider::Provider;
    use std::fs;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cost_resilience_concurrent_writes() {
        let temp_dir = std::env::temp_dir().join(format!("ohc_resilience_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();
        let p = LocalProvider::new(&temp_dir).unwrap();
        let provider = Arc::new(p);

        let mut handles = vec![];
        for i in 0..25 {
            let p_clone = provider.clone();
            handles.push(tokio::spawn(async move {
                let key = format!("tenant1/file_{}.txt", i);
                let data = vec![0u8; 1024 * 1024]; // 1MB each
                p_clone.write_blob(&key, &data).await
            }));
        }

        for h in handles {
            let res = h.await.unwrap();
            assert!(res.is_ok());
        }

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_audit_metric_lossless() {
        use crate::services::billing::auditor::{CostAuditor, AuditEvent};
        use ::server_pricing::calculator::CostConfig;

        let config = CostConfig::default();
        let auditor = CostAuditor::new(config);

        // Record 100 events
        for _ in 0..100 {
            auditor.record_event(AuditEvent {
                agent_id: "test-agent".to_string(),
                input_tokens: 100,
                output_tokens: 100,
                cached_input_tokens: 0,
                local_embedding_tokens: 0,
            });
        }

        assert!(auditor.get_total_cost() > 0.0);
        assert_eq!(auditor.get_agent_cost("test-agent") as i64, (auditor.get_total_cost()) as i64);
    }
}
