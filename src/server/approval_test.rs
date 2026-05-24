#[cfg(test)]
mod tests {
    use crate::tasks::TaskManager;
    use crate::hub::Hub;
    use crate::MyHubService;
    use crate::proto::orchestration::{UpdateTaskStatusRequest, ApproveTaskRequest, ActionRisk as ProtoActionRisk};
    use tonic::Request;
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use sqlx::postgres::PgPoolOptions;

    async fn setup_hub_service() -> MyHubService {
        let (tx, _) = mpsc::channel(100);
        // Using a dummy pool as we won't be doing DB operations that require a real connection in these unit tests
        let pool = PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let db = Arc::new(crate::db::DB {
            store: crate::db::DbStore::Postgres,
            pool: pool.clone(),
        });
        MyHubService::new(hub, pool, db)
    }

    #[tokio::test]
    async fn test_approval_workflow_grpc() {
        let svc = setup_hub_service().await;
        let org_id = "test-org".to_string();
        let agent_id = "agent-1".to_string();

        // 1. Create a task
        let task = svc.hub.task_manager().create_task(
            org_id.clone(),
            "mission-1".to_string(),
            "Approval Task".to_string(),
            "Description".to_string(),
            "P1".to_string()
        ).unwrap();

        // 2. Claim the task
        svc.hub.task_manager().claim_task(&task.id, agent_id.clone()).unwrap();

        // 3. Agent requests approval via gRPC
        let req = UpdateTaskStatusRequest {
            task_id: task.id.clone(),
            status: "PENDING_APPROVAL".to_string(),
            agent_id: agent_id.clone(),
            result: String::new(),
            proposed_content: "Drafted Email Content".to_string(),
            action_risk: ProtoActionRisk::High as i32,
        };

        svc.update_task_status(Request::new(req)).await.unwrap();

        // Verify task state
        let task_after_req = svc.hub.task_manager().get_task(&task.id).unwrap();
        assert_eq!(task_after_req.status, "PENDING_APPROVAL");
        assert_eq!(task_after_req.approval_status, Some("PENDING".to_string()));
        assert_eq!(task_after_req.proposed_content, Some("Drafted Email Content".to_string()));

        // 4. Human approves the task via gRPC
        let mut auth_req = Request::new(ApproveTaskRequest {
            task_id: task.id.clone(),
            is_approved: true,
        });

        // Mock claims for the interceptor/handler
        auth_req.extensions_mut().insert(::server_common::Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: Some(org_id.clone()),
            username: "user".to_string(),
            email: "user@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        });

        svc.approve_task(auth_req).await.unwrap();

        // Verify task state
        let task_after_appr = svc.hub.task_manager().get_task(&task.id).unwrap();
        assert_eq!(task_after_appr.status, "APPROVED");
        assert_eq!(task_after_appr.approval_status, Some("APPROVED".to_string()));

        // 5. Agent polls and claims the approved task
        let claimed_tasks = svc.hub.task_manager().poll_tasks(&agent_id, 10);
        assert!(claimed_tasks.iter().any(|t| t.id == task.id));

        let task_final = svc.hub.task_manager().get_task(&task.id).unwrap();
        assert_eq!(task_final.status, "IN_PROGRESS");
        assert_eq!(task_final.assigned_agent_id, Some(agent_id));
    }

    #[tokio::test]
    async fn test_rejection_workflow_grpc() {
        let svc = setup_hub_service().await;
        let org_id = "test-org".to_string();
        let agent_id = "agent-1".to_string();

        let task = svc.hub.task_manager().create_task(
            org_id.clone(),
            "mission-1".to_string(),
            "Rejection Task".to_string(),
            "Description".to_string(),
            "P1".to_string()
        ).unwrap();

        svc.hub.task_manager().claim_task(&task.id, agent_id.clone()).unwrap();

        let req = UpdateTaskStatusRequest {
            task_id: task.id.clone(),
            status: "PENDING_APPROVAL".to_string(),
            agent_id: agent_id.clone(),
            result: String::new(),
            proposed_content: "Bad Content".to_string(),
            action_risk: ProtoActionRisk::High as i32,
        };
        svc.update_task_status(Request::new(req)).await.unwrap();

        let mut auth_req = Request::new(ApproveTaskRequest {
            task_id: task.id.clone(),
            is_approved: false,
        });
        auth_req.extensions_mut().insert(::server_common::Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: Some(org_id.clone()),
            username: "user".to_string(),
            email: "user@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        });

        svc.approve_task(auth_req).await.unwrap();

        let task_after_rej = svc.hub.task_manager().get_task(&task.id).unwrap();
        assert_eq!(task_after_rej.status, "REJECTED");
        assert_eq!(task_after_rej.approval_status, Some("REJECTED".to_string()));
    }
}
