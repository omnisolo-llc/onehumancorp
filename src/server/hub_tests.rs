mod tests {
    use super::*;
    use tokio::sync::mpsc;


    #[tokio::test]
    async fn test_publish_mesh_event() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let mut rx = hub.subscribe_mesh_events("test_topic".to_string());

        let event = MeshEvent {
            event_id: "test_id".to_string(),
            topic: "test_topic".to_string(),
            payload: b"test_payload".to_vec(),
            timestamp: 0,
        };

        hub.publish_mesh_event(event.clone()).unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_id, event.event_id);
        assert_eq!(received.topic, event.topic);
        assert_eq!(received.payload, event.payload);
    }

    #[tokio::test]
    async fn test_sanitize_hub_event_redaction() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let raw = serde_json::json!({
            "type": "TestEvent",
            "password": "secret-password",
            "email": "test@example.com",
            "nested": {
                "auth_token": "token123"
            }
        });

        let sanitized = hub.sanitize_hub_event(raw);
        let payload: serde_json::Value = serde_json::from_str(&sanitized.payload).unwrap();

        assert_eq!(payload["password"], "[REDACTED]");
        assert_eq!(payload["email"], "[REDACTED]");
        assert_eq!(payload["nested"]["auth_token"], "[REDACTED]");
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        // 1. Initial get caches empty state
        let agents = hub.get_agents();
        assert_eq!(agents.len(), 0);

        // Cache should be populated
        assert!(hub.agent_cache.read().unwrap().is_some());

        // 2. Registering an agent invalidates the cache
        hub.register_agent(Agent {
            id: "agent1".to_string(),
            name: "Agent 1".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "test".to_string(),
        });
        assert!(hub.agent_cache.read().unwrap().is_none());

        // 3. Get agents caches again
        let agents = hub.get_agents();
        assert_eq!(agents.len(), 1);
        assert!(hub.agent_cache.read().unwrap().is_some());

        // 4. Fire agent invalidates
        hub.fire_agent("agent1");
        assert!(hub.agent_cache.read().unwrap().is_none());

        // 5. Open meeting invalidates both caches
        let meetings = hub.get_meetings();
        assert_eq!(meetings.len(), 0);
        assert!(hub.meetings_cache.read().unwrap().is_some());

        hub.open_meeting("meeting1".to_string(), vec![], "agenda".to_string());
        assert!(hub.meetings_cache.read().unwrap().is_none());
        assert!(hub.agent_cache.read().unwrap().is_none());

        // 6. Publish invalidates meeting cache
        let meetings = hub.get_meetings();
        assert_eq!(meetings.len(), 1);
        assert!(hub.meetings_cache.read().unwrap().is_some());

        let _ = hub.clone().publish(Message {
            id: "msg1".to_string(),
            from_agent: "sys".to_string(),
            to_agent: "all".to_string(),
            r#type: "test".to_string(),
            content: "test".to_string(),
            occurred_at_unix: 0,
            meeting_id: "meeting1".to_string(),
        });
        assert!(hub.meetings_cache.read().unwrap().is_none());
    }
    #[tokio::test]
    async fn test_delegate_sub_task_invalid_sender() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let res = hub.delegate_sub_task(
            "non_existent_agent",
            "developer",
            "fix the bug",
            "thread123",
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "sender agent is not registered");
    }

    #[tokio::test]
    async fn test_delegate_sub_task_valid_hierarchy() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        hub.register_agent(Agent {
            id: "manager_agent".to_string(),
            name: "Manager".to_string(),
            role: "Manager".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });

        let res = hub.delegate_sub_task(
            "manager_agent",
            "developer",
            "fix the bug",
            "thread123",
        );

        assert!(res.is_ok());
        let spawned_id = res.unwrap();
        assert!(spawned_id.starts_with("sub-agent-developer-"));
    }

    #[tokio::test]
    async fn test_check_health() {
        // Skip test if no database is available
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        // Since test db is likely unmigrated/empty, we connect lazily
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let health = hub.check_health().await.unwrap();

        // When lazily connected, if DB doesn't exist, status might be degraded,
        // or we might get an error depending on how check_health handles failure.
        // In our check_health, failure to query SELECT 1 results in db_ping = 0.
        // We just ensure the response contains the fields we expect.
        assert!(health.get("status").is_some());
        assert!(health.get("db_ping_ms").is_some());
        assert!(health.get("hybrid_mode_ready").is_some());
        assert!(health.get("local_to_cloud_sync_queue").is_some());
    }
}
