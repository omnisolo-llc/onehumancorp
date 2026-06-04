use crate::db::{DB, DbStore};
use crate::tasks::SharedTask;
use std::sync::Arc;
use sqlx::Row;

pub struct TaskDbService {
    db: Arc<DB>,
    sqlite_mu: tokio::sync::Mutex<()>,
}

impl TaskDbService {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            sqlite_mu: tokio::sync::Mutex::new(()),
        }
    }

    pub async fn claim_task(&self, agent_id: &str) -> Result<Option<SharedTask>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT * FROM shared_tasks
                    WHERE status = 'PENDING'
                    FOR UPDATE SKIP LOCKED
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = row_opt {
                    let id: String = row.get("id");

                    sqlx::query(
                        r#"
                        UPDATE shared_tasks
                        SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
                        WHERE id = $2
                        "#
                    )
                    .bind(agent_id)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    let updated_row = sqlx::query("SELECT * FROM shared_tasks WHERE id = $1")
                        .bind(&id)
                        .fetch_one(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(Some(Self::pg_row_to_task(updated_row)))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mu.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let row_opt = sqlx::query(
                    r#"
                    SELECT * FROM shared_tasks
                    WHERE status = 'PENDING'
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = row_opt {
                    let id: String = row.get("id");

                    sqlx::query(
                        r#"
                        UPDATE shared_tasks
                        SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
                        WHERE id = ?
                        "#
                    )
                    .bind(agent_id)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                    let updated_row = sqlx::query("SELECT * FROM shared_tasks WHERE id = ?")
                        .bind(&id)
                        .fetch_one(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(Some(Self::sqlite_row_to_task(updated_row)))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
        }
    }

    fn pg_row_to_task(row: sqlx::postgres::PgRow) -> SharedTask {
        let deps_value: Option<serde_json::Value> = row.get("dependencies");
        let dependencies = deps_value
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            parent_plan_id: row.get("parent_plan_id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            assigned_agent_id: row.get("assigned_agent_id"),
            dependencies,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            mission_id: String::new(),
            priority: "NORMAL".to_string(),
            payload: String::new(),
            locked_until: None,
            ultraplan_phase: None,
            deliberation_log: None,
            depth: None,
            action_risk: None,
            approval_status: None,
            proposed_content: None,
        }
    }

    fn sqlite_row_to_task(row: sqlx::sqlite::SqliteRow) -> SharedTask {
        let deps_str: Option<String> = row.get("dependencies");
        let dependencies = deps_str
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let created_str: String = row.get("created_at");
        let dt_created = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S")
            .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
            .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
            .unwrap_or_else(|_| chrono::Utc::now());

        let updated_str: String = row.get("updated_at");
        let dt_updated = chrono::NaiveDateTime::parse_from_str(&updated_str, "%Y-%m-%d %H:%M:%S")
            .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
            .or_else(|_| chrono::DateTime::parse_from_rfc3339(&updated_str).map(|d| d.with_timezone(&chrono::Utc)))
            .unwrap_or_else(|_| chrono::Utc::now());

        SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            parent_plan_id: row.get("parent_plan_id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            assigned_agent_id: row.get("assigned_agent_id"),
            dependencies,
            created_at: dt_created,
            updated_at: dt_updated,
            mission_id: String::new(),
            priority: "NORMAL".to_string(),
            payload: String::new(),
            locked_until: None,
            ultraplan_phase: None,
            deliberation_log: None,
            depth: None,
            action_risk: None,
            approval_status: None,
            proposed_content: None,
        }
    }
}
