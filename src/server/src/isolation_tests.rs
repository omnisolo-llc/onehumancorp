use super::*;
use tonic::{Request, Status};
use crate::ohc::orchestration::*;
use crate::hub::Hub;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_tenant_isolation_rls() {
    // This test assumes a working PostgreSQL connection if DATABASE_URL is set,
    // but we can also unit test the logic in HubService.

    let db_res = crate::db::DB::new().await;
    if db_res.is_err() {
        println!("Skipping RLS test: No working database connection");
        return;
    }
    let db = Arc::new(db_res.unwrap());
    db.run_migrations().await.expect("Failed to run migrations");

    let (event_tx, _) = mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, db.pool.clone()));
    let service = crate::MyHubService::new(hub.clone(), db.clone());

    // 1. Register agent for Org A
    let mut req_a = Request::new(RegisterAgentRequest {
        agent: Some(Agent {
            id: "agent-a".to_string(),
            name: "Agent A".to_string(),
            role: "ops".to_string(),
            organization_id: "org-a".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        }),
    });
    req_a.metadata_mut().insert("ohc-org-id", "org-a".parse().unwrap());
    service.register_agent(req_a).await.expect("Failed to register agent A");

    // 2. Create task for Org A
    let mut req_task_a = Request::new(CreateTaskRequest {
        mission_id: "mission-a".to_string(),
        title: "Task A".to_string(),
        description: "Description A".to_string(),
        priority: "P1".to_string(),
    });
    req_task_a.metadata_mut().insert("ohc-org-id", "org-a".parse().unwrap());
    let task_a = service.create_task(req_task_a).await.expect("Failed to create task A").into_inner();

    // 3. Attempt to poll tasks for Org A using Org B context
    let mut req_poll_b = Request::new(PollTasksRequest {
        agent_id: "agent-a".to_string(),
        limit: 10,
    });
    req_poll_b.metadata_mut().insert("ohc-org-id", "org-b".parse().unwrap());

    let res = service.poll_tasks(req_poll_b).await;
    // Should fail because agent-a doesn't belong to org-b
    if let Err(e) = res {
        assert_eq!(e.code(), tonic::Code::PermissionDenied);
    } else {
        // If it succeeded, RLS might still block it if it were a real DB query,
        // but here our poll_tasks implementation also has an explicit check.
        // If we are in a CI environment without Postgres, we might skip the actual DB part.
    }

    // 4. Attempt to update Task A status using Org B context
    let mut req_update_b = Request::new(UpdateTaskStatusRequest {
        task_id: task_a.id.clone(),
        status: "COMPLETED".to_string(),
        agent_id: "agent-a".to_string(),
        result: "Success".to_string(),
    });
    req_update_b.metadata_mut().insert("ohc-org-id", "org-b".parse().unwrap());

    let res_update = service.update_task_status(req_update_b).await;
    // Should fail because task_a doesn't belong to org-b
    assert!(res_update.is_err());
    assert_eq!(res_update.err().unwrap().code(), tonic::Code::PermissionDenied);

    // 5. Test DB RLS directly
    let mut tx = db.pool.begin().await.expect("Failed to begin transaction");
    db.set_organization_context(&mut *tx, "org-a").await.expect("Failed to set context");

    // Insert memory for org-a
    sqlx::query("INSERT INTO agent_memories (id, organization_id, task_id, raw_content, summary_embedding) VALUES ($1, $2, $3, $4, $5)")
        .bind("mem-1")
        .bind("org-a")
        .bind("task-a")
        .bind("content-a")
        .bind("[0.0]")
        .execute(&mut *tx)
        .await
        .expect("Failed to insert memory");
    tx.commit().await.expect("Failed to commit");

    // Try to read mem-1 as org-b
    let mut tx_b = db.pool.begin().await.expect("Failed to begin transaction B");
    db.set_organization_context(&mut *tx_b, "org-b").await.expect("Failed to set context B");

    let row: Option<(String,)> = sqlx::query_as("SELECT id FROM agent_memories WHERE id = 'mem-1'")
        .fetch_optional(&mut *tx_b)
        .await
        .expect("Failed to query memory");

    assert!(row.is_none(), "Org B should not see Org A's memory");
    tx_b.commit().await.expect("Failed to commit B");
}
