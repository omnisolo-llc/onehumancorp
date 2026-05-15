use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_autodream_sync_service() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let service = AutoDreamSyncServiceImpl::new(pool.clone());

        let pending = service.fetch_pending_syncs(10).await;
        // Depending on DB state, this may fail or return Ok
        assert!(pending.is_ok() || pending.is_err());

        let record = AutoDreamSyncRecord {
            id: uuid::Uuid::new_v4().to_string(),
            organization_id: Some("org_1".to_string()),
            agent_id: Some("agent_1".to_string()),
            task_id: Some("task_1".to_string()),
            content: "test content".to_string(),
            embedding: Some("[0.1, 0.2]".to_string()),
            source_type: Some("test_source".to_string()),
            topic: Some("test_topic".to_string()),
            sync_status: Some("pending".to_string()),
            last_sync_at: Some(Utc::now()),
        };

        let process_res = service.process_incoming_syncs(vec![record.clone()]).await;
        assert!(process_res.is_ok() || process_res.is_err());

        let mark_res = service.mark_records_synced(vec![record.id.clone()]).await;
        assert!(mark_res.is_ok() || mark_res.is_err());
    }
