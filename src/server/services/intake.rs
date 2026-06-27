use crate::domain::intake::{ProjectIntake, ProjectTask};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct IntakeService {
    pool: Arc<PgPool>,
}

impl IntakeService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_intake(&self, tenant_id: &str, source: &str, raw_content: &str, client_info: Option<serde_json::Value>) -> Result<ProjectIntake, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let intake = ProjectIntake {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            source: source.to_string(),
            raw_content: raw_content.to_string(),
            client_info: client_info.clone(),
            status: "PENDING".to_string(),
            created_at: now,
            updated_at: now,
        };

        sqlx::query(
            r#"
            INSERT INTO project_intakes (id, tenant_id, source, raw_content, client_info, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&intake.id)
        .bind(&intake.tenant_id)
        .bind(&intake.source)
        .bind(&intake.raw_content)
        .bind(&intake.client_info)
        .bind(&intake.status)
        .bind(&intake.created_at)
        .bind(&intake.updated_at)
        .execute(&*self.pool)
        .await?;

        Ok(intake)
    }

    pub async fn list_intakes(&self, tenant_id: &str) -> Result<Vec<ProjectIntake>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, tenant_id, source, raw_content, client_info, status, created_at, updated_at
            FROM project_intakes
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&*self.pool)
        .await?;

        let mut intakes = Vec::new();
        for row in rows {
            intakes.push(ProjectIntake {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                source: row.get("source"),
                raw_content: row.get("raw_content"),
                client_info: row.get("client_info"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        Ok(intakes)
    }

    pub async fn create_task_from_proposal(&self, tenant_id: &str, proposal_id: &str, title: &str, description: Option<String>, assigned_to: Option<String>) -> Result<ProjectTask, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let task = ProjectTask {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            proposal_id: Some(proposal_id.to_string()),
            title: title.to_string(),
            description: description.clone(),
            assigned_to: assigned_to.clone(),
            status: "TODO".to_string(),
            created_at: now,
            updated_at: now,
        };

        sqlx::query(
            r#"
            INSERT INTO project_tasks (id, tenant_id, proposal_id, title, description, assigned_to, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(&task.id)
        .bind(&task.tenant_id)
        .bind(&task.proposal_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.assigned_to)
        .bind(&task.status)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .execute(&*self.pool)
        .await?;

        Ok(task)
    }

    pub async fn get_tasks_for_proposal(&self, tenant_id: &str, proposal_id: &str) -> Result<Vec<ProjectTask>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, tenant_id, proposal_id, title, description, assigned_to, status, created_at, updated_at
            FROM project_tasks
            WHERE tenant_id = $1 AND proposal_id = $2
            ORDER BY created_at ASC
            "#,
        )
        .bind(tenant_id)
        .bind(proposal_id)
        .fetch_all(&*self.pool)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(ProjectTask {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                proposal_id: row.get("proposal_id"),
                title: row.get("title"),
                description: row.get("description"),
                assigned_to: row.get("assigned_to"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        Ok(tasks)
    }
}
