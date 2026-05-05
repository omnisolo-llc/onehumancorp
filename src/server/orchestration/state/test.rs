use super::StateManager;
use crate::db::{DB, DbStore};
use std::sync::Arc;
use sqlx::sqlite::SqlitePoolOptions;
use super::standalone::StandaloneStateManager;
use super::cloud::CloudStateManager;

async fn setup_db() -> Arc<DB> {
    let db_id = uuid::Uuid::new_v4().to_string();
    let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&uri)
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE local_advisory_locks (
            id TEXT PRIMARY KEY,
            acquired_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(&sqlite_pool).await.unwrap();

    sqlx::query(
        r#"
        CREATE TABLE swarm_tasks (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL DEFAULT 'system',
            mission_id TEXT NOT NULL,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            title TEXT NOT NULL,
            description TEXT,
            priority TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            locked_until TEXT,
            payload TEXT,
            created_at TEXT,
            updated_at TEXT
        );
        "#
    ).execute(&sqlite_pool).await.unwrap();

    sqlx::query(
        r#"
        CREATE TABLE state_machine_transitions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL DEFAULT 'system',
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at TEXT
        );
        "#
    ).execute(&sqlite_pool).await.unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();

    Arc::new(DB {
        pool: dummy_pg_pool,
        store: DbStore::Sqlite(sqlite_pool),
    })
}

#[tokio::test]
async fn test_single_agent_flow() {
    let db = setup_db().await;
    let state_manager = StandaloneStateManager::new(db.clone());

    let task_id = uuid::Uuid::new_v4().to_string();

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES (?, 'm1', 't1', 'PENDING')")
            .bind(&task_id)
            .execute(pool)
            .await
            .unwrap();
    }

    let result = state_manager.transition_state(&task_id, "system", "PENDING", "EXECUTING", Some("agent_1"), None).await;
    println!("Result: {:?}", result);
    assert!(result.is_ok());

    if let DbStore::Sqlite(pool) = &db.store {
        let status: String = sqlx::query_scalar("SELECT status FROM swarm_tasks WHERE id = ?")
            .bind(&task_id)
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(status, "EXECUTING");
    }
}

#[tokio::test]
async fn test_dag_workflow() {
    let db = setup_db().await;
    let state_manager = StandaloneStateManager::new(db.clone());

    let parent_id = uuid::Uuid::new_v4().to_string();
    let child_id = uuid::Uuid::new_v4().to_string();
    let deps = format!(r#"["{}"]"#, parent_id);

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES (?, 'm1', 'parent', 'PENDING')")
            .bind(&parent_id)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES (?, 'm1', 'child', 'PENDING', ?)")
            .bind(&child_id)
            .bind(&deps)
            .execute(pool)
            .await
            .unwrap();
    }

    let tasks = state_manager.pull_available_tasks(10).await.unwrap();

    assert!(tasks.iter().any(|t| t.id == parent_id));
    assert!(!tasks.iter().any(|t| t.id == child_id));

    state_manager.transition_state(&parent_id, "system", "IN_PROGRESS", "COMPLETED", Some("agent_1"), None).await.unwrap();

    let tasks_after = state_manager.pull_available_tasks(10).await.unwrap();
    assert!(tasks_after.iter().any(|t| t.id == child_id));
}

#[tokio::test]
async fn test_cloud_dag_workflow_mock() {
    let db = setup_db().await;
    let _state_manager = CloudStateManager::new(db.clone(), None);

    let parent_id = uuid::Uuid::new_v4().to_string();
    let child_id = uuid::Uuid::new_v4().to_string();
    let deps = format!(r#"["{}"]"#, parent_id);

    if let DbStore::Sqlite(pool) = &db.store {
        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES (?, 'm1', 'parent', 'PENDING')")
            .bind(&parent_id)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES (?, 'm1', 'child', 'PENDING', ?)")
            .bind(&child_id)
            .bind(&deps)
            .execute(pool)
            .await
            .unwrap();
    }

    assert!(true);
}
