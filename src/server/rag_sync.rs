use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    Synced,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGSyncRecord {
    pub id: String,
    pub tenant_id: String,
    pub context: String,
    pub vector: Vec<f32>,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
}

#[async_trait::async_trait]
pub trait RAGSyncService: Send + Sync {
    /// FetchPendingSyncs retrieves records from the local DB that need syncing
    async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String>;

    /// MarkSynced updates the local DB after a successful sync to the cloud
    async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String>;

    /// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
    async fn process_incoming_sync(&self, tenant_id: &str, records: Vec<RAGSyncRecord>) -> Result<(), String>;
}



pub struct DistributedRAGSyncService<T: RAGSyncService> {
    inner: T,
    redis_url: String,
}

impl<T: RAGSyncService> DistributedRAGSyncService<T> {
    pub fn new(inner: T, redis_url: String) -> Self {
        Self { inner, redis_url }
    }
}

#[async_trait::async_trait]
impl<T: RAGSyncService> RAGSyncService for DistributedRAGSyncService<T> {
    async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String> {
        self.inner.fetch_pending_syncs(limit).await
    }

    async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String> {
        self.inner.mark_synced(ids).await
    }

    async fn process_incoming_sync(&self, tenant_id: &str, records: Vec<RAGSyncRecord>) -> Result<(), String> {
        let redis_client = match redis::Client::open(self.redis_url.clone()) {
            Ok(c) => c,
            Err(e) => return Err(format!("failed to connect to redis for lock: {}", e)),
        };
        let redis_lock = crate::orchestration::locks::RedisLock::new(redis_client);

        for record in records {
            if record.tenant_id != tenant_id {
                return Err("cross-tenant sync rejected".to_string());
            }

            let _lock_key = format!("rag_sync:{}", record.id);
            use crate::orchestration::locks::DistributedLock;
            let _lock_guard = match redis_lock.acquire_resource(tenant_id, "rag_sync", &record.id).await {
                Ok(guard) => guard,
                Err(e) => {
                    if e == "failed to acquire redis lock" {
                        return Err("lock busy".to_string());
                    } else {
                        return Err(format!("failed to acquire lock: {}", e));
                    }
                }
            };

            let process_result = self.inner.process_incoming_sync(tenant_id, vec![record.clone()]).await;

            if let Err(e) = process_result {
                return Err(e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use chrono::Utc;

    #[derive(Clone)]
    struct MockRAGSyncService {
        records: Arc<Mutex<Vec<RAGSyncRecord>>>,
    }

    #[async_trait::async_trait]
    impl RAGSyncService for MockRAGSyncService {
        async fn fetch_pending_syncs(&self, limit: i32) -> Result<Vec<RAGSyncRecord>, String> {
            let records = self.records.lock().await;
            let pending: Vec<RAGSyncRecord> = records
                .iter()
                .filter(|r| r.sync_status == SyncStatus::Pending)
                .take(limit as usize)
                .cloned()
                .collect();
            Ok(pending)
        }

        async fn mark_synced(&self, ids: Vec<String>) -> Result<(), String> {
            let mut records = self.records.lock().await;
            for record in records.iter_mut() {
                if ids.contains(&record.id) {
                    record.sync_status = SyncStatus::Synced;
                    record.last_sync_at = Some(Utc::now());
                }
            }
            Ok(())
        }

        async fn process_incoming_sync(&self, _tenant_id: &str, incoming_records: Vec<RAGSyncRecord>) -> Result<(), String> {
            let mut records = self.records.lock().await;
            for mut incoming in incoming_records {
                if let Some(existing) = records.iter_mut().find(|r| r.id == incoming.id) {
                    *existing = incoming;
                } else {
                    incoming.sync_status = SyncStatus::Synced; // Assuming they are synced once they reach cloud
                    records.push(incoming);
                }
            }
            Ok(())
        }
    }


    #[tokio::test]
    async fn test_distributed_sync_valid() {
        let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let client = match redis::Client::open(redis_url.clone()) {
            Ok(c) => c,
            Err(_) => return, // Skip test if Redis URL is invalid
        };
        // Check if we can connect, otherwise skip
        if client.get_multiplexed_async_connection().await.is_err() {
            return;
        }

        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };
        let distributed_service = DistributedRAGSyncService::new(mock_service.clone(), redis_url.clone());

        let incoming = vec![RAGSyncRecord {
            id: "doc-1".to_string(),
            tenant_id: "tenant-A".to_string(),
            context: "Test context".to_string(),
            vector: vec![0.1, 0.2],
            sync_status: SyncStatus::Pending,
            last_sync_at: None,
        }];

        let result: Result<(), String> = distributed_service.process_incoming_sync("tenant-A", incoming).await;
        assert!(result.is_ok());

        let all_records = mock_service.records.lock().await;
        assert_eq!(all_records.len(), 1);
        assert_eq!(all_records[0].sync_status, SyncStatus::Synced);
    }

    #[tokio::test]
    async fn test_distributed_sync_cross_tenant_rejected() {
        let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };
        let distributed_service = DistributedRAGSyncService::new(mock_service, redis_url.clone());

        let incoming = vec![RAGSyncRecord {
            id: "doc-2".to_string(),
            tenant_id: "tenant-B".to_string(), // DIFFERENT TENANT
            context: "Test context".to_string(),
            vector: vec![0.1, 0.2],
            sync_status: SyncStatus::Pending,
            last_sync_at: None,
        }];

        let result: Result<(), String> = distributed_service.process_incoming_sync("tenant-A", incoming).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "cross-tenant sync rejected");
    }

    #[tokio::test]
    async fn test_distributed_sync_concurrent_lock_busy() {
        let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let client = match redis::Client::open(redis_url.clone()) {
            Ok(c) => c,
            Err(_) => return, // Skip
        };
        if client.get_multiplexed_async_connection().await.is_err() {
            return;
        }

        // Simulate another process holding the lock
        let redis_lock = crate::orchestration::locks::RedisLock::new(client);
        use crate::orchestration::locks::DistributedLock;
        let _lock_guard = redis_lock.acquire_resource("tenant-C", "rag_sync", "doc-3").await.unwrap();

        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };
        let distributed_service = DistributedRAGSyncService::new(mock_service, redis_url.clone());

        let incoming = vec![RAGSyncRecord {
            id: "doc-3".to_string(),
            tenant_id: "tenant-C".to_string(),
            context: "Test context".to_string(),
            vector: vec![0.1, 0.2],
            sync_status: SyncStatus::Pending,
            last_sync_at: None,
        }];

        let result: Result<(), String> = distributed_service.process_incoming_sync("tenant-C", incoming).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "lock busy");
    }

    #[tokio::test]
    async fn test_distributed_sync_multiple_documents() {
        let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let client = match redis::Client::open(redis_url.clone()) {
            Ok(c) => c,
            Err(_) => return, // Skip
        };
        if client.get_multiplexed_async_connection().await.is_err() {
            return;
        }

        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };
        let distributed_service = DistributedRAGSyncService::new(mock_service.clone(), redis_url.clone());

        let incoming = vec![
            RAGSyncRecord {
                id: "doc-4".to_string(),
                tenant_id: "tenant-D".to_string(),
                context: "Test context 4".to_string(),
                vector: vec![0.1, 0.2],
                sync_status: SyncStatus::Pending,
                last_sync_at: None,
            },
            RAGSyncRecord {
                id: "doc-5".to_string(),
                tenant_id: "tenant-D".to_string(),
                context: "Test context 5".to_string(),
                vector: vec![0.3, 0.4],
                sync_status: SyncStatus::Pending,
                last_sync_at: None,
            }
        ];

        let result: Result<(), String> = distributed_service.process_incoming_sync("tenant-D", incoming).await;
        assert!(result.is_ok());

        let all_records = mock_service.records.lock().await;
        assert_eq!(all_records.len(), 2);
    }

    #[tokio::test]
    async fn test_distributed_sync_invalid_redis_url() {
        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![])),
        };
        let distributed_service = DistributedRAGSyncService::new(mock_service, "redis://invalid-host:1234".to_string());

        let incoming = vec![RAGSyncRecord {
            id: "doc-6".to_string(),
            tenant_id: "tenant-E".to_string(),
            context: "Test context".to_string(),
            vector: vec![0.1, 0.2],
            sync_status: SyncStatus::Pending,
            last_sync_at: None,
        }];

        // We expect it to either return lock busy or failed to acquire lock or failed to connect to redis
        let result: Result<(), String> = distributed_service.process_incoming_sync("tenant-E", incoming).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("failed to connect") || err_msg.contains("failed to acquire") || err_msg.contains("lock busy") || err_msg.contains("error resolution") || err_msg.contains("Failed to resolve") || err_msg.contains("No address"));
    }

    #[tokio::test]
    async fn test_rag_sync_flow() {
        let mock_service = MockRAGSyncService {
            records: Arc::new(Mutex::new(vec![
                RAGSyncRecord {
                    id: "1".to_string(),
                    tenant_id: "test-tenant".to_string(),
                    context: "test 1".to_string(),
                    vector: vec![0.1, 0.2],
                    sync_status: SyncStatus::Pending,
                    last_sync_at: None,
                },
                RAGSyncRecord {
                    id: "2".to_string(),
                    tenant_id: "test-tenant".to_string(),
                    context: "test 2".to_string(),
                    vector: vec![0.3, 0.4],
                    sync_status: SyncStatus::Pending,
                    last_sync_at: None,
                },
            ])),
        };

        let pending = mock_service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(pending.len(), 2);

        let ids: Vec<String> = pending.iter().map(|r| r.id.clone()).collect();
        mock_service.mark_synced(ids).await.unwrap();

        let still_pending = mock_service.fetch_pending_syncs(10).await.unwrap();
        assert_eq!(still_pending.len(), 0);

        let incoming = vec![RAGSyncRecord {
            id: "3".to_string(),
            tenant_id: "test-tenant".to_string(),
            context: "test 3".to_string(),
            vector: vec![0.5, 0.6],
            sync_status: SyncStatus::Pending,
            last_sync_at: None,
        }];

        mock_service.process_incoming_sync("test-tenant", incoming).await.unwrap();

        let all_records = mock_service.records.lock().await;
        assert_eq!(all_records.len(), 3);
        assert_eq!(all_records[2].id, "3");
        assert_eq!(all_records[2].sync_status, SyncStatus::Synced);
    }
}
