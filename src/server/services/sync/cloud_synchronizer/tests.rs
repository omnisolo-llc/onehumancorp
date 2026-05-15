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