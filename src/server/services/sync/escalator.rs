use std::sync::Arc;
use tokio::time::{self, Duration};
use sqlx::Row;

pub struct SyncEscalator {
    pool: sqlx::PgPool,
    client: reqwest::Client,
}

impl SyncEscalator {
    pub fn new(pool: sqlx::PgPool) -> Self {
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
        let rows = sqlx::query("SELECT id, tenant_id, payload FROM local_mcp_rag_tasks WHERE escalation_status = 'local'")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows {
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let payload: String = row.get("payload");

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
                        sqlx::query("UPDATE local_mcp_rag_tasks SET escalation_status = 'cloud' WHERE id = $1 AND tenant_id = $2")
                            .bind(&id)
                            .bind(&tenant_id)
                            .execute(&self.pool)
                            .await
                            .map_err(|e| e.to_string())?;
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
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            let pool = sqlx::PgPool::connect_lazy(&db_url).unwrap();
            if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
            let escalator = Arc::new(SyncEscalator::new(pool));

            let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);

            escalator.start(shutdown_rx, Duration::from_millis(10));

            tokio::time::sleep(Duration::from_millis(50)).await;

            let _ = shutdown_tx.send(());
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}
