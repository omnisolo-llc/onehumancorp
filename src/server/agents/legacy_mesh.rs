use redis::AsyncCommands;
use std::time::Duration;
use uuid::Uuid;
use sqlx::Row;



pub struct Mission {
    pub mission_id: String,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub priority: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn claim_mission(pool: &sqlx::PgPool, agent_id: &str) -> Result<Option<Mission>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let query = r#"
        UPDATE ohc_tasks.mission_queue
        SET status = 'IN_PROGRESS',
            assigned_agent = $1,
            updated_at = NOW()
        WHERE mission_id = (
            SELECT mission_id
            FROM ohc_tasks.mission_queue
            WHERE status = 'QUEUED'
            FOR UPDATE SKIP LOCKED
            LIMIT 1
        )
        RETURNING mission_id, title, status, assigned_agent, priority, payload, created_at, updated_at
    "#;

    let row = sqlx::query(query)
        .bind(agent_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(row) = row {
        let payload_str: String = row.get("payload");
        let m = Mission {
            mission_id: row.get("mission_id"),
            title: row.get("title"),
            status: row.get("status"),
            assigned_agent: row.get("assigned_agent"),
            priority: row.get("priority"),
            payload: serde_json::from_str(&payload_str).unwrap_or_default(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(Some(m))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_claim_mission_no_db() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let res = claim_mission(&pool, "agent-1").await;
        // Should fail because table doesn't exist or connection fails on execution!
        assert!(res.is_err());
    }
}
