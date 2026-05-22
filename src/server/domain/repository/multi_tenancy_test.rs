use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::domain::repository::models::Task;
use crate::domain::repository::task_repo::TaskRepository;
use sqlx::sqlite::SqlitePoolOptions;
use chrono::Utc;

async fn setup_test_db() -> Arc<DB> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE tasks (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            parent_task_id TEXT,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
            priority TEXT DEFAULT 'P2',
            assigned_agent_role VARCHAR(100),
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    // Use a lazy postgres pool that won't actually connect unless used
    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();

    Arc::new(DB {
        pool: pg_pool,
        store: DbStore::Sqlite(pool),
    })
}

#[tokio::test]
async fn test_multi_tenancy_isolation() {
    let db = setup_test_db().await;
    let repo = TaskRepository::new(db);

    let tenant_a = "tenant_a";
    let tenant_b = "tenant_b";

    // Create task for tenant A
    let task_a = Task {
        id: "task_a".to_string(),
        tenant_id: tenant_a.to_string(),
        parent_task_id: None,
        title: "Task A".to_string(),
        description: None,
        status: "PENDING".to_string(),
        assigned_agent_role: None,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    repo.create_task(task_a).await.unwrap();

    // Create task for tenant B
    let task_b = Task {
        id: "task_b".to_string(),
        tenant_id: tenant_b.to_string(),
        parent_task_id: None,
        title: "Task B".to_string(),
        description: None,
        status: "PENDING".to_string(),
        assigned_agent_role: None,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    repo.create_task(task_b).await.unwrap();

    // Tenant A should only see task A
    let tasks_a = repo.get_tasks_by_tenant(tenant_a).await.unwrap();
    assert_eq!(tasks_a.len(), 1);
    assert_eq!(tasks_a[0].id, "task_a");

    // Tenant B should only see task B
    let tasks_b = repo.get_tasks_by_tenant(tenant_b).await.unwrap();
    assert_eq!(tasks_b.len(), 1);
    assert_eq!(tasks_b[0].id, "task_b");

    // Verify cross-tenant update failure (simulated via repository logic)
    let result = repo.update_task_status(tenant_a, "task_b", "COMPLETED").await;
    assert!(result.is_err(), "Tenant A should not be able to update Tenant B's task");
}
