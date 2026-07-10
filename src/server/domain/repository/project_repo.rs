use anyhow::Result;
use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub tenant_id: String,
    pub quote_id: Option<String>,
    pub customer_id: String,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectTask {
    pub id: String,
    pub tenant_id: String,
    pub project_id: String,
    pub title: String,
    pub status: String,
    pub milestone_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectMilestone {
    pub id: String,
    pub tenant_id: String,
    pub project_id: String,
    pub title: String,
    pub amount_cents: i64,
    pub status: String,
    pub payment_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ProjectRepo {
    pool: PgPool,
}

impl ProjectRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_project(&self, project: &Project) -> Result<()> {
        sqlx::query(
            "INSERT INTO projects (id, tenant_id, quote_id, customer_id, title, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&project.id)
        .bind(&project.tenant_id)
        .bind(&project.quote_id)
        .bind(&project.customer_id)
        .bind(&project.title)
        .bind(&project.status)
        .bind(project.created_at)
        .bind(project.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_projects(&self, tenant_id: &str) -> Result<Vec<Project>> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE tenant_id = $1 ORDER BY created_at DESC"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(projects)
    }

    pub async fn create_milestone(&self, milestone: &ProjectMilestone) -> Result<()> {
        sqlx::query(
            "INSERT INTO project_milestones (id, tenant_id, project_id, title, amount_cents, status, payment_link, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&milestone.id)
        .bind(&milestone.tenant_id)
        .bind(&milestone.project_id)
        .bind(&milestone.title)
        .bind(milestone.amount_cents)
        .bind(&milestone.status)
        .bind(&milestone.payment_link)
        .bind(milestone.created_at)
        .bind(milestone.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_milestones(&self, tenant_id: &str, project_id: &str) -> Result<Vec<ProjectMilestone>> {
        let milestones = sqlx::query_as::<_, ProjectMilestone>(
            "SELECT * FROM project_milestones WHERE tenant_id = $1 AND project_id = $2 ORDER BY created_at ASC"
        )
        .bind(tenant_id)
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(milestones)
    }

    pub async fn update_milestone_status(&self, tenant_id: &str, id: &str, status: &str) -> Result<()> {
        sqlx::query(
            "UPDATE project_milestones SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND id = $3"
        )
        .bind(status)
        .bind(tenant_id)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_task(&self, task: &ProjectTask) -> Result<()> {
        sqlx::query(
            "INSERT INTO project_tasks (id, tenant_id, project_id, title, status, milestone_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&task.id)
        .bind(&task.tenant_id)
        .bind(&task.project_id)
        .bind(&task.title)
        .bind(&task.status)
        .bind(&task.milestone_id)
        .bind(task.created_at)
        .bind(task.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_tasks(&self, tenant_id: &str, project_id: &str) -> Result<Vec<ProjectTask>> {
        let tasks = sqlx::query_as::<_, ProjectTask>(
            "SELECT * FROM project_tasks WHERE tenant_id = $1 AND project_id = $2 ORDER BY created_at ASC"
        )
        .bind(tenant_id)
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(tasks)
    }

    pub async fn update_task_status(&self, tenant_id: &str, id: &str, status: &str) -> Result<()> {
        sqlx::query(
            "UPDATE project_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND id = $3"
        )
        .bind(status)
        .bind(tenant_id)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_milestone_by_id(&self, tenant_id: &str, id: &str) -> Result<Option<ProjectMilestone>> {
        let milestone = sqlx::query_as::<_, ProjectMilestone>(
            "SELECT * FROM project_milestones WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(milestone)
    }

    pub async fn get_project_by_id(&self, tenant_id: &str, id: &str) -> Result<Option<Project>> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(project)
    }
}
