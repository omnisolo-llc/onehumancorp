use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{AgentDepartment, AgentTask, TaskApproval};
use sqlx::Row;
use uuid::Uuid;
use chrono::Utc;

pub struct AgentDepartmentRepository {
    db: Arc<DB>,
}

impl AgentDepartmentRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_department(&self, department: AgentDepartment) -> Result<AgentDepartment, String> {
        let tenant_id = Uuid::parse_str(&department.tenant_id).map_err(|e| e.to_string())?;
        let id = Uuid::parse_str(&department.id).map_err(|e| e.to_string())?;
        let auto_execute = department.auto_execute;
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO agent_departments (id, tenant_id, name, description, auto_execute, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(id)
                .bind(tenant_id)
                .bind(&department.name)
                .bind(&department.description)
                .bind(auto_execute)
                .bind(&department.created_at)
                .bind(&department.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO agent_departments (id, tenant_id, name, description, auto_execute, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(department.id.clone())
                .bind(department.tenant_id.clone())
                .bind(&department.name)
                .bind(&department.description)
                .bind(auto_execute)
                .bind(&department.created_at)
                .bind(&department.updated_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(department)
    }

    pub async fn create_task(&self, task: AgentTask) -> Result<AgentTask, String> {
        let tenant_id = Uuid::parse_str(&task.tenant_id).map_err(|e| e.to_string())?;
        let id = Uuid::parse_str(&task.id).map_err(|e| e.to_string())?;
        let department_id = Uuid::parse_str(&task.department_id).map_err(|e| e.to_string())?;

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO agent_tasks (id, tenant_id, department_id, title, description, status, action_risk, event_payload, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                    "#
                )
                .bind(id)
                .bind(tenant_id)
                .bind(department_id)
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.action_risk)
                .bind(&task.event_payload)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let payload_str = task.event_payload.as_ref().map(|p| p.to_string());
                sqlx::query(
                    r#"
                    INSERT INTO agent_tasks (id, tenant_id, department_id, title, description, status, action_risk, event_payload, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(task.id.clone())
                .bind(task.tenant_id.clone())
                .bind(task.department_id.clone())
                .bind(&task.title)
                .bind(&task.description)
                .bind(&task.status)
                .bind(&task.action_risk)
                .bind(payload_str)
                .bind(&task.created_at)
                .bind(&task.updated_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(task)
    }

    pub async fn create_approval(&self, approval: TaskApproval) -> Result<TaskApproval, String> {
        let tenant_id = Uuid::parse_str(&approval.tenant_id).map_err(|e| e.to_string())?;
        let id = Uuid::parse_str(&approval.id).map_err(|e| e.to_string())?;
        let task_id = Uuid::parse_str(&approval.task_id).map_err(|e| e.to_string())?;

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO task_approvals (id, tenant_id, task_id, status, proposed_action, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(id)
                .bind(tenant_id)
                .bind(task_id)
                .bind(&approval.status)
                .bind(&approval.proposed_action)
                .bind(&approval.created_at)
                .bind(&approval.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO task_approvals (id, tenant_id, task_id, status, proposed_action, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(approval.id.clone())
                .bind(approval.tenant_id.clone())
                .bind(approval.task_id.clone())
                .bind(&approval.status)
                .bind(&approval.proposed_action)
                .bind(&approval.created_at)
                .bind(&approval.updated_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(approval)
    }

    pub async fn get_pending_approvals(&self, tenant_id: &str) -> Result<Vec<TaskApproval>, String> {
        let tenant_id_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;
        let mut approvals = Vec::new();
        match &self.db.store {
            DbStore::Postgres => {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, task_id, status, proposed_action, created_at, updated_at
                    FROM task_approvals
                    WHERE tenant_id = $1 AND status = 'PENDING'
                    "#
                )
                .bind(tenant_id_uuid)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                for row in rows {
                    approvals.push(TaskApproval {
                        id: row.get::<Uuid, _>("id").to_string(),
                        tenant_id: row.get::<Uuid, _>("tenant_id").to_string(),
                        task_id: row.get::<Uuid, _>("task_id").to_string(),
                        status: row.get("status"),
                        proposed_action: row.get("proposed_action"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    });
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, task_id, status, proposed_action, created_at, updated_at
                    FROM task_approvals
                    WHERE tenant_id = ? AND status = 'PENDING'
                    "#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                for row in rows {
                    let id_str: String = row.get("id");
                    let tenant_id_str: String = row.get("tenant_id");
                    let task_id_str: String = row.get("task_id");
                    approvals.push(TaskApproval {
                        id: id_str,
                        tenant_id: tenant_id_str,
                        task_id: task_id_str,
                        status: row.get("status"),
                        proposed_action: row.get("proposed_action"),
                        created_at: None, // Simplified for Sqlite in this example
                        updated_at: None,
                    });
                }
            }
        }
        Ok(approvals)
    }

    pub async fn update_approval_status(&self, tenant_id: &str, id: &str, status: &str) -> Result<(), String> {
        let tenant_id_uuid = Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;
        let id_uuid = Uuid::parse_str(id).map_err(|e| e.to_string())?;
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    UPDATE task_approvals SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4
                    "#
                )
                .bind(status)
                .bind(Utc::now())
                .bind(id_uuid)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    UPDATE task_approvals SET status = ?, updated_at = ? WHERE id = ?
                    "#
                )
                .bind(status)
                .bind(Utc::now().to_rfc3339())
                .bind(id)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
