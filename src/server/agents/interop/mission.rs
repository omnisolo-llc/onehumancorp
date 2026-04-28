use async_trait::async_trait;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Mission {
    pub mission_id: String,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub priority: String,
    pub payload: sqlx::types::Json<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // Local SQLite sync fields
    pub synced_to_cloud: Option<bool>,
    pub cloud_mission_id: Option<String>,
    pub sync_error: Option<String>,
    pub last_synced_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
pub trait MissionProvider: Send + Sync {
    async fn claim_mission(&self, agent_id: &str) -> Result<Option<Mission>, String>;
}

pub struct CloudMissionProvider {
    pool: sqlx::PgPool,
}

impl CloudMissionProvider {
    pub fn new(pool: sqlx::PgPool) -> Self {
        CloudMissionProvider { pool }
    }
}

#[async_trait]
impl MissionProvider for CloudMissionProvider {
    async fn claim_mission(&self, agent_id: &str) -> Result<Option<Mission>, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

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
            let m = Mission {
                mission_id: row.get("mission_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                priority: row.get("priority"),
                payload: row.get("payload"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                synced_to_cloud: None,
                cloud_mission_id: None,
                sync_error: None,
                last_synced_at: None,
            };
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(Some(m))
        } else {
            Ok(None)
        }
    }
}

pub struct StandaloneMissionProvider {
    // In a real scenario this might be a SqlitePool, but for now we'll mock it
    // with an in-memory queue to satisfy the requirement
    queue: tokio::sync::Mutex<Vec<Mission>>,
}

impl StandaloneMissionProvider {
    pub fn new() -> Self {
        StandaloneMissionProvider {
            queue: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    // Helper for testing
    pub async fn push_mission(&self, mission: Mission) {
        let mut q = self.queue.lock().await;
        q.push(mission);
    }
}

#[async_trait]
impl MissionProvider for StandaloneMissionProvider {
    async fn claim_mission(&self, agent_id: &str) -> Result<Option<Mission>, String> {
        let mut q = self.queue.lock().await;

        // Find first QUEUED mission
        if let Some(pos) = q.iter().position(|m| m.status == "QUEUED") {
            let mut mission = q.remove(pos);
            mission.status = "IN_PROGRESS".to_string();
            mission.assigned_agent = Some(agent_id.to_string());
            mission.updated_at = chrono::Utc::now();

            // Re-insert at the end (or just keep it in another list if we want to track it)
            // For simplicity, we just return it
            q.push(mission.clone());

            return Ok(Some(mission));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cloud_claim_mission_no_db() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let provider = CloudMissionProvider::new(pool);
        let res = provider.claim_mission("agent-1").await;
        // Should fail because table doesn't exist or connection fails
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_standalone_claim_mission() {
        let provider = StandaloneMissionProvider::new();

        let mission = Mission {
            mission_id: "m-123".to_string(),
            title: "Test Task".to_string(),
            status: "QUEUED".to_string(),
            assigned_agent: None,
            priority: "HIGH".to_string(),
            payload: sqlx::types::Json(serde_json::json!({})),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            synced_to_cloud: None,
            cloud_mission_id: None,
            sync_error: None,
            last_synced_at: None,
        };

        provider.push_mission(mission).await;

        let res = provider.claim_mission("agent-1").await.unwrap();
        assert!(res.is_some());

        let claimed = res.unwrap();
        assert_eq!(claimed.status, "IN_PROGRESS");
        assert_eq!(claimed.assigned_agent, Some("agent-1".to_string()));

        // Next claim should be None
        let res2 = provider.claim_mission("agent-2").await.unwrap();
        assert!(res2.is_none());
    }
}
