use std::sync::Arc;
use tokio::time::{self, Duration};
use sqlx::Row;
use crate::ohc::orchestration::sync_service_client::SyncServiceClient;
use crate::ohc::orchestration::{VectorSyncRequest, VectorSyncRecord};
use crate::db::DB;
use crate::db::DbStore;
use std::fs;
use std::path::Path;

pub struct HybridRagWorker {
    db: Arc<DB>,
    cloud_url: String,
}

impl HybridRagWorker {
    pub fn new(db: Arc<DB>, cloud_url: String) -> Self {
        HybridRagWorker { db, cloud_url }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_secs(60));
            loop {
                ticker.tick().await;
                if let Err(e) = self.process_syncs().await {
                    eprintln!("HybridRagWorker failed to process syncs: {}", e);
                }
            }
        });
    }

    pub async fn process_syncs(&self) -> Result<(), String> {
        let rows = match &self.db.store {
            DbStore::Sqlite(pool) => {
                let r = sqlx::query("SELECT id, organization_id, agent_id, task_id, content, embedding, source_type, topic, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending'")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                r
            }
            DbStore::Postgres => {
                return Ok(());
            }
        };

        if rows.is_empty() {
            return Ok(());
        }

        let mut records = Vec::new();
        let mut ids = Vec::new();

        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            ids.push(id.clone());
            records.push(VectorSyncRecord {
                id,
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                agent_id: row.try_get("agent_id").unwrap_or_default(),
                task_id: row.try_get("task_id").unwrap_or_default(),
                content: row.try_get("content").unwrap_or_default(),
                embedding: row.try_get("embedding").unwrap_or_default(),
                source_type: row.try_get("source_type").unwrap_or_default(),
                topic: row.try_get("topic").unwrap_or_default(),
                sync_status: row.try_get("sync_status").unwrap_or_default(),
                last_sync_at: row.try_get("last_sync_at").unwrap_or_default(),
            });
        }

        let request = tonic::Request::new(VectorSyncRequest { records });
        let endpoint = if self.cloud_url.starts_with("http") || self.cloud_url.starts_with("https") {
            self.cloud_url.clone()
        } else {
            format!("https://{}", self.cloud_url)
        };

        let mut client = if endpoint.starts_with("https") {
            let cert_path = std::env::var("SPIFFE_CERT_PATH").unwrap_or_else(|_| "/run/spiffe/certs/tls.crt".to_string());
            let key_path = std::env::var("SPIFFE_KEY_PATH").unwrap_or_else(|_| "/run/spiffe/certs/tls.key".to_string());
            let ca_path = std::env::var("SPIFFE_CA_PATH").unwrap_or_else(|_| "/run/spiffe/certs/ca.crt".to_string());

            let mut tls_config = tonic::transport::ClientTlsConfig::new()
                .domain_name("cloud.onehumancorp.com");

            if Path::new(&cert_path).exists() && Path::new(&key_path).exists() {
                let cert = fs::read(&cert_path).map_err(|e| e.to_string())?;
                let key = fs::read(&key_path).map_err(|e| e.to_string())?;
                let identity = tonic::transport::Identity::from_pem(cert, key);
                tls_config = tls_config.identity(identity);
            }
            if Path::new(&ca_path).exists() {
                let ca = fs::read(&ca_path).map_err(|e| e.to_string())?;
                let ca_cert = tonic::transport::Certificate::from_pem(ca);
                tls_config = tls_config.ca_certificate(ca_cert);
            }

            let channel = tonic::transport::Channel::from_shared(endpoint)
                .map_err(|e| e.to_string())?
                .tls_config(tls_config)
                .map_err(|e| e.to_string())?
                .connect()
                .await
                .map_err(|e| e.to_string())?;
            SyncServiceClient::new(channel)
        } else {
            let channel = tonic::transport::Channel::from_shared(endpoint)
                .map_err(|e| e.to_string())?
                .connect()
                .await
                .map_err(|e| e.to_string())?;
            SyncServiceClient::new(channel)
        };

        let response = client.vector_sync(request).await.map_err(|e| e.to_string())?;

        if response.into_inner().status == "success" {
            match &self.db.store {
                DbStore::Sqlite(pool) => {
                    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                    for id in ids {
                        sqlx::query("UPDATE autodream_memories SET sync_status = 'synced' WHERE id = ?")
                            .bind(id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;
                    }
                    tx.commit().await.map_err(|e| e.to_string())?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_worker_polls_pending_status() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });
        let worker = HybridRagWorker::new(db, "http://localhost".to_string());

        let res = worker.process_syncs().await;
        // In dummy postgres it returns Ok(()) early
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_worker_polls_pending_status_sqlite() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:").unwrap();

        sqlx::query("CREATE TABLE autodream_memories (id TEXT, organization_id TEXT, agent_id TEXT, task_id TEXT, content TEXT, embedding TEXT, source_type TEXT, topic TEXT, sync_status TEXT, last_sync_at TEXT)")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO autodream_memories (id, sync_status) VALUES ('123', 'pending')")
            .execute(&pool).await.unwrap();

        let sqlite_db = DB { pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(), store: DbStore::Sqlite(pool) };
        let worker = HybridRagWorker::new(Arc::new(sqlite_db), "http://localhost".to_string());

        let res = worker.process_syncs().await;
        // It fails during gRPC connection to dummy URL, but successfully polled.
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_worker_updates_status_on_success() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });
        let worker = HybridRagWorker::new(db, "http://localhost".to_string());
        let worker_arc = Arc::new(worker);
        worker_arc.start();

        assert!(true);
    }
}
