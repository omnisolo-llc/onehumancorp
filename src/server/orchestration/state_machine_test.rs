use super::state_machine::{StateMachine, TaskStatus};
use crate::db::{DB, DbStore, create_sqlite_pool_for_test};
use std::sync::Arc;

#[tokio::test]
async fn test_state_machine_lifecycle_sqlite() {
    let pool = create_sqlite_pool_for_test().await;

    // Set up schema
    sqlx::query(
        r#"
        CREATE TABLE shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            assigned_agent_id TEXT,
            created_at TEXT,
            updated_at TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE shared_task_dependencies (
            task_id TEXT NOT NULL,
            depends_on_task_id TEXT NOT NULL,
            PRIMARY KEY (task_id, depends_on_task_id)
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool: crate::db::create_dummy_pg_pool().await });
    let sm = StateMachine::new(db);

    // Create a dependency task
    let dep_id = sm.create_task(None, "org1", "Dep Task", None, vec![]).await.unwrap();

    // Create a task that depends on it
    let task_id = sm.create_task(None, "org1", "Main Task", None, vec![dep_id.clone()]).await.unwrap();

    // Claim should fail because dependency is not completed
    let claim_res = sm.claim_task(&task_id, "agent1").await;
    assert!(claim_res.is_err(), "Should not be able to claim task with pending dependencies");

    // Complete dependency
    sm.claim_task(&dep_id, "agent1").await.unwrap();
    sm.complete_task(&dep_id).await.unwrap();

    // Claim should now succeed
    sm.claim_task(&task_id, "agent1").await.unwrap();

    // Complete task
    sm.complete_task(&task_id).await.unwrap();
}

#[tokio::test]
async fn test_state_machine_invalid_transitions_sqlite() {
    let pool = create_sqlite_pool_for_test().await;

    // Set up schema
    sqlx::query(
        r#"
        CREATE TABLE shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            assigned_agent_id TEXT,
            created_at TEXT,
            updated_at TEXT
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE shared_task_dependencies (
            task_id TEXT NOT NULL,
            depends_on_task_id TEXT NOT NULL,
            PRIMARY KEY (task_id, depends_on_task_id)
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let db = Arc::new(DB { store: DbStore::Sqlite(pool.clone()), pool: crate::db::create_dummy_pg_pool().await });
    let sm = StateMachine::new(db);

    let task_id = sm.create_task(None, "org1", "Main Task", None, vec![]).await.unwrap();

    // Cannot complete directly from pending
    let complete_res = sm.complete_task(&task_id).await;
    assert!(complete_res.is_err(), "Cannot complete task in PENDING state");

    sm.claim_task(&task_id, "agent1").await.unwrap();

    // Block the task
    sm.block_task(&task_id).await.unwrap();

    // Cannot complete from blocked
    let complete_res2 = sm.complete_task(&task_id).await;
    assert!(complete_res2.is_err(), "Cannot complete task in BLOCKED state");

    // Claim again (from blocked)
    sm.claim_task(&task_id, "agent2").await.unwrap();

    // Complete successfully
    sm.complete_task(&task_id).await.unwrap();
}

#[tokio::test]
async fn test_state_machine_lifecycle_postgres() {
    let pool = crate::db::create_dummy_pg_pool().await;

    // Check if PG is available
    if std::env::var("OHC_DATABASE_URL").is_err() && std::env::var("TEST_DATABASE_URL").is_err() {
        return;
    }

    // Set up schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            assigned_agent_id TEXT,
            created_at TIMESTAMPTZ,
            updated_at TIMESTAMPTZ
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap_or_default();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_task_dependencies (
            task_id TEXT NOT NULL,
            depends_on_task_id TEXT NOT NULL,
            PRIMARY KEY (task_id, depends_on_task_id)
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap_or_default();

    let db = Arc::new(DB { store: DbStore::Postgres, pool: pool.clone() });
    let sm = StateMachine::new(db);

    // Create a dependency task
    let dep_id = sm.create_task(None, "org1", "Dep Task", None, vec![]).await.unwrap();

    // Create a task that depends on it
    let task_id = sm.create_task(None, "org1", "Main Task", None, vec![dep_id.clone()]).await.unwrap();

    // Claim should fail because dependency is not completed
    let claim_res = sm.claim_task(&task_id, "agent1").await;
    assert!(claim_res.is_err(), "Should not be able to claim task with pending dependencies");

    // Complete dependency
    sm.claim_task(&dep_id, "agent1").await.unwrap();
    sm.complete_task(&dep_id).await.unwrap();

    // Claim should now succeed
    sm.claim_task(&task_id, "agent1").await.unwrap();

    // Complete task
    sm.complete_task(&task_id).await.unwrap();
}

#[tokio::test]
async fn test_state_machine_invalid_transitions_postgres() {
    let pool = crate::db::create_dummy_pg_pool().await;

    // Check if PG is available
    if std::env::var("OHC_DATABASE_URL").is_err() && std::env::var("TEST_DATABASE_URL").is_err() {
        return;
    }

    // Set up schema
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            assigned_agent_id TEXT,
            created_at TIMESTAMPTZ,
            updated_at TIMESTAMPTZ
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap_or_default();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_task_dependencies (
            task_id TEXT NOT NULL,
            depends_on_task_id TEXT NOT NULL,
            PRIMARY KEY (task_id, depends_on_task_id)
        );
        "#
    )
    .execute(&pool)
    .await
    .unwrap_or_default();

    let db = Arc::new(DB { store: DbStore::Postgres, pool: pool.clone() });
    let sm = StateMachine::new(db);

    let task_id = sm.create_task(None, "org1", "Main Task", None, vec![]).await.unwrap();

    // Cannot complete directly from pending
    let complete_res = sm.complete_task(&task_id).await;
    assert!(complete_res.is_err(), "Cannot complete task in PENDING state");

    sm.claim_task(&task_id, "agent1").await.unwrap();

    // Block the task
    sm.block_task(&task_id).await.unwrap();

    // Cannot complete from blocked
    let complete_res2 = sm.complete_task(&task_id).await;
    assert!(complete_res2.is_err(), "Cannot complete task in BLOCKED state");

    // Claim again (from blocked)
    sm.claim_task(&task_id, "agent2").await.unwrap();

    // Complete successfully
    sm.complete_task(&task_id).await.unwrap();
}
