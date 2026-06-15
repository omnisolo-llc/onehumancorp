use sqlx::{PgPool, Row};
use crate::services::sync::local_repository::{LocalRepository, LocalMission, MissionPayload};

pub struct PgLocalRepository {
    pool: PgPool,
}

impl PgLocalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl LocalRepository for PgLocalRepository {
    async fn get_pending_sync(&self, organization_id: &str, limit: i32) -> Result<Vec<LocalMission>, String> {
        let rows = sqlx::query(
            "SELECT id, organization_id, status, payload, created_at, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at
             FROM agent_missions
             WHERE organization_id = $1 AND synced_to_cloud = FALSE AND (sync_error IS NULL OR last_synced_at < NOW() - INTERVAL '5 minutes')
             LIMIT $2"
        )
        .bind(organization_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut missions = Vec::new();
        for row in rows {
            let payload_val: serde_json::Value = row.get("payload");
            let payload_str = payload_val.to_string();
            let payload: MissionPayload = serde_json::from_str(&payload_str).unwrap_or(MissionPayload {
                role: "".to_string(),
                task: "".to_string(),
                context: None,
                action_risk: None,
            });

            missions.push(LocalMission {
                id: row.get("id"),
                organization_id: row.get("organization_id"),
                status: row.get("status"),
                payload,
                created_at: row.try_get("created_at").unwrap_or_default(),
                synced_to_cloud: row.try_get("synced_to_cloud").unwrap_or(false),
                cloud_mission_id: row.try_get("cloud_mission_id").unwrap_or(None),
                sync_error: row.try_get("sync_error").unwrap_or(None),
                last_synced_at: row.try_get("last_synced_at").unwrap_or(None),
            });
        }

        Ok(missions)
    }

    async fn mark_synced(&self, organization_id: &str, local_id: &str, cloud_id: &str) -> Result<(), String> {
        sqlx::query(
            "UPDATE agent_missions
             SET synced_to_cloud = TRUE, cloud_mission_id = $1, sync_error = NULL, last_synced_at = NOW()
             WHERE id = $2 AND organization_id = $3"
        )
        .bind(cloud_id)
        .bind(local_id)
        .bind(organization_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn mark_sync_error(&self, organization_id: &str, local_id: &str, sync_error: &str) -> Result<(), String> {
        sqlx::query(
            "UPDATE agent_missions
             SET sync_error = $1, last_synced_at = NOW()
             WHERE id = $2 AND organization_id = $3"
        )
        .bind(sync_error)
        .bind(local_id)
        .bind(organization_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_active_escalations(&self, organization_id: &str) -> Result<Vec<LocalMission>, String> {
        let rows = sqlx::query(
            "SELECT id, organization_id, status, payload, created_at, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at
             FROM agent_missions
             WHERE organization_id = $1 AND synced_to_cloud = TRUE AND cloud_mission_id IS NOT NULL AND status NOT IN ('COMPLETED', 'FAILED')"
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut missions = Vec::new();
        for row in rows {
            let payload_val: serde_json::Value = row.get("payload");
            let payload_str = payload_val.to_string();
            let payload: MissionPayload = serde_json::from_str(&payload_str).unwrap_or(MissionPayload {
                role: "".to_string(),
                task: "".to_string(),
                context: None,
                action_risk: None,
            });

            missions.push(LocalMission {
                id: row.get("id"),
                organization_id: row.get("organization_id"),
                status: row.get("status"),
                payload,
                created_at: row.try_get("created_at").unwrap_or_default(),
                synced_to_cloud: row.try_get("synced_to_cloud").unwrap_or(false),
                cloud_mission_id: row.try_get("cloud_mission_id").unwrap_or(None),
                sync_error: row.try_get("sync_error").unwrap_or(None),
                last_synced_at: row.try_get("last_synced_at").unwrap_or(None),
            });
        }

        Ok(missions)
    }

    async fn update_local_status(&self, organization_id: &str, local_id: &str, new_status: &str) -> Result<(), String> {
        sqlx::query(
            "UPDATE agent_missions
             SET status = $1, updated_at = NOW()
             WHERE id = $2 AND organization_id = $3"
        )
        .bind(new_status)
        .bind(local_id)
        .bind(organization_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
