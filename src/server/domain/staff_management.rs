use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct StaffTask {
    pub id: String,
    pub tenant_id: String,
    pub staff_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub status: String,
    pub created_by_agent: Option<String>,
    pub due_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ShiftSummary {
    pub id: String,
    pub tenant_id: String,
    pub shift_date: String, // format YYYY-MM-DD
    pub summary_text: String,
    pub generated_by_agent: Option<String>,
    pub metrics: Option<Value>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_staff_task(
    tenant_id: &str,
    title: &str,
    description: Option<&str>,
    staff_id: Option<&str>,
    priority: &str,
    created_by_agent: Option<&str>,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<String, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();

    // In a real execution, we'd use set_org_context within a transaction.
    // Assuming this might be called in the context of an agent, we enforce it here manually for the single query.
    let mut tx = pool.begin().await?;
    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO staff_tasks (id, tenant_id, title, description, staff_id, priority, created_by_agent)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(title)
    .bind(description)
    .bind(staff_id)
    .bind(priority)
    .bind(created_by_agent)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(id)
}

pub async fn get_staff_tasks(
    tenant_id: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<StaffTask>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let tasks = sqlx::query_as::<_, StaffTask>(
        r#"
        SELECT
            id, tenant_id, staff_id, title, description, priority, status, created_by_agent,
            due_at::text as due_at,
            completed_at::text as completed_at,
            created_at::text as created_at,
            updated_at::text as updated_at
        FROM staff_tasks
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#
    ).bind(tenant_id)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(tasks)
}

pub async fn update_staff_task(
    tenant_id: &str,
    task_id: &str,
    status: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let completed_at = if status == "completed" {
        Some(chrono::Utc::now().to_rfc3339())
    } else {
        None
    };

    sqlx::query(
        r#"
        UPDATE staff_tasks
        SET status = $1, completed_at = $2::timestamptz, updated_at = CURRENT_TIMESTAMP
        WHERE id = $3 AND tenant_id = $4
        "#
    )
    .bind(status)
    .bind(completed_at)
    .bind(task_id)
    .bind(tenant_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn create_shift_summary(
    tenant_id: &str,
    summary_text: &str,
    generated_by_agent: Option<&str>,
    metrics: Option<Value>,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<String, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let shift_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let mut tx = pool.begin().await?;
    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO shift_summaries (id, tenant_id, shift_date, summary_text, generated_by_agent, metrics)
        VALUES ($1, $2, $3::date, $4, $5, $6)
        "#
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(&shift_date)
    .bind(summary_text)
    .bind(generated_by_agent)
    .bind(metrics)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(id)
}

pub async fn get_shift_summary(
    tenant_id: &str,
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Option<ShiftSummary>, sqlx::Error> {
    let shift_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let mut tx = pool.begin().await?;
    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    let summary = sqlx::query_as::<_, ShiftSummary>(
        r#"
        SELECT
            id, tenant_id, shift_date::text as shift_date, summary_text, generated_by_agent, metrics,
            created_at::text as created_at,
            updated_at::text as updated_at
        FROM shift_summaries
        WHERE tenant_id = $1 AND shift_date = $2::date
        ORDER BY created_at DESC
        LIMIT 1
        "#
    ).bind(tenant_id).bind(shift_date)
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(summary)
}
