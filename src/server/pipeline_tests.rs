use super::*;

    #[test]
    fn test_parse_spec_approved() {
        let content = "branch=feature-1,details=Implement feature 1";
        let event = Orchestrator::parse_spec_approved(content).unwrap();
        assert_eq!(event.branch, "feature-1");
        assert_eq!(event.details, "Implement feature 1");
    }

    #[tokio::test]
    async fn test_handle_spec_approved() {
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/test").unwrap();
        let hub = Arc::new(Hub::new(tx, pool));
        let orchestrator = Orchestrator::new(hub.clone());

        let msg = Message {
            id: "msg-1".to_string(),
            from_agent: "user".to_string(),
            to_agent: "hub".to_string(),
            r#type: "SpecApproved".to_string(),
            content: "branch=feature-2,details=Implement feature 2".to_string(),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };

        orchestrator.handle_spec_approved(msg).await.unwrap();

        let state = orchestrator.get_pipeline_state("feature-2").unwrap();
        assert_eq!(state, PipelineState::Implementing);
    }
