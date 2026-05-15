use super::*;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_and_get_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, "PENDING");

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.id, task.id);
    }
    #[test]
    fn test_claim_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        let claimed = tm.claim_task(&task.id, "agent1".to_string()).unwrap();
        assert!(claimed.is_some());
        let claimed = claimed.unwrap();
        assert_eq!(claimed.status, "IN_PROGRESS");
        assert_eq!(claimed.assigned_agent_id, Some("agent1".to_string()));

        // Try to claim again
        let claimed_again = tm.claim_task(&task.id, "agent2".to_string()).unwrap();
        assert!(claimed_again.is_none());
    }
    #[test]
    fn test_review_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        tm.claim_task(&task.id, "agent1".to_string()).unwrap();

        tm.review_task(&task.id, "agent1").unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.status, "REVIEW");

        // Try to review with wrong agent
        assert!(tm.review_task(&task.id, "agent2").is_err());
    }
    #[test]
    fn test_fail_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        tm.claim_task(&task.id, "agent1".to_string()).unwrap();

        tm.fail_task(&task.id, "agent1", "Error reason").unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.status, "FAILED");

        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["error"], "Error reason");
        assert!(payload["failed_at"].is_string());
    }
    #[test]
    fn test_complete_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        tm.claim_task(&task.id, "agent1".to_string()).unwrap();

        tm.complete_task(&task.id, "agent1", "Success result".to_string()).unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.status, "COMPLETED");

        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["result"], "Success result");
        assert!(payload["completed_at"].is_string());
    }

    #[test]
    fn test_get_pending_approvals() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Pending Approval Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        task.approval_status = Some("PENDING".to_string());
        task.action_risk = Some(ActionRisk::High);

        tm.insert_task(task.clone());

        let mut ignored_task = tm.create_task("org1".to_string(), "mission1".to_string(), "Other Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        ignored_task.approval_status = Some("APPROVED".to_string());
        tm.insert_task(ignored_task.clone());

        let mut ignored_task2 = tm.create_task("org2".to_string(), "mission1".to_string(), "Other Org Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        ignored_task2.approval_status = Some("PENDING".to_string());
        tm.insert_task(ignored_task2.clone());

        let pending = tm.get_pending_approvals("org1");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, task.id);
        assert_eq!(pending[0].action_risk, Some(ActionRisk::High));
    }

    #[tokio::test]
    async fn test_approve_task() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Task to Approve".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        tm.approve_task(&task.id, true, "org1").await.unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.approval_status, Some("APPROVED".to_string()));
        assert_eq!(fetched.status, "IN_PROGRESS");
    }

    #[tokio::test]
    async fn test_reject_task() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Task to Reject".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        tm.approve_task(&task.id, false, "org1").await.unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.approval_status, Some("REJECTED".to_string()));
        assert_eq!(fetched.status, "FAILED");
        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["error"], "Task was rejected by user");
    }
}

    #[tokio::test]
    async fn test_approve_task_integration() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE shared_tasks_decomposition (id TEXT PRIMARY KEY, status TEXT, dependencies TEXT, assigned_agent_id TEXT, updated_at TEXT, payload TEXT, title TEXT, description TEXT, priority TEXT, locked_until TEXT, ultraplan_phase TEXT, deliberation_log TEXT, depth INTEGER, created_at TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT, organization_id TEXT, mission_id TEXT, parent_plan_id TEXT)"
        ).execute(&pool).await;

        let db = std::sync::Arc::new(crate::db::DB {
            store: crate::db::DbStore::Sqlite(pool.clone()),
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
        });

        let tm = TaskManager::with_db(db);
        let mut task = tm.create_task("org_int".to_string(), "mission1".to_string(), "Int Task".to_string(), "Desc".to_string(), "P2".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        // Insert into DB directly for the query test
        let _ = sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, status) VALUES (?, ?, ?)")
            .bind(&task.id).bind("org_int").bind("PENDING")
            .execute(&pool).await.unwrap();

        tm.approve_task(&task.id, true, "org_int").await.unwrap();

        let row: (String,) = sqlx::query_as("SELECT approval_status FROM shared_tasks_decomposition WHERE id = ?")
            .bind(&task.id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, "APPROVED");
    }
