use super::*;

        use std::time::Duration;

    struct MockHandler;

    #[async_trait]
    impl JobPayloadHandler for MockHandler {
        async fn handle(&self, payload: Vec<u8>) -> Result<(), String> {
            let s = String::from_utf8(payload).unwrap();
            tracing::info!("MockHandler received: {}", s);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_in_mem_job_queue_worker_pool() {
        let queue = Arc::new(InMemJobQueue::new());
        let handler = Arc::new(MockHandler);
        let pool = WorkerPool::new(queue.clone(), "test_topic".to_string(), 3, handler);

        let (tx, rx) = tokio::sync::broadcast::channel(1);
        // Ensure that we don't drop the rx to keep the channel open
        let _rx = rx;

        pool.start(tx.clone()).await;

        queue.push("test_topic", b"hello".to_vec()).await.unwrap();
        queue.push("test_topic", b"world".to_vec()).await.unwrap();

        tokio::time::sleep(Duration::from_millis(500)).await;

        let _ = tx.send(());
    }

    #[tokio::test]
    async fn test_task_queue_service_push_claim() {
        // Create an actual pool to hit a local database for integration testing.
        // During CI, we assume postgres is available at this URL.
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })

                .connect_lazy(&db_url)
                .unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let service = TaskQueueService::new(pool.clone());

            // Initialize schema for test
            sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id VARCHAR PRIMARY KEY, parent_id VARCHAR, epic_id VARCHAR, title VARCHAR NOT NULL, status VARCHAR NOT NULL, assigned_agent VARCHAR, payload JSONB, tenant_id VARCHAR, dependencies JSONB DEFAULT '[]', created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)")
                .execute(&pool)
                .await
                .unwrap();

            let task_id = uuid::Uuid::new_v4().to_string();
            let task = SharedTaskModel {
                id: task_id.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Test Task".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({"action": "test"}),
                dependencies: serde_json::json!([]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // Push
            let push_res = service.push_task(task).await;
            assert!(push_res.is_ok());

            // Claim
            let claim_res = service.claim_task("agent_1").await.unwrap();
            assert!(claim_res.is_some());
            let claimed = claim_res.unwrap();
            assert_eq!(claimed.id, task_id);
            assert_eq!(claimed.assigned_agent.unwrap(), "agent_1");

            // Complete
            let comp_res = service.complete_task(&task_id).await;
            assert!(comp_res.is_ok());
        }
    }


    #[tokio::test]
    async fn test_queue_manager_tenant_isolation() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy(&db_url)
                .unwrap();

            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }

            let qm = QueueManager::new(pool.clone());
            let job_id = uuid::Uuid::new_v4().to_string();
            let org_id = "tenant-a".to_string();

            // Ignore table creation errors if it already exists
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id VARCHAR PRIMARY KEY, tenant_id VARCHAR NOT NULL, parent_task_id VARCHAR, payload TEXT, status VARCHAR, worker_id VARCHAR, scheduled_at TIMESTAMP, completed_at TIMESTAMP, created_at TIMESTAMP, updated_at TIMESTAMP)")
                .execute(&pool)
                .await;

            let job = SubAgentJob {
                id: job_id.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            qm.enqueue(job).await.unwrap();

            // Attempt to complete with the WRONG tenant
            let res = qm.mark_completed(&job_id, "wrong-tenant").await;
            assert!(res.is_ok()); // The query executes successfully but updates 0 rows

            // Verify status is still QUEUED
            let status: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = $1")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status.0, "QUEUED");

            // Complete with CORRECT tenant
            let res2 = qm.mark_completed(&job_id, &org_id).await;
            assert!(res2.is_ok());

            let status_updated: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = $1")
                .bind(&job_id)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status_updated.0, "COMPLETED");

            // Test mark_failed isolation
            let job_id2 = uuid::Uuid::new_v4().to_string();
            let job2 = SubAgentJob {
                id: job_id2.clone(),
                tenant_id: org_id.clone(),
                parent_task_id: "task-1".to_string(),
                payload: serde_json::json!({"action": "test2"}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            qm.enqueue(job2).await.unwrap();

            let _ = qm.mark_failed(&job_id2, "error", "wrong-tenant").await;
            let status_failed1: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = $1")
                .bind(&job_id2)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status_failed1.0, "QUEUED");

            let _ = qm.mark_failed(&job_id2, "error", &org_id).await;
            let status_failed2: (String,) = sqlx::query_as("SELECT status FROM sub_agent_queue WHERE id = $1")
                .bind(&job_id2)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(status_failed2.0, "FAILED");
        }
    }

    #[tokio::test]
    async fn test_task_queue_service_fail_task() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })

                .connect_lazy(&db_url)
                .unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let service = TaskQueueService::new(pool.clone());

            let task_id = uuid::Uuid::new_v4().to_string();
            let task = SharedTaskModel {
                id: task_id.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Test Task to Fail".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({"action": "test_fail"}),
                dependencies: serde_json::json!([]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            service.push_task(task).await.unwrap();

            // Claim it
            let claimed = service.claim_task("agent_1").await.unwrap().unwrap();
            assert_eq!(claimed.id, task_id);

            // Fail it
            service.fail_task(&task_id, "Some failure occurred").await.unwrap();

            // Fetch manually to check
            let row = sqlx::query("SELECT status, payload FROM shared_tasks WHERE id = $1")
                .bind(&task_id)
                .fetch_one(&pool)
                .await
                .unwrap();

            let status: String = sqlx::Row::get(&row, "status");
            let payload: serde_json::Value = sqlx::Row::get(&row, "payload");

            assert_eq!(status, "FAILED");
            assert_eq!(payload["error"], "Some failure occurred");
        }
    }

    #[tokio::test]
    async fn test_task_queue_service_with_dependencies() {
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })

                .connect_lazy(&db_url)
                .unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let service = TaskQueueService::new(pool.clone());

            let task_id_parent = uuid::Uuid::new_v4().to_string();
            let task_id_child = uuid::Uuid::new_v4().to_string();

            let parent_task = SharedTaskModel {
                id: task_id_parent.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Parent Task".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({}),
                dependencies: serde_json::json!([]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            let child_task = SharedTaskModel {
                id: task_id_child.clone(),
                tenant_id: "org1".to_string(),
                parent_id: None,
                epic_id: None,
                title: "Child Task".to_string(),
                status: "PENDING".to_string(),
                assigned_agent: None,
                payload: serde_json::json!({}),
                dependencies: serde_json::json!([task_id_parent]),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            service.push_task(parent_task).await.unwrap();
            service.push_task(child_task).await.unwrap();

            // Claiming should ONLY claim the parent since child is blocked by parent
            let claim_1 = service.claim_task("agent_1").await.unwrap().unwrap();
            assert_eq!(claim_1.id, task_id_parent);

            // Second claim should return None because child is blocked
            let claim_2 = service.claim_task("agent_1").await.unwrap();
            assert!(claim_2.is_none());

            // Complete parent
            service.complete_task(&task_id_parent).await.unwrap();

            // Now child should be claimable
            let claim_3 = service.claim_task("agent_2").await.unwrap().unwrap();
            assert_eq!(claim_3.id, task_id_child);
        }
    }
