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
                    UPDATE shared_tasks
                    SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM shared_tasks
                        WHERE status = 'PENDING'
                        FOR UPDATE SKIP LOCKED
                        LIMIT 1
                    )
                    RETURNING *
                    "#
                )
                .bind(agent_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = row_opt {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(Some(Self::pg_row_to_task(row)))
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
                    UPDATE shared_tasks
                    SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM shared_tasks
                        WHERE status = 'PENDING'
                        LIMIT 1
                    )
                    RETURNING *
                    "#
                )
                .bind(agent_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = row_opt {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    Ok(Some(Self::sqlite_row_to_task(row)))
                } else {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    Ok(None)
                }
            }
        }
    }

    fn pg_row_to_task(row: sqlx::postgres::PgRow) -> SharedTask {
        let deps_value: Option<serde_json::Value> = row.try_get("dependencies").unwrap_or(None);
        let locked_until: Option<chrono::DateTime<chrono::Utc>> = row.try_get("locked_until").unwrap_or(None);
        let parent_plan_id_opt: Option<String> = row.try_get("parent_plan_id").unwrap_or(None);
        let dependencies = deps_value
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            parent_plan_id: parent_plan_id_opt.unwrap_or_default(),
            title: row.get("title"),
            description: row.try_get("description").unwrap_or(None),
            status: row.get("status"),
            assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or(None),
            dependencies,
            created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            mission_id: String::new(),
            priority: "NORMAL".to_string(),
            payload: String::new(),
            locked_until,
            ultraplan_phase: None,
            deliberation_log: None,
            depth: None,
            action_risk: None,
            approval_status: None,
            proposed_content: None,
        }
    }

    fn sqlite_row_to_task(row: sqlx::sqlite::SqliteRow) -> SharedTask {
        let deps_str: Option<String> = row.try_get("dependencies").unwrap_or(None);
        let locked_until_str: Option<String> = row.try_get("locked_until").unwrap_or(None);
        let locked_until = locked_until_str.and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                .or_else(|_| chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&chrono::Utc)))
                .ok()
        });

        let parent_plan_id_opt: Option<String> = row.try_get("parent_plan_id").unwrap_or(None);
        let dependencies = deps_str
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let created_str_opt: Option<String> = row.try_get("created_at").unwrap_or(None);
        let dt_created = created_str_opt.and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                .or_else(|_| chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&chrono::Utc)))
                .ok()
        }).unwrap_or_else(|| chrono::Utc::now());





        let updated_str_opt: Option<String> = row.try_get("updated_at").unwrap_or(None);
        let dt_updated = updated_str_opt.and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                .or_else(|_| chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&chrono::Utc)))
                .ok()
        }).unwrap_or_else(|| chrono::Utc::now());





        SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            parent_plan_id: parent_plan_id_opt.unwrap_or_default(),
            title: row.get("title"),
            description: row.try_get("description").unwrap_or(None),
            status: row.get("status"),
            assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or(None),
            dependencies,
            created_at: dt_created,
            updated_at: dt_updated,
            mission_id: String::new(),
            priority: "NORMAL".to_string(),
            payload: String::new(),
            locked_until,
            ultraplan_phase: None,
            deliberation_log: None,
            depth: None,
            action_risk: None,
            approval_status: None,
            proposed_content: None,
        }
    }
}
