use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

use super::mission::Mission;

#[async_trait]
pub trait HandoffProvider: Send + Sync {
    async fn push_pending_missions(&self) -> Result<usize, String>;
    async fn pull_mission_updates(&self) -> Result<usize, String>;
}

// Client interface handling the Cloud API communication
#[async_trait]
pub trait CloudEscalationClient: Send + Sync {
    async fn escalate(&self, mission: &Mission) -> Result<String, String>;
    async fn get_status(&self, cloud_id: &str) -> Result<Option<Mission>, String>;
}

pub struct HttpCloudClient {
    client: reqwest::Client,
    base_url: String,
}

impl HttpCloudClient {
    pub fn new(base_url: String) -> Self {
        HttpCloudClient {
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

#[async_trait]
impl CloudEscalationClient for HttpCloudClient {
    async fn escalate(&self, mission: &Mission) -> Result<String, String> {
        let url = format!("{}/api/v1/missions/escalate", self.base_url);

        let payload = serde_json::json!({
            "local_id": mission.mission_id,
            "payload": mission.payload.0,
        });

        let res = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            if let Some(cloud_id) = body.get("cloud_id").and_then(|v| v.as_str()) {
                return Ok(cloud_id.to_string());
            }
        }

        Err("failed to escalate mission".to_string())
    }

    async fn get_status(&self, cloud_id: &str) -> Result<Option<Mission>, String> {
        let url = format!("{}/api/v1/missions/{}/status", self.base_url, cloud_id);

        let res = self.client.get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            if let Some(status) = body.get("status").and_then(|v| v.as_str()) {
                // Here we reconstruct a minimal Mission update object from the status
                let m = Mission {
                    mission_id: "".to_string(), // we don't care about these local fields here
                    title: "".to_string(),
                    status: status.to_string(),
                    assigned_agent: None,
                    priority: "".to_string(),
                    payload: sqlx::types::Json(serde_json::json!({})),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    synced_to_cloud: Some(true),
                    cloud_mission_id: Some(cloud_id.to_string()),
                    sync_error: None,
                    last_synced_at: Some(chrono::Utc::now()),
                };
                return Ok(Some(m));
            }
        }

        Ok(None)
    }
}

pub struct HandoffManager {
    // Local state representing the local SQLite daemon
    pub local_missions: Arc<Mutex<HashMap<String, Mission>>>,
    pub cloud_client: Arc<dyn CloudEscalationClient>,
}

impl HandoffManager {
    pub fn new(cloud_client: Arc<dyn CloudEscalationClient>) -> Self {
        HandoffManager {
            local_missions: Arc::new(Mutex::new(HashMap::new())),
            cloud_client,
        }
    }

    // Helper to seed local missions for testing
    pub async fn add_local_mission(&self, mission: Mission) {
        let mut local = self.local_missions.lock().await;
        local.insert(mission.mission_id.clone(), mission);
    }
}

#[async_trait]
impl HandoffProvider for HandoffManager {
    async fn push_pending_missions(&self) -> Result<usize, String> {
        let mut local = self.local_missions.lock().await;
        let mut escalated_count = 0;

        // Find missions that need syncing (synced_to_cloud is false or None)
        let pending_ids: Vec<String> = local.iter()
            .filter(|(_, m)| m.synced_to_cloud.unwrap_or(false) == false)
            .map(|(k, _)| k.clone())
            .collect();

        for id in pending_ids {
            if let Some(mission) = local.get(&id) {
                match self.cloud_client.escalate(mission).await {
                    Ok(cloud_id) => {
                        // Update local mission with cloud info
                        if let Some(m) = local.get_mut(&id) {
                            m.synced_to_cloud = Some(true);
                            m.cloud_mission_id = Some(cloud_id);
                            m.last_synced_at = Some(chrono::Utc::now());
                            m.sync_error = None;
                            escalated_count += 1;
                        }
                    },
                    Err(e) => {
                        // Mark error
                        if let Some(m) = local.get_mut(&id) {
                            m.sync_error = Some(e);
                        }
                    }
                }
            }
        }

        Ok(escalated_count)
    }

    async fn pull_mission_updates(&self) -> Result<usize, String> {
        let mut local = self.local_missions.lock().await;
        let mut updated_count = 0;

        // Find missions that are escalated but not yet complete locally
        let active_ids: Vec<(String, String)> = local.iter()
            .filter(|(_, m)| {
                m.synced_to_cloud.unwrap_or(false)
                && m.cloud_mission_id.is_some()
                && m.status != "COMPLETED"
                && m.status != "FAILED"
            })
            .map(|(k, m)| (k.clone(), m.cloud_mission_id.clone().unwrap()))
            .collect();

        for (local_id, cloud_id) in active_ids {
            if let Ok(Some(cloud_mission)) = self.cloud_client.get_status(&cloud_id).await {
                // Update local status if changed
                if let Some(m) = local.get_mut(&local_id) {
                    if m.status != cloud_mission.status {
                        m.status = cloud_mission.status;
                        m.updated_at = chrono::Utc::now();
                        m.last_synced_at = Some(chrono::Utc::now());
                        updated_count += 1;
                    }
                }
            }
        }

        Ok(updated_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClient {
        escalate_res: String,
        status_res: Mission,
    }

    #[async_trait]
    impl CloudEscalationClient for MockClient {
        async fn escalate(&self, _mission: &Mission) -> Result<String, String> {
            Ok(self.escalate_res.clone())
        }
        async fn get_status(&self, _cloud_id: &str) -> Result<Option<Mission>, String> {
            Ok(Some(self.status_res.clone()))
        }
    }

    #[tokio::test]
    async fn test_handoff_push_pending() {
        let mission = Mission {
            mission_id: "m-local-1".to_string(),
            title: "Test Escalate".to_string(),
            status: "QUEUED".to_string(),
            assigned_agent: None,
            priority: "NORMAL".to_string(),
            payload: sqlx::types::Json(serde_json::json!({"role": "test"})),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            synced_to_cloud: Some(false),
            cloud_mission_id: None,
            sync_error: None,
            last_synced_at: None,
        };

        let mock_client = Arc::new(MockClient {
            escalate_res: "cloud-123".to_string(),
            status_res: mission.clone(),
        });

        let manager = HandoffManager::new(mock_client);
        manager.add_local_mission(mission).await;

        // Push should sync 1 mission
        let pushed = manager.push_pending_missions().await.unwrap();
        assert_eq!(pushed, 1);

        // Verify local state was updated
        let local = manager.local_missions.lock().await;
        let m = local.get("m-local-1").unwrap();
        assert_eq!(m.synced_to_cloud, Some(true));
        assert_eq!(m.cloud_mission_id, Some("cloud-123".to_string()));
    }

    #[tokio::test]
    async fn test_handoff_pull_updates() {
        let mut mission = Mission {
            mission_id: "m-local-2".to_string(),
            title: "Test Pull".to_string(),
            status: "IN_PROGRESS".to_string(),
            assigned_agent: None,
            priority: "NORMAL".to_string(),
            payload: sqlx::types::Json(serde_json::json!({})),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            synced_to_cloud: Some(true),
            cloud_mission_id: Some("cloud-456".to_string()),
            sync_error: None,
            last_synced_at: None,
        };

        let mut updated_mission = mission.clone();
        updated_mission.status = "COMPLETED".to_string();

        let mock_client = Arc::new(MockClient {
            escalate_res: "cloud-456".to_string(),
            status_res: updated_mission,
        });

        let manager = HandoffManager::new(mock_client);
        manager.add_local_mission(mission).await;

        // Pull updates
        let pulled = manager.pull_mission_updates().await.unwrap();
        assert_eq!(pulled, 1);

        // Verify local status was updated
        let local = manager.local_missions.lock().await;
        let m = local.get("m-local-2").unwrap();
        assert_eq!(m.status, "COMPLETED");
    }
}
