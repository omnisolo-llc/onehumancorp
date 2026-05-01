use std::sync::Arc;
use crate::services::sync::local_repository::LocalRepository;
use reqwest::Client;

pub struct CloudSynchronizerImpl {
    repo: Arc<dyn LocalRepository>,
    client: Client,
    cloud_url: String,
}

impl CloudSynchronizerImpl {
    pub fn new(repo: Arc<dyn LocalRepository>, cloud_url: String) -> Self {
        Self {
            repo,
            client: Client::new(),
            cloud_url,
        }
    }

    pub async fn push_pending_missions(&self, organization_id: &str) -> Result<(), String> {
        let pending = self.repo.get_pending_sync(organization_id, 50).await?;

        for mission in pending {
            let endpoint = format!("{}/api/v1/missions/escalate", self.cloud_url);

            let payload = serde_json::json!({
                "local_id": &mission.id,
                "payload": &mission.payload,
            });

            let resp = self.client.post(&endpoint)
                .json(&payload)
                .send()
                .await;

            match resp {
                Ok(response) => {
                    if response.status().is_success() {
                        let json: serde_json::Value = response.json().await.unwrap_or_default();
                        if let Some(cloud_id) = json.get("cloud_id").and_then(|v| v.as_str()) {
                            self.repo.mark_synced(organization_id, &mission.id, cloud_id).await?;
                        }
                    } else {
                        self.repo.mark_sync_error(organization_id, &mission.id, &format!("HTTP {}", response.status())).await?;
                    }
                }
                Err(e) => {
                    self.repo.mark_sync_error(organization_id, &mission.id, &e.to_string()).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn pull_mission_updates(&self, organization_id: &str) -> Result<(), String> {
        let active = self.repo.get_active_escalations(organization_id).await?;

        for mission in active {
            if let Some(cloud_id) = mission.cloud_mission_id {
                let endpoint = format!("{}/api/v1/missions/{}/status", self.cloud_url, cloud_id);

                let resp = self.client.get(&endpoint)
                    .send()
                    .await;

                if let Ok(response) = resp {
                    if response.status().is_success() {
                        let json: serde_json::Value = response.json().await.unwrap_or_default();
                        if let Some(status) = json.get("status").and_then(|v| v.as_str()) {
                            self.repo.update_local_status(organization_id, &mission.id, status).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait CloudSynchronizer: Send + Sync {
    async fn push_pending_missions(&self, organization_id: &str) -> Result<(), String>;
    async fn pull_mission_updates(&self, organization_id: &str) -> Result<(), String>;
}

#[async_trait::async_trait]
impl CloudSynchronizer for CloudSynchronizerImpl {
    async fn push_pending_missions(&self, organization_id: &str) -> Result<(), String> {
        self.push_pending_missions(organization_id).await
    }

    async fn pull_mission_updates(&self, organization_id: &str) -> Result<(), String> {
        self.pull_mission_updates(organization_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    use std::sync::Mutex;
    use std::collections::HashMap;

    use crate::services::sync::local_repository::LocalMission;

    struct MockLocalRepository {
        pending: Mutex<Vec<LocalMission>>,
        active: Mutex<Vec<LocalMission>>,
        synced: Mutex<HashMap<String, String>>,
        errors: Mutex<HashMap<String, String>>,
        status: Mutex<HashMap<String, String>>,
    }

    impl MockLocalRepository {
        fn new() -> Self {
            Self {
                pending: Mutex::new(Vec::new()),
                active: Mutex::new(Vec::new()),
                synced: Mutex::new(HashMap::new()),
                errors: Mutex::new(HashMap::new()),
                status: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait::async_trait]
    impl LocalRepository for MockLocalRepository {
        async fn get_pending_sync(&self, _organization_id: &str, _limit: i32) -> Result<Vec<LocalMission>, String> {
            Ok(self.pending.lock().unwrap().clone())
        }

        async fn mark_synced(&self, _organization_id: &str, local_id: &str, cloud_id: &str) -> Result<(), String> {
            self.synced.lock().unwrap().insert(local_id.to_string(), cloud_id.to_string());
            Ok(())
        }

        async fn mark_sync_error(&self, _organization_id: &str, local_id: &str, sync_error: &str) -> Result<(), String> {
            self.errors.lock().unwrap().insert(local_id.to_string(), sync_error.to_string());
            Ok(())
        }

        async fn get_active_escalations(&self, _organization_id: &str) -> Result<Vec<LocalMission>, String> {
            Ok(self.active.lock().unwrap().clone())
        }

        async fn update_local_status(&self, _organization_id: &str, local_id: &str, new_status: &str) -> Result<(), String> {
            self.status.lock().unwrap().insert(local_id.to_string(), new_status.to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_push_pending_missions_empty() {
        let repo = Arc::new(MockLocalRepository::new());
        let sync = CloudSynchronizerImpl::new(repo, "http://localhost:8080".to_string());
        let res = sync.push_pending_missions("test_org").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_pull_mission_updates_empty() {
        let repo = Arc::new(MockLocalRepository::new());
        let sync = CloudSynchronizerImpl::new(repo, "http://localhost:8080".to_string());
        let res = sync.pull_mission_updates("test_org").await;
        assert!(res.is_ok());
    }
}
