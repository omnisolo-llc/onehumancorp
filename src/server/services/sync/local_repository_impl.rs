use crate::services::sync::local_repository::{LocalMission, LocalRepository, MissionPayload};
use sqlx::{PgPool, Row};

const UPDATE_LOCAL_STATUS_SQL: &str =
    "UPDATE agent_missions SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3";

fn parse_mission_payload(payload: &str) -> Result<MissionPayload, String> {
    serde_json::from_str(payload).map_err(|error| format!("invalid mission payload: {error}"))
}

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
    async fn get_pending_sync(
        &self,
        organization_id: &str,
        limit: i32,
    ) -> Result<Vec<LocalMission>, String> {
        let mut transaction = self.pool.begin().await.map_err(|error| error.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *transaction, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let rows = sqlx::query(
            "SELECT id, tenant_id AS organization_id, status, payload, created_at, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at
             FROM agent_missions
             WHERE tenant_id = $1 AND synced_to_cloud = false AND (sync_error IS NULL OR last_synced_at < NOW() - INTERVAL '5 minutes')
             ORDER BY created_at, id
             LIMIT $2"
        )
        .bind(organization_id)
        .bind(limit)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;

        let mut missions = Vec::new();
        for row in rows {
            let payload_text: String = row.try_get("payload").map_err(|e| e.to_string())?;
            let payload = parse_mission_payload(&payload_text)?;

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

    async fn mark_synced(
        &self,
        organization_id: &str,
        local_id: &str,
        cloud_id: &str,
    ) -> Result<(), String> {
        let mut transaction = self.pool.begin().await.map_err(|error| error.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *transaction, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let result = sqlx::query(
            "UPDATE agent_missions
             SET synced_to_cloud = true, cloud_mission_id = $1, sync_error = NULL, last_synced_at = NOW()
             WHERE id = $2 AND tenant_id = $3"
        )
        .bind(cloud_id)
        .bind(local_id)
        .bind(organization_id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;

        if result.rows_affected() != 1 {
            return Err("mission was not found in the requested tenant".to_string());
        }

        Ok(())
    }

    async fn mark_sync_error(
        &self,
        organization_id: &str,
        local_id: &str,
        sync_error: &str,
    ) -> Result<(), String> {
        let mut transaction = self.pool.begin().await.map_err(|error| error.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *transaction, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let result = sqlx::query(
            "UPDATE agent_missions
             SET sync_error = $1, last_synced_at = NOW()
             WHERE id = $2 AND tenant_id = $3",
        )
        .bind(sync_error)
        .bind(local_id)
        .bind(organization_id)
        .execute(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;

        if result.rows_affected() != 1 {
            return Err("mission was not found in the requested tenant".to_string());
        }

        Ok(())
    }

    async fn get_active_escalations(
        &self,
        organization_id: &str,
    ) -> Result<Vec<LocalMission>, String> {
        let mut transaction = self.pool.begin().await.map_err(|error| error.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *transaction, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let rows = sqlx::query(
            "SELECT id, tenant_id AS organization_id, status, payload, created_at, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at
             FROM agent_missions
             WHERE tenant_id = $1 AND synced_to_cloud = true AND cloud_mission_id IS NOT NULL AND status NOT IN ('COMPLETED', 'FAILED')
             ORDER BY created_at, id"
        )
        .bind(organization_id)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;

        let mut missions = Vec::new();
        for row in rows {
            let payload_text: String = row.try_get("payload").map_err(|e| e.to_string())?;
            let payload = parse_mission_payload(&payload_text)?;

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

    async fn update_local_status(
        &self,
        organization_id: &str,
        local_id: &str,
        new_status: &str,
    ) -> Result<(), String> {
        let mut transaction = self.pool.begin().await.map_err(|error| error.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *transaction, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let result = sqlx::query(UPDATE_LOCAL_STATUS_SQL)
            .bind(new_status)
            .bind(local_id)
            .bind(organization_id)
            .execute(&mut *transaction)
            .await
            .map_err(|e| e.to_string())?;
        transaction
            .commit()
            .await
            .map_err(|error| error.to_string())?;

        if result.rows_affected() != 1 {
            return Err("mission was not found in the requested tenant".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_text_payload_is_parsed_without_a_json_type_assumption() {
        let payload = parse_mission_payload(
            r#"{"role":"ops","task":"restock","context":null,"action_risk":"low"}"#,
        )
        .expect("valid mission JSON stored in TEXT should parse");

        assert_eq!(payload.role, "ops");
        assert_eq!(payload.task, "restock");
        assert_eq!(payload.action_risk.as_deref(), Some("low"));
    }

    #[test]
    fn malformed_database_payload_is_not_silently_replaced_with_an_empty_mission() {
        assert!(parse_mission_payload("not-json").is_err());
    }

    #[test]
    fn status_updates_use_the_canonical_tenant_column() {
        assert!(UPDATE_LOCAL_STATUS_SQL.contains("tenant_id = $3"));
        assert!(!UPDATE_LOCAL_STATUS_SQL.contains("organization_id"));
    }
}
