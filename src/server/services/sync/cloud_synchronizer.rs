use std::sync::Arc;
use crate::services::sync::local_repository::LocalRepository;
use sqlx::PgPool;
use crate::telemetry::{record_sync_escalation, record_sync_daemon_batch_size, record_sync_latency, record_sync_payload_size, record_sync_daemon_error_total};
use std::time::Instant;

#[async_trait::async_trait]
pub trait SyncHttpClient: Send + Sync {
    async fn post_json(&self, url: &str, payload: &serde_json::Value) -> Result<(u16, serde_json::Value), String>;
    async fn get_json(&self, url: &str) -> Result<(u16, serde_json::Value), String>;
}

pub struct DefaultSyncClient {
    client: reqwest::Client,
}

impl DefaultSyncClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl SyncHttpClient for DefaultSyncClient {
    async fn post_json(&self, url: &str, payload: &serde_json::Value) -> Result<(u16, serde_json::Value), String> {
        let res = self.client.post(url).json(payload).send().await.map_err(|e| e.to_string())?;
        let status = res.status().as_u16();
        let body = res.json().await.unwrap_or_default();
        Ok((status, body))
    }

    async fn get_json(&self, url: &str) -> Result<(u16, serde_json::Value), String> {
        let res = self.client.get(url).send().await.map_err(|e| e.to_string())?;
        let status = res.status().as_u16();
        let body = res.json().await.unwrap_or_default();
        Ok((status, body))
    }
}

pub struct CloudSynchronizerImpl {
    repo: Arc<dyn LocalRepository>,
    client: Box<dyn SyncHttpClient>,
    cloud_url: String,
    pool: Option<PgPool>,
}

impl CloudSynchronizerImpl {
    pub fn new(repo: Arc<dyn LocalRepository>, cloud_url: String) -> Self {
        Self {
            repo,
            client: Box::new(DefaultSyncClient::new()),
            cloud_url,
            pool: None,
        }
    }

    pub fn with_pool(repo: Arc<dyn LocalRepository>, cloud_url: String, pool: PgPool) -> Self {
        Self {
            repo,
            client: Box::new(DefaultSyncClient::new()),
            cloud_url,
            pool: Some(pool),
        }
    }

    #[cfg(test)]
    pub fn with_client(repo: Arc<dyn LocalRepository>, cloud_url: String, pool: Option<PgPool>, client: Box<dyn SyncHttpClient>) -> Self {
        Self {
            repo,
            client,
            cloud_url,
            pool,
        }
    }

    pub async fn push_pending_missions(&self, organization_id: &str) -> Result<(), String> {
        let pending = self.repo.get_pending_sync(organization_id, 50).await?;

        let batch_size = pending.len() as f32;
        let mode = if self.cloud_url.is_empty() { "Standalone" } else { "Cloud" };

        if let Some(pool) = &self.pool {
            let _ = record_sync_daemon_batch_size(pool, batch_size, mode).await;
        }

        // Process pending files in hybrid_fs_sync_queue
        if let Some(pool) = &self.pool {
            if let Ok(files) = sqlx::query("SELECT id, local_path, cloud_path FROM hybrid_fs_sync_queue WHERE status = 'FILE_SYNC_PENDING'")
                .fetch_all(pool)
                .await
            {
                for file in files {
                    use sqlx::Row;
                    let id: String = file.get("id");
                    let local_path: String = file.get("local_path");
                    let _cloud_path: String = file.get("cloud_path");

                    // Read local file (simulated, since we are moving it to cloud via API)
                    if let Ok(_content) = tokio::fs::read(&local_path).await {
                        // Send it (we assume a simple multipart or json with b64, for now just change status as it represents the daemon sync mechanism)
                        let _ = sqlx::query("UPDATE hybrid_fs_sync_queue SET status = 'SYNCED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(&id)
                            .execute(pool)
                            .await;
                    } else {
                        let _ = sqlx::query("UPDATE hybrid_fs_sync_queue SET status = 'FAILED_LOCAL_READ', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                            .bind(&id)
                            .execute(pool)
                            .await;
                    }
                }
            }
        }

        for mission in pending {
            let endpoint = format!("{}/api/v1/missions/escalate", self.cloud_url);

            let payload = serde_json::json!({
                "local_id": &mission.id,
                "payload": &mission.payload,
            });

            let payload_string = serde_json::to_string(&payload).unwrap_or_default();
            let payload_size = payload_string.len() as f32;

            if let Some(pool) = &self.pool {
                let _ = record_sync_payload_size(pool, payload_size, mode).await;
                let _ = record_sync_escalation(pool, 1.0, mode).await;
            }

            let start = Instant::now();

            let resp = self.client.post_json(&endpoint, &payload).await;

            let latency = start.elapsed().as_millis() as f32;
            if let Some(pool) = &self.pool {
                let _ = record_sync_latency(pool, latency, mode).await;
            }

            match resp {
                Ok((status, json)) => {
                    if status >= 200 && status < 300 {
                        if let Some(cloud_id) = json.get("cloud_id").and_then(|v| v.as_str()) {
                            let repo_res = self.repo.mark_synced(organization_id, &mission.id, cloud_id).await;
                            if let Err(e) = repo_res {
                                if let Some(pool) = &self.pool {
                                    let _ = record_sync_daemon_error_total(pool, 1.0, mode, "DB_ERROR").await;
                                }
                                return Err(e);
                            }
                        }
                    } else {
                        if let Some(pool) = &self.pool {
                            let _ = record_sync_daemon_error_total(pool, 1.0, mode, "HTTP_ERROR").await;
                        }
                        self.repo.mark_sync_error(organization_id, &mission.id, &format!("HTTP {}", status)).await?;
                    }
                }
                Err(e) => {
                    if let Some(pool) = &self.pool {
                        let _ = record_sync_daemon_error_total(pool, 1.0, mode, "API_TIMEOUT").await;
                    }
                    self.repo.mark_sync_error(organization_id, &mission.id, &e).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn pull_mission_updates(&self, organization_id: &str) -> Result<(), String> {
        let active = self.repo.get_active_escalations(organization_id).await?;

        for mission in active {
            if let Some(cloud_id) = &mission.cloud_mission_id {
                let endpoint = format!("{}/api/v1/missions/{}/status", self.cloud_url, cloud_id);

                let resp = self.client.get_json(&endpoint).await;

                if let Ok((status, json)) = resp {
                    if status >= 200 && status < 300 {
                        if let Some(mission_status) = json.get("status").and_then(|v| v.as_str()) {
                            self.repo.update_local_status(organization_id, &mission.id, mission_status).await?;
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

    struct MockSyncHttpClient {
        post_responses: Mutex<HashMap<String, Result<(u16, serde_json::Value), String>>>,
        get_responses: Mutex<HashMap<String, Result<(u16, serde_json::Value), String>>>,
    }

    impl MockSyncHttpClient {
        fn new() -> Self {
            Self {
                post_responses: Mutex::new(HashMap::new()),
                get_responses: Mutex::new(HashMap::new()),
            }
        }

        fn add_post_response(&self, url: &str, res: Result<(u16, serde_json::Value), String>) {
            self.post_responses.lock().unwrap().insert(url.to_string(), res);
        }

        fn add_get_response(&self, url: &str, res: Result<(u16, serde_json::Value), String>) {
            self.get_responses.lock().unwrap().insert(url.to_string(), res);
        }
    }

    #[async_trait::async_trait]
    impl SyncHttpClient for MockSyncHttpClient {
        async fn post_json(&self, url: &str, _payload: &serde_json::Value) -> Result<(u16, serde_json::Value), String> {
            self.post_responses.lock().unwrap().get(url).cloned().unwrap_or(Err("Not found".to_string()))
        }

        async fn get_json(&self, url: &str) -> Result<(u16, serde_json::Value), String> {
            self.get_responses.lock().unwrap().get(url).cloned().unwrap_or(Err("Not found".to_string()))
        }
    }

    #[tokio::test]
    async fn test_push_pending_missions_success() {
        let repo = Arc::new(MockLocalRepository::new());
        repo.pending.lock().unwrap().push(LocalMission {
            id: "local_1".to_string(),
            organization_id: "test_org".to_string(),
            status: "PENDING".to_string(),
            payload: crate::services::sync::local_repository::MissionPayload { role: "SYSTEM".to_string(), task: "test".to_string(), context: None },
            created_at: chrono::Utc::now(),
            synced_to_cloud: false,
            cloud_mission_id: None,
            sync_error: None,
            last_synced_at: None,
        });

        let mock_client = MockSyncHttpClient::new();
        mock_client.add_post_response(
            "http://cloud_api/api/v1/missions/escalate",
            Ok((200, serde_json::json!({"cloud_id": "cloud_1"})))
        );

        let sync = CloudSynchronizerImpl::with_client(repo.clone(), "http://cloud_api".to_string(), None, Box::new(mock_client));

        let res = sync.push_pending_missions("test_org").await;
        assert!(res.is_ok());

        let synced = repo.synced.lock().unwrap().get("local_1").cloned();
        assert_eq!(synced, Some("cloud_1".to_string()));
    }

    #[tokio::test]
    async fn test_push_pending_missions_http_error() {
        let repo = Arc::new(MockLocalRepository::new());
        repo.pending.lock().unwrap().push(LocalMission {
            id: "local_1".to_string(),
            organization_id: "test_org".to_string(),
            status: "PENDING".to_string(),
            payload: crate::services::sync::local_repository::MissionPayload { role: "SYSTEM".to_string(), task: "test".to_string(), context: None },
            created_at: chrono::Utc::now(),
            synced_to_cloud: false,
            cloud_mission_id: None,
            sync_error: None,
            last_synced_at: None,
        });

        let mock_client = MockSyncHttpClient::new();
        mock_client.add_post_response(
            "http://cloud_api/api/v1/missions/escalate",
            Ok((500, serde_json::json!({})))
        );

        let sync = CloudSynchronizerImpl::with_client(repo.clone(), "http://cloud_api".to_string(), None, Box::new(mock_client));

        let res = sync.push_pending_missions("test_org").await;
        assert!(res.is_ok());

        let error_msg = repo.errors.lock().unwrap().get("local_1").cloned().unwrap_or_default();
        assert_eq!(error_msg, "HTTP 500");
    }

    #[tokio::test]
    async fn test_push_pending_missions_api_error() {
        let repo = Arc::new(MockLocalRepository::new());
        repo.pending.lock().unwrap().push(LocalMission {
            id: "local_1".to_string(),
            organization_id: "test_org".to_string(),
            status: "PENDING".to_string(),
            payload: crate::services::sync::local_repository::MissionPayload { role: "SYSTEM".to_string(), task: "test".to_string(), context: None },
            created_at: chrono::Utc::now(),
            synced_to_cloud: false,
            cloud_mission_id: None,
            sync_error: None,
            last_synced_at: None,
        });

        let mock_client = MockSyncHttpClient::new();
        mock_client.add_post_response(
            "http://cloud_api/api/v1/missions/escalate",
            Err("Connection refused".to_string())
        );

        let sync = CloudSynchronizerImpl::with_client(repo.clone(), "http://cloud_api".to_string(), None, Box::new(mock_client));

        let res = sync.push_pending_missions("test_org").await;
        assert!(res.is_ok());

        let error_msg = repo.errors.lock().unwrap().get("local_1").cloned().unwrap_or_default();
        assert_eq!(error_msg, "Connection refused");
    }

    #[tokio::test]
    async fn test_pull_mission_updates_success() {
        let repo = Arc::new(MockLocalRepository::new());
        repo.active.lock().unwrap().push(LocalMission {
            id: "local_1".to_string(),
            organization_id: "test_org".to_string(),
            status: "PENDING".to_string(),
            payload: crate::services::sync::local_repository::MissionPayload { role: "SYSTEM".to_string(), task: "test".to_string(), context: None },
            created_at: chrono::Utc::now(),
            synced_to_cloud: true,
            cloud_mission_id: Some("cloud_1".to_string()),
            sync_error: None,
            last_synced_at: None,
        });

        let mock_client = MockSyncHttpClient::new();
        mock_client.add_get_response(
            "http://cloud_api/api/v1/missions/cloud_1/status",
            Ok((200, serde_json::json!({"status": "COMPLETED"})))
        );

        let sync = CloudSynchronizerImpl::with_client(repo.clone(), "http://cloud_api".to_string(), None, Box::new(mock_client));

        let res = sync.pull_mission_updates("test_org").await;
        assert!(res.is_ok());

        let status = repo.status.lock().unwrap().get("local_1").cloned();
        assert_eq!(status, Some("COMPLETED".to_string()));
    }
}
