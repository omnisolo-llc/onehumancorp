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
        let deps_value: Option<serde_json::Value> = row.try_get("dependencies").unwrap_or(None);
        let dependencies = deps_value
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();

        let action_risk_str: Option<String> = row.try_get("action_risk").unwrap_or(None);
        let action_risk = action_risk_str.and_then(|s| crate::tasks::ActionRisk::from_str(&s).into());

        let delib_value: Option<serde_json::Value> = row.try_get("deliberation_log").unwrap_or(None);
        let deliberation_log = delib_value.map(|v| v.to_string());

        SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            parent_plan_id: row.try_get("parent_plan_id").unwrap_or_else(|_| String::new()),
            title: row.get("title"),
            description: row.try_get("description").unwrap_or(None),
            status: row.get("status"),
            assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or(None),
            dependencies,
            created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            mission_id: row.try_get("mission_id").unwrap_or_else(|_| String::new()),
            priority: row.try_get("priority").unwrap_or_else(|_| "NORMAL".to_string()),
            payload: row.try_get("payload").unwrap_or_else(|_| String::new()),
            locked_until: row.try_get("locked_until").unwrap_or(None),
            ultraplan_phase: row.try_get("ultraplan_phase").unwrap_or(None),
            deliberation_log,
            depth: row.try_get("depth").unwrap_or(None),
            action_risk: Some(action_risk.unwrap_or(crate::tasks::ActionRisk::Unspecified)),
            approval_status: row.try_get("approval_status").unwrap_or(None),
            proposed_content: row.try_get("proposed_content").unwrap_or(None),
        }
    }

    fn sqlite_row_to_task(row: sqlx::sqlite::SqliteRow) -> SharedTask {
        let deps_str: Option<String> = row.try_get("dependencies").unwrap_or(None);
        let dependencies = deps_str
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        let delib_str: Option<String> = row.try_get("deliberation_log").unwrap_or(None);

        let created_str: String = row.try_get("created_at").unwrap_or_else(|_| String::new());
        let dt_created = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S")
            .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
            .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
            .unwrap_or_else(|_| chrono::Utc::now());

        let updated_str: String = row.try_get("updated_at").unwrap_or_else(|_| String::new());
        let dt_updated = chrono::NaiveDateTime::parse_from_str(&updated_str, "%Y-%m-%d %H:%M:%S")
            .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
            .or_else(|_| chrono::DateTime::parse_from_rfc3339(&updated_str).map(|d| d.with_timezone(&chrono::Utc)))
            .unwrap_or_else(|_| chrono::Utc::now());

        let locked_until_str: Option<String> = row.try_get("locked_until").unwrap_or(None);
        let locked_until = locked_until_str.and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                .or_else(|_| chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&chrono::Utc)))
                .ok()
        });

        let action_risk_str: Option<String> = row.try_get("action_risk").unwrap_or(None);
        let action_risk = action_risk_str.and_then(|s| crate::tasks::ActionRisk::from_str(&s).into());

        SharedTask {
            id: row.get("id"),
            organization_id: row.get("organization_id"),
            parent_plan_id: row.try_get("parent_plan_id").unwrap_or_else(|_| String::new()),
            title: row.get("title"),
            description: row.try_get("description").unwrap_or(None),
            status: row.get("status"),
            assigned_agent_id: row.try_get("assigned_agent_id").unwrap_or(None),
            dependencies,
            created_at: dt_created,
            updated_at: dt_updated,
            mission_id: row.try_get("mission_id").unwrap_or_else(|_| String::new()),
            priority: row.try_get("priority").unwrap_or_else(|_| "NORMAL".to_string()),
            payload: row.try_get("payload").unwrap_or_else(|_| String::new()),
            locked_until,
            ultraplan_phase: row.try_get("ultraplan_phase").unwrap_or(None),
            deliberation_log: delib_str,
            depth: row.try_get("depth").unwrap_or(None),
            action_risk: Some(action_risk.unwrap_or(crate::tasks::ActionRisk::Unspecified)),
            approval_status: row.try_get("approval_status").unwrap_or(None),
            proposed_content: row.try_get("proposed_content").unwrap_or(None),
        }
    }
}
