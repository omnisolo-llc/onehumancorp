#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::orchestration::mesh::{CentrifugeNode, get_mesh_transport};
    use crate::orchestration::tasks::TaskDecompositionService;
    use crate::db::{DB, DbStore};
    use crate::tasks::SharedTask;
    use ohc_builtin_agent::mesh::transport::{IpcTransport, MeshTransport};
    use crate::ohc::orchestration::TeammateMeshEvent as MeshMessage;
    use prost::Message;
    use chrono::Utc;

    #[tokio::test]
    async fn test_mesh_task_dispatch_integration() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/integration_test_{}.sqlite", tmp_dir, Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let db_url = format!("sqlite://{}", db_path);

        // Initialize IpcTransport
        let transport = IpcTransport::new(&db_url).await.expect("Failed to create transport");
        let transport_arc: Arc<dyn MeshTransport> = Arc::new(transport.clone());

        let t_worker = transport.clone();
        tokio::spawn(async move { t_worker.start_worker().await; });

        // Initialize Mesh Node
        let mesh = Arc::new(CentrifugeNode::new(transport_arc.clone()));

        // Setup Subscriber (simulating builtin agent)
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let _cancel = transport_arc.subscribe("agent_jobs", Box::new(move |msg| {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                let _ = tx_clone.send(msg).await;
            });
        })).await.expect("Failed to subscribe");

        // Initialize DB and Service
        let pool = sqlx::sqlite::SqlitePool::connect(&db_url).await.expect("Failed to connect to DB");
        // Mock DB structure
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        // Create tables needed for TaskDecompositionService
        sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (id TEXT PRIMARY KEY, organization_id TEXT, mission_id TEXT, parent_plan_id TEXT, dependencies TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, payload TEXT, deliberation_log TEXT, depth INTEGER, created_at TEXT, updated_at TEXT, assigned_agent_id TEXT, locked_until TEXT, ultraplan_phase TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS state_machine_transitions (id TEXT PRIMARY KEY, task_id TEXT, from_state TEXT, to_state TEXT, agent_id TEXT, transitioned_at TEXT)").execute(&pool).await.unwrap();

        let service = TaskDecompositionService::new(db.clone(), mesh.clone());

        // Create and Claim Task
        let task = SharedTask {
            id: "test-task-1".to_string(),
            organization_id: "test-org".to_string(),
            mission_id: "test-mission".to_string(),
            parent_plan_id: String::new(),
            dependencies: vec![],
            title: "Test Task".to_string(),
            description: Some("Do something".to_string()),
            status: "PENDING".to_string(),
            priority: "P1".to_string(),
            payload: "{}".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ..Default::default()
        };

        service.create_task(task).await.expect("Failed to create task");
        let claimed = service.claim_task("test-agent").await.expect("Failed to claim task");
        assert!(claimed.is_some());

        // Verify message received on mesh
        let msg = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await
            .expect("Timeout waiting for mesh message")
            .expect("No message received");

        assert_eq!(msg.action, "agent_jobs");

        // Decode Protobuf payload
        let run_req = crate::ohc::agent::service::RunTaskRequest::decode(&msg.payload[..])
            .expect("Failed to decode RunTaskRequest");

        assert_eq!(run_req.task_id, "test-task-1");
        assert_eq!(run_req.task, "Do something");

        // Test Acknowledgement
        transport_arc.ack("agent_jobs", &msg.msg_id).await.expect("Failed to ack");

        // Verify it's in processed_messages
        let processed: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM processed_messages WHERE msg_id = ?)")
            .bind(&msg.msg_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(processed);

        // Test Reliability / Restart Resume
        // We use a clean DB for this sub-test to avoid interference from the previous worker
        let db_path_restart = format!("{}/integration_restart_{}.sqlite", tmp_dir, Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let db_url_restart = format!("sqlite://{}", db_path_restart);

        {
            let transport_pre = IpcTransport::new(&db_url_restart).await.unwrap();
            let msg2 = MeshMessage {
                agent_id: "test".to_string(),
                action: "agent_jobs".to_string(),
                status: "ok".to_string(),
                payload: b"restart_payload".to_vec(),
                msg_id: "msg-restart-test".to_string(),
            };
            // Publish WITHOUT a worker running
            transport_pre.publish("agent_jobs", msg2).await.unwrap();
        }

        // Now start a fresh transport and worker
        // Using a manual worker step since nested runtimes are forbidden in this test env
        let transport_restart = IpcTransport::new(&db_url_restart).await.expect("Failed to create transport");
        let transport_restart_arc: Arc<dyn MeshTransport> = Arc::new(transport_restart.clone());

        // Use a different subscriber ID via env var manually
        unsafe { std::env::set_var("OHC_MESH_SUBSCRIBER_ID", "restart_test_node"); }

        let (tx2, mut rx2) = tokio::sync::mpsc::channel(10);
        let _cancel2 = transport_restart_arc.subscribe("agent_jobs", Box::new(move |m| {
            let tx_clone = tx2.clone();
            tokio::spawn(async move {
                let _ = tx_clone.send(m).await;
            });
        })).await.unwrap();

        let t_worker2 = transport_restart.clone();
        let worker_handle = tokio::spawn(async move { t_worker2.start_worker().await; });

        // Should receive the un-acked message from the log
        let received_after_restart = tokio::time::timeout(std::time::Duration::from_secs(5), rx2.recv()).await
            .expect("Timeout waiting for message after restart")
            .expect("No message received after restart");

        assert_eq!(received_after_restart.msg_id, "msg-restart-test");
        assert_eq!(received_after_restart.payload, b"restart_payload".to_vec());

        worker_handle.abort();
        unsafe { std::env::remove_var("OHC_MESH_SUBSCRIBER_ID"); }
    }
}
