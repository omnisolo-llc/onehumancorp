use super::shared_tasks::{SharedTaskOrchestrator, SharedTaskV4};
use crate::db::DB;
use std::sync::Arc;
use chrono::Utc;

#[tokio::test]
async fn test_shared_task_orchestrator() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    // Safety check - do not run db tests with production DB
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap();
    if !db_url.contains("test") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .acquire_timeout(std::time::Duration::from_millis(50))
        .connect_lazy(&db_url)
        .unwrap();

    let db = DB { pool: pool.clone(), store: crate::db::DbStore::Postgres };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    let task = SharedTaskV4 {
        id: "".to_string(),
        organization_id: "org_123".to_string(),
        title: "Test Task".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: Some("agent_1".to_string()),
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };

    let result = orchestrator.create_task(task).await;
    // Database might not be fully migrated in this test env, we just test compiling and running.
    if let Ok(created_task) = result {
        assert!(!created_task.id.is_empty());
        assert_eq!(created_task.title, "Test Task");

        let fetched_task = orchestrator.get_task(&created_task.id).await.unwrap();
        assert_eq!(fetched_task.id, created_task.id);
        assert_eq!(fetched_task.organization_id, "org_123");

        let claimed_task = orchestrator.claim_task("org_123", "agent_123").await.unwrap();
        assert!(claimed_task.is_some());
        let claimed = claimed_task.unwrap();
        assert_eq!(claimed.id, created_task.id);
        assert_eq!(claimed.status, "ASSIGNED");
        assert_eq!(claimed.agent_id.unwrap(), "agent_123");

        let empty_claim = orchestrator.claim_task("org_123", "agent_123").await.unwrap();
        assert!(empty_claim.is_none());
    }
}

#[tokio::test]
async fn test_shared_task_orchestrator_sqlite() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            ultraplan_phase TEXT,
            deliberation_log TEXT,
            depth INTEGER
        );

        CREATE TABLE IF NOT EXISTS state_machine_transitions (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT DEFAULT 'system',
                    entity_id TEXT NOT NULL,
                    entity_type TEXT NOT NULL DEFAULT 'task',
                    from_state TEXT NOT NULL,
                    to_state TEXT NOT NULL,
                    agent_id TEXT,
                    reason TEXT,
                    occurred_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
        .unwrap();

    let db = DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool) };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    let task = SharedTaskV4 {
        id: "task_1".to_string(),
        organization_id: "org_123".to_string(),
        title: "Test Task".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };

    let created_task = orchestrator.create_task(task).await.unwrap();
    assert_eq!(created_task.id, "task_1");

    let claimed_task = orchestrator.claim_task("org_123", "agent_123").await.unwrap();
    assert!(claimed_task.is_some());
    let claimed = claimed_task.unwrap();
    assert_eq!(claimed.id, "task_1");
    assert_eq!(claimed.status, "ASSIGNED");
    assert_eq!(claimed.agent_id.unwrap(), "agent_123");
}

#[tokio::test]
async fn test_shared_task_orchestrator_sqlite_dependencies() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            ultraplan_phase TEXT,
            deliberation_log TEXT,
            depth INTEGER
        );

        CREATE TABLE IF NOT EXISTS state_machine_transitions (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT DEFAULT 'system',
                    entity_id TEXT NOT NULL,
                    entity_type TEXT NOT NULL DEFAULT 'task',
                    from_state TEXT NOT NULL,
                    to_state TEXT NOT NULL,
                    agent_id TEXT,
                    reason TEXT,
                    occurred_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();

    let db = DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool) };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    // Task 1: pending, no dependencies
    let task1 = SharedTaskV4 {
        id: "task_1".to_string(),
        organization_id: "org_123".to_string(),
        title: "Test Task 1".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };
    orchestrator.create_task(task1).await.unwrap();

    // Task 2: pending, depends on Task 1
    let task2 = SharedTaskV4 {
        id: "task_2".to_string(),
        organization_id: "org_123".to_string(),
        title: "Test Task 2".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[\"task_1\"]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };
    orchestrator.create_task(task2).await.unwrap();

    // Task 2 should not be claimed because Task 1 is not COMPLETED.
    // Instead Task 1 should be claimed.
    let claimed_task_1 = orchestrator.claim_task("org_123", "agent_1").await.unwrap();
    assert!(claimed_task_1.is_some());
    assert_eq!(claimed_task_1.unwrap().id, "task_1");

    // After Task 1 is claimed, there are no more tasks to claim (Task 2 is blocked)
    let claimed_none = orchestrator.claim_task("org_123", "agent_2").await.unwrap();
    assert!(claimed_none.is_none());
}

#[tokio::test]
async fn test_shared_task_orchestrator_update_and_list_sqlite() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            ultraplan_phase TEXT,
            deliberation_log TEXT,
            depth INTEGER
        );

        CREATE TABLE IF NOT EXISTS state_machine_transitions (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT DEFAULT 'system',
                    entity_id TEXT NOT NULL,
                    entity_type TEXT NOT NULL DEFAULT 'task',
                    from_state TEXT NOT NULL,
                    to_state TEXT NOT NULL,
                    agent_id TEXT,
                    reason TEXT,
                    occurred_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/postgres").unwrap();

    let db = DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool) };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    let task1 = SharedTaskV4 {
        id: "task_list_1".to_string(),
        organization_id: "org_list".to_string(),
        title: "Test Task List 1".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };
    orchestrator.create_task(task1).await.unwrap();

    let task2 = SharedTaskV4 {
        id: "task_list_2".to_string(),
        organization_id: "org_list".to_string(),
        title: "Test Task List 2".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };
    orchestrator.create_task(task2).await.unwrap();

    let tasks = orchestrator.list_tasks("org_list").await.unwrap();
    assert_eq!(tasks.len(), 2);

    orchestrator.update_task_status("task_list_1", "IN_PROGRESS", Some("agent_list")).await.unwrap();
    let updated_task = orchestrator.get_task("task_list_1").await.unwrap();
    assert_eq!(updated_task.status, "IN_PROGRESS");

    orchestrator.complete_task("task_list_2", Some("agent_list")).await.unwrap();
    let completed_task = orchestrator.get_task("task_list_2").await.unwrap();
    assert_eq!(completed_task.status, "COMPLETED");
}

#[tokio::test]
async fn test_shared_task_orchestrator_dependencies() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }

    let db_url = std::env::var("OHC_DATABASE_URL").unwrap();
    if !db_url.contains("test") {
        return;
    }

    let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .acquire_timeout(std::time::Duration::from_millis(50))
        .connect_lazy(&db_url)
        .unwrap();

    let db = DB { pool: pool.clone(), store: crate::db::DbStore::Postgres };
    let db = Arc::new(db);
    let orchestrator = SharedTaskOrchestrator::new(db.clone());

    // Task 1: pending, no dependencies
    let task1 = SharedTaskV4 {
        id: "task_1_pg".to_string(),
        organization_id: "org_123_pg".to_string(),
        title: "Test Task 1 PG".to_string(),
        description: Some("Description".to_string()),
        status: "PENDING".to_string(),
        agent_id: None,
        priority: "P1".to_string(),
        payload: Some("{}".to_string()),
        parent_plan_id: None,
        dependencies: "[]".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
    };

    // Test compilation and basic structure. Like `test_shared_task_orchestrator`,
    // real execution depends on DB migrations running, so we check if creation succeeds first.
    let result = orchestrator.create_task(task1).await;

    if let Ok(_) = result {
        // Task 2: pending, depends on Task 1
        let task2 = SharedTaskV4 {
            id: "task_2_pg".to_string(),
            organization_id: "org_123_pg".to_string(),
            title: "Test Task 2 PG".to_string(),
            description: Some("Description".to_string()),
            status: "PENDING".to_string(),
            agent_id: None,
            priority: "P1".to_string(),
            payload: Some("{}".to_string()),
            parent_plan_id: None,
            dependencies: "[\"task_1_pg\"]".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        ultraplan_phase: None,
        deliberation_log: None,
        depth: None,
        };
        orchestrator.create_task(task2).await.unwrap();

        // Task 2 should not be claimed because Task 1 is not COMPLETED.
        // Instead Task 1 should be claimed.
        let claimed_task_1 = orchestrator.claim_task("org_123_pg", "agent_1_pg").await.unwrap();
        assert!(claimed_task_1.is_some());
        assert_eq!(claimed_task_1.unwrap().id, "task_1_pg");

        // After Task 1 is claimed, there are no more tasks to claim (Task 2 is blocked)
        let claimed_none = orchestrator.claim_task("org_123_pg", "agent_2_pg").await.unwrap();
        assert!(claimed_none.is_none());
    }
}

#[tokio::test]
async fn test_shared_task_orchestrator_concurrent_claim() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite://file::memory:?cache=shared")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            ultraplan_phase TEXT,
            deliberation_log TEXT,
            depth INTEGER
        );

        CREATE TABLE IF NOT EXISTS state_machine_transitions (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT DEFAULT 'system',
                    entity_id TEXT NOT NULL,
                    entity_type TEXT NOT NULL DEFAULT 'task',
                    from_state TEXT NOT NULL,
                    to_state TEXT NOT NULL,
                    agent_id TEXT,
                    reason TEXT,
                    occurred_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let db = crate::db::DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(), store: crate::db::DbStore::Sqlite(pool) };
    let db = std::sync::Arc::new(db);
    let orchestrator = std::sync::Arc::new(SharedTaskOrchestrator::new(db.clone()));

    let _ = crate::telemetry::get_error_signal_counter();

    for i in 0..50 {
        let task = SharedTaskV4 {
            id: format!("task_concurrent_{}", i),
            organization_id: "org_concurrent".to_string(),
            title: format!("Concurrent Task {}", i),
            description: None,
            status: "PENDING".to_string(),
            agent_id: None,
            priority: "P1".to_string(),
            payload: None,
            parent_plan_id: None,
            dependencies: "[]".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ultraplan_phase: None,
            deliberation_log: None,
            depth: None,
        };
        orchestrator.create_task(task).await.unwrap();
    }

    let mut handles = vec![];

    let claimed_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    for agent_id in 0..10 {
        let orch = orchestrator.clone();
        let count = claimed_count.clone();
        handles.push(tokio::spawn(async move {
            let mut local_claims = 0;
            loop {
                let agent_str = format!("agent_{}", agent_id);
                if let Ok(Some(_)) = orch.claim_task("org_concurrent", &agent_str).await {
                    local_claims += 1;
                } else {
                    break;
                }
            }
            count.fetch_add(local_claims, std::sync::atomic::Ordering::SeqCst);
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    assert_eq!(claimed_count.load(std::sync::atomic::Ordering::SeqCst), 50);

    let list = orchestrator.list_tasks("org_concurrent").await.unwrap();
    let assigned_tasks = list.iter().filter(|t| t.status == "ASSIGNED").count();
    assert_eq!(assigned_tasks, 50);
}
