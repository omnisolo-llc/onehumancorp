use crate::orchestration::tasks_db::TaskDbService;
use crate::db::DB;
use std::sync::Arc;

#[tokio::test]
async fn test_tasks_db_claim_task_postgres_real() {
    let pg_db = crate::orchestration::state::parity_test::setup_postgres_db().await;
    if let Some(db) = pg_db {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS shared_tasks (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                parent_plan_id TEXT,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'PENDING',
                assigned_agent_id TEXT,
                dependencies JSONB DEFAULT '[]',
                created_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ,
                locked_until TIMESTAMPTZ,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            );
            "#
        )
        .execute(&db.pool)
        .await
        .unwrap();

        let service = TaskDbService::new(Arc::new(db.clone()));

        sqlx::query(
            "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('1_pg', 'org1', 'Task 1', 'PENDING')"
        )
        .execute(&db.pool)
        .await
        .unwrap();

        let task = service.claim_task("agent1").await.unwrap();
        assert!(task.is_some());
        let task = task.unwrap();
        assert_eq!(task.id, "1_pg");
        assert_eq!(task.status, "ASSIGNED");
    }
}
