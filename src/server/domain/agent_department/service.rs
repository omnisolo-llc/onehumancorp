use std::sync::Arc;
use super::models::{AgentDepartment, AgentTask, TaskApproval};
use super::repository::AgentDepartmentRepository;
use uuid::Uuid;
use chrono::Utc;

pub struct AgentDepartmentService {
    repo: Arc<AgentDepartmentRepository>,
}

impl AgentDepartmentService {
    pub fn new(repo: Arc<AgentDepartmentRepository>) -> Self {
        Self { repo }
    }

    pub async fn provision_default_departments(&self, tenant_id: &str) -> Result<Vec<AgentDepartment>, String> {
        let defaults = vec![
            ("Operations", "The Manager: Handles day-to-day execution"),
            ("Marketing & Advertising", "The Promoter: Gets the business noticed"),
            ("Sales & Acquisition", "The Salesperson: Turns interest into revenue"),
            ("Customer Success", "The Ambassador: Keeps customers happy"),
            ("Finance & Payments", "The Accountant: Makes sure money flows correctly"),
            ("Legal & Compliance", "The Protector: Keeps the business safe"),
            ("Business Advisory", "The Advisor: Acts as a personal business consultant"),
        ];

        let mut created = Vec::new();
        for (name, desc) in defaults {
            let dept = AgentDepartment {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                name: name.to_string(),
                description: Some(desc.to_string()),
                auto_execute: false, // Default to false (draft for review)
                created_at: Some(Utc::now()),
                updated_at: Some(Utc::now()),
            };
            let result = self.repo.create_department(dept).await?;
            created.push(result);
        }
        Ok(created)
    }

    pub async fn dispatch_event(&self, tenant_id: &str, event_type: &str, payload: serde_json::Value) -> Result<(), String> {
        // Mock routing logic based on event_type
        let (_dept_name, task_title, action_risk, proposed_action) = match event_type {
            "order.created" => {
                // Operations handles inventory, Customer Success drafts email
                // In a real system, we'd query departments by name and create tasks for both.
                // For this mock, we'll simulate the "Customer Success" drafting an email.
                (
                    "Customer Success",
                    "Draft Order Confirmation",
                    "LOW",
                    Some("Drafted email: Thank you for your order...".to_string()),
                )
            }
            _ => return Ok(()), // Ignore unhandled events
        };

        // For simplicity in this demo, just generate fake department IDs or assume one exists.
        // In a real implementation, we'd query the DB for the department ID.
        let mock_dept_id = Uuid::new_v4().to_string();

        let task = AgentTask {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            department_id: mock_dept_id,
            title: task_title.to_string(),
            description: Some(format!("Handling event: {}", event_type)),
            status: "PENDING_APPROVAL".to_string(),
            action_risk: action_risk.to_string(),
            event_payload: Some(payload),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        let created_task = self.repo.create_task(task).await?;

        let approval = TaskApproval {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            task_id: created_task.id,
            status: "PENDING".to_string(),
            proposed_action,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        self.repo.create_approval(approval).await?;

        Ok(())
    }

    pub async fn get_pending_approvals(&self, tenant_id: &str) -> Result<Vec<TaskApproval>, String> {
        self.repo.get_pending_approvals(tenant_id).await
    }

    pub async fn review_approval(&self, tenant_id: &str, approval_id: &str, action: &str) -> Result<(), String> {
        let status = match action.to_uppercase().as_str() {
            "APPROVE" => "APPROVED",
            "REJECT" => "REJECTED",
            "MODIFY" => "MODIFIED",
            _ => return Err("Invalid action".to_string()),
        };
        self.repo.update_approval_status(tenant_id, approval_id, status).await
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports, unused_variables)]
    use super::*;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::Executor;
    use tokio;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Initial setup for tenants
        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS tenants (
                tenant_id TEXT PRIMARY KEY,
                owner_id TEXT,
                business_name TEXT,
                tier TEXT,
                created_at TEXT
            );
            "#
        ).await.unwrap();

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS agent_departments (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                auto_execute BOOLEAN NOT NULL DEFAULT 0,
                created_at TEXT,
                updated_at TEXT
            );
            "#
        ).await.unwrap();

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS agent_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                department_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                action_risk TEXT NOT NULL,
                event_payload TEXT,
                created_at TEXT,
                updated_at TEXT
            );
            "#
        ).await.unwrap();

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS task_approvals (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                task_id TEXT NOT NULL,
                status TEXT NOT NULL,
                proposed_action TEXT,
                created_at TEXT,
                updated_at TEXT
            );
            "#
        ).await.unwrap();

                Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_provision_default_departments() {
        let db = setup_test_db().await;
        // Fix up db store


        let repo = Arc::new(AgentDepartmentRepository::new(db));
        let service = AgentDepartmentService::new(repo);

        let tenant_id = Uuid::new_v4().to_string();

        let result = service.provision_default_departments(&tenant_id).await;
        assert!(result.is_ok());
        let depts = result.unwrap();
        assert_eq!(depts.len(), 7);
        assert_eq!(depts[0].name, "Operations");
        assert_eq!(depts[6].name, "Business Advisory");
    }

    #[tokio::test]
    async fn test_dispatch_event_creates_task_and_approval() {
        let db = setup_test_db().await;
        // Fix up db store


        let repo = Arc::new(AgentDepartmentRepository::new(db));
        let service = AgentDepartmentService::new(repo);

        let tenant_id = Uuid::new_v4().to_string();

        let result = service.dispatch_event(&tenant_id, "order.created", serde_json::json!({"order_id": "123"})).await;
        assert!(result.is_ok());

        let pending = service.get_pending_approvals(&tenant_id).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].status, "PENDING");
        assert_eq!(pending[0].proposed_action.as_deref(), Some("Drafted email: Thank you for your order..."));
    }

    #[tokio::test]
    async fn test_review_approval() {
        let db = setup_test_db().await;
        // Fix up db store


        let repo = Arc::new(AgentDepartmentRepository::new(db));
        let service = AgentDepartmentService::new(repo);

        let tenant_id = Uuid::new_v4().to_string();

        service.dispatch_event(&tenant_id, "order.created", serde_json::json!({"order_id": "123"})).await.unwrap();

        let pending = service.get_pending_approvals(&tenant_id).await.unwrap();
        assert_eq!(pending.len(), 1);

        let approval_id = &pending[0].id;

        let review_result = service.review_approval(&tenant_id, approval_id, "APPROVE").await;
        assert!(review_result.is_ok());

        // Ensure no pending approvals left
        let pending_after = service.get_pending_approvals(&tenant_id).await.unwrap();
        assert_eq!(pending_after.len(), 0);
    }
}
