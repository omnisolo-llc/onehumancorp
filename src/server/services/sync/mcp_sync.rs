use std::env;
use axum::response::IntoResponse;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SyncDelta {
    pub id: String,
    pub data: String,
    pub updated_at: String,
}

#[async_trait::async_trait]
pub trait SyncDeltas {
    async fn sync_deltas(&self, deltas: Vec<SyncDelta>) -> Result<(), String>;
}

pub struct McpSyncTool {
    pub is_cloud: bool,
    pub pool: sqlx::AnyPool,
    pub cloud_url: Option<String>,
}

impl McpSyncTool {
    pub fn new(is_cloud: bool, pool: sqlx::AnyPool, cloud_url: Option<String>) -> Self {
        McpSyncTool { is_cloud, pool, cloud_url }
    }

    pub async fn read_deltas_from_sqlite(&self) -> Result<Vec<SyncDelta>, String> {
        let rows = sqlx::query_as::<_, (String, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, data, updated_at FROM mcp_deltas_log"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let deltas = rows.into_iter().map(|(id, data, updated_at)| SyncDelta {
            id,
            data,
            updated_at: updated_at.to_rfc3339(),
        }).collect();

        Ok(deltas)
    }

    pub async fn push_deltas_to_cloud(&self, deltas: Vec<SyncDelta>) -> Result<(), String> {
        if let Some(cloud_url) = &self.cloud_url {
            let client = reqwest::Client::new();
            let url = format!("{}/api/v1/sync/mcp-deltas", cloud_url);
            let res = client.post(&url)
                .json(&deltas)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            if !res.status().is_success() {
                return Err(format!("Cloud sync failed with status: {}", res.status()));
            }
            Ok(())
        } else {
            Err("Cloud URL not configured".to_string())
        }
    }
}

#[async_trait::async_trait]
impl SyncDeltas for McpSyncTool {
    async fn sync_deltas(&self, deltas: Vec<SyncDelta>) -> Result<(), String> {
        let telemetry_enabled = env::var("OHC_TELEMETRY_ENABLED").unwrap_or_else(|_| "false".to_string()) == "true";

        if !self.is_cloud && telemetry_enabled {
            tracing::info!("Telemetry: Local MCP tool syncing {} deltas to cloud", deltas.len());
        }

        if !self.is_cloud {
            // Standalone Mode: log to sqlite
            for delta in &deltas {
                let parsed_date = chrono::DateTime::parse_from_rfc3339(&delta.updated_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&chrono::Utc);

                sqlx::query("INSERT INTO mcp_deltas_log (id, data, updated_at) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at")
                    .bind(&delta.id)
                    .bind(&delta.data)
                    .bind(parsed_date)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            // push to cloud as background task
            // In a full implementation, this should be driven by a cron/worker
            let tool_clone = McpSyncTool::new(self.is_cloud, self.pool.clone(), self.cloud_url.clone());
            let deltas_clone = deltas.clone();
            tokio::spawn(async move {
                if let Err(e) = tool_clone.push_deltas_to_cloud(deltas_clone).await {
                     tracing::error!("Failed to push to cloud asynchronously: {}", e);
                }
            });
        } else {
            // Cloud Mode: ingest deltas to postgres
            for delta in deltas {
                let parsed_date = chrono::DateTime::parse_from_rfc3339(&delta.updated_at)
                    .map_err(|e| e.to_string())?
                    .with_timezone(&chrono::Utc);

                sqlx::query("INSERT INTO mcp_deltas (id, data, updated_at) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at WHERE mcp_deltas.updated_at < EXCLUDED.updated_at")
                    .bind(&delta.id)
                    .bind(&delta.data)
                    .bind(parsed_date)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}

pub async fn mcp_deltas_handler(
    axum::extract::State(hub): axum::extract::State<std::sync::Arc<crate::hub::Hub>>,
    axum::Json(payload): axum::Json<Vec<SyncDelta>>
) -> impl axum::response::IntoResponse {
    let pool = hub.pool.clone().into();
    let tool = McpSyncTool::new(true, pool, None);
    match tool.sync_deltas(payload).await {
        Ok(_) => (axum::http::StatusCode::OK, "Synced successfully").into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_sync_tool_sqlite_standalone() {
        sqlx::any::install_default_drivers();
        let pool = sqlx::any::AnyPoolOptions::new().connect("sqlite::memory:").await.unwrap();

        sqlx::query("CREATE TABLE mcp_deltas_log (id TEXT PRIMARY KEY, data TEXT NOT NULL, updated_at TIMESTAMPTZ NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        let tool = McpSyncTool::new(false, pool.clone(), Some("http://mock-cloud".to_string()));
        let deltas = vec![
            SyncDelta {
                id: "1".to_string(),
                data: "test data 1".to_string(),
                updated_at: "2023-10-27T10:00:00Z".to_string(),
            },
        ];

        let result = tool.sync_deltas(deltas).await;
        assert!(result.is_ok());

        // verify it logged to sqlite
        let stored = tool.read_deltas_from_sqlite().await.unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].id, "1");
    }

    #[tokio::test]
    async fn test_mcp_sync_tool_postgres_cloud() {
        sqlx::any::install_default_drivers();
        let pool = sqlx::any::AnyPoolOptions::new().connect("sqlite::memory:").await.unwrap();

        sqlx::query("CREATE TABLE mcp_deltas (id TEXT PRIMARY KEY, data TEXT NOT NULL, updated_at TIMESTAMPTZ NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        let tool = McpSyncTool::new(true, pool.clone(), None);
        let deltas = vec![
            SyncDelta {
                id: "2".to_string(),
                data: "test data 2".to_string(),
                updated_at: "2023-10-27T10:05:00Z".to_string(),
            },
        ];

        let result = tool.sync_deltas(deltas).await;
        assert!(result.is_ok());

        let row: (String,) = sqlx::query_as("SELECT id FROM mcp_deltas").fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, "2");
    }
}
