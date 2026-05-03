use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::env;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use super::{Tool, ToolExecutor};

#[async_trait::async_trait]
pub trait BlobManager: Send + Sync {
    async fn write_blob(&self, tenant_id: &str, key: &str, data: &[u8]) -> Result<String, String>;
    async fn read_blob(&self, key: &str) -> Result<Vec<u8>, String>;
}

pub struct HybridBlobManager {
    is_cloud: bool,
    local_dir: Option<PathBuf>,
    cloud_mock_store: dashmap::DashMap<String, Vec<u8>>,
}

impl HybridBlobManager {
    pub async fn new() -> Self {
        let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or_default();
        let is_cloud = !s3_endpoint.is_empty();

        let local_dir = if !is_cloud {
            let tmp_dir = env::var("OHC_BLOB_DIR").unwrap_or_else(|_| "/tmp/ohc_blobs".to_string());
            let path = PathBuf::from(tmp_dir);
            if let Err(e) = fs::create_dir_all(&path).await {
                eprintln!("Failed to create local blob directory: {}", e);
            }
            Some(path)
        } else {
            None
        };

        Self {
            is_cloud,
            local_dir,
            cloud_mock_store: dashmap::DashMap::new(),
        }
    }

    // Test injection constructor
    pub async fn new_for_test(local_dir: PathBuf) -> Self {
        if let Err(e) = fs::create_dir_all(&local_dir).await {
            eprintln!("Failed to create test local blob directory: {}", e);
        }
        Self {
            is_cloud: false,
            local_dir: Some(local_dir),
            cloud_mock_store: dashmap::DashMap::new(),
        }
    }

    // Cloud test injection
    pub fn new_cloud_for_test() -> Self {
        Self {
            is_cloud: true,
            local_dir: None,
            cloud_mock_store: dashmap::DashMap::new(),
        }
    }

    fn sanitize_key(key: &str) -> Result<String, String> {
        if key.contains("..") || key.starts_with('/') {
            return Err("Invalid blob key: path traversal not allowed".to_string());
        }
        Ok(key.to_string())
    }
}

#[async_trait::async_trait]
impl BlobManager for HybridBlobManager {
    async fn write_blob(&self, tenant_id: &str, key: &str, data: &[u8]) -> Result<String, String> {
        let safe_key = Self::sanitize_key(key)?;

        let mut final_data = data.to_vec();
        let mut final_key = safe_key.to_string();

        if key.ends_with(".png") || key.ends_with(".jpg") || key.ends_with(".jpeg") {
            let data_clone = data.to_vec();
            if let Ok(Some(webp_data)) = tokio::task::spawn_blocking(move || {
                if let Ok(img) = image::load_from_memory(&data_clone) {
                    if let Ok(encoder) = webp::Encoder::from_image(&img) {
                        return Some(encoder.encode(80.0).to_vec());
                    }
                }
                None
            }).await {
                final_data = webp_data;
                let p = std::path::Path::new(&safe_key);
                if let Some(stem) = p.file_stem() {
                    if let Some(parent) = p.parent() {
                        let new_path = parent.join(format!("{}.webp", stem.to_string_lossy()));
                        final_key = new_path.to_string_lossy().to_string();
                    } else {
                        final_key = format!("{}.webp", stem.to_string_lossy());
                    }
                }
            }
        }

        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let tier_key = format!("tenant:{}:tier", tenant_id);
                    let tier_str: Option<String> = redis::cmd("GET").arg(&tier_key).query_async(&mut conn).await.unwrap_or(None);

                    let limit_mb = match tier_str.as_deref() {
                        Some("Starter") => 5000i64,
                        Some("Pro") => 50000i64,
                        Some("Business") => i64::MAX,
                        _ => 500i64, // Free tier default
                    };

                    let tenant_key = format!("tenant:{}:storage_used", tenant_id);
                    let current_used: i64 = redis::cmd("GET").arg(&tenant_key).query_async(&mut conn).await.unwrap_or(0);

                    if limit_mb != i64::MAX && current_used + (final_data.len() as i64) > limit_mb * 1024 * 1024 {
                        return Err(format!("Storage quota exceeded: You have reached your storage limit of {} MB.", limit_mb));
                    }
                }
            }
        }

        if self.is_cloud {
            // Mock S3 put object
            println!("Cloud: Writing to S3 key {}", final_key);
            self.cloud_mock_store.insert(final_key.clone(), final_data.to_vec());
        } else {
            let local_dir = self.local_dir.as_ref().ok_or("Local directory not configured")?;
            let path = local_dir.join(&final_key);

            if let Some(parent) = path.parent() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    return Err(format!("Failed to create directories for blob: {}", e));
                }
            }

            let mut file = fs::File::create(&path).await.map_err(|e| format!("Failed to create file: {}", e))?;
            file.write_all(&final_data).await.map_err(|e| format!("Failed to write data: {}", e))?;
        }

        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let tenant_key = format!("tenant:{}:storage_used", tenant_id);
                    let _: Result<i64, _> = redis::cmd("INCRBY").arg(&tenant_key).arg(final_data.len() as i64).query_async(&mut conn).await;
                }
            }
        }

        Ok(final_key)
    }

    async fn read_blob(&self, key: &str) -> Result<Vec<u8>, String> {
        let safe_key = Self::sanitize_key(key)?;

        if self.is_cloud {
            // Mock S3 get object
            println!("Cloud: Reading from S3 key {}", safe_key);
            if let Some(data) = self.cloud_mock_store.get(&safe_key) {
                Ok(data.clone())
            } else {
                Err("Blob not found in cloud mock".to_string())
            }
        } else {
            let local_dir = self.local_dir.as_ref().ok_or("Local directory not configured")?;
            let path = local_dir.join(&safe_key);

            let mut file = fs::File::open(&path).await.map_err(|e| format!("Failed to open file: {}", e))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).await.map_err(|e| format!("Failed to read data: {}", e))?;

            Ok(buffer)
        }
    }
}


struct HybridBlobExecutor {
    manager: Arc<dyn BlobManager>,
    tenant_id: String,
}

#[async_trait::async_trait]
impl ToolExecutor for HybridBlobExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["Action"].as_str().ok_or_else(|| ToolError::LlmRecoverable("hybrid_blob: Action is required".to_string()))?;
        let key = args["Key"].as_str().ok_or_else(|| ToolError::LlmRecoverable("hybrid_blob: Key is required".to_string()))?;

        match action {
            "read" => {
                let data = self.manager.read_blob(key).await.map_err(|e| ToolError::LlmRecoverable(e))?;
                // Attempt to return as UTF-8 string, otherwise base64 encode
                let content = if let Ok(s) = String::from_utf8(data.clone()) {
                    s
                } else {
                    use base64::{Engine as _, engine::general_purpose};
                    general_purpose::STANDARD.encode(&data)
                };

                Ok(json!({
                    "status": "read",
                    "key": key,
                    "data": content
                }).to_string())
            }
            "write" => {
                let data_str = args["Data"].as_str().ok_or_else(|| ToolError::LlmRecoverable("hybrid_blob: Data is required for write".to_string()))?;
                let data_bytes = data_str.as_bytes();

                let new_key = self.manager.write_blob(&self.tenant_id, key, data_bytes).await.map_err(|e| ToolError::LlmRecoverable(e))?;

                Ok(json!({
                    "status": "written",
                    "key": new_key
                }).to_string())
            }
            _ => Err(ToolError::LlmRecoverable("invalid action".to_string())),
        }
    }
}

pub fn hybrid_blob_tool(tenant_id: Option<String>) -> Tool {
    // Need a blocking way to instantiate if we aren't awaiting
    let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or_default();
    let is_cloud = !s3_endpoint.is_empty();

    let local_dir = if !is_cloud {
        let tmp_dir = env::var("OHC_BLOB_DIR").unwrap_or_else(|_| "/tmp/ohc_blobs".to_string());
        let path = PathBuf::from(tmp_dir);
        // We do synchronous dir creation here since this is tool init
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("Failed to create tool local blob directory: {}", e);
        }
        Some(path)
    } else {
        None
    };

    let manager = Arc::new(HybridBlobManager {
        is_cloud,
        local_dir,
        cloud_mock_store: dashmap::DashMap::new(),
    });

    Tool {
        name: "hybrid_blob".to_string(),
        description: "Reads and writes blobs across standalone and cloud environments".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "Action": {
                    "type": "string",
                    "description": "Action to perform: read, write"
                },
                "Key": {
                    "type": "string",
                    "description": "Blob key"
                },
                "Data": {
                    "type": "string",
                    "description": "Data to write (required for write action)"
                }
            },
            "required": ["Action", "Key"]
        }),
        execute: Arc::new(HybridBlobExecutor {
            manager,
            tenant_id: tenant_id.unwrap_or_else(|| "system".to_string()),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_local_blob_write_read() {
        let dir = tempdir().unwrap();
        let manager = HybridBlobManager::new_for_test(dir.path().to_path_buf()).await;

        let key = "test/image.png";
        let data = b"fake png data";

        manager.write_blob("system", key, data).await.unwrap();
        let read_data = manager.read_blob(key).await.unwrap();

        assert_eq!(read_data, data);

        let path = dir.path().join(key);
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_path_traversal_prevention() {
        let dir = tempdir().unwrap();
        let manager = HybridBlobManager::new_for_test(dir.path().to_path_buf()).await;

        let key = "../../../etc/passwd";
        let data = b"hack";

        let res = manager.write_blob("system", key, data).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("path traversal not allowed"));
    }

    #[tokio::test]
    async fn test_cloud_mock() {
        let manager = HybridBlobManager::new_cloud_for_test();

        let key = "org-1/test.txt";
        let data = b"cloud data";

        // Mock should just return ok
        manager.write_blob("system", key, data).await.unwrap();
        let read_data = manager.read_blob(key).await.unwrap();

        assert_eq!(read_data, b""); // Mock read returns empty
    }
}
