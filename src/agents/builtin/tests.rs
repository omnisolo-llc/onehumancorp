    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_load_cascading_agents_md() {
        let base_dir = std::path::PathBuf::from(format!("/tmp/ohc_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&base_dir).unwrap();

        let mut root_file = std::fs::File::create(base_dir.join("AGENTS.md")).unwrap();
        root_file.write_all(b"ROOT INSTRUCTION").unwrap();
        root_file.flush().unwrap();

        std::fs::create_dir_all(base_dir.join("nested")).unwrap();
        let mut nested_file = std::fs::File::create(base_dir.join("nested").join("AGENTS.md")).unwrap();
        nested_file.write_all(b"NESTED INSTRUCTION").unwrap();
        nested_file.flush().unwrap();

        let result = load_cascading_agents_md(&base_dir, Some("nested")).await;

        let _ = std::fs::remove_dir_all(&base_dir);

        assert_eq!(result, "ROOT INSTRUCTION\n\nNESTED INSTRUCTION");
    }

    #[tokio::test]
    async fn test_load_cascading_agents_md_truncation() {
        let base_dir = std::path::PathBuf::from(format!("/tmp/ohc_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&base_dir).unwrap();

        let mut root_file = std::fs::File::create(base_dir.join("AGENTS.md")).unwrap();
        let massive_str = "A".repeat(40000);
        root_file.write_all(massive_str.as_bytes()).unwrap();
        root_file.flush().unwrap();

        std::fs::create_dir_all(base_dir.join("nested")).unwrap();
        let mut nested_file = std::fs::File::create(base_dir.join("nested").join("AGENTS.md")).unwrap();
        nested_file.write_all(b"CRITICAL_LEAF").unwrap();
        nested_file.flush().unwrap();

        let result = load_cascading_agents_md(&base_dir, Some("nested")).await;

        let _ = std::fs::remove_dir_all(&base_dir);

        assert!(result.len() <= 32768);
        assert!(result.ends_with("CRITICAL_LEAF"));
    }

    #[tokio::test]
    async fn test_start_builtin_agent_task_assigned_subscribe() {
        use crate::mesh::transport::MemoryTransport;
        use crate::mesh::transport::MeshTransport;
        use std::sync::Arc;
        use prost::Message;
        use crate::auth::AuthMode;

        let transport = Arc::new(MemoryTransport::new());
        let svc = Arc::new(AgentServiceImpl::new("test_agent", AgentConfig::default(), AuthMode::Disabled));

        crate::service::start_builtin_agent(transport.clone(), svc.clone()).await;

        let shared_task = crate::proto::hub::SharedTask {
            id: "task-123".to_string(),
            organization_id: "org1".to_string(),
            title: "Test Task".to_string(),
            description: "Task Description".to_string(),
            payload: serde_json::json!({
                "model": "gpt-4-test",
                "department": "sales"
            }).to_string(),
            ..Default::default()
        };

        let mut buf = Vec::new();
        let _ = shared_task.encode(&mut buf);

        // The MemoryTransport internally executes local subscribers immediately.
        // It's a bit tricky to assert side-effects of tokio::spawn inside without mocking the entire service,
        // but we verify the publish is correctly handled by the framework without crashing.
        let result = transport.publish("task.assigned", crate::mesh::transport::Message {
            agent_id: "agent".to_string(),
            action: "task.assigned".to_string(),
            status: "ok".to_string(),
            payload: buf,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await;

        assert!(result.is_ok());

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
