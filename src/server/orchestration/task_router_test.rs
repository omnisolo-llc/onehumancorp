use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::mesh::LocalTeammateMesh;
use crate::hub::Hub;
use crate::orchestration::task_router::{DynamicTaskRouter, TaskClaimPayload};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_task_claiming_race_condition() {
    let hub = Arc::new(Hub::new());
    let mesh = Arc::new(LocalTeammateMesh::new(hub));
    let db = Arc::new(DB::new().await.unwrap());

    // Make sure we have a task to claim
    let task_id = uuid::Uuid::new_v4().to_string();

    // Create the task using raw SQL for the test setup
    match &db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES (?, 'org1', 'Test Task', 'PENDING')"
            )
            .bind(&task_id)
            .execute(pool)
            .await
            .unwrap();
        },
        DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ($1, 'org1', 'Test Task', 'PENDING')"
            )
            .bind(&task_id)
            .execute(&db.pool)
            .await
            .unwrap();
        }
    }

    let router = Arc::new(DynamicTaskRouter::new(db.clone(), mesh));

    // Simulate 3 agents claiming simultaneously
    let mut tasks = vec![];
    for i in 0..3 {
        let agent_id = format!("agent_{}", i);
        let task_id_clone = task_id.clone();
        let router_clone = router.clone();

        tasks.push(tokio::spawn(async move {
            let payload = TaskClaimPayload {
                task_id: task_id_clone,
                agent_id: agent_id,
                capability_score: 90 + i,
            };

            router_clone.handle_claim(payload).await.unwrap()
        }));
    }

    let mut successes = 0;
    for task in tasks {
        let result = task.await.unwrap();
        if result {
            successes += 1;
        }
    }

    // Only one agent should successfully claim the task
    assert_eq!(successes, 1);

    // Verify in db that it's claimed
    match &db.store {
        DbStore::Sqlite(pool) => {
            let row = sqlx::query("SELECT claimed_by, claim_status FROM shared_tasks WHERE id = ?")
                .bind(&task_id)
                .fetch_one(pool)
                .await
                .unwrap();

            use sqlx::Row;
            let status: String = row.get("claim_status");
            assert_eq!(status, "CLAIMED");
        },
        DbStore::Postgres => {
            let row = sqlx::query("SELECT claimed_by, claim_status FROM shared_tasks WHERE id = $1")
                .bind(&task_id)
                .fetch_one(&db.pool)
                .await
                .unwrap();

            use sqlx::Row;
            let status: String = row.get("claim_status");
            assert_eq!(status, "CLAIMED");
        }
    }
}
