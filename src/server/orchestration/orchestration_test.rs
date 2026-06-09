use crate::db::{DB, DbStore};
use std::sync::Arc;
use crate::orchestration::tasks::TaskDecompositionService;
use crate::tasks::SharedTask;
use crate::orchestration::mesh::TeammateMesh;
use chrono::Utc;

#[tokio::test]
async fn test_task_decomposition_service() {
    // Mock db to avoid pool timeouts for isolated test
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to initialize database");
    let dummy_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
    let db = DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) };

    match &db.store {
        DbStore::Postgres => {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
                    id TEXT PRIMARY KEY,
                    organization_id TEXT NOT NULL,
                    mission_id TEXT,
                    parent_plan_id TEXT,
                    dependencies JSONB DEFAULT '[]',
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'PENDING',
                    priority TEXT NOT NULL DEFAULT 'P2',
                    payload JSONB DEFAULT '{}',
                    deliberation_log JSONB DEFAULT '[]',
                    depth INTEGER,
                    ultraplan_phase TEXT,
                    action_risk TEXT,
                    approval_status TEXT,
                    proposed_content TEXT,
                    tokens_consumed INTEGER DEFAULT 0,
                    assigned_agent_id TEXT,
                    agent_role TEXT,
                    model TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
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
            .execute(&db.pool)
            .await
            .unwrap();
        }
        DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
                    id TEXT PRIMARY KEY,
                    organization_id TEXT NOT NULL,
                    mission_id TEXT,
                    parent_plan_id TEXT,
                    dependencies JSONB DEFAULT '[]',
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'PENDING',
                    priority TEXT NOT NULL DEFAULT 'P2',
                    payload JSONB DEFAULT '{}',
                    deliberation_log JSONB DEFAULT '[]',
                    depth INTEGER,
                    ultraplan_phase TEXT,
                    action_risk TEXT,
                    approval_status TEXT,
                    proposed_content TEXT,
                    tokens_consumed INTEGER DEFAULT 0,
                    assigned_agent_id TEXT,
                    agent_role TEXT,
                    model TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
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
            .execute(sqlite_pool)
            .await
            .unwrap();
        }
    }


    let svc_mesh_transport = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new())));
    let svc = TaskDecompositionService::new(Arc::new(db.clone()), svc_mesh_transport.clone());
    let mesh_clone = svc_mesh_transport.clone();
    tokio::spawn(async move {
        let mesh_inner = mesh_clone.clone();
        let _ = mesh_clone.subscribe("task.assigned", Box::new(move |msg| {
            let msg_id = msg.msg_id.clone();
            let ack_topic = format!("mesh:ack:{}", msg_id);
            let mesh_inner2 = mesh_inner.clone();
            tokio::spawn(async move {
                let _ = mesh_inner2.publish(&ack_topic, b"ack".to_vec()).await;
            });
        })).await;
    });

    let now = Utc::now();
    let dep_task = SharedTask {
        id: uuid::Uuid::new_v4().to_string(),
        organization_id: "org1".to_string(),
        mission_id: "m1".to_string(),
        parent_plan_id: "p1".to_string(),
        dependencies: vec![],
        title: "Dep Task".to_string(),
        description: None,
        assigned_agent_id: None,
        status: "COMPLETED".to_string(),
        priority: "P1".to_string(),
        payload: "{}".to_string(),
        locked_until: None,
        ultraplan_phase: None,
        deliberation_log: None,
        depth: Some(1),
        created_at: now,
        updated_at: now,
        action_risk: None,
        approval_status: None,
        proposed_content: None,
    };

    let main_task = SharedTask {
        id: uuid::Uuid::new_v4().to_string(),
        organization_id: "org1".to_string(),
        mission_id: "m1".to_string(),
        parent_plan_id: "p1".to_string(),
        dependencies: vec![dep_task.id.clone()],
        title: "Main Task".to_string(),
        description: None,
        assigned_agent_id: None,
        status: "PENDING".to_string(),
        priority: "P1".to_string(),
        payload: "{}".to_string(),
        locked_until: None,
        ultraplan_phase: None,
        deliberation_log: None,
        depth: Some(1),
        created_at: now,
        updated_at: now,
        action_risk: None,
        approval_status: None,
        proposed_content: None,
    };

    svc.create_task(dep_task.clone()).await.unwrap();
    svc.create_task(main_task.clone()).await.unwrap();

    let claimed = svc.claim_task("agent1").await.unwrap();
    assert!(claimed.is_some());

    svc.update_status(&main_task.id, "REVIEW", "agent1").await.unwrap();
}

#[tokio::test]
async fn test_task_decomposition_dag_blocked() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to initialize database");
    let dummy_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();
    let db = DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) };
    match &db.store {
        DbStore::Postgres => {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
                    id TEXT PRIMARY KEY,
                    organization_id TEXT NOT NULL,
                    mission_id TEXT,
                    parent_plan_id TEXT,
                    dependencies JSONB DEFAULT '[]',
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'PENDING',
                    priority TEXT NOT NULL DEFAULT 'P2',
                    payload JSONB DEFAULT '{}',
                    deliberation_log JSONB DEFAULT '[]',
                    depth INTEGER,
                    ultraplan_phase TEXT,
                    action_risk TEXT,
                    approval_status TEXT,
                    proposed_content TEXT,
                    tokens_consumed INTEGER DEFAULT 0,
                    assigned_agent_id TEXT,
                    agent_role TEXT,
                    model TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
                "#
            ).execute(&db.pool).await.unwrap();
        }
        DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
                    id TEXT PRIMARY KEY,
                    organization_id TEXT NOT NULL,
                    mission_id TEXT,
                    parent_plan_id TEXT,
                    dependencies JSONB DEFAULT '[]',
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'PENDING',
                    priority TEXT NOT NULL DEFAULT 'P2',
                    payload JSONB DEFAULT '{}',
                    deliberation_log JSONB DEFAULT '[]',
                    depth INTEGER,
                    ultraplan_phase TEXT,
                    action_risk TEXT,
                    approval_status TEXT,
                    proposed_content TEXT,
                    tokens_consumed INTEGER DEFAULT 0,
                    assigned_agent_id TEXT,
                    agent_role TEXT,
                    model TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
                "#
            ).execute(sqlite_pool).await.unwrap();
            sqlx::query(
                r#"
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
            ).execute(sqlite_pool).await.unwrap();
        }
    }


    let svc_mesh_transport = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new())));
    let svc = TaskDecompositionService::new(Arc::new(db.clone()), svc_mesh_transport.clone());
    let mesh_clone = svc_mesh_transport.clone();
    tokio::spawn(async move {
        let mesh_inner = mesh_clone.clone();
        let _ = mesh_clone.subscribe("task.assigned", Box::new(move |msg| {
            let msg_id = msg.msg_id.clone();
            let ack_topic = format!("mesh:ack:{}", msg_id);
            let mesh_inner2 = mesh_inner.clone();
            tokio::spawn(async move {
                let _ = mesh_inner2.publish(&ack_topic, b"ack".to_vec()).await;
            });
        })).await;
    });

    let now = Utc::now();
    let dep_task = SharedTask {
        id: uuid::Uuid::new_v4().to_string(),
        organization_id: "org1".to_string(),
        mission_id: "m1".to_string(),
        parent_plan_id: "p1".to_string(),
        dependencies: vec![],
        title: "Dep Task".to_string(),
        description: None,
        assigned_agent_id: None,
        status: "PENDING".to_string(),
        priority: "P1".to_string(),
        payload: "{}".to_string(),
        locked_until: None,
        ultraplan_phase: None,
        deliberation_log: None,
        depth: Some(1),
        created_at: now,
        updated_at: now,
        action_risk: None,
        approval_status: None,
        proposed_content: None,
    };

    let main_task = SharedTask {
        id: uuid::Uuid::new_v4().to_string(),
        organization_id: "org1".to_string(),
        mission_id: "m1".to_string(),
        parent_plan_id: "p1".to_string(),
        dependencies: vec![dep_task.id.clone()],
        title: "Main Task".to_string(),
        description: None,
        assigned_agent_id: None,
        status: "PENDING".to_string(),
        priority: "P1".to_string(),
        payload: "{}".to_string(),
        locked_until: None,
        ultraplan_phase: None,
        deliberation_log: None,
        depth: Some(1),
        created_at: now,
        updated_at: now,
        action_risk: None,
        approval_status: None,
        proposed_content: None,
    };

    svc.create_task(dep_task.clone()).await.unwrap();
    svc.create_task(main_task.clone()).await.unwrap();

    let claimed = svc.claim_task("agent1").await.unwrap();
    assert!(claimed.is_some());
    assert_eq!(claimed.unwrap().id, dep_task.id);

    let claimed2 = svc.claim_task("agent2").await.unwrap();
    assert!(claimed2.is_none());
}

#[tokio::test]
async fn test_task_decomposition_service_fail_task() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to initialize database");
    let dummy_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
        .unwrap();
    let db = DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) };
    match &db.store {
        DbStore::Postgres => {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
                    id TEXT PRIMARY KEY,
                    organization_id TEXT NOT NULL,
                    mission_id TEXT,
                    parent_plan_id TEXT,
                    dependencies JSONB DEFAULT '[]',
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'PENDING',
                    priority TEXT NOT NULL DEFAULT 'P2',
                    payload JSONB DEFAULT '{}',
                    deliberation_log JSONB DEFAULT '[]',
                    depth INTEGER,
                    ultraplan_phase TEXT,
                    action_risk TEXT,
                    approval_status TEXT,
                    proposed_content TEXT,
                    tokens_consumed INTEGER DEFAULT 0,
                    assigned_agent_id TEXT,
                    agent_role TEXT,
                    model TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
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
            ).execute(&db.pool).await.unwrap();
        }
        DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
                    id TEXT PRIMARY KEY,
                    organization_id TEXT NOT NULL,
                    mission_id TEXT,
                    parent_plan_id TEXT,
                    dependencies JSONB DEFAULT '[]',
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL DEFAULT 'PENDING',
                    priority TEXT NOT NULL DEFAULT 'P2',
                    payload JSONB DEFAULT '{}',
                    deliberation_log JSONB DEFAULT '[]',
                    depth INTEGER,
                    ultraplan_phase TEXT,
                    action_risk TEXT,
                    approval_status TEXT,
                    proposed_content TEXT,
                    tokens_consumed INTEGER DEFAULT 0,
                    assigned_agent_id TEXT,
                    agent_role TEXT,
                    model TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
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
            ).execute(sqlite_pool).await.unwrap();
        }
    }


    let svc_mesh_transport = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new())));
    let svc = TaskDecompositionService::new(Arc::new(db.clone()), svc_mesh_transport.clone());
    let mesh_clone = svc_mesh_transport.clone();
    tokio::spawn(async move {
        let mesh_inner = mesh_clone.clone();
        let _ = mesh_clone.subscribe("task.assigned", Box::new(move |msg| {
            let msg_id = msg.msg_id.clone();
            let ack_topic = format!("mesh:ack:{}", msg_id);
            let mesh_inner2 = mesh_inner.clone();
            tokio::spawn(async move {
                let _ = mesh_inner2.publish(&ack_topic, b"ack".to_vec()).await;
            });
        })).await;
    });

    let now = Utc::now();
    let main_task = SharedTask {
        id: uuid::Uuid::new_v4().to_string(),
        organization_id: "org1".to_string(),
        mission_id: "m1".to_string(),
        parent_plan_id: "p1".to_string(),
        dependencies: vec![],
        title: "Main Task".to_string(),
        description: None,
        assigned_agent_id: None,
        status: "PENDING".to_string(),
        priority: "P1".to_string(),
        payload: "{}".to_string(),
        locked_until: None,
        ultraplan_phase: None,
        deliberation_log: None,
        depth: Some(1),
        created_at: now,
        updated_at: now,
        action_risk: None,
        approval_status: None,
        proposed_content: None,
    };

    svc.create_task(main_task.clone()).await.unwrap();

    let claimed = svc.claim_task("agent1").await.unwrap();
    assert!(claimed.is_some());
    assert_eq!(claimed.unwrap().id, main_task.id);

    svc.fail_task(&main_task.id, "agent1", "Test Error").await.unwrap();


    let (status, payload_str): (String, String) = match &db.store {
        DbStore::Postgres => {
            let row = sqlx::query("SELECT status, payload FROM shared_tasks_decomposition WHERE id = $1")
                .bind(&main_task.id)
                .fetch_one(&db.pool)
                .await
                .unwrap();
            let s: String = sqlx::Row::get(&row, "status");
            let p: serde_json::Value = sqlx::Row::get(&row, "payload");
            (s, serde_json::to_string(&p).unwrap())
        }
        DbStore::Sqlite(pool) => {
            let row = sqlx::query("SELECT status, payload FROM shared_tasks_decomposition WHERE id = ?")
                .bind(&main_task.id)
                .fetch_one(pool)
                .await
                .unwrap();
            let s: String = sqlx::Row::get(&row, "status");
            let p: String = sqlx::Row::get(&row, "payload");
            (s, p)
        }
    };

    assert_eq!(status, "FAILED");

    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap();
    assert_eq!(payload["error"], "Test Error");

}
