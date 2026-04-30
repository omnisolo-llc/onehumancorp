use std::sync::Arc;
use tokio::time::{self, Duration};
use sqlx::Row;
use crate::utils::dialect::{PoolType, DatabaseKind, dialect_query};

pub struct SyncEscalator {
    pool: PoolType,
    client: reqwest::Client,
}

impl SyncEscalator {
    pub fn new(pool: PoolType) -> Self {
        SyncEscalator {
            pool,
            client: reqwest::Client::new(),
        }
    }

    pub fn start(self: Arc<Self>, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>, interval: Duration) {
        tokio::spawn(async move {
            let mut ticker = time::interval(interval);
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        println!("SyncEscalator shutting down");
                        break;
                    }
                    _ = ticker.tick() => {
                        if let Err(e) = self.process_escalations().await {
                            eprintln!("failed to process escalations: {}", e);
                        }
                    }
                }
            }
        });
    }

    async fn process_escalations(&self) -> Result<(), String> {
        let rows: Vec<(String, String, String)> = match &self.pool {
            PoolType::Pg(p) => {
                let q = dialect_query("SELECT id, tenant_id, payload FROM local_mcp_rag_tasks WHERE escalation_status = 'local'", DatabaseKind::Postgres);
                let r = sqlx::query(&q)
                    .fetch_all(p)
                    .await
                    .map_err(|e| e.to_string())?;
                r.into_iter().map(|row| (row.get("id"), row.get("tenant_id"), row.get("payload"))).collect()
            },
            PoolType::Sqlite(p) => {
                let q = dialect_query("SELECT id, tenant_id, payload FROM local_mcp_rag_tasks WHERE escalation_status = 'local'", DatabaseKind::Sqlite);
                let r = sqlx::query(&q)
                    .fetch_all(p)
                    .await
                    .map_err(|e| e.to_string())?;
                r.into_iter().map(|row| (row.get("id"), row.get("tenant_id"), row.get("payload"))).collect()
            }
        };

        for (id, tenant_id, payload) in rows {
            let payload_str = format!(r#"{{"id": "{}", "tenant_id": "{}", "data": "{}"}}"#, id, tenant_id, payload);
            
            let mut req = self.client.post("https://cloud.onehumancorp.com/api/v1/orchestration/escalate")
                .header("Content-Type", "application/json")
                .body(payload_str);

            if let Ok(spiffe_token) = std::env::var("SPIFFE_IDENTITY_TOKEN") {
                req = req.header("Authorization", format!("Bearer {}", spiffe_token));
            }

            match req.send().await {
                Ok(resp) => {
                    if resp.status() == reqwest::StatusCode::OK {
                        match &self.pool {
                            PoolType::Pg(p) => {
                                let q2 = dialect_query("UPDATE local_mcp_rag_tasks SET escalation_status = 'cloud' WHERE id = $1 AND tenant_id = $2", DatabaseKind::Postgres);
                                sqlx::query(&q2)
                                    .bind(&id)
                                    .bind(&tenant_id)
                                    .execute(p)
                                    .await
                                    .map_err(|e| e.to_string())?;
                            },
                            PoolType::Sqlite(p) => {
                                let q2 = dialect_query("UPDATE local_mcp_rag_tasks SET escalation_status = 'cloud' WHERE id = $1 AND tenant_id = $2", DatabaseKind::Sqlite);
                                sqlx::query(&q2)
                                    .bind(&id)
                                    .bind(&tenant_id)
                                    .execute(p)
                                    .await
                                    .map_err(|e| e.to_string())?;
                            }
                        }
                    } else {
                        eprintln!("escalation failed with status: {}", resp.status());
                    }
                }
                Err(e) => {
                    eprintln!("failed to send escalation request: {}", e);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_sync_escalator() {
        let pool = PoolType::Pg(sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/mydb").unwrap());
        let escalator = Arc::new(SyncEscalator::new(pool));
        
        let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);
        
        escalator.start(shutdown_rx, Duration::from_millis(10));
        
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        shutdown_tx.send(()).unwrap();
    }
}
