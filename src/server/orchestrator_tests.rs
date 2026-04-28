use crate::hub::Hub;
use crate::orchestrator::{DefaultTaskOrchestrator, TaskOrchestrator};
use std::sync::Arc;
use sqlx::PgPool;

#[tokio::test]
async fn test_acquire_ready_task() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let pool = PgPool::connect(&database_url)
        .await
        .unwrap();

    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, pool.clone()));
    let orchestrator = DefaultTaskOrchestrator::new(hub.clone(), pool.clone());

    // Insert a pending task
    let org_id = "test-org";
    let title = "Test Task";

    // First clear old tasks to make it predictable
    sqlx::query("DELETE FROM shared_tasks_decomposition WHERE organization_id = $1")
        .bind(org_id)
        .execute(&pool)
        .await
        .unwrap();

    let id = uuid::Uuid::new_v4();
    sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ($1, $2, $3, 'PENDING')")
        .bind(id)
        .bind(org_id)
        .bind(title)
        .execute(&pool)
        .await
        .unwrap();

    // Acquire task
    let agent_id = "agent-1";
    let task = orchestrator
        .acquire_ready_task(agent_id, vec![])
        .await
        .unwrap();

    assert!(task.is_some());
    let task = task.unwrap();
    assert_eq!(task.organization_id, org_id);
    assert_eq!(task.status, "IN_PROGRESS");
    assert_eq!(task.assigned_agent_id, Some(agent_id.to_string()));

    // Verify it was updated in the DB
    let row: (String, String) = sqlx::query_as("SELECT status, assigned_agent_id FROM shared_tasks_decomposition WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.0, "IN_PROGRESS");
    assert_eq!(row.1, agent_id);
}
